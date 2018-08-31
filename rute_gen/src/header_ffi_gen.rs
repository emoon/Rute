use api_parser::{ApiDef, Enum, Function};
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
    /// Generate forward declarations of needed
    ///
    fn gen_forward_declaration<W: Write>(
        &mut self,
        dest: &mut W,
        struct_name: &str,
    ) -> Result<()>;

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
    fn gen_create<W: Write>(&mut self, dest: &mut W, struct_name: &str) -> Result<()>;

    ///
    /// Generate function
    ///
    fn gen_function<W: Write>(&mut self, dest: &mut W, func: &Function) -> Result<()>;

    ///
    /// Generate void data entry
    ///
    fn gen_void_ptr_data<W: Write>(&mut self, dest: &mut W, name: &str) -> Result<()>;

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
    /// Performe the generator
    ///
    pub fn generate<T: HeaderFFIGen>(filename: &str, api_def: &ApiDef, mut imp: T) -> Result<()> {
        let mut dest = BufWriter::with_capacity(1024 * 1024, File::create(filename)?);

        // Generate header
        imp.gen_forward_declaration(&mut dest, "test")?;
        imp.gen_header(&mut dest)?;

        // Generate forward declarations if needed
        for sdef in &api_def.class_structs {
           imp.gen_forward_declaration(&mut dest, &sdef.name)?;
        }

        // Generate all enums
        for enum_def in &api_def.enums {
           imp.gen_enum(&mut dest, enum_def)?;
        }

        // Generate any last bits if needed
        imp.generate_post_declarations(&mut dest, api_def)
    }
}
