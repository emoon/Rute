extern crate clang;
extern crate heck;
extern crate rayon;
extern crate walkdir;
extern crate qdoc_parser;

#[macro_use]
extern crate pest_derive;
extern crate pest;

//mod doc_parser;
mod header_gen;

//use doc_parser::{DocEntry, DocInfo, DocParser};
use header_gen::{Generator};
use std::collections::HashMap;

use qdoc_parser::{QDocParser, QDocFilterable};

fn filter_func(data: &str, _doc_type: QDocFilterable) -> String {
	data.to_owned()
}

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
    //let source_directory = vec!["/Users/danielcollin/Qt/5.11.2/Src/qtbase/src/widgets/widgets/qtextedit.cpp"];
    //let source_directory = vec!["/Users/danielcollin/temp/test.cpp"];
    //let res = QDocParser::parse_files(source_directory);
	
	let doc_parser = QDocParser::new(filter_func);
	let docs = doc_parser.parse_files(&source_directory);

	/*
	for (_, e) in res {
		println!("-----------------------------------------");
		for entry in e.0 {
			println!("{:?}", entry.data);
		}
	}
	*/

	/*
    for (fname, entry) in res {
    	let t = entry.unwrap();

    	for e in t.0 {
    		println!("{}", e);
    	}
    }
    */

    // Build some lookup info for faster lookup when generating the output data
	/*
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
    */

	/*
    let lookups = DocLookups {
    	cpp_name: HashMap::new(),
    	class_name: HashMap::new(),
    	property: HashMap::new(),
    };

    //println!("{:?}", cpp_name_lookup);
    */

    Generator::generate(
        output_directory,
        header_files_path,
        compile_args,
        &docs,
    );
}
