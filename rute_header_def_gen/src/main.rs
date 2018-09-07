extern crate clang;
extern crate heck;
extern crate rayon;
extern crate walkdir;

use clang::*;
use heck::SnakeCase;
use rayon::prelude::*;
use std::borrow::Cow;
use std::collections::HashSet;
use std::sync::RwLock;
use walkdir::WalkDir;

enum FunctionType {
    Regular,
    Slot,
}

const SKIP_NAMES: &[&'static str] = &[
    "Private",
    "operator",
    "metaclass",
    "tr",
    "metacall",
    "metaObject",
    "d_func",
    "qt_metacast",
    "dynamic_meta_object",
];

///
/// Translate Qt/C++ type to API def type
///
fn get_pod_type<'a>(name: &'a str) -> &'a str {
    match name {
        "int" => "i32",
        "bool" => "bool",
        "qreal64" => "f64",
        "qreal32" => "f32",
        "qreal" => "f32",
        "const char *" => "String",
        _ => name,
    }
}

///
/// Translate a parsed type into API Def type
///
fn get_arg_type<'a>(t: &'a Type) -> Cow<'static, str> {
    let name = t.get_display_name();

    if t.is_pod() {
        get_pod_type(&name).to_owned().into()
    } else {
        let type_name;

        if name.starts_with("const ") {
            type_name = &name[6..];
        } else {
            type_name = &name[1..];
        }

        if type_name.find("QList<").is_some() {
            let end_tag = type_name.find('>').unwrap();
            let t = &type_name[6..end_tag];
            let ret;

            // check if pointer
            if t.find('*').is_some() {
                ret = format!("<&{}Type>", &t[..t.len() - 2]);
            } else {
                ret = format!("<{}Type>", t);
            }

            return ret.into();
        } else {


        }

        type_name.to_owned().into()
    }
}

///
///
///
fn print_arg(arg: &Entity, arg_count: &mut usize) {
    let arg_type = get_arg_type(&arg.get_type().unwrap());

    print!("{} ", arg_type);

    if let Some(name) = arg.get_name() {
        print!("{}", name);
    } else {
        print!("arg{}", arg_count);
        *arg_count += 1;
    }
}

///
/// Print a functio/method definition
///
fn print_func(entry: &Entity, func_type: FunctionType) {
    let name = match entry.get_name() {
        Some(name) => name,
        None => return,
    };

    for skip in SKIP_NAMES {
        if name.contains(skip) {
            return;
        }
    }

    print!("    {}", name.to_snake_case());

    if let Some(args) = entry.get_arguments() {
        if args.is_empty() {
            print!("()");
        } else {
            let mut count = 0;

            print!("(");

            print_arg(&args[0], &mut count);

            for arg in &args[1..] {
                print!(", ");
                print_arg(arg, &mut count);
            }

            print!(")");
        }
    } else {
        print!("()");
    }

    if let Some(res) = entry.get_result_type() {
        let display_name = res.get_display_name();

        if display_name.contains("void") {
            println!(",");
        } else {
            println!(" -> {},", get_arg_type(&res));
        }
    } else {
        print!("\n");
    }
}

///
/// Print a class. This code a a bit hacky but should do for this usage
///
fn print_class(entry: &Entity) {
    let name = match entry.get_name() {
        Some(name) => name,
        None => return,
    };

    // Only deal with Q* types
    if name.as_bytes()[0] != b'Q' {
        return;
    }

    if name.contains("Private") {
        return;
    }

    // Make sure class is public

    if let Some(access) = entry.get_accessibility() {
        match access {
            Accessibility::Private => return,
            Accessibility::Protected => return,
            _ => (),
        }
    }

    //println!("name {}", name);
    let mut base_classes = Vec::new();
    let mut method_count = 0;

    for field in entry.get_children() {
        match field.get_kind() {
            EntityKind::BaseSpecifier => {
                //println!("    base: {:?}\n\n", field.get_name());
                base_classes.push(field.get_name().unwrap().to_owned());
            }

            EntityKind::Method => method_count += 1,

            _ => (),
        }
    }

    if method_count == 0 {
        return;
    }

    // Print class and the inharitance

    if base_classes.is_empty() {
        println!("{}", &name[1..]);
    } else {
        if base_classes.len() > 1 {
            print!("{} : {}", &name[1..], &base_classes[0][6..]);

            for base in &base_classes[1..] {
                print!(", {}", &base[6..]);
            }
            println!("");
        } else {
            println!("{} : {}", name, &base_classes[0][6..]);
        }
    }

    // Find methods and print them

    for field in entry.get_children() {
        match field.get_kind() {
            EntityKind::Method => {
                print_func(&field, FunctionType::Regular);
            }

            _ => (),
        }
    }
}

fn is_header_file(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".h"))
        .unwrap_or(false)
}

fn main() {
    // Acquire an instance of `Clang`
    let clang = Clang::new().unwrap();

    let mut lock = RwLock::new(HashSet::new());

    // Get all the files to parse

    let mut header_files = Vec::new();

    for entry in WalkDir::new(
        "/Users/danielcollin/Qt/5.10.0/clang_64/lib/QtWidgets.framework/Headers/qlistwidget.h",
    ) {
        let entry = entry.unwrap();

        if !is_header_file(&entry) {
            continue;
        }

        header_files.push(entry.path().to_owned());
    }

    //println!("{:?}", header_files);

    // Do the thing on threads

    header_files.par_iter().for_each(|filename| {
        println!("filename {:?}", filename);

        // Create a new `Index`
        let index = Index::new(&clang, false, false);

        // Parse a source file into a translation unit
        let tu = index
            .parser(&filename)
            .arguments(&[
                "-std=c++11",
                "-F/Users/danielcollin/Qt/5.10.0/clang_64/lib",
                "-i /Users/danielcollin/Qt/5.10.0/clang_64/lib/QtWidgets.framework/Headers",
                "-i /Users/danielcollin/Qt/5.10.0/clang_64/lib/QtWidgets.framework/Headers/5.10.0",
            ]).parse()
            .unwrap();

        // Get the structs in this translation unit
        let structs = tu
            .get_entity()
            .get_children()
            .into_iter()
            .filter(|e| e.get_kind() == EntityKind::ClassDecl)
            .collect::<Vec<_>>();

        for struct_ in structs {
            if let Some(name) = struct_.get_name() {
                let t = name.to_owned();
                {
                    let data = lock.read().unwrap();

                    if data.contains(&t) || struct_.get_children().is_empty() {
                        continue;
                    }
                }

                {
                    let mut w = lock.write().unwrap();
                    w.insert(t);
                }

                print_class(&struct_);
            }
        }
    });
}
