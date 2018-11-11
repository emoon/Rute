extern crate clang;
extern crate heck;
extern crate qdoc_parser;
extern crate rayon;
extern crate walkdir;

mod header_gen;

//use doc_parser::{DocEntry, DocInfo, DocParser};
use header_gen::Generator;
use std::path::Path;
use std::env;

use qdoc_parser::{QDocFilterable, QDocParser};

fn filter_func(data: &str, _doc_type: QDocFilterable) -> String {
    data.to_owned()
}

fn add_path_arg(dest: &mut Vec<String>, base_path: &str, path: &str) {
    dest.push(Path::new(base_path).join(path).to_str().unwrap().to_owned())
}

fn add_flag_path_arg(dest: &mut Vec<String>, flag: &str, base_path: &str, path: &str) {
    dest.push(format!(
        "{}{}",
        flag,
        Path::new(base_path).join(path).to_str().unwrap().to_owned()
    ))
}

fn add_flag(dest: &mut Vec<String>, flag: &str) {
    dest.push(flag.to_owned())
}

#[cfg(target_os = "macos")]
fn add_paths_compile_args(
    source_paths: &mut Vec<String>,
    paths: &mut Vec<String>,
    args: &mut Vec<String>,
    base_path: &str,
) {
    add_path_arg(paths, base_path, "lib/QtWidgets.framework/Headers");
    add_path_arg(
        paths,
        base_path,
        "lib/QtCore.framework/Versions/5/Headers/qnamespace.h",
    );
    add_path_arg(paths, base_path, "lib/QtGui.framework/Versions/5/Headers");

    add_flag_path_arg(args, "-F", base_path, "lib");
    add_flag_path_arg(args, "-I", base_path, "lib/QtWidgets.framework/Headers");
    add_flag_path_arg(
        args,
        "-I",
        base_path,
        "lib/QtWidgets.framework/Headers/5.11.2",
    );
}

#[cfg(target_os = "linux")]
fn add_paths_compile_args(
    header_paths: &mut Vec<String>,
    args: &mut Vec<String>,
    base_path: &str,
) {
    //add_path_arg(header_paths, base_path, "include/QtGui/qtextlayout.h");
    add_path_arg(header_paths, base_path, "include/QtWidgets");
    add_path_arg(header_paths, base_path, "include/QtGui");
    add_path_arg(header_paths, base_path, "include/QtCore/qnamespace.h");

    add_flag(args, "-std=c++11");
    add_flag(args, "-x");
    add_flag(args, "c++");
    add_flag(args, "-fPIC");
    add_flag_path_arg(args, "-I", base_path, "include");
    add_flag_path_arg(args, "-I", base_path, "include/QtWidgets");
    add_flag_path_arg(args, "-I", base_path, "include/QtGui");
    add_flag_path_arg(args, "-I", base_path, "include/QtCore");
    add_flag_path_arg(args, "-I", base_path, "include/QtGui/5.11.2");
    add_flag_path_arg(args, "-I", base_path, "include/QtWidgets/5.11.2");
    add_flag_path_arg(args, "-I", base_path, "include/QtCore/5.11.2");

}

fn main() {
    // Get all the files to parse
    let output_directory = "output";
    let mut header_files_paths = Vec::<String>::new();
    let mut compile_args = Vec::<String>::new();
    let mut source_directory = Vec::<String>::new();

    rayon::ThreadPoolBuilder::new()
        .num_threads(28)
        .build_global()
        .unwrap();

    let base_path = env::var("QT5").unwrap_or_else(|_| {
        panic!(
"\n\nUnable to find QT5 environment variable. This needs to be set in order to use Rute.
You can download Qt from https://www.qt.io and the set the variable. Examples:
macOS: export QT5=/Users/USER_NAME/Qt/5.11.2/clang_64\n
GNU/Linux: export QT5=/opt/qt510\n\n");
    });

    add_path_arg(&mut source_directory, &base_path, "../Src/qtbase/src");
    add_paths_compile_args(&mut header_files_paths, &mut compile_args, &base_path);

    let doc_parser = QDocParser::new(filter_func);
    let docs = doc_parser.parse_files(&source_directory);

    Generator::generate(output_directory, header_files_paths, compile_args, &docs);
}
