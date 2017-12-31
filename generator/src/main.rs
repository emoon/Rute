extern crate argparse;
extern crate heck;
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod api_parser;
pub mod c_api_gen;
pub mod qt;
mod rust_ffi_gen;
mod rust_gen;

use argparse::{ArgumentParser, Store};
use std::process::exit;
use std::io;

/*
static INPUT_API: &str = "src/api.def";

static C_API_HEADER: &str = "../../src/prodbg/PluginUI/generated/c_api.h";
static QT_API_IMPL: &str = "../../src/prodbg/PluginUI/generated/qt_api_gen.cpp";
static QT_API_IMPL_HEADER: &str = "../../src/prodbg/PluginUI/generated/qt_api_gen.h";

static RUST_FFI_FILE: &str = "../../api/rust/prodbg_ui/src/ffi_gen.rs";
static UI_FILE: &str = "../../api/rust/prodbg_ui/src/lib.rs";
*/

#[derive(Clone, Default)]
struct Options {
    api_spec: String,
    c_api_header: String,
    qt_cpp: String,
    qt_manual_cpp: String,
    qt_header: String,
    rust_ffi: String,
    rust_impl: String,
}

fn generate_code() -> io::Result<()> {
    let mut options = Options::default();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Generates Rust bindings for Qt");

        ap.refer(&mut options.api_spec).add_option(
            &["-a", "--api_spec"],
            Store,
            "Input API Spec file",
        );
        ap.refer(&mut options.c_api_header).add_option(
            &["-c", "--c_header"],
            Store,
            "Output C API header",
        );
        ap.refer(&mut options.qt_cpp).add_option(
            &["-q", "--qt_cpp"],
            Store,
            "Output Qt C++ code impl",
        );
        ap.refer(&mut options.qt_manual_cpp).add_option(
            &["-m", "--qt_manul_cpp"],
            Store,
            "Input Qt C++ manual code",
        );
        ap.refer(&mut options.qt_header).add_option(
            &["-b", "--qt_header"],
            Store,
            "Output Qt C++ Header",
        );

        ap.refer(&mut options.rust_impl)
            .add_option(&["-r", "--rust"], Store, "Output Rust impl");

        ap.refer(&mut options.rust_ffi).add_option(
            &["-f", "--rust_ffi"],
            Store,
            "Output Rust FFI bindings",
        );

        match ap.parse_args() {
            Ok(()) => {}
            Err(x) => {
                exit(x);
            }
        }
    }

    let api_def = api_parser::ApiDef::new(&options.api_spec);

    c_api_gen::generate_c_api(&options.c_api_header, &api_def)?;
    qt::generate_qt_bindings(&options.qt_cpp, &options.qt_header, &options.qt_manual_cpp, &api_def)?;

    rust_ffi_gen::generate_ffi_bindings(&options.rust_ffi, &api_def, &api_def.entries)?;
    rust_gen::generate_rust_bindings(&options.rust_impl, &api_def)?;

    Ok(())
}

fn main() {
    if let Err(err) = generate_code() {
        panic!("Unable to generate err {:?}", err);
    }
}
