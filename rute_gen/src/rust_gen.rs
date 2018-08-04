use std::io;
use std::io::{BufWriter, Write};
use std::fs::File;
use api_parser::*;

//use heck::{CamelCase, SnakeCase};

pub struct RustGenerator;

static HEADER: &'static [u8] = b"
use std::cell::Cell;
use std::marker::PhantomData;
use std::mem::transmute;
use std::os::raw::{c_void, c_char};
use std::cell::RefCell;
use std::rc::Rc;
use std::ffi::CString;\n\n";

///
/// Generate the structs
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
///
///
fn generate_rute<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
    // Generate all stucts that doesn't have owned data 
    for sdef in api_def
        .class_structs
        .iter()
        .filter(|s| s.should_have_create_func() && !s.should_gen_wrap_class())
    {
        f.write_fmt(format_args!(
            "    struct RU{} (*create_{})(struct RUBase* priv_data, void* user_data);\n",
            sdef.name,
            sdef.name.to_snake_case()
        ))?;
    }

    Ok(())
}

impl RustGenerator {
    pub fn generate(filename: &str, api_def: &ApiDef) -> io::Result<()> {
        let mut f = BufWriter::new(File::create(filename)?);

        // write header
        f.write_all(HEADER)?;

        // write all the structs
        generate_structs(&mut f, api_def)?;

        Ok(())
    }
}

