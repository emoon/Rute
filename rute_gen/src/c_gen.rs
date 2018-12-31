use crate::api_parser::*;
use crate::header_ffi_gen::HeaderFFIGen;
use heck::SnakeCase;
use std::collections::BTreeMap;
///
/// This code is responisble for generating the Rute.h file that allows usage of Rute from C
///
use std::io;
use std::io::Write;

///
/// Base header for all header files
///
static HEADER: &str = "// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once\n
#include <stdint.h>
#include <stdbool.h>\n
#include \"../rute_base.h\"

#ifdef __cplusplus
extern \"C\" {
#endif";

///
/// Base header for all header files
///
static MAIN_HEADER: &str = "// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once\n
#include <stdint.h>
#include <stdbool.h>\n
#include \"../rute_base.h\"
";

static MAIN_EXTERN_C: &str = "#ifdef __cplusplus
extern \"C\" {
#endif
";

///
/// Footer that is generated at the end of the the file
///
static FOOTER: &str = "
#ifdef __cplusplus
}
#endif\n";

pub struct CapiHeaderGen {
    temp_string: String,
}

impl HeaderFFIGen for CapiHeaderGen {
    ///
    /// Generate the header for the file
    ///
    fn gen_header<W: Write>(&mut self, dest: &mut W) -> io::Result<()> {
        writeln!(dest, "{}", HEADER)
    }

    ///
    /// Generate forward declarations
    ///
    fn gen_forward_declaration<W: Write>(&mut self, dest: &mut W, sdef: &Struct) -> io::Result<()> {
        let mut includes = BTreeMap::new();

        for func in &sdef.functions {
            if let Some(ref ret_val) = func.return_val {
                match ret_val.vtype {
                    VariableType::Regular => {
                        includes.insert(&ret_val.type_name, ());
                    }
                    VariableType::Reference => {
                        includes.insert(&ret_val.type_name, ());
                    }

                    _ => (),
                }
            }
        }

        for (name, _) in includes {
            writeln!(dest, "#include \"{}_ffi.h\"", name.to_snake_case())?;
        }

        writeln!(dest, "")?;

        writeln!(dest, "struct RU{}Funcs;", sdef.name)?;
        writeln!(dest, "struct RU{};", sdef.name)?;

        writeln!(dest, "")
    }

    ///
    /// Generate enum
    ///
    fn gen_enum<W: Write>(&mut self, dest: &mut W, enum_def: &Enum) -> io::Result<()> {
        writeln!(dest, "typedef enum RU{} {{\n", enum_def.name)?;

        for e in &enum_def.entries {
            writeln!(dest, "    RU{}_{} = {},", enum_def.name, e.name, e.value)?;
        }

        writeln!(dest, "}} RU{};\n", enum_def.name)
    }

    ///
    /// Generate start of struct declaration
    ///
    fn gen_struct_declaration<W: Write>(
        &mut self,
        dest: &mut W,
        struct_name: &str,
    ) -> io::Result<()> {
        writeln!(dest, "typedef struct {} {{", struct_name)
    }

    ///
    /// Generate end of struct declaration
    ///
    fn gen_struct_end_declaration<W: Write>(
        &mut self,
        dest: &mut W,
        struct_name: &str,
    ) -> io::Result<()> {
        writeln!(dest, "}} {};\n", struct_name)
    }

    ///
    /// Generate destroy function
    ///
    fn gen_destroy_func<W: Write>(&mut self, dest: &mut W, _function_name: &str) -> io::Result<()> {
        writeln!(dest, "    void (*destroy)(struct RUBase* self);")
    }

