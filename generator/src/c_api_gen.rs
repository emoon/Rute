use std::io;
use std::fs::File;
use std::io::Write;
use api_parser::*;
use std::io::{Error, ErrorKind};
use std::collections::HashMap;

use heck::{CamelCase, SnakeCase};

static HEADER: &'static [u8] = b"
#pragma once\n
#include <stdint.h>
#include <stdbool.h>\n
#ifdef __cplusplus
extern \"C\" {
#endif\n\n";

static FOOTER: &'static [u8] = b"
#ifdef __cplusplus
}
#endif\n";

pub fn generate_c_function_args(func: &Function) -> String {
    let mut function_args = String::new();
    let len = func.function_args.len();

    // write arguments
    for (i, arg) in func.function_args.iter().enumerate() {
        function_args.push_str(&arg.get_c_type());
        function_args.push_str(" ");
        function_args.push_str(&arg.name);

        if i != len - 1 {
            function_args.push_str(", ");
        }
    }

    function_args
}

fn generate_func_def(f: &mut File, func: &Function) -> io::Result<()> {
    let ret_value = func.return_val
        .as_ref()
        .map_or("void".to_owned(), |r| r.get_c_type());

    // write return value and function name
    f.write_fmt(format_args!("    {} (*{})(", ret_value, func.name))?;

    func.write_c_func_def(f, |_, arg| {
        (arg.get_c_type(), arg.name.to_owned())
    })?;

    f.write_all(b";\n")
}

///
/// Generate def for connecting events
///
pub fn callback_fun_def_name(def: bool, name: &str, func: &Function) -> String {
    let mut func_def;

    if def {
        func_def = format!("void (*connect_{})(void* object, void* user_data, void (*callback)(", name);
    } else {
        func_def = format!("void connect_{}(void* object, void* user_data, void (*callback)(", name);
    }

    let arg_count = func.function_args.len();

    for (i, arg) in func.function_args.iter().enumerate() {
        func_def.push_str(&arg.get_c_type());
        func_def.push_str(" ");
        func_def.push_str(&arg.name);

        if i != arg_count - 1 {
            func_def.push_str(", ");
        }
    }

    func_def.push_str("))");
    func_def
}

fn generate_callback_def(f: &mut File, func: &Function) -> io::Result<()> {
    f.write_fmt(format_args!("    {};\n", callback_fun_def_name(true, &func.name, func)))
}

///
/// Generate the struct body
///
///
fn generate_struct_body_recursive(f: &mut File, api_def: &ApiDef, sdef: &Struct) -> io::Result<()> {
    if let Some(ref inherit_name) = sdef.inherit {
        for sdef in &api_def.entries {
            if &sdef.name == inherit_name {
                generate_struct_body_recursive(f, api_def, &sdef)?;
            }
        }
    }

    for entry in &sdef.entries {
        match *entry {
            StructEntry::Var(ref var) => {
                f.write_fmt(format_args!("    {} {};\n", var.get_c_type(), var.name))?;
            }

            StructEntry::Function(ref func) => {
                match func.func_type {
                    FunctionType::Regular => generate_func_def(f, func)?,
                    FunctionType::Callback => generate_callback_def(f, func)?,
                    _ => (),
                }
            }
        }
    }

    Ok(())
}

///
///
///
fn generate_event_func_defs(f: &mut File, api_def: &ApiDef) -> io::Result<()> {
    let mut event_names: HashMap<String, Function> = HashMap::new();

    for sdef in api_def.entries.iter().filter(|s| !s.is_pod()) {
        let mut funcs = Vec::new();
        sdef.get_event_functions(&mut funcs);

        for func in &funcs {
            let args = generate_c_function_args(&func);
            let mut found = true;

            if let Some(ref f) = event_names.get(&func.name) {
                let current_args = generate_c_function_args(&f);
                if &current_args != &args {
                    println!("Event: {} - has versions with diffrent args {} - {}", func.name, current_args, args);
                    return Err(Error::new(ErrorKind::Other, "Fail"));
                }
            } else {
                found = false;
            }

            if !found {
                event_names.insert(func.name.clone(), func.clone());
            }
        }
    }

    let mut event_list = event_names.iter().collect::<Vec<(&String, &Function)>>();
    event_list.sort_by(|a, b| a.0.cmp(b.0));

    // typedef void (*myfunc)();

    // Generate the typedefs for the function defs

    for funcs in event_list {
        f.write_fmt(format_args!("typedef void (*PU{}Func)(", funcs.0.to_camel_case()))?;

        funcs.1.write_c_func_def(f, |index, arg| {
            match index {
                0 => ("void* self_c".to_owned(), "".to_owned()),
                1 => (format!("struct PU{}*", arg.vtype.to_owned()), arg.name.to_owned()),
                _ => (arg.vtype.to_owned(), arg.name.to_owned()),
            }
        })?;

        f.write_all(b";\n")?;
    }

    f.write_all(b"\n")?;

    Ok(())
}


///
/// Main entry for generate the C API def
///
pub fn generate_c_api(filename: &str, api_def: &ApiDef) -> io::Result<()> {
    let mut f = File::create(filename)?;

    f.write_all(HEADER)?;

    // Write forward declarations

    for sdef in &api_def.entries {
        f.write_fmt(format_args!("struct PU{};\n", sdef.name))?;
    }

    f.write_all(b"\n")?;

    // Write event/callback types

    generate_event_func_defs(&mut f, api_def)?;

    // Write the struct defs

    for sdef in &api_def.entries {
        f.write_fmt(format_args!("struct PU{} {{\n", sdef.name))?;

        generate_struct_body_recursive(&mut f, api_def, sdef)?;

        if !sdef.is_pod() {
            f.write_all(b"    void* priv_data;\n")?;
        }

        f.write_fmt(format_args!("}};\n\n"))?;
    }

    // generate C_API entry

    f.write_all(b"typedef struct PU { \n")?;

    for sdef in api_def.entries.iter().filter(|s| !s.is_pod() && s.should_have_create_func()) {
        f.write_fmt(format_args!("    struct PU{}* (*create_{})(void* self);\n",
                                    sdef.name,
                                    sdef.name.to_snake_case()))?;
    }

    f.write_all(b"    void* priv_data;\n} PU;\n")?;

    f.write_all(FOOTER)?;

    Ok(())
}
