extern crate clang;
extern crate heck;
extern crate qdoc_parser;
extern crate rayon;
extern crate walkdir;

#[macro_use]
extern crate pest_derive;
extern crate pest;

//mod doc_parser;
mod header_gen;

//use doc_parser::{DocEntry, DocInfo, DocParser};
use header_gen::Generator;
use std::collections::HashMap;

use qdoc_parser::{QDocFilterable, QDocParser};

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
    let header_files_path = vec![
        "/Users/danielcollin/Qt/5.11.2/clang_64/lib/QtWidgets.framework/Headers".to_owned(),
        "/Users/danielcollin/Qt/5.11.2/clang_64/lib/QtCore.framework/Versions/5/Headers/qnamespace.h".to_owned(),
        "/Users/danielcollin/Qt/5.11.2/clang_64/lib/QtGui.framework/Versions/5/Headers".to_owned()
    ];

    // TODO: Don't hardcode these
    let compile_args = vec![
        "-std=c++11".to_owned(),
        "-F/Users/danielcollin/Qt/5.11.2/clang_64/lib".to_owned(),
        "-i /Users/danielcollin/Qt/5.11.2/clang_64/lib/QtWidgets.framework/Headers".to_owned(),
        "-i /Users/danielcollin/Qt/5.11.2/clang_64/lib/QtWidgets.framework/Headers/5.11.2".to_owned(),
    ];

    // Parse source files for documentation

    let source_directory = ["/Users/danielcollin/Qt/5.11.2/Src/qtbase/src"];

    let doc_parser = QDocParser::new(filter_func);
    let docs = doc_parser.parse_files(&source_directory);

    Generator::generate(output_directory, header_files_path, compile_args, &docs);
}
