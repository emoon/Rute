extern crate clang;
extern crate heck;
extern crate rayon;
extern crate walkdir;

use clang::*;
use heck::SnakeCase;
use rayon::prelude::*;
use std::borrow::Cow;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write, SeekFrom, BufReader};
use std::sync::RwLock;
use std::io::{Seek, BufRead};
use walkdir::WalkDir;
use std::path::PathBuf;

#[derive(PartialEq, Clone, Copy, Debug)]
enum AccessLevel {
    Public,
    Signal,
    Protected,
    Private
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum IsReturnType {
    Yes,
    No,
}

#[derive(Default, Debug)]
struct ArgType {
    name: String,
    array: bool,
    pointer: bool,
}

const SKIP_NAMES: &[&'static str] = &[
    "Private",
    "operator",
    "metaclass",
    "tr",
    "metacall",
    "metaObject",
    "QString",
    "d_func",
    "qt_metacast",
    "dynamic_meta_object",
];

///
/// Translate Qt/C++ type to API def type
///
fn get_pod_type<'a>(name: &'a str) -> &'a str {
    match name {
        "quint8" => "u8",
        "quint16" => "u16",
        "quint32" => "u32",
        "qlonglong" => "i64",
        "qint64" => "i64",
        "int" => "i32",
        "bool" => "bool",
        "float" => "f32",
        "qreal64" => "f64",
        "qreal32" => "f32",
        "qreal" => "f32",
        "const char *" => "String",
        _ => name,
    }
}

///
/// Get the range of the array
///
fn get_array_range<'a>(name: &'a str) -> &str {
    let start = name.find('<').unwrap();
    let end = name.find('>').unwrap();
    &name[start + 1..end]
}

fn get_non_array_type(arg: &mut ArgType, in_name: &str, is_return: IsReturnType) {
    let name;

    // remove const
    if in_name.starts_with("const ") {
        name = &in_name[6..];
    } else {
        name = in_name;
    }

    if name.find('*').is_some() || name.find('&').is_some() {
        arg.pointer = true;
    }

    let new_name;

    // find end marker if there has been a & or *

    if let Some(offset) = name.find(" ") {
        new_name = &name[..offset];
    } else {
        new_name = name;
    }

    let new_name = get_pod_type(new_name);

    // Strip 'Q'

    if new_name.as_bytes()[0] == b'Q' {
        if new_name == "QString" {
            arg.name = "String".to_owned();
        } else {
            if is_return == IsReturnType::No {
                arg.name = format!("{}Type", &new_name[1..]);
            } else {
                arg.name = format!("{}", &new_name[1..]);
            }
        }
    } else {
        arg.name = new_name.to_owned();
    }
}

///
///
///
fn get_access_level(entry: &Entity) -> AccessLevel {
    let mut level = AccessLevel::Public;

    if let Some(clang_level) = entry.get_accessibility() {
        match clang_level {
            Accessibility::Private => level = AccessLevel::Private,
            Accessibility::Public => level = AccessLevel::Public,
            Accessibility::Protected => level = AccessLevel::Protected,
        }
    }

    // Parse file to figure out if we have slots or not

    if let Some(location) = entry.get_location() {
        if let Some(filename) = location.get_file_location().file {
            if let Ok(mut file) = File::open(filename.get_path()) {
                file.seek(SeekFrom::Start(location.get_file_location().offset as u64)).unwrap();
                let mut line = String::with_capacity(128);
                let mut reader = BufReader::new(file);
                reader.read_line(&mut line).unwrap();

                if level == AccessLevel::Public && line.contains("SIGNAL") {
                    level = AccessLevel::Signal;
                }
            }
        }
    }

    level
}


