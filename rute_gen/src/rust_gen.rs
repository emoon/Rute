use api_parser::*;
use heck::SnakeCase;
use liquid::{Object, ParserBuilder, Template, Value};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

use rust_gen_templates::*;

///
/// Keeps track of all the type handlers and when to apply them
///
struct TypeHandler {
    pub mapping: HashMap<&'static str, Box<TypeHandlerTrait>>,
    // We use a separate type handler for traits as we don't match the types for it as we do a
    // basic string matching on the name instead of register all the types
    pub trait_handler: Box<TypeHandlerTrait>,
    // We also use a separate handler for enums with same motivation as above as we can just check
    // the type instead of matching all enum names
    pub enum_handler: Box<TypeHandlerTrait>,
}

///
/// The Rust generator
///
pub struct RustGenerator {
    rust_func_template: Template,
    rust_no_wrap_template: Template,
    rust_create_template: Template,
    get_static_template: Template,
    callback_template: Template,
    trait_impl_end_template: Template,
    trait_impl_template: Template,
    struct_impl_template: Template,
    drop_template: Template,
    impl_trait_static_template: Template,
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

    fn gen_body(&self, _arg: &Variable, _index: usize) -> (Cow<str>, Cow<str>);
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

    fn gen_body(&self, arg: &Variable, index: usize) -> (Cow<str>, Cow<str>) {
        let arg_name = format!("str_in_{}_{}", arg.name, index);
        (
            format!("{}.as_ptr()", arg_name).into(),
            format!("let {} = CString::new({}).unwrap();\n", arg_name, arg.name).into(),
        )
    }
}

///
/// String type handler
///
#[derive(Clone)]
struct TraitTypeHandler;

///
/// We need to handle strings in a special way. They need to be sent down using CString and the
/// pointer to it so have a generator for it
///
impl TypeHandlerTrait for TraitTypeHandler {
    fn gen_body(&self, arg: &Variable, index: usize) -> (Cow<str>, Cow<str>) {
        let type_name = &arg.type_name[..arg.type_name.len() - 4];
        let arg_name = format!("obj_{}_{}", arg.name, index);
        (
            arg_name.to_owned().into(),
            //format!("{}.as_ptr()", arg_name).into(),
            format!("let ({}, _funcs) = {}.get_{}_obj_funcs();\n", arg_name, arg.name, type_name.to_snake_case()).into(),
        )
    }
}

///
/// Enum type handler
///
#[derive(Clone)]
struct EnumTypeHandler;

///
/// Enums are being handled separately as we need to cast them between Rust and native for C++
///
impl TypeHandlerTrait for EnumTypeHandler {
    fn gen_body_return(&self, varible: &Variable) -> Cow<str> {
        format!("unsafe {{ transmute::<i32, {}>(ret_val) }}", varible.type_name).into()
    }

    fn gen_body(&self, arg: &Variable, index: usize) -> (Cow<str>, Cow<str>) {
        let arg_name = format!("enum_{}_{}", arg.name, index);
        (
            arg_name.to_owned().into(),
            format!("let {} = {} as i32;\n", arg_name, arg.name).into()
        )
    }
}

///
///
///
impl TypeHandler {
    ///
    /// Create a new instance of the handle
    ///
    fn new() -> TypeHandler {
        TypeHandler {
            mapping: HashMap::new(),
            trait_handler: Box::new(TraitTypeHandler {}),
            enum_handler: Box::new(EnumTypeHandler {}),
        }
    }

    ///
    /// Get a handler for a given type. This code will special case if the type ends with "Type"
    /// such as "WidgetType". In that case it will return a TraitHandler as "*Type" is expected to
    /// always be traits. If no handler was found None is returned
    ///
    fn get(&self, arg: &Variable) -> Option<&Box<TypeHandlerTrait>> {
        let type_name = arg.type_name.as_str();

        if arg.vtype == VariableType::Enum {
            Some(&self.enum_handler)
        } else if type_name.ends_with("Type") {
            Some(&self.trait_handler)
        } else {
            self.mapping.get(type_name)
        }
    }
}

///
/// Generate the structs with static only functions. The structs will be generated in this style
///
/// pub struct ApplicationStatic<'a> {
///     data: RUApplication,
///     _marker: PhantomData<::std::cell::Cell<&'a ()>>,
/// }
///
fn generate_static_structs<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
    for sdef in api_def
        .class_structs
        .iter()
        .filter(|s| s.has_static_functions())
    {
        f.write_all(b"#[derive(Clone)]\n")?;
        f.write_fmt(format_args!(
"pub struct {}Static<'a> {{
    pub all_funcs: *const RU{}AllFuncs,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,\n}}\n\n",
            sdef.name, sdef.name
        ))?;
    }

    Ok(())
}