    ///
    /// Generate create function for owned data function
    ///
    fn gen_owned_data_create<W: Write>(
        &mut self,
        dest: &mut W,
        struct_name: &str,
    ) -> io::Result<()> {
        writeln!(
            dest,
            "    struct RU{} (*create_{})(
        struct RUBase* priv_data,
        RUDeleteCallback delete_callback, void* host_data);",
            struct_name,
            struct_name.to_snake_case()
        )
    }

    ///
    /// Generate create function
    ///
    fn gen_create_gen<W: Write>(
        &mut self,
        dest: &mut W,
        prefix: &str,
        struct_name: &str,
    ) -> io::Result<()> {
        writeln!(
            dest,
            "    struct RU{} (*{}_{})(struct RUBase* priv_data);",
            struct_name,
            prefix,
            struct_name.to_snake_case()
        )
    }
    ///
    /// Generate the funcs declaration
    ///
    fn gen_funcs_declaration<W: Write>(
        &mut self,
        dest: &mut W,
        name: &str,
        type_name: &str,
    ) -> io::Result<()> {
        writeln!(
            dest,
            "    struct RU{}Funcs* {}_funcs;",
            type_name,
            name.to_snake_case()
        )
    }

    ///
    /// Generate function
    ///
    fn gen_function<W: Write>(&mut self, dest: &mut W, func: &Function) -> io::Result<()> {
        match func.func_type {
            FunctionType::Regular => self.generate_func_def(dest, func)?,
            FunctionType::Static => self.generate_func_def(dest, func)?,
            FunctionType::Signal => {
                self.generate_callback_def(dest, "_event", func)?;
                writeln!(dest, ");")?;
            }
            FunctionType::Event => {
                self.generate_callback_def(dest, "_event", func)?;
                writeln!(dest, ");")?;
                writeln!(dest, "    void (*remove_{})(void* object);", func.name)?;
            }
        }

        Ok(())
    }

    ///
    /// Generate void data entry
    ///
    fn gen_rubase_ptr_data<W: Write>(&mut self, dest: &mut W, name: &str) -> io::Result<()> {
        writeln!(dest, "    RUBase* {};", name)
    }

    ///
    /// Generate forward declarations of needed
    ///
    fn generate_post_declarations<W: Write>(
        &mut self,
        dest: &mut W,
        api_def: &ApiDef,
    ) -> io::Result<()> {
        // generate extern declarations for all funcs so other files can access them

        for sdef in &api_def.class_structs {
            let snake_name = sdef.name.to_snake_case();
            writeln!(dest, "extern RU{}Funcs s_{}_funcs;", sdef.name, snake_name)?;
            writeln!(
                dest,
                "extern RU{}AllFuncs s_{}_all_funcs;",
                sdef.name, snake_name
            )?;
        }
        write!(dest, "{}", FOOTER)
    }

    ///
    /// Generate header for the main FFI file
    ///
    fn gen_main_header<W: Write>(&mut self, dest: &mut W, api_defs: &[ApiDef]) -> io::Result<()> {
        writeln!(dest, "{}", MAIN_HEADER)?;

        for sdef in api_defs {
            // TODO: Fix me
            if sdef.base_filename != "qnamespace" {
                writeln!(
                    dest,
                    "#include \"{}_ffi.h\"",
                    sdef.base_filename.to_snake_case()
                )?;
            }
        }

        writeln!(dest, "\n{}", MAIN_EXTERN_C)
    }

    ///
    /// Generate the footer for the main FFI file
    ///
    fn gen_main_footer<W: Write>(&mut self, dest: &mut W, _api_defs: &[ApiDef]) -> io::Result<()> {
        writeln!(dest, "extern RuteFFI* rute_static_ffi_get();\n")?;
        writeln!(dest, "{}", FOOTER)
    }
}

impl CapiHeaderGen {
    pub fn new() -> CapiHeaderGen {
        CapiHeaderGen {
            temp_string: String::with_capacity(1024),
        }
    }

    ///
    /// Generate def for connecting events
    ///
    /// TODO: Cleanup this code
    pub fn callback_fun_def_name(
        dest: &mut String,
        def: bool,
        name: &str,
        _post_name: &str,
        func: &Function,
    ) {
        use std::fmt::Write;

        let ret_value = func
            .return_val
            .as_ref()
            .map_or("void".into(), |r| r.get_c_type(IsReturnType::Yes));

        if def {
            write!(dest,
                "void (*set_{}_event)(void* object, void* user_data, void* wrapped_func, {} (*event)(",
                func.get_name_skip_event(), ret_value).unwrap()
        } else {
            write!(
                dest,
                "void set_{}_event(void* object, void* user_data, void* wrapped_func, {} (*event)(",
                func.get_name_skip_event(), ret_value,
            ).unwrap();
        };

        write!(
            dest,
            "{})",
            func.gen_c_def_filter(Some(Some("void*, void*".into())), |_, _| None)
        ).unwrap()
    }

    ///
    /// Code to write down callback def
    ///
    fn generate_callback_def<W: Write>(
        &mut self,
        dest: &mut W,
        post_name: &str,
        func: &Function,
    ) -> io::Result<()> {
        self.temp_string.clear();

        Self::callback_fun_def_name(&mut self.temp_string, true, &func.name, post_name, func);
        write!(dest, "    {}", self.temp_string)
    }

    ///
    /// Generate function definition in the style of
    ///
    /// struct Foo (*foobar)(uint32_t x, uint32_t)
    ///
    fn generate_func_def<W: Write>(&mut self, dest: &mut W, func: &Function) -> io::Result<()> {
        let ret_value = func
            .return_val
            .as_ref()
            .map_or("void".into(), |r| r.get_c_type(IsReturnType::Yes));

        // write return value and function name
        writeln!(
            dest,
            "    {} (*{})({});",
            ret_value,
            func.name,
            func.generate_c_function_def(FirstArgType::Keep)
        )
    }
}
