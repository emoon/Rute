extern crate argparse;
extern crate liquid;
extern crate heck;
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

// Code for C generation (comman helpers for C style code)
mod c_helper;

// Code for C++ generation
mod cpp_gen;

// Code for C generation
mod c_gen;

use api_parser::{ApiParser, ApiDef};
use c_gen::CapiGenerator;
use rust_ffi_gen::RustFFIGenerator;

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

    // This pass will run and "fixup" types that might be out of order and needs to be relinked
    api_def.second_pass();

    // Generate bindings for each backend

    CapiGenerator::generate("../rute/c_cpp/Rute.h", &api_def).unwrap();
    RustFFIGenerator::generate("../rute/src/ffi.rs", &api_def).unwrap();
}