///
/// Setup the type handlers
///

fn setup_type_handlers() -> TypeHandler {
    let mut type_handler = TypeHandler::new();
    type_handler.mapping.insert("String", Box::new(StringTypeHandler {}));
    type_handler
}

impl RustGenerator {
    pub fn new() -> RustGenerator {
        let parser = ParserBuilder::with_liquid().build();

        RustGenerator {
            type_handler: setup_type_handlers(),
            rust_func_template: parser.parse(RUST_FUNC_IMPL_TEMPLATE).unwrap(),
            rust_create_template: parser.parse(RUST_CREATE_TEMPLATE).unwrap(),
            rust_no_wrap_template: parser.parse(RUST_NO_WRAP_TEMPLATE).unwrap(),
            get_static_template: parser.parse(RUST_GET_STATIC_TEMPLATE).unwrap(),
            drop_template: parser.parse(RUST_DROP_TEMPLATE).unwrap(),
            callback_template: parser.parse(RUST_CALLBACK_TEMPLATE).unwrap(),
            trait_impl_template: parser.parse(RUST_IMPL_TRAIT_TEMPLATE).unwrap(),
            struct_impl_template: parser.parse(RUST_STRUCT_IMPL_TEMPLATE).unwrap(),
            trait_impl_end_template: parser.parse(RUST_IMPL_TRAIT_END_TEMPLATE).unwrap(),
            impl_trait_static_template: parser.parse(RUST_IMPL_TRAIT_STATIC_TEMPLATE).unwrap(),
        }
    }

    ///
    /// Takes in a varibale and generates a Rust function variable argument
    ///
    fn generate_arg_type(&self, dest: &mut String, var: &Variable, return_arg: bool) {
        dest.clear();

        // Run code to replace type if neeed
        let type_name = {
            match self.type_handler.get(var) {
                Some(handler) => handler.replace_type(var, return_arg),
                None => var.type_name.clone().into(),
            }
        };

        if var.optional {
            dest.push_str("Option<");
        }

        match var.vtype {
            VariableType::None => dest.push_str("<None>"),
            VariableType::SelfType => dest.push_str("&self"),
            VariableType::Primitive => dest.push_str(&type_name),
            VariableType::Enum => dest.push_str(&var.enum_sub_type),
            VariableType::Regular => dest.push_str(&type_name),
            VariableType::Str =>  {
                if return_arg {
                    dest.push_str("String");
                } else {
                    dest.push_str("&str");
                }
            },
            VariableType::Reference => {
                dest.push('&');
                dest.push_str(&type_name);
                //dest.push_str("<'a>");
            }
        }

        if var.optional {
            dest.push('>');
        }
    }

