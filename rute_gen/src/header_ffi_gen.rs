use api_parser::{ApiDef, Struct, Function};

///
/// As generating the C/C++ header and the Rust FFI is so close the code to generate them is
/// very similar. So instead of having pretty much the same code duplicated each "backend"
/// implements this trait so the top-level code can be shared
///
pub trait HeaderFFIGen {
    ///
    /// Generate the header for the file. Depending on the backend this includes, etc
    ///
    fn gen_header(dest: &mut String),

    ///
    /// Generate forward declarations of needed
    ///
    fn gen_forward_declaration(dest: &mut String, struct_name: &str),

    ///
    /// Generate start of enum declaration
    ///
    fn gen_enum_declaration(dest: &mut String, enum_name: &str),

    ///
    /// Generate start of end enum declaration
    ///
    fn gen_enum_declaration_end(dest: &mut String, enum_name: &str),

    ///
    /// Generate the enum entry
    ///
    fn gen_enum_entry(dest: &mut String, enum_name: &str, enum_def: &EnumEntry),

    ///
    /// Generate start of struct funcs declaration
    ///
    fn gen_funcs_declaration(dest: &mut String, struct_name: &str),

    ///
    /// Generate destroy function
    ///
    fn gen_destroy_func(dest: &mut String, function_name: &str),

    ///
    /// Generate create function for owned data function
    ///
    fn gen_owned_data_create(dest: &mut String, function_name: &str),

    ///
    /// Generate create function
    ///
    fn gen_create(dest: &mut String, function_name: &str),

    ///
    /// Generate function
    ///
    fn gen_function(dest: &mut String, func: &Function),

    ///
    /// Generate forward declarations of needed
    ///
    fn generate_post_declarations(dest: &mut String, api_def: &ApiDef),
}
