use std::env;
//use std::fs::File;
//use std::io::Write;
use std::path::Path;
use std::process::Command;

extern crate cc;

fn main() {
    let qt_dir = env::var("QT5").unwrap_or_else(|_| { panic!(
"\n\nUnable to find QT5 enviroment variable. This needs to be set in order to use this Crate.
You can download Qt from https://www.qt.io and the set the varible. Examples:
macOS: export QT5=/Users/USER_NAME/Qt/5.10.0/clang_64\n\n");
    });

    let moc_exe = format!("{}/bin/moc", qt_dir.as_str());

    // Generate the moc code

    Command::new(moc_exe)
            args(&["c_cpp/auto/rute_cpp.h", "-o", "c_cpp/auto/rute_moc.cpp"])
            // Used for temporary test-bed
            //.args(&["c_cpp/test_bed/rute_test_bed.h", "-o", "c_cpp/auto/rute_moc.cpp"])
            .spawn()
            .expect("failed to execute process");

    // only support mac for now

    let i0 = Path::new(&qt_dir).join("lib/QtWidgets.framework/Headers");
    let i1 = Path::new(&qt_dir).join("lib/QtCore.framework/Headers");
    let i2 = Path::new(&qt_dir).join("lib/QtGui.framework/Headers");

    let i3 = Path::new(&qt_dir).join("include");
    let i4 = Path::new(&qt_dir).join("include/QtCore");
    let i5 = Path::new(&qt_dir).join("include/QtGui");
    let i6 = Path::new(&qt_dir).join("include/QtWidgets");

    let framework_dir = format!("{}/lib", qt_dir.as_str());
    let f_flag = format!("-F{}/lib", qt_dir.as_str());

    //let mut f = File::create(&dest_path).unwrap();

    cc::Build::new()
        .file("c_cpp/auto/rute_cpp.cpp")
        // Used for temporary test-bed
        //.file("c_cpp/test_bed/rute_test_bed.cpp")
        .file("c_cpp/auto/rute_moc.cpp")
        .file("c_cpp/rute_manual.cpp")
        .include(i0)
        .include(i1)
        .include(i2)
        .include(i3)
        .include(i4)
        .include(i5)
        .include(i6)
        .flag(&f_flag)
        .flag("-std=c++11")
        .cpp_link_stdlib("stdc++")
        .compile("rute_cpp");

    println!("cargo:rustc-link-lib={}", "stdc++");
    println!("cargo:rustc-link-search=framework={}", &framework_dir);
    println!("cargo:rustc-link-lib=framework={}", "Cocoa");
    println!("cargo:rustc-link-lib=framework={}", "QtWidgets");
    println!("cargo:rustc-link-lib=framework={}", "QtGui");
    println!("cargo:rustc-link-lib=framework={}", "QtCore");
    //println!("cargo:rustc-flags={}", "-l,-rpath,/Users/danielcollin/Qt/5.10.0/clang_64/lib");
}

