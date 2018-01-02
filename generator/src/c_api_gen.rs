use std::io;
use std::fs::File;
use std::io::Write;
use api_parser::*;
use heck::SnakeCase;

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

    func.write_c_func_def(f, |_, arg| (arg.get_c_type(), arg.name.to_owned()))?;

    f.write_all(b";\n")
}

///
/// Generate def for connecting events
///
pub fn callback_fun_def_name(def: bool, name: &str, func: &Function) -> String {
    let mut func_def;

    if def {
        func_def = format!(
            "void (*set_{}_event)(void* object, void* user_data, void (*event)(",
            name
        );
    } else {
        func_def = format!(
            "void set_{}_event(void* object, void* user_data, void (*event)(",
            name
        );
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
    f.write_fmt(format_args!(
        "    {};\n",
        callback_fun_def_name(true, &func.name, func)
    ))
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

    // Add destroy function to all structs that has a create function

    for entry in &sdef.entries {
        match *entry {
            StructEntry::Var(ref var) => {
                f.write_fmt(format_args!("    {} {};\n", var.get_c_type(), var.name))?;
            }

            StructEntry::Function(ref func) => match func.func_type {
                FunctionType::Regular => generate_func_def(f, func)?,
                FunctionType::Callback => generate_callback_def(f, func)?,
                _ => (),
            },
        }
    }

    Ok(())
}

///
/// Generate the struct body
///
///
fn generate_struct_events(f: &mut File, sdef: &Struct) -> io::Result<()> {
    for entry in &sdef.entries {
        match *entry {
            StructEntry::Function(ref func) => match func.func_type {
                FunctionType::Event => generate_callback_def(f, func)?,
                _ => (),
            },

            _ => (),
        }
    }

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

        if !sdef.is_pod() {
            f.write_fmt(format_args!("struct PU{}Funcs;\n", sdef.name))?;
        }
    }

    for trait_name in api_def.get_all_traits() {
        f.write_fmt(format_args!("struct PU{};\n", trait_name))?;
    }

    f.write_all(b"\n")?;

    // Write the struct for pods

    for sdef in api_def.entries.iter().filter(|s| s.is_pod()) {
        f.write_fmt(format_args!("struct PU{} {{\n", sdef.name))?;

        generate_struct_body_recursive(&mut f, api_def, sdef)?;
        generate_struct_events(&mut f, sdef)?;

        f.write_all(b"};\n\n")?;
    }

    // Write non-pod structs

    for sdef in api_def.entries.iter().filter(|s| !s.is_pod()) {
        f.write_fmt(format_args!("struct PU{}Funcs {{\n", sdef.name))?;

        if sdef.should_have_create_func() {
            f.write_all(b"    void (*destroy)(void* self_c);\n")?;
        }

        generate_struct_body_recursive(&mut f, api_def, sdef)?;
        generate_struct_events(&mut f, sdef)?;
        f.write_all(b"};\n\n")?;

        f.write_fmt(format_args!("struct PU{} {{\n", sdef.name))?;
        f.write_fmt(format_args!("    struct PU{}Funcs* funcs;\n", sdef.name))?;
        f.write_all(b"    void* priv_data;\n")?;

        f.write_all(b"};\n\n")?;
    }

    // generate C_API entry

    f.write_all(b"typedef struct PU { \n")?;

    for sdef in api_def
        .entries
        .iter()
        .filter(|s| !s.is_pod() && s.should_have_create_func())
    {
        f.write_fmt(format_args!(
            "    struct PU{} (*create_{})(void* self);\n",
            sdef.name,
            sdef.name.to_snake_case()
        ))?;
    }

    f.write_all(b"    void* priv_data;\n} PU;\n")?;

    f.write_all(FOOTER)?;

    Ok(())
}
