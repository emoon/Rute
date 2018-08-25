use std::io;
use std::io::{BufWriter, Write};
use std::fs::File;
use api_parser::*;
use heck::{SnakeCase};
use std::borrow::Cow;
use std::collections::HashMap;
use liquid::{Template, ParserBuilder, Object, Value};

use rust_gen_templates::*;

///
/// Keeps track of all the type handlers and when to apply them
///
type TypeHandler = HashMap<&'static str, Box<TypeHandlerTrait>>;

pub struct RustGenerator {
    rust_func_template: Template,
    rust_no_wrap_template: Template,
    rust_create_template: Template,
    callback_template: Template,
    drop_template: Template,
    type_handler: TypeHandler,
}

///
/// Trait for handling different types that needs to be overriden
///
trait TypeHandlerTrait {
    fn replace_type(&self, arg: &Variable, _is_return_value: bool) -> Cow<str> {
        arg.type_name.clone().into()
    }

    fn gen_body_return(&self, _varible: &Variable) -> Cow<str> {
        "".into()
    }

    fn gen_body(&self, _arg: &str, _index: usize) -> (Cow<str>, Cow<str>);
}


///
/// String type handler
///
#[derive(Clone)]
struct StringTypeHandler;

///
/// We need to handle strings in a special way. They need to be sent down using CString and the
/// pointer to it so have a generator for it
///
impl TypeHandlerTrait for StringTypeHandler {
    fn replace_type(&self, _arg: &Variable, is_ret_value: bool) -> Cow<str> {
        match is_ret_value {
            true => "String".into(),
            false => "&str".into(),
        }
    }

    fn gen_body_return(&self, _varible: &Variable) -> Cow<str> {
        "CStr::from_ptr(ret_val).to_string_lossy().into_owned()".into()
    }

    fn gen_body(&self, arg: &str, index: usize) -> (Cow<str>, Cow<str>) {
        let arg_name = format!("str_in_{}_{}", arg, index);
        (format!("{}.as_ptr()", arg_name).into(),
         format!("let {} = CString::new({}).unwrap();\n", arg_name, arg).into())
    }
}


///
/// Generate the structs. The structs will be generated in this style
///
/// pub struct Application<'a> {
///     data: Rc<Cell<Option<RUApplication>>>,
///     _marker: PhantomData<::std::cell::Cell<&'a ()>>,
/// }
///
fn generate_structs<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
    // generate the pod structs which re-uses the FFI structs
    for sdef in &api_def.pod_structs {
        f.write_fmt(format_args!(
            "pub use ffi_gen::RU{} as {};\n",
            sdef.name, sdef.name
        ))?;
    }

    for sdef in &api_def.class_structs {
        f.write_all(b"#[derive(Clone)]\n")?;
        f.write_fmt(format_args!("pub struct {}<'a> {{
    data: Rc<Cell<Option<RU{}>>>,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,\n}}\n\n",
            sdef.name, sdef.name))?;
    }

    Ok(())
}

///
/// Setup the type handlers
///

fn setup_type_handlers(_api_def: &ApiDef) -> TypeHandler {
    let mut type_handler = TypeHandler::new();
    type_handler.insert("String", Box::new(StringTypeHandler {}));
    type_handler
}

impl RustGenerator {
    pub fn new(api_def: &ApiDef) -> RustGenerator {
        let parser = ParserBuilder::with_liquid().build();

        RustGenerator {
            type_handler: setup_type_handlers(api_def),
            rust_func_template: parser.parse(RUST_FUNC_IMPL_TEMPLATE).unwrap(),
            rust_create_template: parser.parse(RUST_CREATE_TEMPLATE).unwrap(),
            rust_no_wrap_template: parser.parse(RUST_NO_WRAP_TEMPLATE).unwrap(),
            drop_template: parser.parse(RUST_DROP_TEMPLATE).unwrap(),
            callback_template: parser.parse(RUST_CALLBACK_TEMPLATE).unwrap(),
        }
    }

    ///
    /// Takes in a varibale and generates a Rust function variable argument
    ///
    fn generate_arg_type(&self, dest: &mut String, var: &Variable, return_arg: bool) {
        dest.clear();

        // Run code to replace type if neeed
        let type_name = {
            match self.type_handler.get(var.type_name.as_str()) {
                Some(handler) => handler.replace_type(var, return_arg),
                None => var.type_name.clone().into(),
            }
        };

        match var.vtype {
            VariableType::None => dest.push_str("<None>"),
            VariableType::SelfType => dest.push_str("&self"),
            VariableType::Primitive => dest.push_str(&type_name),
            VariableType::Enum => dest.push_str(&type_name),
            VariableType::Regular => {
                dest.push_str(&type_name)
            },
            VariableType::Reference => {
                dest.push('&');
                dest.push_str(&type_name);
                dest.push_str("<'a>");
            },
            VariableType::Optional => {
                dest.push_str("Option<");
                dest.push_str(&type_name);
                dest.push('>');
            }
        }
    }

