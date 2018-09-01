extern crate argparse;
extern crate heck;
extern crate liquid;
extern crate pest;
extern crate walkdir;

#[macro_use]
extern crate pest_derive;

// Trait for Header and FFI generation
mod header_ffi_gen;

// Code for parsing the API def
mod api_parser;

// Code for FFI generation
mod rust_ffi_gen;

// Code for Rust generation
mod rust_gen;
mod rust_gen_templates;

// Code for C generation (comman helpers for C style code)
mod c_helper;

// Code for Qt C++ generation
mod qt_gen;
mod qt_gen_templates;

// Code for C/Header generation
mod c_gen;

use api_parser::{ApiDef, ApiParser};
use header_ffi_gen::HeaderFFIGenerator;

use c_gen::CapiHeaderGen;
use rust_ffi_gen::RustFFIGenerator;
use qt_gen::QtGenerator;
use rust_gen::RustGenerator;
use std::fs;
use std::sync::Arc;
use std::thread;

fn main() {
    let mut api = ApiDef::default();

    // Parse all the files in defs
    for entry in walkdir::WalkDir::new("defs") {
        let entry = entry.unwrap();

        // only parse files and skip directories.
        if entry.path().metadata().unwrap().is_file() {
            ApiParser::parse_file(&entry.path(), &mut api);
        }
    }

    // Run a second pass to match up types that may be out of order
    ApiParser::second_pass(&mut api);

    // This holds all the structs,variables,etc
    let api_def = Arc::new(api);

    // TODO: Correct error handling here
    let _ = fs::create_dir("../rute/c_cpp/auto");
    let _ = fs::create_dir("../rute/src/auto");

    // Generate bindings for each backend in threads

    let c_api_def = api_def.clone();
    let c_api_thread = thread::spawn(move || {
        HeaderFFIGenerator::generate(
            "../rute/c_cpp/auto/Rute.h",
            &c_api_def,
            CapiHeaderGen::new(),
        ).unwrap();
    });

    let ffi_api_def = api_def.clone();
    let ffi_api_thread = thread::spawn(move || {
        HeaderFFIGenerator::generate(
            "../rute/src/auto/rute_auto_ffi.rs",
            &ffi_api_def,
            RustFFIGenerator::new(),
        ).unwrap();
    });

    let cpp_api_def = api_def.clone();
    let cpp_api_thread = thread::spawn(move || {
        let qt_gen = QtGenerator::new();
        qt_gen
            .generate("../rute/c_cpp/auto/rute_qt", &cpp_api_def)
            .unwrap();
    });

    let rust_api_def = api_def.clone();
    let rust_gen = RustGenerator::new(&rust_api_def);
    rust_gen
        .generate("../rute/src/auto/rute_auto.rs", &rust_api_def)
        .unwrap();

    // wait for all of them to finish

    c_api_thread.join().unwrap();
    ffi_api_thread.join().unwrap();
    cpp_api_thread.join().unwrap();
}