    ///
    /// Generate a function implementation
    ///
    fn generate_func_def(&self, func: &Function, _struct_name: &str) -> String {
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
            func_imp.push_str(" -> &Self");
            //func_imp.push_str(" -> &");
            //func_imp.push_str(struct_name);
            //func_imp.push_str("<'a>");
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
        if sdef.has_event_replace_functions() {
            f.write_fmt(format_args!("impl<'a> {}<'a> {{", sdef.name))?;

            for func in &sdef.functions {
                let res = match func.func_type {
                    //FunctionType::Regular => self.generate_function(&func, &sdef.name),
                    //FunctionType::Static => self.generate_function(&func, &sdef.name),
                    FunctionType::Replace => self.generate_callback(&func, &sdef.name),
                    FunctionType::Event => self.generate_callback(&func, &sdef.name),
                    _ => String::new(),
                };

                if !res.is_empty() {
                    f.write_all(res.as_bytes())?;
                }
            }

            f.write_all(b"}\n")?;
        }

        self.generate_trait_impl(f, &sdef.name, FunctionType::Regular, &sdef)?;

        for name in &sdef.full_inherit {
            let mut template_data = Object::new();
            template_data.insert("trait_name".to_owned(), Value::str(&name));
            template_data.insert("target_name".to_owned(), Value::str(&sdef.name));
            template_data.insert("type_name".to_owned(), Value::str(&name));
            template_data.insert("target_name_snake".to_owned(), Value::Str(name.to_snake_case()));
            template_data.insert("target_name_snake_org".to_owned(), Value::Str(name.to_snake_case()));

            let out = self.trait_impl_template.render(&template_data).unwrap();
            f.write_all(out.as_bytes())?;
        }

        if sdef.has_static_functions() {
            let static_name = &format!("{}Static", sdef.name);
            self.generate_trait_impl(f, static_name, FunctionType::Static, &sdef)?;

            // implement the static trait for the regular one as well

            let mut template_data = Object::new();
            template_data.insert("trait_name".to_owned(), Value::str(&static_name));
            template_data.insert("target_name".to_owned(), Value::str(&sdef.name));
            template_data.insert("type_name".to_owned(), Value::str(&sdef.name));
            template_data.insert("target_name_snake".to_owned(), Value::Str(static_name.to_snake_case()));
            template_data.insert("target_name_snake_org".to_owned(), Value::Str(sdef.name.to_snake_case()));

            let out = self.trait_impl_template.render(&template_data).unwrap();
            f.write_all(out.as_bytes())?;

            // Implement the static trait for the static type

            let out = self.impl_trait_static_template.render(&template_data).unwrap();
            f.write_all(out.as_bytes())?;
        }

        Ok(())
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

        let len = func.function_args[1..].len().wrapping_sub(1);

        for (index, arg) in func.function_args[1..].iter().enumerate() {
            self.generate_arg_type(&mut temp_str, &arg, false);

            function_arguments.push_str(&arg.name);
            function_arguments.push_str(": ");
            function_arguments.push_str(&arg.type_name);

            function_params.push_str(&arg.name);

            function_arg_types.push_str(&arg.type_name);

            if index != len {
                function_arguments.push_str(", ");
                function_params.push_str(", ");
                function_arg_types.push_str(", ");
            }
        }

        template_data.insert("event_name".to_owned(), Value::Str(func.name.clone()));
        template_data.insert(
            "function_arguments".to_owned(),
            Value::Str(function_arguments),
        );
        template_data.insert("function_params".to_owned(), Value::Str(function_params));
        template_data.insert(
            "function_arg_types".to_owned(),
            Value::Str(function_arg_types),
        );
        template_data.insert("widget_name".to_owned(), Value::str(struct_name));
        template_data.insert(
            "widget_snake_name".to_owned(),
            Value::Str(struct_name.to_snake_case()),
        );

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
        template_data.insert(
            "obj_funcs_name".to_owned(),
            Value::Str(struct_name.to_snake_case()),
        );

        let mut function_args = Vec::with_capacity(func.function_args.len());
        let mut body_setup = String::with_capacity(4096);

        // Setup the input args
        for (i, arg) in func.function_args.iter().enumerate() {
            match self.type_handler.get(&arg) {
                Some(handler) => {
                    let (gen_arg, body) = handler.gen_body(&arg, i);
                    function_args.push((true, gen_arg));
                    body_setup.push_str("        ");
                    body_setup += &body;
                }
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

        template_data.insert(
            "return_value".to_owned(),
            Value::Bool(func.return_val.is_some()),
        );
        template_data.insert("function_args".to_owned(), Value::Str(func_args));

        func.return_val.as_ref().map(|ret_val| {
            template_data.insert("return_type".to_owned(), Value::str("none"));
            template_data.insert("optional_return".to_owned(), Value::Bool(ret_val.optional));

            match self.type_handler.get(&ret_val) {
                Some(handler) => {
                    let ret = handler.gen_body_return(&ret_val);
                    template_data.insert("return_type".to_owned(), Value::str("replaced"));
                    template_data.insert("replaced_return".to_owned(), Value::Str(ret.into()));
                }
                None => {
                    match ret_val.vtype {
                        VariableType::Regular => {
                            template_data.insert("return_type".to_owned(), Value::str("pointer_ref"));
                            template_data.insert(
                                "return_vtype".to_owned(),
                                Value::Str(ret_val.type_name.to_owned()),
                            );
                        }

                        VariableType::Reference => {
                            template_data.insert("return_type".to_owned(), Value::str("pointer_ref"));
                            template_data.insert(
                                "return_vtype".to_owned(),
                                Value::Str(ret_val.type_name.to_owned()),
                            );
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

        self.rust_func_template.render(&template_data).unwrap()
    }

    ///
    /// Generate the structs. The structs will be generated in this style
    ///
    /// pub struct Application<'a> {
    ///     data: Rc<Cell<Option<RUApplication>>>,
    ///     _marker: PhantomData<::std::cell::Cell<&'a ()>>,
    /// }
    ///
    fn generate_structs<W: Write>(&self, dest: &mut W, api_def: &ApiDef) -> io::Result<()> {
        for sdef in &api_def.class_structs {
            let mut template_data = Object::new();
            template_data.insert("struct_name".to_owned(), Value::str(&sdef.name));

            let output = self.struct_impl_template.render(&template_data).unwrap();
            dest.write_all(output.as_bytes())?;
        }

        Ok(())
    }

    ///
    /// Generates the implementations for the structs
    ///
    fn generate_structs_impl<W: Write>(&self, f: &mut W, api_def: &ApiDef) -> io::Result<()> {
        api_def.class_structs.iter().try_for_each(|s| {
            self.generate_struct_impl(f, s)?;

            // Implement drop for structs that needs it

            if s.should_generate_drop() {
                let mut template_data = Object::new();

                template_data.insert("type_name".to_owned(), Value::Str(s.name.clone()));
                template_data.insert(
                    "type_snake_name".to_owned(),
                    Value::Str(s.name.to_snake_case()),
                );

                let output = self.drop_template.render(&template_data).unwrap();

                f.write_all(output.as_bytes())?;
            }

            Ok(())
        })
    }
    ///
    /// Generate trait implementation
    ///
    fn generate_trait_impl<W: Write>(
        &self,
        dest: &mut W,
        name: &str,
        func_type: FunctionType,
        sdef: &Struct,
    ) -> io::Result<()> {
        let struct_name = format!("{}Type", name);
        writeln!(dest, "pub trait {} {{", struct_name)?;

        sdef.functions
            .iter()
            .filter(|f| f.func_type == func_type)
            .for_each(|f| {

            let res = self.generate_function(&f, name);
            dest.write_all(res.as_bytes()).unwrap();
        });

        let mut template_data = Object::new();
        template_data.insert("type_name".to_owned(), Value::str(&sdef.name));
        template_data.insert("type_name_snake".to_owned(), Value::Str(name.to_snake_case()));

        let out = self.trait_impl_end_template.render(&template_data).unwrap();
        dest.write_all(out.as_bytes())
    }

    ///
    ///
    ///
    pub fn generate_rute(&self, filename: &str, api_defs: &[ApiDef]) -> io::Result<()> {
        let mut f = BufWriter::new(File::create(filename)?);

        // write header
        f.write_all(HEADER)?;

        // impl header
        f.write_all(RUTE_IMPL_HEADER)?;

        // Generate all stucts that have owned data the generated style is outlined in RUST_CREATE_TEMPLATE

        for sdef in api_defs
            .iter()
            .flat_map(|s| s.class_structs.iter())
            .filter(|s| s.should_have_create_func())
        {
            let mut template_data = Object::new();
            let name = sdef.name.to_snake_case();

            template_data.insert("widget_snake_name".to_owned(), Value::Str(name.clone()));
            template_data.insert("widget_name".to_owned(), Value::Str(sdef.name.clone()));

            let output = if sdef.should_gen_wrap_class() {
                self.rust_create_template.render(&template_data).unwrap()
            } else {
                self.rust_no_wrap_template.render(&template_data).unwrap()
            };

            f.write_all(output.as_bytes())?;

            if sdef.has_static_functions() {
                let out = self.get_static_template.render(&template_data).unwrap();
                f.write_all(out.as_bytes())?;
            }
        }

        f.write_all(b"}")
    }
    ///
    /// Generates the Rust mod file to import all FFI and Rust generated code
    ///
    pub fn generate_auto_mod(filename: &str, api_defs: &[ApiDef]) -> io::Result<()> {
        let mut dest = BufWriter::new(File::create(filename).unwrap());

        writeln!(dest, "// This file is auto-generated by rute_gen. DO NOT EDIT.\n")?;

        // Collect all then struct names (snake case) and get a sorted list in the end
        let mut names =
            api_defs
            .iter()
                .flat_map(|api_def| api_def.class_structs.iter())
                .map(|s| s.name.to_snake_case())
                .collect::<Vec<String>>();

        // rute auto has some extra stuff used by all the files
        //names.push("rute_auto".to_owned());
        names.sort();

        for entry in &names {
            writeln!(dest, "pub mod {}_ffi;", entry)?;
            writeln!(dest, "pub mod {};", entry)?;
        }

        writeln!(dest, "pub mod rute_ffi;")?;
        writeln!(dest, "pub mod rute;\n")?;

        for entry in &names {
            // TODO: Fixme
            writeln!(dest, "pub use {}::*;", entry)?;
            writeln!(dest, "pub use {}_ffi::*;", entry)?;
        }

        writeln!(dest, "pub use rute_ffi::*;")?;
        writeln!(dest, "pub use rute::*;")
    }

    pub fn generate(&self, filename: &str, api_def: &ApiDef) -> io::Result<()> {
        let mut f = BufWriter::new(File::create(filename)?);

        // write header
        f.write_all(HEADER)?;

        // As we may need types/enums/etc from other types we need to generate that
        //self.generate_mod_usage(&mut f, api_def);

        // write all the structs
        self.generate_structs(&mut f, api_def)?;

        // write all the structs with static functions
        generate_static_structs(&mut f, api_def)?;

        // Generate the implementations for the structs
        self.generate_structs_impl(&mut f, api_def)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use api_parser::*;


    //
    // Create a default function to reduce duplication a bit
    //
    fn build_default_func() -> Function {
        Function {
            name: "test".to_owned(),
            function_args: vec![Variable {
                name: "self".to_owned(),
                ..Variable::default()
            }],
            ..Function::default()
        }
    }

    //
    // Test function def with self only
    //
    #[test]
    fn test_function_self_only() {
        let rust_gen = RustGenerator::new();
        let func = build_default_func();
        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl, "(&self) -> &Self");
    }

    //
    // Test function def with one primitive
    //
    #[test]
    fn test_function_one_primitive() {
        let rust_gen = RustGenerator::new();
        let mut func = build_default_func();
        func.function_args.push(Variable {
            name: "foo".to_owned(),
            type_name: "i32".to_owned(),
            vtype: VariableType::Primitive,
            ..Variable::default()
        });

        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl, "(&self, foo: i32) -> &Self");
    }

    #[test]
    fn test_function_string() {
        let rust_gen = RustGenerator::new();
        let mut func = build_default_func();
        func.function_args.push(Variable {
            name: "foo".to_owned(),
            type_name: "String".to_owned(),
            vtype: VariableType::Regular,
            ..Variable::default()
        });

        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl, "(&self, foo: &str) -> &Self");
    }

    //
    // Test function def with two primitives
    //
    #[test]
    fn test_function_two_primitive() {
        let mut func = build_default_func();
        let rust_gen = RustGenerator::new();

        func.function_args.push(Variable {
            name: "width".to_owned(),
            type_name: "i32".to_owned(),
            vtype: VariableType::Primitive,
            ..Variable::default()
        });
        func.function_args.push(Variable {
            name: "height".to_owned(),
            type_name: "f32".to_owned(),
            vtype: VariableType::Primitive,
            ..Variable::default()
        });

        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(
            func_impl,
            "(&self, width: i32, height: f32) -> &Self"
        );
    }

    //
    // Test function def with self and primitive return
    //
    #[test]
    fn test_function_primitive_return() {
        let mut func = build_default_func();
        let rust_gen = RustGenerator::new();

        func.return_val = Some(Variable {
            type_name: "i32".to_owned(),
            vtype: VariableType::Primitive,
            ..Variable::default()
        });

        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl, "(&self) -> i32");
    }

    //
    // Test function def with self and optional return
    //
    #[test]
    fn test_function_primitive_optional_return() {
        let mut func = build_default_func();
        let rust_gen = RustGenerator::new();

        func.return_val = Some(Variable {
            type_name: "f32".to_owned(),
            vtype: VariableType::Primitive,
            optional: true,
            ..Variable::default()
        });

        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl, "(&self) -> Option<f32>");
    }

    //
    // Test function def with self and array return
    //
    #[test]
    fn test_function_primitive_array_return() {
        let mut func = build_default_func();
        let rust_gen = RustGenerator::new();

        func.return_val = Some(Variable {
            type_name: "f32".to_owned(),
            vtype: VariableType::Primitive,
            array: true,
            ..Variable::default()
        });

        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl, "(&self) -> RefArray<f32, WrapperRcOwn>");
    }

    //
    // Test function def with self, array input
    //
    #[test]
    fn test_function_array_input() {
        let mut func = build_default_func();
        let rust_gen = RustGenerator::new();

        func.function_args.push(Variable {
            name: "width".to_owned(),
            type_name: "i32".to_owned(),
            vtype: VariableType::Primitive,
            array: true,
            ..Variable::default()
        });

        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl, "(&self, width: &[i32]) -> &Self");
    }
}