    ///
    /// Generate a function implementation
    ///
    fn generate_func_def(&self, func: &Function, struct_name: &str) -> String  {
        let mut temp_str = String::with_capacity(128);
        let mut func_imp = String::with_capacity(128);

        func_imp.push_str("(&self");

        for arg in &func.function_args[1..] {
            self.generate_arg_type(&mut temp_str, &arg, false);

            func_imp.push_str(", ");
            func_imp.push_str(&arg.name);
            func_imp.push_str(": ");

            if arg.array {
                func_imp.push_str("&[");
                func_imp.push_str(&temp_str);
                func_imp.push(']');
            } else {
                func_imp.push_str(&temp_str);
            }
        }

        func_imp.push(')');

        //
        // If we don't have any return value we alwayes return self
        //
        if func.return_val.is_none() {
            func_imp.push_str(" -> &");
            func_imp.push_str(struct_name);
            func_imp.push_str("<'a>");
        } else {
            let ret_val = func.return_val.as_ref().unwrap();

            self.generate_arg_type(&mut temp_str, ret_val, true);
            func_imp.push_str(" -> ");

            if ret_val.array {
                func_imp.push_str("RefArray<");
                func_imp.push_str(&temp_str);
                func_imp.push_str(", WrapperRcOwn>");
            } else {
                func_imp.push_str(&temp_str);
            }
        }

        func_imp
    }

    ///
    /// Generates the implementations for the structs
    ///
    fn generate_struct_impl<W: Write>(&self, f: &mut W, sdef: &Struct) -> io::Result<()> {
        f.write_fmt(format_args!("impl<'a> {}<'a> {{", sdef.name))?;

        // Generate all regular functions

        for func in &sdef.functions {
            let res = match func.func_type {
                FunctionType::Regular => self.generate_function(&func, &sdef.name),
                FunctionType::Static => self.generate_function(&func, &sdef.name),
                FunctionType::Callback => self.generate_callback(&func, &sdef.name),
                FunctionType::Event => self.generate_callback(&func, &sdef.name),
            };

            f.write_all(res.as_bytes())?;
        }

        // Generate the functions
        f.write_all(Self::generate_get_obj_funcs(&sdef.name).as_bytes())?;

        f.write_all(b"}\n")
    }

    ///
    /// Generate something like this
    ///
    /// fn get_widget_obj_funcs(&self) -> (*const RUBase, *const RUWidgetFuncs) {
    ///     let obj = self.data.get().unwrap();
    ///     (obj.privd, obj.widget_funcs)
    /// }