///
///
///
fn format_arg_type(arg: &ArgType, is_return: IsReturnType) -> String {
    let mut res = String::with_capacity(128);

    if arg.array {
        res.push('[');
    }

    if is_return == IsReturnType::Yes {
        res.push_str(&arg.name);
        if arg.pointer { res.push('?') }
    } else {
        if arg.pointer && arg.name != "String" {
            res.push('&');
        }

        res.push_str(&arg.name);
    }

    if arg.array {
        res.push(']');
    }

    res
}

///
///
///
fn get_complex_arg(name: &str, is_return: IsReturnType) -> String {
    let mut arg_type = ArgType::default();

    // Check if this is an enum

    if name.contains("::") {
        if name.starts_with("Qt::") {
            return format!("Rute::{}", &name[4..]).to_owned();
        } else {
            return name[1..].to_owned();
        }
    }

    let fixed_lists = &[
        ("StringList", "[String]"),
        ("ModelIndexList", "[ModelIndex]"),
    ];

    for (list_name, replace) in fixed_lists {
        if name.find(list_name).is_some() {
            if is_return == IsReturnType::No {
                return format!("&{}", replace);
            } else {
                return replace.to_string();
            }
        }
    }

    // Do the rest

    if name.contains("QList<") || name.contains("QVector<") {
        arg_type.array = true;
        get_non_array_type(&mut arg_type, get_array_range(name), is_return);
    } else {
        get_non_array_type(&mut arg_type, name, is_return);
    }

    let res = format_arg_type(&arg_type, is_return);

    res
}

///
/// Translate a parsed type into API Def type
///
fn get_arg_type<'a>(t: &'a Type, is_return: IsReturnType) -> Cow<'static, str> {
    let name = t.get_display_name();
    get_complex_arg(&name, is_return).into()
}

///
///
///
fn print_arg<W: Write>(dest: &mut W, arg: &Entity, arg_count: &mut usize) {
    let t = arg.get_type().unwrap();
    let arg_type = get_arg_type(&t, IsReturnType::No);

    if let Some(name) = arg.get_name() {
        write!(dest, "{}: ", name.to_snake_case());
    } else {
        write!(dest, "arg{}: ", arg_count);
        *arg_count += 1;
    }

    write!(dest, "{}", arg_type);
}

///
/// Print a functio/method definition
///
fn print_func<W: Write>(dest: &mut W, entry: &Entity, func_type: AccessLevel) {
    let name = match entry.get_name() {
        Some(name) => name,
        None => return,
    };

    // Never generate private functions
    if func_type == AccessLevel::Private {
        return;
    }

    for skip in SKIP_NAMES {
        if name.contains(skip) {
            return;
        }
    }

    if func_type == AccessLevel::Signal {
        write!(dest, "    [event] ");
    } else if entry.is_virtual_method() {
        write!(dest, "    [replace] ");
    } else if entry.is_static_method() {
        write!(dest, "    [static] ");
    } else {
        write!(dest, "    ");
    }

    write!(dest, "{}", name.to_snake_case());

    if let Some(args) = entry.get_arguments() {
        if args.is_empty() {
            write!(dest, "()");
        } else {
            let mut count = 0;

            write!(dest, "(");

            print_arg(dest, &args[0], &mut count);

            for arg in &args[1..] {
                write!(dest, ", ");
                print_arg(dest, arg, &mut count);
            }

            write!(dest, ")");
        }
    } else {
        write!(dest, "()");
    }

    if let Some(res) = entry.get_result_type() {
        let display_name = res.get_display_name();

        if display_name.contains("void") {
            writeln!(dest, ",");
        } else {
            writeln!(dest, " -> {},", get_arg_type(&res, IsReturnType::Yes));
        }
    } else {
        writeln!(dest, "\n");
    }
}

///
/// Print enums
///
fn print_enums<W: Write>(dest: &mut W, entry: &Entity, struct_name: &str, org_class_name: &str) {
    writeln!(dest, "[org_name({})]", org_class_name);

    if let Some(enum_name) = entry.get_display_name() {
        writeln!(dest, "enum {} {{", enum_name);
    } else {
        writeln!(dest, "enum {}FixMeEnums {{", struct_name);
    }

    for entry in entry.get_children() {
        match entry.get_kind() {
            EntityKind::EnumConstantDecl => {
                writeln!(dest, "    {},", entry.get_display_name().unwrap());
            }

            _ => (),
        }
    }

    writeln!(dest, "}}\n");
}

