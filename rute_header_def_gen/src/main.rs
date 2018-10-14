extern crate clang;
extern crate heck;
extern crate rayon;
extern crate walkdir;

#[macro_use]
extern crate pest_derive;
extern crate pest;

mod header_gen;
mod doc_parser;

use doc_parser::DocParser;
use header_gen::Generator;

fn main() {
    /*
    // Get all the files to parse
    let output_directory = "output";

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

    Generator::generate(output_directory, header_files_path, compile_args);
    */

    DocParser::parse_file("/Users/danielcollin/Qt/5.11.2/Src/qtbase/src/widgets/widgets/qpushbutton.cpp");
    //DocParser::parse_file("/Users/danielcollin/temp/test.cpp");
}

