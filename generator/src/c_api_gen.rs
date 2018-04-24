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
#endif\n
struct PUBase;\n
struct PUArray {
    void* priv_data;
    void* elements;
    uint32_t count;
};\n\n";

static FOOTER: &'static [u8] = b"
#ifdef __cplusplus
}
#endif\n";

pub fn generate_c_function_args(func: &Function) -> String {
    let mut function_args = String::new();
    let len = func.function_args.len();

    // write arguments
    for (i, arg) in func.function_args.iter().enumerate() {
        function_args.push_str(&arg.get_c_type(false));
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
        .map_or("void".to_owned(), |r| r.get_c_type(false));

    // write return value and function name
    f.write_fmt(format_args!("    {} (*{})(", ret_value, func.name))?;

    func.write_c_func_def(f, |_, arg| (arg.get_c_type(false), arg.name.to_owned()))?;

    f.write_all(b";\n")
}

//#define PUWidget_show(widget) widget.funcs->show(widget.priv_data)

fn generate_define_func(f: &mut File, base_name: &str, func: &Function) -> io::Result<()> {
    f.write_fmt(format_args!("#define PU{}_{}(", base_name, func.name))?;

    func.write_c_func_def(f, |index, arg| {
		if index == 0 {
    		(String::new(), "obj".to_owned())
		} else {
    		(String::new(), arg.name.to_owned())
		}
    })?;

    f.write_fmt(format_args!(" obj.funcs->{}(", func.name))?;

    func.write_c_func_def(f, |index, arg| {
		if index == 0 {
    		(String::new(), "obj.priv_data".to_owned())
		} else {
    		(String::new(), arg.name.to_owned())
		}
    })?;

    f.write_all(b"\n")
}

fn generate_define_static_func(f: &mut File, base_name: &str, func: &Function) -> io::Result<()> {
    f.write_fmt(format_args!("#define PU{}_{}(", base_name, func.name))?;

    func.write_c_func_def(f, |index, arg| {
		if index == 0 {
    		(String::new(), "ui".to_owned())
		} else {
    		(String::new(), arg.name.to_owned())
		}
    })?;

    f.write_fmt(format_args!(" ui->{}(", func.name))?;

    func.write_c_func_def(f, |index, arg| {
		if index == 0 {
    		(String::new(), "obj.priv_data".to_owned())
		} else {
    		(String::new(), arg.name.to_owned())
		}
    })?;

    f.write_all(b"\n")
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
        if i == 0 {
            func_def.push_str("void*");
        } else {
            func_def.push_str(&arg.get_c_type(false));
        }

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
                f.write_fmt(format_args!("    {} {};\n", var.get_c_type(false), var.name))?;
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
fn generate_defines_recursive(f: &mut File, base_name: &str, api_def: &ApiDef, sdef: &Struct) -> io::Result<()> {
    if let Some(ref inherit_name) = sdef.inherit {
        for sdef in &api_def.entries {
            if &sdef.name == inherit_name {
                generate_defines_recursive(f, base_name, api_def, &sdef)?;
            }
        }
    }

    // Add destroy function to all structs that has a create function

    for entry in &sdef.entries {
        match *entry {
            StructEntry::Function(ref func) => match func.func_type {
                FunctionType::Regular => generate_define_func(f, base_name, func)?,
                FunctionType::Callback => {
                	f.write_fmt(
                		format_args!("#define PU{}_set_{}_event(obj, user_data, event) obj.funcs->set_{}_event(obj.priv_data, user_data, event)\n",
                		base_name, func.name, func.name))?;
                }
                _ => (),
            },

            _ => (),
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
/// Generate defines
///

fn generate_defines(f: &mut File, api_def: &ApiDef) -> io::Result<()> {
    for sdef in api_def.entries.iter().filter(|s| !s.is_pod()) {
        generate_defines_recursive(f, &sdef.name, api_def, sdef)?;
    	f.write_all(b"\n")?;

        //generate_struct_events(&mut f, sdef)?;
    }

    for sdef in api_def
        .entries
        .iter()
        .filter(|s| !s.is_pod() && s.should_have_create_func())
    {
		let name = sdef.name.to_snake_case();

        f.write_fmt(format_args!(
            "#define PU_create_{}(ui) ui->create_{}(ui->priv_data)\n",
            name, name))?;
    }

	f.write_all(b"\n")?;

    for func in api_def.get_all_static_functions() {
		generate_define_static_func(f, "", &func)?;
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

    for enum_def in &api_def.enums {
        f.write_fmt(format_args!("typedef enum PU{} {{\n", enum_def.name))?;

        for entry in &enum_def.entries {
            match *entry {
                EnumEntry::Enum(ref name) => f.write_fmt(format_args!("    PU{}_{},\n", enum_def.name, name))?,
                EnumEntry::EnumValue(ref name, ref val) => f.write_fmt(format_args!("    PU{}_{} = {},\n", enum_def.name, name, val))?,
            }
        }

        f.write_fmt(format_args!("}} PU{};\n\n", enum_def.name))?;
    }

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
            f.write_all(b"    void (*destroy)(struct PUBase* self_c);\n")?;
        }

        generate_struct_body_recursive(&mut f, api_def, sdef)?;
        generate_struct_events(&mut f, sdef)?;
        f.write_all(b"};\n\n")?;

        f.write_fmt(format_args!("struct PU{} {{\n", sdef.name))?;
        f.write_fmt(format_args!("    struct PU{}Funcs* funcs;\n", sdef.name))?;
        f.write_all(b"    struct PUBase* priv_data;\n")?;

        f.write_all(b"};\n\n")?;
    }

    ////////////////////////////
    // generate plugin instance

    f.write_all(b"typedef struct PUPluginUI { \n")?;

    for sdef in api_def
        .entries
        .iter()
        .filter(|s| !s.is_pod() && s.should_have_create_func_plugin())
    {
        f.write_fmt(format_args!(
            "    struct PU{} (*create_{})(struct PUBase* self);\n",
            sdef.name,
            sdef.name.to_snake_case()
        ))?;
    }

    for func in api_def.get_all_static_functions() {
        generate_func_def(&mut f, &func)?;
    }

    f.write_all(b"    struct PUBase* priv_data;\n} PUPlugin;\n\n")?;

    ///////////////////////
    // generate C_API entry

    f.write_all(b"typedef struct PU { \n")?;

    for sdef in api_def
        .entries
        .iter()
        .filter(|s| !s.is_pod() && s.should_have_create_func())
    {
        f.write_fmt(format_args!(
            "    struct PU{} (*create_{})(struct PUBase* self);\n",
            sdef.name,
            sdef.name.to_snake_case()
        ))?;
    }

    for func in api_def.get_all_static_functions() {
        generate_func_def(&mut f, &func)?;
    }

    // generate func for create and destroy a plugin interface instance

    f.write_all(b"    struct PUPluginUI* (*create_plugin_ui)(struct PUBase* self, struct PUBase* parent);\n")?;
    f.write_all(b"    void (*destroy_plugin_ui)(struct PUPluginUI* self);\n")?;
    f.write_all(b"    struct PUBase* priv_data;\n} PU;\n\n")?;

    // Generate all the defines to make usage of the C api easier

	generate_defines(&mut f, api_def)?;

    f.write_all(FOOTER)?;

    Ok(())
}