///
/// Print a class. This code a a bit hacky but should do for this usage
///
fn print_class(target_path: &str, entry: &Entity) {
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
    let mut enum_count = 0;

    for field in entry.get_children() {
        match field.get_kind() {
            EntityKind::BaseSpecifier => {
                //println!("    base: {:?}\n\n", field.get_name());
                base_classes.push(field.get_name().unwrap().to_owned());
            }

            EntityKind::EnumDecl => enum_count += 1,
            EntityKind::Method => method_count += 1,

            _ => (),
        }
    }

    if method_count == 0 && enum_count == 0 {
        return;
    }

    let filename = format!("{}/{}.def", target_path, &name[1..].to_snake_case());
    let mut dest = BufWriter::with_capacity(16 * 1024, File::create(filename).unwrap());

    // print all enums for the class

    for field in entry.get_children() {
        match field.get_kind() {
            EntityKind::EnumDecl => print_enums(&mut dest, &field, &name[1..], &name),
            _ => (),
        }
    }

    if method_count == 0 {
        return;
    }

    // Print class and the inharitance

    if base_classes.is_empty() {
        writeln!(dest, "\nstruct {} {{", &name[1..]);
    } else {
        if base_classes.len() > 1 {
            write!(dest, "struct {} : {}", &name[1..], &base_classes[0][7..]);

            for base in &base_classes[1..] {
                write!(dest, ", {}", &base[7..]);
            }
            writeln!(dest, " {{");
        } else {
            writeln!(dest, "struct {} : {} {{", &name[1..], &base_classes[0][7..]);
        }
    }

    let mut access_level = AccessLevel::Public;

    // Find methods and print them

    for field in entry.get_children() {
        match field.get_kind() {
            EntityKind::Method => {
                print_func(&mut dest, &field, access_level);
            }

            EntityKind::AccessSpecifier => {
                access_level = get_access_level(&field);
            }

            _ => (),
        }
    }

    writeln!(dest, "}}\n\n// vim: syntax=rust expandtab ts=4 sw=4");
}

fn is_header_file(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".h"))
        .unwrap_or(false)
}

fn is_private_file(entry: &walkdir::DirEntry) -> bool {
    entry.path().to_str().unwrap().contains("private")
}

///
/// Function for adding files/paths to process
///
fn add_process_path(dest: &mut Vec<PathBuf>, path: &str) {
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();

        if !is_header_file(&entry) {
            continue;
        }

        if is_private_file(&entry) {
            continue;
        }

        dest.push(entry.path().to_owned());
    }
}

