use std::io;
use std::io::{BufWriter, Write};
use std::fs::File;
use api_parser::*;
use heck::{CamelCase, SnakeCase};
use std::borrow::Cow;
use std::collections::HashMap;
//use std::fmt::Write;

pub struct RustGenerator;

static HEADER: &'static [u8] = b"
use std::cell::Cell;
use std::marker::PhantomData;
use std::mem::transmute;
use std::os::raw::{c_void, c_char};
use std::cell::RefCell;
use std::rc::Rc;
use std::ffi::CString;\n\n";

static RUTE_IMPL_HEADER: &'static [u8] = b"
pub struct Rute<'a> {
    rute_ffi: *const RuteFFI,
    priv_data: *const c_void,
    _marker: PhantomData<std::cell::Cell<&'a ()>>,
}

impl<'a> Rute<'a> {
    pub fn new() -> Rute<'a> {
        Rute {
            rute_ffi: unsafe { rute_get() },
            _marker: PhantomData,
        }
    }
";

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
/// Keeps track of all the type handlers and when to apply them
///
type TypeHandler = HashMap<&'static str, Box<TypeHandlerTrait>>;

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
    fn replace_type(&self, arg: &Variable, is_ret_value: bool) -> Cow<str> {
        match is_ret_value {
            true => "String".into(),
            false => "str".into(),
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
///     _marker: PhantomData<std::cell::Cell<&'a ()>>,
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
    _marker: PhantomData<std::cell::Cell<&'a ()>>,\n}}",
            sdef.name, sdef.name))?;
    }

    Ok(())
}

