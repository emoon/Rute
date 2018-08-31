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

// Code for C++ generation
mod cpp_gen;
mod cpp_gen_templates;

// Code for C/Header generation
mod c_gen;

use api_parser::{ApiDef, ApiParser};
use header_ffi_gen::HeaderFFIGenerator;

use c_gen::CapiHeaderGen;
//use rust_ffi_gen::RustFFIGenerator;
use cpp_gen::CppGenerator;
use rust_gen::RustGenerator;
use std::fs;
use std::thread;
use std::sync::Arc;

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
        HeaderFFIGenerator::generate("../rute/c_cpp/auto/Rute.h", &c_api_def, CapiHeaderGen::new()).unwrap();
    });

    /*
    let ffi_api_def = api_def.clone();
    let ffi_api_thread = thread::spawn(move || {
        RustFFIGenerator::generate("../rute/src/auto/rute_auto_ffi.rs", &ffi_api_def).unwrap();
    });
    */

    let cpp_api_def = api_def.clone();
    let cpp_api_thread = thread::spawn(move || {
        let cpp_gen = CppGenerator::new();
        cpp_gen.generate("../rute/c_cpp/auto/rute_cpp", &cpp_api_def).unwrap();
    });

    let rust_api_def = api_def.clone();
    let rust_gen = RustGenerator::new(&rust_api_def);
    rust_gen.generate("../rute/src/auto/rute_auto.rs", &rust_api_def).unwrap();

    // wait for all of them to finish

    c_api_thread.join().unwrap();
    //ffi_api_thread.join().unwrap();
    cpp_api_thread.join().unwrap();
}