    fn generate_get_obj_funcs(struct_name: &str) -> String {
        let snake_name = struct_name.to_snake_case();
        format!("    fn get_{}_obj_funcs(&self) -> (*const RUBase, *const RU{}Funcs) {{
        let obj = self.data.get().unwrap();
        (obj.privd, obj.{}_funcs)\n    }}\n", snake_name, struct_name, snake_name)
    }

    //
    //
    //
    fn generate_callback(&self, func: &Function, struct_name: &str) -> String {
        let mut template_data = Object::new();
        let mut function_arguments = String::with_capacity(128);
        let mut function_params = String::with_capacity(128);
        let mut function_arg_types = String::with_capacity(128);

        let mut temp_str = String::with_capacity(128);

        for arg in &func.function_args[1..] {
            self.generate_arg_type(&mut temp_str, &arg, false);

            function_arguments.push_str(", ");
            function_arguments.push_str(&arg.name);
            function_arguments.push_str(": ");
            function_arguments.push_str(&arg.type_name);

            function_params.push_str(", ");
            function_params.push_str(&arg.name);

            function_arg_types.push_str(", ");
            function_arg_types.push_str(&arg.type_name);
        }

        template_data.insert("event_name".to_owned(), Value::Str(func.name.clone()));
        template_data.insert("function_arguments".to_owned(), Value::Str(function_arguments));
        template_data.insert("function_params".to_owned(), Value::Str(function_params));
        template_data.insert("function_arg_types".to_owned(), Value::Str(function_arg_types));
        template_data.insert("widget_name".to_owned(), Value::str(struct_name));
        template_data.insert("widget_snake_name".to_owned(), Value::Str(struct_name.to_snake_case()));

        self.callback_template.render(&template_data).unwrap()
    }

    //
    // Do the actual function generation
    //
    fn generate_function(&self, func: &Function, struct_name: &str) -> String {
        let mut template_data = Object::new();

        // Generate function declaration
        let func_def = self.generate_func_def(func, struct_name);

        template_data.insert("func_name".to_owned(), Value::Str(func.name.clone()));
        template_data.insert("function_def".to_owned(), Value::Str(func_def));
        template_data.insert("obj_funcs_name".to_owned(), Value::Str(struct_name.to_snake_case()));

        let mut function_args = Vec::with_capacity(func.function_args.len());
        let mut body_setup = String::with_capacity(4096);

        // Setup the input args
        for (i, arg) in func.function_args.iter().enumerate() {
            match self.type_handler.get(arg.type_name.as_str()) {
                Some(handler) => {
                    let (gen_arg, body) = handler.gen_body(&arg.name, i);
                    function_args.push((true, gen_arg));
                    body_setup += &body;
                },
                None => {
                    function_args.push((false, arg.name.clone().into()));
                }
            }
        }

        if body_setup.is_empty() {
            template_data.insert("body_setup".to_owned(), Value::Nil);
        } else {
            template_data.insert("body_setup".to_owned(), Value::Str(body_setup));
        }

        let mut func_args = String::with_capacity(128);

        // Generate the function call
        for (index, arg) in func.function_args.iter().enumerate() {
            match arg.vtype {
                VariableType::SelfType => func_args.push_str("obj_data"),
                _ => {
                    func_args.push_str(", ");
                    func_args.push_str(&function_args[index].1);
                }
            }
        }

        //
        // Setup the return part of the template
        //

        template_data.insert("return_value".to_owned(), Value::Bool(func.return_val.is_some()));
        template_data.insert("function_args".to_owned(), Value::Str(func_args));

        func.return_val.as_ref().map(|ret_val| {
            template_data.insert("return_type".to_owned(), Value::str("none"));

            match self.type_handler.get(ret_val.type_name.as_str()) {
                Some(handler) => {
                    let ret = handler.gen_body_return(&ret_val);
                    template_data.insert("return_type".to_owned(), Value::str("replaced"));
                    template_data.insert("replaced_return".to_owned(), Value::Str(ret.into()));
                },
                None => {
                    match ret_val.vtype {
                        /*
                        VariableType::Optional(ref vtype) => {
                            template_data.insert("rust_return_type".to_owned(), Value::Str(vtype.clone()));
                            template_data.insert("return_type".to_owned(), Value::str("optional"));
                        },
                        */
                        VariableType::Regular => {
                            template_data.insert("return_type".to_owned(), Value::str("no_wrap"));
                            template_data.insert("return_vtype".to_owned(), Value::Str(ret_val.type_name.to_owned()));
                        }
                        /*
                        VariableType::Array(ref vtype) => {
                            template_data.insert("rust_return_type".to_owned(), Value::Str(vtype.clone()));
                            template_data.insert("return_type".to_owned(), Value::str("array"));
                        },
                        */
                        _ => (),
                    }
                }
            }
        });

        let output = self.rust_func_template.render(&template_data).unwrap();

        output
    }

    ///
    /// Generates the implementations for the structs
    ///
    fn generate_structs_impl<W: Write>(&self, f: &mut W, api_def: &ApiDef) -> io::Result<()> {
        api_def
            .class_structs
            .iter()
            .try_for_each(|s| {
                if s.should_generate_trait {
                    self.generate_struct_impl(f, s)?;
                } else {
                    self.generate_struct_impl(f, s)?;
                }

                // Implement drop for structs that needs it

                if s.should_generate_drop() {
                    let mut template_data = Object::new();

                    template_data.insert("type_name".to_owned(), Value::Str(s.name.clone()));
                    template_data.insert("type_snake_name".to_owned(), Value::Str(s.name.to_snake_case()));

                    let output = self.drop_template.render(&template_data).unwrap();

                    f.write_all(output.as_bytes())?;
                }

                Ok(())
            })
    }

    ///
    ///
    ///
    fn generate_rute<W: Write>(&self, f: &mut W, api_def: &ApiDef) -> io::Result<()> {
        // write header
        f.write_all(RUTE_IMPL_HEADER)?;

        // Generate all stucts that have owned data the generated style is outlined in RUST_CREATE_TEMPLATE

        for sdef in api_def
            .class_structs
            .iter()
            .filter(|s| s.should_have_create_func())
        {
            let mut template_data = Object::new();
            let name = sdef.name.to_snake_case();

            template_data.insert("widget_snake_name".to_owned(), Value::Str(name.clone()));
            template_data.insert("widget_name".to_owned(), Value::Str(sdef.name.clone()));

            let output;

            if sdef.should_gen_wrap_class() {
                output = self.rust_create_template.render(&template_data).unwrap();
            } else {
                output = self.rust_no_wrap_template.render(&template_data).unwrap();
            }

            f.write_all(output.as_bytes())?;
        }

        f.write_all(b"}\n")
    }

    pub fn generate(&self, filename: &str, api_def: &ApiDef) -> io::Result<()> {
        let mut f = BufWriter::new(File::create(filename)?);

        // write header
        f.write_all(HEADER)?;

        // write all the structs
        generate_structs(&mut f, api_def)?;

        // Generate the implementations for the structs
        self.generate_structs_impl(&mut f, api_def)?;

        // Generate the main Rute entry
        self.generate_rute(&mut f, api_def)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //
    // Create a default function to reduce duplication a bit
    //
    fn build_default_func() -> Function {
        Function {
            name: "test".to_owned(),
            function_args: vec![Variable {
                name: "self".to_owned(),
                .. Variable::default()
            }],
            ..Function::default()
        }
    }

	//
	// Test function def with self only
	//
	#[test]
	fn test_function_self_only() {
	    let rust_gen = RustGenerator::new(&ApiDef::default());
        let func = build_default_func();
		let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl, "(&self) -> &TestStruct<'a>");
	}

	//
	// Test function def with one primitive
	//
	#[test]
	fn test_function_one_primitive() {
	    let rust_gen = RustGenerator::new(&ApiDef::default());
        let mut func = build_default_func();
        func.function_args.push(Variable { name: "foo".to_owned(), type_name: "i32".to_owned(), vtype: VariableType::Primitive, .. Variable::default() });

		let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl, "(&self, foo: i32) -> &TestStruct<'a>");
	}

	#[test]
	fn test_function_string() {
	    let rust_gen = RustGenerator::new(&ApiDef::default());
        let mut func = build_default_func();
        func.function_args.push(Variable { name: "foo".to_owned(), type_name: "String".to_owned(), vtype: VariableType::Regular, .. Variable::default() });

		let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl, "(&self, foo: &str) -> &TestStruct<'a>");
	}

	//
	// Test function def with two primitives
	//
	#[test]
	fn test_function_two_primitive() {
        let mut func = build_default_func();
	    let rust_gen = RustGenerator::new(&ApiDef::default());

        func.function_args.push(Variable { name: "width".to_owned(), type_name: "i32".to_owned(), vtype: VariableType::Primitive, .. Variable::default() });
        func.function_args.push(Variable { name: "height".to_owned(), type_name: "f32".to_owned(), vtype: VariableType::Primitive, .. Variable::default() });

		let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl, "(&self, width: i32, height: f32) -> &TestStruct<'a>");
	}

	//
	// Test function def with self and primitive return
	//
	#[test]
	fn test_function_primitive_return() {
        let mut func = build_default_func();
	    let rust_gen = RustGenerator::new(&ApiDef::default());

        func.return_val = Some(Variable { type_name: "i32".to_owned(), vtype: VariableType::Primitive, .. Variable::default() });

		let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl, "(&self) -> i32");
	}

	//
	// Test function def with self and optional return
	//
	#[test]
	fn test_function_primitive_optional_return() {
        let mut func = build_default_func();
	    let rust_gen = RustGenerator::new(&ApiDef::default());

        func.return_val = Some(Variable { type_name: "f32".to_owned(), vtype: VariableType::Optional, .. Variable::default() });

		let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl, "(&self) -> Option<f32>");
	}

	//
	// Test function def with self and array return
	//
	#[test]
	fn test_function_primitive_array_return() {
        let mut func = build_default_func();
	    let rust_gen = RustGenerator::new(&ApiDef::default());

        func.return_val = Some(Variable { type_name: "f32".to_owned(), vtype: VariableType::Primitive, array: true, .. Variable::default()});

		let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl, "(&self) -> RefArray<f32, WrapperRcOwn>");
	}

	//
	// Test function def with self, array input
	//
	#[test]
	fn test_function_array_input() {
        let mut func = build_default_func();
	    let rust_gen = RustGenerator::new(&ApiDef::default());

        func.function_args.push(Variable { name: "width".to_owned(), type_name: "i32".to_owned(), vtype: VariableType::Primitive, array: true, .. Variable::default() });

		let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl, "(&self, width: &[i32]) -> &TestStruct<'a>");
	}
}

