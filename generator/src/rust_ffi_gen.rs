use std::io;
use std::fs::File;
use std::io::Write;
use heck::SnakeCase;
use api_parser::*;

impl Variable {
    pub fn get_rust_ffi_type(&self) -> String {
        if self.primitive {
            self.vtype.clone()
        } else {
            if self.vtype == "String" {
                "*const ::std::os::raw::c_char".to_owned()
            } else if self.vtype == "self" || self.reference {
                "*const PUBase".to_owned()
            } else if self.array {
                "PUArray".to_owned()
            } else {
                format!(" PU{}", self.vtype)
            }
        }
    }
}

///
/// Generate ffi function
///
fn generate_ffi_function(f: &mut File, func: &Function) -> io::Result<()> {
    f.write_fmt(format_args!("    pub {}: extern \"C\" fn", func.name))?;

    func.write_func_def_full(f, |_, arg| (arg.name.to_owned(), arg.get_rust_ffi_type()))?;
    f.write_all(b",\n")
}

///
/// Generate ffi function
///
fn generate_ffi_callback(f: &mut File, func: &Function) -> io::Result<()> {
    f.write_fmt(format_args!(
        "    pub set_{}_event: extern \"C\" fn(object: *const PUBase, user_data: *const c_void,
                                        callback: extern \"C\" fn(",
        func.name
    ))?;
    func.write_func_def(f, |index, arg| {
        if index == 0 {
            (arg.name.to_owned(), "*const c_void".to_owned())
        } else {
            (arg.name.to_owned(), arg.get_rust_ffi_type())
        }
    })?;
    f.write_all(b")),\n")
}

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
                f.write_fmt(format_args!(
                    "    pub {}: {},\n",
                    var.name,
                    var.get_rust_ffi_type()
                ))?;
            }

            StructEntry::Function(ref func) => match func.func_type {
                FunctionType::Regular => generate_ffi_function(f, func)?,
                FunctionType::Callback => generate_ffi_callback(f, func)?,
                _ => (),
            },
        }
    }

    Ok(())
}

fn generate_struct_body_events(f: &mut File, sdef: &Struct) -> io::Result<()> {
    for entry in &sdef.entries {
        match *entry {
            StructEntry::Function(ref func) => match func.func_type {
                FunctionType::Event => generate_ffi_callback(f, func)?,
                _ => (),
            },

            _ => (),
        }
    }

    Ok(())
}

pub fn generate_ffi_bindings(
    filename: &str,
    api_def: &ApiDef,
    structs: &Vec<Struct>,
) -> io::Result<()> {
    let mut f = File::create(filename)?;

    f.write_all(b"use std::os::raw::c_void;\n\n")?;

    f.write_all(b"#[repr(C)]\n")?;
    f.write_all(b"#[derive(Default, Copy, Clone, Debug)]\n")?;
    f.write_all(b"pub struct PUBase {\n")?;
    f.write_all(b"    _unused: [u8; 0],\n")?;
    f.write_all(b"}\n\n")?;

    f.write_all(b"#[repr(C)]\n")?;
    f.write_all(b"#[derive(Copy, Clone, Debug)]\n")?;
    f.write_all(b"pub struct PUArray {\n")?;
    f.write_all(b"    elements: *const c_void,\n")?;
    f.write_all(b"    count: i32,\n")?;
    f.write_all(b"}\n\n")?;

    // Write the trait forward structs

    for trait_name in api_def.get_all_traits() {
        f.write_all(b"#[repr(C)]\n")?;
        f.write_all(b"#[derive(Default, Copy, Clone, Debug)]\n")?;
        f.write_fmt(format_args!("pub struct PU{} {{\n", trait_name))?;
        f.write_all(b"    _unused: [u8; 0],\n")?;
        f.write_all(b"}\n\n")?;
    }

    // Write the struct for pods

    for sdef in api_def.entries.iter().filter(|s| s.is_pod()) {
        f.write_all(b"#[repr(C)]\n")?;
        f.write_fmt(format_args!("pub struct PU{} {{\n", sdef.name))?;

        generate_struct_body_recursive(&mut f, api_def, sdef)?;
        generate_struct_body_events(&mut f, sdef)?;

        f.write_all(b"}\n\n")?;
    }

    // Write non-pod structs

    for sdef in api_def.entries.iter().filter(|s| !s.is_pod()) {
        f.write_all(b"#[repr(C)]\n")?;
        f.write_fmt(format_args!("pub struct PU{}Funcs {{\n", sdef.name))?;

        if sdef.should_have_create_func() {
            f.write_all(b"    pub destroy: extern \"C\" fn(self_c: *const PUBase),\n")?;
        }

        generate_struct_body_recursive(&mut f, api_def, &sdef)?;
        generate_struct_body_events(&mut f, &sdef)?;
        f.write_all(b"}\n\n")?;

        f.write_all(b"#[repr(C)]\n")?;
        f.write_all(b"#[derive(Copy, Clone)]\n")?;
        f.write_fmt(format_args!("pub struct PU{} {{\n", sdef.name))?;
        f.write_fmt(format_args!(
            "    pub funcs: *const PU{}Funcs,\n",
            sdef.name
        ))?;
        f.write_all(b"    pub privd: *const PUBase,\n")?;
        f.write_all(b"}\n\n")?;
    }

    f.write_all(b"#[repr(C)]\n")?;
    f.write_all(b"pub struct PU {\n")?;

    for struct_ in structs
        .iter()
        .filter(|s| !s.is_pod() && s.should_have_create_func())
    {
        f.write_fmt(format_args!(
            "    pub create_{}: extern \"C\" fn(priv_data: *const PUBase) -> PU{},\n",
            struct_.name.to_snake_case(),
            struct_.name
        ))?;
    }

    f.write_all(b"    pub privd: *const PUBase,\n")?;
    f.write_all(b"}\n\n")?;

    Ok(())
}
