extern crate clang;
extern crate heck;
extern crate rayon;
extern crate walkdir;

#[macro_use]
extern crate pest_derive;
extern crate pest;

mod doc_parser;
mod header_gen;

use doc_parser::{DocEntry, DocInfo, DocParser};
use header_gen::{Generator, DocLookups};
use std::collections::HashMap;


fn main() {
    // Get all the files to parse
    let output_directory = "output";

	/*
    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build_global()
        .unwrap();
    */

    // TODO: Don't hardcode these
    let header_files_path = &[
        "/Users/danielcollin/Qt/5.11.2/clang_64/lib/QtWidgets.framework/Headers",
        "/Users/danielcollin/Qt/5.11.2/clang_64/lib/QtCore.framework/Versions/5/Headers/qnamespace.h",
        "/Users/danielcollin/Qt/5.11.2/clang_64/lib/QtGui.framework/Versions/5/Headers"
    ];

    // TODO: Don't hardcode these
    let compile_args = &[
        "-std=c++11",
        "-F/Users/danielcollin/Qt/5.11.2/clang_64/lib",
        "-i /Users/danielcollin/Qt/5.11.2/clang_64/lib/QtWidgets.framework/Headers",
        "-i /Users/danielcollin/Qt/5.11.2/clang_64/lib/QtWidgets.framework/Headers/5.11.2",
    ];

    // Parse source files for documentation

    let source_directory = ["/Users/danielcollin/Qt/5.11.2/Src/qtbase/src"];
    //let source_directory = ["/Users/danielcollin/Qt/5.11.2/Src/qtbase/src/widgets/widgets/qtextedit.cpp"];
    let docs = DocParser::parse_files(&source_directory);

    // Build some lookup info for faster lookup when generating the output data

    let mut cpp_name_lookup: HashMap<&str, &DocEntry> = HashMap::new();
    let mut class_lookup: HashMap<&str, &DocEntry> = HashMap::new();
    let mut property_lookup: HashMap<&str, &DocEntry> = HashMap::new();

    for doc in &docs {
        for entry in &doc.entries {
            if !entry.target_function.is_empty() {
                cpp_name_lookup.insert(&entry.target_function, entry);
            }

            if !entry.class_name.is_empty() {
                class_lookup.insert(&entry.class_name, entry);
            }

            if !entry.property.is_empty() {
                property_lookup.insert(&entry.class_name, entry);
            }
        }
    }

    let lookups = DocLookups {
    	cpp_name: cpp_name_lookup,
    	class_name: class_lookup,
    	property: property_lookup,
    };

    //println!("{:?}", cpp_name_lookup);

    Generator::generate(
        output_directory,
        header_files_path,
        compile_args,
        &lookups,
    );
}
