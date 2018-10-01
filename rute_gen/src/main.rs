extern crate argparse;
extern crate heck;
extern crate liquid;
extern crate pest;
extern crate rayon;
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
use c_gen::CapiHeaderGen;
use header_ffi_gen::HeaderFFIGenerator;
use qt_gen::QtGenerator;
use rayon::prelude::*;
use rust_ffi_gen::RustFFIGenerator;
use rust_gen::RustGenerator;
use std::process::Command;
use std::fs;
use std::sync::RwLock;
use walkdir::WalkDir;

///
/// Function for creating a directory and just bail in case it already exists.
/// If there is an error here this code will panic as these directories are required in order for
/// this program to work correctly.
///

fn create_dir(path: &str) {
    // dir already existits so just bail
    if let Ok(p) = fs::metadata(path) {
        if p.is_dir() {
            return;
        }
    }

    // This function is expected to succed now when we have checked that the directory exists
    fs::create_dir(path).unwrap();
}

///
/// Run Rustfmt on generated file
///
fn run_rustfmt(filename: &str) {
    Command::new("cargo")
        .arg("fmt")
        .arg(filename)
        .output()
        .expect("failed to execute cargo fmt");
}

///
/// Main
///
fn main() {
    let wd = WalkDir::new("defs");
    /*
    // temporary set to one thread during debugging
    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build_global()
        .unwrap();
    */

    let rust_dest_dir = "../rute/src/auto";
    let qt_dest = "../rute/qt_cpp/auto";

    // Create the output dirs before doing anything else
    create_dir(qt_dest);
    create_dir(rust_dest_dir);

    // Collect all files that needs to be parsed
    let files = wd
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().metadata().unwrap().is_file())
        .collect::<Vec<_>>();

    let api_defs = RwLock::new(Vec::with_capacity(files.len()));

    // Pass 1: Parse all the files
    // Parse the files threaded

    files.par_iter().for_each(|f| {
        let mut api_def = ApiDef::default();

        println!("Parsing file {:?}", f.path());
        ApiParser::parse_file(&f.path(), &mut api_def);

        // Insert the api_def for later usage
        {
            let mut data = api_defs.write().unwrap();
            data.push(api_def);
        }
    });

    // patch up some refs, sort by filename for second pass

    {
        let mut data = api_defs.write().unwrap();
        ApiParser::second_pass(&mut data);
        data.sort_by(|a, b| a.filename.cmp(&b.filename))
    }

    let api_defs_read = api_defs.read().unwrap();

    // Pass 2:
    // Generate all the code.

    api_defs_read.par_iter().enumerate().for_each(|(index, api_def)| {
        let base_filename = &api_def.base_filename;

        // On the first thread we start with generating a bunch of main files so we have this
        // generation running threaded as well. Next time when index isn't 0 anymore regular work
        // will come along here.

        if index == 0 {
            let main_rute_rust = format!("{}/{}.rs", rust_dest_dir, "rute");
            let main_mod_rust = format!("{}/{}.rs", rust_dest_dir, "mod");
            let signal_wrappers = format!("{}/{}.h", qt_dest, "rute_signal_wrappers");
            let enum_mapping = format!("{}/{}.cpp", qt_dest, "qt_enum_mapping");
            let qt_bulk_cpp = format!("{}/{}.cpp", qt_dest, "qt_bulk");
            let qt_rute_cpp = format!("{}/{}.cpp", qt_dest, "qt_rute");
            let main_ffi_header = format!("{}/{}.h", qt_dest, "rute");
            let main_ffi = format!("{}/{}.rs", rust_dest_dir, "rute_ffi");

            // Generate the main rute.rs file
            println!("    Generating main Rute Rust: {}", main_rute_rust);
            RustGenerator::new().generate_rute(&main_rute_rust, &api_defs_read).unwrap();

            // Generate the mod file for the auto generated Rust code
            println!("    Generating Rute auto mod: {}", main_mod_rust);
            RustGenerator::generate_auto_mod(&main_mod_rust, &api_defs_read).unwrap();

            // Generate all the signal wrappers for Qt C++
            println!("    Generating Qt signal wrappers: {}", signal_wrappers);
            QtGenerator::new().generate_all_signal_wrappers(&signal_wrappers, &api_defs_read).unwrap();

            // Generate all the signal wrappers for Qt C++
            println!("    Generating Qt enum mapping: {}", enum_mapping);
            QtGenerator::new().generate_enum_mappings(&enum_mapping, &api_defs_read).unwrap();

            // Generate bulk cpp file for Qt C++ wrapper code
            println!("    Generating Qt bulk file mapping: {}", qt_bulk_cpp);
            QtGenerator::generate_bulk_cpp(&qt_bulk_cpp, &api_defs_read).unwrap();

            // Generate the rute main file
            println!("    Generating Main Qt file {}", qt_rute_cpp);
            QtGenerator::generate_rute_struct(&qt_rute_cpp, &api_defs_read).unwrap();

            // Generate rute main FFI for C++
            println!("    Generating C++ main header: {}", main_ffi_header);
            HeaderFFIGenerator::generate_main(&main_ffi_header, &api_defs_read, CapiHeaderGen::new()).unwrap();

            // Generate rute main FFI for Rust
            println!("    Generating Rust main file: {}", main_ffi);
            HeaderFFIGenerator::generate_main(&main_ffi, &api_defs_read, RustFFIGenerator::new()).unwrap();

            // Rust Rustfmt on rust files
            run_rustfmt(&main_rute_rust);
            run_rustfmt(&main_mod_rust);
            run_rustfmt(&main_ffi);
        }

        // We handle this file a bit special because it contains enums that are used for pretty
        // much all of the Qt code. So we generated only a special Rust file for it and then
        // it will be used for the general enum mapping generation in the C++ code
        if base_filename == "qnamespace" {
            let rust_target = format!("{}/{}.rs", rust_dest_dir, "rute_enums");

            // Generate the Rust high-level code
            println!("    Generating Rust global enums: {}", rust_target);
            RustGenerator::new().generate(&rust_target, &api_def).unwrap();

            // Rust Rustfmt on rust files
            run_rustfmt(&rust_target);
        } else {
            // Build target filenames
            let rust_ffi_target = format!("{}/{}_ffi.rs", rust_dest_dir, base_filename);
            let rust_target = format!("{}/{}.rs", rust_dest_dir, base_filename);
            let header_target = format!("{}/{}_ffi.h", qt_dest, base_filename);
            let qt_cpp_target = format!("{}/{}", qt_dest, base_filename);

            // Generate Rust FFI
            println!("    Generating Rust FFI: {}", rust_ffi_target);
            HeaderFFIGenerator::generate(&rust_ffi_target, &api_def, RustFFIGenerator::new()).unwrap();

            // Generate C/C++ Header for FFI structs
            println!("    Generating C/C++ header: {}", header_target);
            HeaderFFIGenerator::generate(&header_target, &api_def, CapiHeaderGen::new()).unwrap();

            // Generate the Rust high-level code
            println!("    Generating Rust: {}", rust_target);
            RustGenerator::new().generate(&rust_target, &api_def).unwrap();

            // Generate the Qt wrapping
            println!("    Generating Qt C++ wrapper: {}.cpp/h", qt_cpp_target);
            QtGenerator::new().generate(&qt_cpp_target, &api_def).unwrap();

            // Rust Rustfmt on rust files
            run_rustfmt(&rust_ffi_target);
            run_rustfmt(&rust_target);
        }
    });

    // All done!
    println!("Generation complete!");
}