fn main() {
    // Acquire an instance of `Clang`
    let clang = Clang::new().unwrap();
    let lock = RwLock::new(HashSet::new());

    // Get all the files to parse

    let output_directory = "output";

    let mut header_files: Vec<PathBuf> = Vec::new();

    add_process_path(&mut header_files,
        //"/Users/danielcollin/temp/test.h",
        //"/Users/danielcollin/Qt/5.10.0/clang_64/lib/QtWidgets.framework/Headers/qlistwidget.h",
        "/Users/danielcollin/Qt/5.11.2/clang_64/lib/QtWidgets.framework/Headers");

    add_process_path(&mut header_files,
        "/Users/danielcollin/Qt/5.11.2/clang_64/lib/QtCore.framework/Versions/5/Headers/qnamespace.h");

    add_process_path(&mut header_files,
        "/Users/danielcollin/Qt/5.11.2/clang_64/lib/QtGui.framework/Versions/5/Headers");

    // Process all files in parallel using Rayon
    header_files.par_iter().for_each(|filename| {
        println!("Processing filename {:?}", filename);

        // Create a new `Index`
        let index = Index::new(&clang, false, false);

        // Parse a source file into a translation unit
        let tu = index
            .parser(&filename)
            .arguments(&[
                "-std=c++11",
                "-F/Users/danielcollin/Qt/5.11.2/clang_64/lib",
                "-i /Users/danielcollin/Qt/5.11.2/clang_64/lib/QtWidgets.framework/Headers",
                "-i /Users/danielcollin/Qt/5.11.2/clang_64/lib/QtWidgets.framework/Headers/5.11.2",
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

                print_class(output_directory, &struct_);
            }
        }

        // This is somewhat of a hack to get the global Qt enums

        if filename.file_name().unwrap().to_str().unwrap().contains("qnamespace") {
            let mut enums = Vec::new();

            for e in tu.get_entity().get_children() {
                match e.get_kind() {
                    EntityKind::Namespace => {
                        if let Some(name) = e.get_display_name() {
                            if name != "Qt" {
                                continue;
                            }

                            for t in e.get_children() {
                                if t.get_kind() == EntityKind::EnumDecl {
                                    enums.push(t);
                                }
                            }
                        }
                    }

                    _ => (),
                }
            }

            // Create enums for the file. We don't need to lock the type here as we take all the enums
            // that isn't in any class and write to output with the same filename

            if !enums.is_empty() {
                let base_name = filename.file_name().unwrap().to_str().unwrap();
                let base_name = &base_name[..base_name.len() - 2];
                let target_filename = format!("{}/{}.def", output_directory, &base_name);
                let mut dest = BufWriter::with_capacity(16 * 1024, File::create(target_filename).unwrap());

                for enum_def in enums {
                    print_enums(&mut dest, &enum_def, &base_name, "Qt");
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_type_pointer() {
        assert_eq!(
            get_complex_arg("QList<QListWidgetItem *>", IsReturnType::No),
            "[&ListWidgetItemType]"
        );
    }

    #[test]
    fn test_array_type() {
        assert_eq!(
            get_complex_arg("QList<QListWidgetItem>", IsReturnType::No),
            "[ListWidgetItemType]"
        );
    }

    #[test]
    fn test_array_vector_type() {
        assert_eq!(get_complex_arg("QVector<int>", IsReturnType::No), "[i32]");
    }

    #[test]
    fn test_pointer() {
        assert_eq!(get_complex_arg("QListWidgetItem *", IsReturnType::No), "&ListWidgetItemType");
    }

    #[test]
    fn test_ref() {
        assert_eq!(get_complex_arg("QListWidgetItem &", IsReturnType::No), "&ListWidgetItemType");
    }

    #[test]
    fn test_const_string() {
        assert_eq!(get_complex_arg("const QString &", IsReturnType::No), "String");
    }

    #[test]
    fn test_string() {
        assert_eq!(get_complex_arg("QString", IsReturnType::No), "String");
    }

    #[test]
    fn test_list_int() {
        assert_eq!(get_complex_arg("QList<int>", IsReturnType::No), "[i32]");
    }

    #[test]
    fn test_vector_type() {
        assert_eq!(get_complex_arg("QVector<QCanBusFrame>", IsReturnType::No), "[CanBusFrameType]");
    }

    #[test]
    fn test_qt_global_enum() {
        assert_eq!(get_complex_arg("Qt::WindowState", IsReturnType::No), "Rute::WindowState");
    }

    #[test]
    fn test_qt_local_enum() {
        assert_eq!(get_complex_arg("QListWidget::State", IsReturnType::No), "ListWidget::State");
    }

    #[test]
    fn test_return_pointer() {
        assert_eq!(get_complex_arg("QListWidget *", IsReturnType::Yes), "ListWidget?");
    }

    #[test]
    fn test_return_regular() {
        assert_eq!(get_complex_arg("QListWidget", IsReturnType::Yes), "ListWidget");
    }
}
