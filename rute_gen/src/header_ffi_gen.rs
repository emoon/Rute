use crate::api_parser::{ApiDef, Enum, Function, Struct};
use std::fs::File;
use std::io::BufWriter;
use std::io::Result;
use std::io::Write;

///
/// As generating the C/C++ header and the Rust FFI is so close the code to generate them is
/// very similar. So instead of having pretty much the same code duplicated each "backend"
/// implements this trait so the top-level code can be shared
///
pub trait HeaderFFIGen {
    ///
    /// Generate the header for the file. Depending on the backend this includes, etc
    ///
    fn gen_header<W: Write>(&mut self, dest: &mut W) -> Result<()>;

    ///
    /// Generate header for the main FFI file
    ///
    fn gen_main_header<W: Write>(&mut self, dest: &mut W, api_defs: &[ApiDef]) -> Result<()>;

    ///
    /// Generate the footer for the main FFI file
    ///
    fn gen_main_footer<W: Write>(&mut self, dest: &mut W, api_defs: &[ApiDef]) -> Result<()>;

    ///
    /// Generate forward declarations of needed
    ///
    fn gen_forward_declaration<W: Write>(&mut self, dest: &mut W, sdef: &Struct) -> Result<()>;

    ///
    /// Generate the enum
    ///
    fn gen_enum<W: Write>(&mut self, dest: &mut W, enum_def: &Enum) -> Result<()>;

    ///
    /// Generate start of struct declaration
    ///
    fn gen_struct_declaration<W: Write>(&mut self, dest: &mut W, struct_name: &str) -> Result<()>;

    ///
    /// Generate end of struct declaration
    ///
    fn gen_struct_end_declaration<W: Write>(
        &mut self,
        dest: &mut W,
        struct_name: &str,
    ) -> Result<()>;

    ///
    /// Generate destroy function
    ///
    fn gen_destroy_func<W: Write>(&mut self, dest: &mut W, function_name: &str) -> Result<()>;

    ///
    /// Generate create function for owned data function
    ///
    fn gen_owned_data_create<W: Write>(&mut self, dest: &mut W, struct_name: &str) -> Result<()>;

    ///
    /// Generate create function
    ///
    fn gen_create_gen<W: Write>(
        &mut self,
        dest: &mut W,
        prefix: &str,
        struct_name: &str,
    ) -> Result<()>;

    ///
    /// Generate function
    ///
    fn gen_function<W: Write>(&mut self, dest: &mut W, func: &Function) -> Result<()>;

    ///
    /// Generate void data entry
    ///
    fn gen_rubase_ptr_data<W: Write>(&mut self, dest: &mut W, name: &str) -> Result<()>;

    ///
    /// Generate the funcs declaration
    ///
    fn gen_funcs_declaration<W: Write>(
        &mut self,
        dest: &mut W,
        name: &str,
        type_name: &str,
    ) -> Result<()>;

    ///
    /// Generate extra things if needed
    ///
    fn generate_post_declarations<W: Write>(
        &mut self,
        dest: &mut W,
        api_def: &ApiDef,
    ) -> Result<()>;
}

pub struct HeaderFFIGenerator;

impl HeaderFFIGenerator {
    ///
    ///  Main generation function that generates the structs
    ///
    pub fn generate<T: HeaderFFIGen>(filename: &str, api_def: &ApiDef, mut imp: T) -> Result<()> {
        let mut temp_string = String::with_capacity(128);
        let mut dest = BufWriter::with_capacity(512 * 1024, File::create(filename)?);

        // Generate header
        imp.gen_header(&mut dest)?;

        // Generate forward declarations if needed
        for sdef in &api_def.class_structs {
            imp.gen_forward_declaration(&mut dest, &sdef)?;
        }

        // generate all structs
        for sdef in &api_def.class_structs {
            // Construct Funcs name
            temp_string.clear();
            temp_string.push_str("RU");
            temp_string.push_str(&sdef.name);
            temp_string.push_str("Funcs");

            // Generate the funcs implementation
            imp.gen_struct_declaration(&mut dest, &temp_string)?;

            if sdef.should_have_create_func() {
                imp.gen_destroy_func(&mut dest, &sdef.name)?;
            }

            // Generate all functions
            for func in &sdef.functions {
                imp.gen_function(&mut dest, &func)?;
            }

            imp.gen_struct_end_declaration(&mut dest, &temp_string)?;

            // Generate the AllFuncs struct
            temp_string.clear();
            temp_string.push_str("RU");
            temp_string.push_str(&sdef.name);
            temp_string.push_str("AllFuncs");

            imp.gen_struct_declaration(&mut dest, &temp_string)?;

            for name in &sdef.full_inherit {
                imp.gen_funcs_declaration(&mut dest, name, name)?;
            }

            imp.gen_struct_end_declaration(&mut dest, &temp_string)?;

            // Construct struct name
            temp_string.clear();
            temp_string.push_str("RU");
            temp_string.push_str(&sdef.name);

            // Generate the struct implementation
            imp.gen_struct_declaration(&mut dest, &temp_string)?;
            imp.gen_rubase_ptr_data(&mut dest, "qt_data")?;
            imp.gen_rubase_ptr_data(&mut dest, "host_data")?;
            imp.gen_funcs_declaration(&mut dest, "all", &format!("{}All", sdef.name))?;
            imp.gen_struct_end_declaration(&mut dest, &temp_string)?;
        }

        imp.generate_post_declarations(&mut dest, api_def)
    }

    ///
    /// Generate the main file (main entry + create functions and such)
    ///
    pub fn generate_main<T: HeaderFFIGen>(
        filename: &str,
        api_defs: &[ApiDef],
        mut imp: T,
    ) -> Result<()> {
        let mut dest = BufWriter::new(File::create(filename)?);

        imp.gen_main_header(&mut dest, api_defs)?;

        // Generate the main entry
        imp.gen_struct_declaration(&mut dest, "RuteFFI")?;

        for sdef in api_defs
            .iter()
            .flat_map(|d| d.class_structs.iter())
            .filter(|sdef| sdef.should_have_create_func())
        {
            if sdef.should_gen_wrap_class() {
                imp.gen_owned_data_create(&mut dest, &sdef.name)?;
            } else {
                imp.gen_create_gen(&mut dest, "create", &sdef.name)?;
            }

            if sdef.has_static_functions() {
                imp.gen_create_gen(&mut dest, "get", &sdef.name)?;
            }
        }

        imp.gen_struct_end_declaration(&mut dest, "RuteFFI")?;
        imp.gen_main_footer(&mut dest, api_defs)
    }
}
