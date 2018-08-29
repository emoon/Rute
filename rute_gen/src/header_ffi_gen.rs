use api_parser::{ApiDef, Function, Enum};

///
/// As generating the C/C++ header and the Rust FFI is so close the code to generate them is
/// very similar. So instead of having pretty much the same code duplicated each "backend"
/// implements this trait so the top-level code can be shared
///
pub trait HeaderFFIGen {
    ///
    /// Generate the header for the file. Depending on the backend this includes, etc
    ///
    fn gen_header(dest: &mut String);

    ///
    /// Generate forward declarations of needed
    ///
    fn gen_forward_declaration(dest: &mut String, struct_name: &str);

    ///
    /// Generate the enums
    ///
    fn gen_enums(dest: &mut String, enum_def: &Enum);

    ///
    /// Generate start of struct declaration
    ///
    fn gen_struct_declaration(dest: &mut String, struct_name: &str);

    ///
    /// Generate end of struct declaration
    ///
    fn gen_struct_end_declaration(dest: &mut String, struct_name: &str);

    ///
    /// Generate destroy function
    ///
    fn gen_destroy_func(dest: &mut String, function_name: &str);

    ///
    /// Generate create function for owned data function
    ///
    fn gen_owned_data_create(dest: &mut String, function_name: &str);

    ///
    /// Generate create function
    ///
    fn gen_create(dest: &mut String, function_name: &str);

    ///
    /// Generate function
    ///
    fn gen_function(dest: &mut String, func: &Function);

    ///
    /// Generate void data entry
    ///
    fn gen_void_ptr_data(dest: &mut String, name: &str);

    ///
    /// Generate extra things if needed 
    ///
    fn generate_post_declarations(dest: &mut String, api_def: &ApiDef);
}
