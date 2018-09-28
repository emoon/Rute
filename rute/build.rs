use std::env;
//use std::fs::File;
//use std::io::Write;
use std::path::Path;
use std::process::Command;

extern crate cc;

fn main() {
    let target = env::var("TARGET").unwrap();

    let qt_dir = env::var("QT5").unwrap_or_else(|_| { panic!(
"\n\nUnable to find QT5 enviroment variable. This needs to be set in order to use this Crate.
You can download Qt from https://www.qt.io and the set the varible. Examples:
macOS: export QT5=/Users/USER_NAME/Qt/5.10.0/clang_64\n\n");
    });

    let moc_exe = format!("{}/bin/moc", qt_dir.as_str());
    let rute_signal_wrappers;
    let rute_cpp;

    // This is used to switch to the test bed for testing things out in rute_test_bed.cpp and
    // rute_test_bed.rs

    let test_bed = false;

    if test_bed {
        rute_signal_wrappers = "c_cpp/test_bed/rute_test_bed.h";
        rute_cpp = "c_cpp/test_bed/rute_test_bed.cpp";
    } else {
        rute_signal_wrappers = "qt_cpp/auto/rute_signal_wrappers.h";
        rute_cpp = "qt_cpp/auto/qt_bulk.cpp";
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("rute_signal_wrapper.cpp");
    let dest_path_str = dest_path.into_os_string().into_string().unwrap();

    // Generate the moc code

    Command::new(moc_exe)
            .args(&[&rute_signal_wrappers, "-o", &dest_path_str])
            .spawn()
            .expect("failed to execute process");

    // TODO: cleanup

    let i3 = Path::new(&qt_dir).join("include");
    let i4 = Path::new(&qt_dir).join("include/QtCore");
    let i5 = Path::new(&qt_dir).join("include/QtGui");
    let i6 = Path::new(&qt_dir).join("include/QtWidgets");

    if target.contains("windows") {
        cc::Build::new()
            .file(&rute_cpp)
            .file(&dest_path_str)
            .file("qt_cpp/rute_manual.cpp")
            .include(i3)
            .include(i4)
            .include(i5)
            .include(i6)
            .compile("rute_cpp");

    } else if target.contains("darwin") {
        let i0 = Path::new(&qt_dir).join("lib/QtWidgets.framework/Headers");
        let i1 = Path::new(&qt_dir).join("lib/QtCore.framework/Headers");
        let i2 = Path::new(&qt_dir).join("lib/QtGui.framework/Headers");

        let framework_dir = format!("{}/lib", qt_dir.as_str());
        let f_flag = format!("-F{}/lib", qt_dir.as_str());

        cc::Build::new()
            .file(&rute_cpp)
            .file(&dest_path_str)
            .file("qt_cpp/rute_manual.cpp")
            .include(i0)
            .include(i1)
            .include(i2)
            .include(i3)
            .include(i4)
            .include(i5)
            .include(i6)
            .flag(&f_flag)
            .flag("-std=c++11")
            .cpp(true)
            .cpp_link_stdlib("c++")
            .cpp_set_stdlib("c++")
            .compile("rute_cpp");

            //println!("cargo:rustc-link-lib={}", "stdc++");
            println!("cargo:rustc-link-search=framework={}", &framework_dir);
            println!("cargo:rustc-link-lib=framework={}", "Cocoa");
            println!("cargo:rustc-link-lib=framework={}", "QtWidgets");
            println!("cargo:rustc-link-lib=framework={}", "QtGui");
            println!("cargo:rustc-link-lib=framework={}", "QtCore");
    } else {

    }
}

