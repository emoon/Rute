use std::io;
use std::io::{BufWriter, Write};
use std::fs::File;
use api_parser::*;
use heck::{CamelCase, SnakeCase};

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
/// Generate a function implementation
///
fn generate_func_impl(func: &Function) -> String  {
	let mut func_imp = String::with_capacity(128);

	func_imp.push_str("(&self");

	for arg in &func.function_args[1..] {
		let tname = match arg.vtype {
			VariableType::Regular => {
				format!("&{}", arg.type_name)
			}

			VariableType::Primitive => {
				arg.type_name.to_owned()
			}

			VariableType::Optional => {
				if arg.type_name == "String" {
					"Option<&str>".to_owned()
				} else {
					format!("Option<{}>", arg.type_name)
				}
			},

			VariableType::Enum => {
				arg.type_name.to_owned()
			}

			VariableType::Reference => {
				format!("&{}", arg.type_name)
			}

			_ => "<illegal>".to_owned(),
		};

		if arg.array {
			func_imp.push_str(&format!(", {}:& [{}]", arg.name, tname));
		} else {
			func_imp.push_str(&format!(", {}: {}", arg.name, tname));
		}
	}

	func_imp.push_str(")");

	//
	// If we don't have any return value we alwayes return self
	//
	if func.return_val.is_none() {
		func_imp.push_str(" -> &Self<'a>");
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
fn generate_rute<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
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


impl RustGenerator {
    pub fn generate(filename: &str, api_def: &ApiDef) -> io::Result<()> {
        let mut f = BufWriter::new(File::create(filename)?);

        // write header
        f.write_all(HEADER)?;

        // write all the structs
        generate_structs(&mut f, api_def)?;

        // Generate the main Rute entry
        generate_rute(&mut f, api_def)?;

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
		let func_impl = generate_func_impl(&func);
        assert_eq!(func_impl, "(&self) -> &Self<'a>");
	}

	//
	// Test function def with one primitive 
	//
	#[test]
	fn test_function_one_primitive() {
        let mut func = build_default_func();
        func.function_args.push(Variable {
			name: "foo".to_owned(),
			type_name: "i32".to_owned(),
			vtype: VariableType::Primitive,
			.. Variable::default()
		});

		let func_impl = generate_func_impl(&func);
        assert_eq!(func_impl, "(&self, foo: i32) -> &Self<'a>");
	}

	//
	// Test function def with two primitives
	//
	#[test]
	fn test_function_two_primitive() {
        let mut func = build_default_func();
        func.function_args.push(Variable {
			name: "width".to_owned(),
			type_name: "i32".to_owned(),
			vtype: VariableType::Primitive,
			.. Variable::default()
		});
        func.function_args.push(Variable {
			name: "height".to_owned(),
			type_name: "f32".to_owned(),
			vtype: VariableType::Primitive,
			.. Variable::default()
		});

		let func_impl = generate_func_impl(&func);
        assert_eq!(func_impl, "(&self, width: i32, height: f32) -> &Self<'a>");
	}

	//
	// Test function def with self and primitive return 
	//
	/*
	#[test]
	fn test_function_two_primitive() {
        let mut func = build_default_func();
        func.return_val = Some(Variable {
			type_name: "i32".to_owned(),
			vtype: VariableType::Primitive,
			.. Variable::default()
		});

		let func_impl = generate_func_impl(&func);
        assert_eq!(func_impl, "(&self) -> i32");
	}
	*/

}

