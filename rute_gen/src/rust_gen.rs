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
        .try_for_each(|s| generate_struct_impl(f, s))
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

