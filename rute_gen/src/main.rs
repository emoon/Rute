extern crate argparse;
extern crate heck;
extern crate liquid;
extern crate pest;
extern crate walkdir;

#[macro_use]
extern crate pest_derive;

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

// Code for C generation
mod c_gen;

use api_parser::{ApiDef, ApiParser};
use c_gen::CapiGenerator;
use rust_ffi_gen::RustFFIGenerator;
use cpp_gen::CppGenerator;
use rust_gen::RustGenerator;
use std::fs;

fn main() {
    // This holds all the structs,variables,etc
    let mut api_def = ApiDef::default();

    // Parse all the files in defs
    for entry in walkdir::WalkDir::new("defs") {
        let entry = entry.unwrap();

        // only parse files and skip directories.
        if entry.path().metadata().unwrap().is_file() {
            ApiParser::parse_file(&entry.path(), &mut api_def);
        }
    }

    // Run a second pass to match up types that may be out of order
    ApiParser::second_pass(&mut api_def);

    // TODO: Correct error handling here
    let _ = fs::create_dir("../rute/c_cpp/auto");
    let _ = fs::create_dir("../rute/src/auto");

    // Generate bindings for each backend

    let rust_gen = RustGenerator::new(&api_def);

    CapiGenerator::generate("../rute/c_cpp/auto/Rute.h", &api_def).unwrap();
    RustFFIGenerator::generate("../rute/src/auto/rute_auto_ffi.rs", &api_def).unwrap();
    CppGenerator::generate("../rute/c_cpp/auto/rute_cpp", &api_def).unwrap();

    rust_gen.generate("../rute/src/auto/rute_auto.rs", &api_def).unwrap();
}
