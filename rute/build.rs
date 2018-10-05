use std::env;
//use std::fs::File;
//use std::io::Write;
use std::path::Path;
use std::process::Command;

extern crate cc;

fn main() {
    let target = env::var("TARGET").unwrap();

    let qt_dir = env::var("QT5").unwrap_or_else(|_| {
        panic!(
"\n\nUnable to find QT5 environment variable. This needs to be set in order to use Rute.
You can download Qt from https://www.qt.io and the set the variable. Examples:
macOS: export QT5=/Users/USER_NAME/Qt/5.11.2/clang_64\n
GNU/Linux: export QT5=/opt/qt510\n\n");
    });

    let moc_exe = format!("{}/bin/moc", qt_dir.as_str());
    let rute_signal_wrappers = "qt_cpp/auto/rute_signal_wrappers.h";
    let rute_cpp = "qt_cpp/auto/qt_bulk.cpp";

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("rute_signal_wrapper.cpp");
    let dest_path_str = dest_path.into_os_string().into_string().unwrap();

    // Generate the moc code
    Command::new(moc_exe)
        .args(&[&rute_signal_wrappers, "-o", &dest_path_str])
        .output()
        .expect("failed to execute process");

    // Add base include dirs for all targets
    let mut build = cc::Build::new();

    let base_include_dirs = [
        Path::new(&qt_dir).join("include"),
        Path::new(&qt_dir).join("include/QtCore"),
        Path::new(&qt_dir).join("include/QtGui"),
        Path::new(&qt_dir).join("include/QtWidgets"),
    ];

    for dir in &base_include_dirs {
        build.include(dir);
    }

    // Add the files we build
    build.file(&rute_cpp);
    build.file(&dest_path_str);
    build.file("qt_cpp/rute_manual.cpp");

    // On macOS we need to add the framework paths also and fiddle around with some
    // framework releated parameters to cc as well
    if target.contains("darwin") {
        let mac_include_dirs = [
            Path::new(&qt_dir).join("lib/QtWidgets.framework/Headers"),
            Path::new(&qt_dir).join("lib/QtCore.framework/Headers"),
            Path::new(&qt_dir).join("lib/QtGui.framework/Headers"),
        ];

        for dir in &mac_include_dirs {
            build.include(dir);
        }

        let framework_dir = format!("{}/lib", qt_dir.as_str());
        let f_flag = format!("-F{}/lib", qt_dir.as_str());
        build
            .flag(&f_flag)
            .flag("-std=c++11")
            .cpp(true)
            .cpp_link_stdlib("c++")
            .cpp_set_stdlib("c++");

        println!("cargo:rustc-link-search=framework={}", &framework_dir);
        println!("cargo:rustc-link-lib=framework={}", "Cocoa");
        println!("cargo:rustc-link-lib=framework={}", "QtWidgets");
        println!("cargo:rustc-link-lib=framework={}", "QtGui");
        println!("cargo:rustc-link-lib=framework={}", "QtCore");
    } else if target.contains("linux") {
        let lib_dir = format!("{}/lib", qt_dir.as_str());
        build.flag("-std=c++11").cpp_link_stdlib("stdc++").cpp(true);

        println!("cargo:rustc-link-search={}", &lib_dir);
        println!("cargo:rustc-link-lib={}", "Qt5Widgets");
        println!("cargo:rustc-link-lib={}", "Qt5Gui");
        println!("cargo:rustc-link-lib={}", "Qt5Core");
    } else if target.contains("windows") {
        let lib_dir = format!("{}\\lib", qt_dir.as_str());

        println!("cargo:rustc-link-search={}", &lib_dir);
        println!("cargo:rustc-link-lib={}", "Qt5Widgets");
        println!("cargo:rustc-link-lib={}", "Qt5Gui");
        println!("cargo:rustc-link-lib={}", "Qt5Core");
    }

    build.compile("rute_cpp");
}