///
/// Takes in a varibale and generates a Rust function variable argument
///
fn generate_arg_type(dest: &mut String, var: &Variable, return_arg: bool, type_handler: &TypeHandler) {
    dest.clear();

    // Run code to replace type if neeed
    let type_name = {
        match type_handler.get(var.type_name.as_str()) {
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
            dest.push('&');
            dest.push_str(&type_name)
        },
        VariableType::Reference => {
            dest.push('&');
            dest.push_str(&type_name)
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
fn generate_func_impl(func: &Function, type_handler: &TypeHandler) -> String  {
    let mut temp_str = String::with_capacity(128);
	let mut func_imp = String::with_capacity(128);

	func_imp.push_str("(&self");

	for arg in &func.function_args[1..] {
        generate_arg_type(&mut temp_str, &arg, false, type_handler);

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
		func_imp.push_str(" -> &Self<'a>");
	} else {
	    let ret_val = func.return_val.as_ref().unwrap();

        generate_arg_type(&mut temp_str, ret_val, true, type_handler);
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
fn generate_struct_impl<W: Write>(f: &mut W, sdef: &Struct) -> io::Result<()> {
    // Generate all regular functions
    for func in sdef
        .functions
        .iter()
        .filter(|f| f.func_type == FunctionType::Regular) {

    }

    Ok(())
}


///
/// Generates the implementations for the structs
///
fn generate_structs_impl<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
    api_def
        .class_structs
        .iter()
        .try_for_each(|s| {
			if s.should_generate_trait {
        		generate_struct_impl(f, s)
			} else {
        		generate_struct_impl(f, s)
			}
		})
}

///
///
///
fn generate_rute<W: Write>(f: &mut W, api_def: &ApiDef, type_handler: &TypeHandler) -> io::Result<()> {
    // write header
    f.write_all(RUTE_IMPL_HEADER)?;

    // Generate all stucts that doesn't have owned data
    // the generated style is
    //
    // pub fn create_widget(&self) -> Widget<'a> {
    //    let ffi_data = unsafe { ((*self.rute_ffi).create_widget)(self.privd) };
    //    Widget {
    //        data: Rc::new(Cell::new(Some(ffi_data))),
    //        _marker: PhantomData,
    //    }
    //}

    for sdef in api_def
        .class_structs
        .iter()
        .filter(|s| s.should_have_create_func() && !s.should_gen_wrap_class())
    {
        let name = sdef.name.to_snake_case();
        f.write_fmt(format_args!("
    pub fn create_{}(&self) -> {}<'a> {{
        let ffi_data = unsafe {{ ((*self.rute_ffi).create_{})(self.privd) }};
        Widget {{
            data: Rc::new(Cell::new(Some(ffi_data))),
            _marker: PhantomData,
        }}
    }}\n", name, sdef.name, name))?;
    }

    f.write_all(b"}\n")
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
    pub fn generate(filename: &str, api_def: &ApiDef) -> io::Result<()> {
        let mut f = BufWriter::new(File::create(filename)?);

        // Setup type handlers
        let type_handler = setup_type_handlers(api_def);

        // write header
        f.write_all(HEADER)?;

        // write all the structs
        generate_structs(&mut f, api_def)?;

        // Generate the main Rute entry
        generate_rute(&mut f, api_def, &type_handler)?;

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
        let func = build_default_func();
		let func_impl = generate_func_impl(&func, &setup_type_handlers(&ApiDef::default()));
        assert_eq!(func_impl, "(&self) -> &Self<'a>");
	}

	//
	// Test function def with one primitive
	//
	#[test]
	fn test_function_one_primitive() {
        let mut func = build_default_func();
        func.function_args.push(Variable { name: "foo".to_owned(), type_name: "i32".to_owned(), vtype: VariableType::Primitive, .. Variable::default() });

		let func_impl = generate_func_impl(&func, &setup_type_handlers(&ApiDef::default()));
        assert_eq!(func_impl, "(&self, foo: i32) -> &Self<'a>");
	}

	#[test]
	fn test_function_string() {
        let mut func = build_default_func();
        func.function_args.push(Variable { name: "foo".to_owned(), type_name: "String".to_owned(), vtype: VariableType::Regular, .. Variable::default() });

		let func_impl = generate_func_impl(&func, &setup_type_handlers(&ApiDef::default()));
        assert_eq!(func_impl, "(&self, foo: &str) -> &Self<'a>");
	}

	//
	// Test function def with two primitives
	//
	#[test]
	fn test_function_two_primitive() {
        let mut func = build_default_func();
        func.function_args.push(Variable { name: "width".to_owned(), type_name: "i32".to_owned(), vtype: VariableType::Primitive, .. Variable::default() });
        func.function_args.push(Variable { name: "height".to_owned(), type_name: "f32".to_owned(), vtype: VariableType::Primitive, .. Variable::default() });

		let func_impl = generate_func_impl(&func, &setup_type_handlers(&ApiDef::default()));
        assert_eq!(func_impl, "(&self, width: i32, height: f32) -> &Self<'a>");
	}

	//
	// Test function def with self and primitive return
	//
	#[test]
	fn test_function_primitive_return() {
        let mut func = build_default_func();
        func.return_val = Some(Variable { type_name: "i32".to_owned(), vtype: VariableType::Primitive, .. Variable::default() });

		let func_impl = generate_func_impl(&func, &setup_type_handlers(&ApiDef::default()));
        assert_eq!(func_impl, "(&self) -> i32");
	}

	//
	// Test function def with self and optional return
	//
	#[test]
	fn test_function_primitive_optional_return() {
        let mut func = build_default_func();
        func.return_val = Some(Variable { type_name: "f32".to_owned(), vtype: VariableType::Optional, .. Variable::default() });

		let func_impl = generate_func_impl(&func, &setup_type_handlers(&ApiDef::default()));
        assert_eq!(func_impl, "(&self) -> Option<f32>");
	}

	//
	// Test function def with self and array return
	//
	#[test]
	fn test_function_primitive_array_return() {
        let mut func = build_default_func();
        func.return_val = Some(Variable { type_name: "f32".to_owned(), vtype: VariableType::Primitive, array: true, .. Variable::default()});

		let func_impl = generate_func_impl(&func, &setup_type_handlers(&ApiDef::default()));
        assert_eq!(func_impl, "(&self) -> RefArray<f32, WrapperRcOwn>");
	}

	//
	// Test function def with self, array input
	//
	#[test]
	fn test_function_array_input() {
        let mut func = build_default_func();
        func.function_args.push(Variable { name: "width".to_owned(), type_name: "i32".to_owned(), vtype: VariableType::Primitive, array: true, .. Variable::default() });

		let func_impl = generate_func_impl(&func, &setup_type_handlers(&ApiDef::default()));
        assert_eq!(func_impl, "(&self, width: &[i32]) -> &Self<'a>");
	}
}

