use pest::iterators::Pair;
use pest::Parser;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::Path;

//#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("api.pest");

///
/// Current primitive types
///
const PRMITIVE_TYPES: &[&str] = &[
    "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64", "bool", "f32", "f64",
];

///
/// Attribute tagged on struct of they shouldn't have a create function in the main
/// exported Rute struct
///
static ATTRIB_NO_CREATE: &'static str = "NoCreate";
static ATTRIB_MANUAL_CREATE: &'static str = "ManualCreate";
static ATTRIB_NO_WRAP_CLASS: &'static str = "NoWrapClass";
static ATTRIB_NO_CPP_CLONE: &'static str = "NoCppClone";
static ATTRIB_DROP: &'static str = "Drop";

///
/// Variable type
///
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum VariableType {
    None,
    /// Self (aka this pointer in C++ and self in Rust)
    SelfType,
    /// Enum type
    Enum,
    /// Struct/other type
    Regular,
    /// String type
    Str,
    /// Prmitive type (such as i32,u64,etc)
    Primitive,
    /// Reference type
    Reference,
}

///
/// Holds the data for a variable. It's name and it's type and additional flags
///
#[derive(Debug, Clone)]
pub struct Variable {
    /// Documentation
    pub doc_comments: String,
    /// Which def file this variable comes from
    pub def_file: String,
    /// Name of the variable
    pub name: String,
    /// Type of the variable
    pub vtype: VariableType,
    /// Name of the variable type
    pub type_name: String,
    /// Name that maps to the QClass/Typename
    pub qt_type_name: String,
    /// Rest name of a enum. "test" in the case of Rute::test
    pub enum_sub_type: String,
    /// If variable is an array
    pub array: bool,
    /// If variable is optional (nullable)
    pub optional: bool,
    /// If the type is a reference it may be a pointer on the C++ side
    /// so this flag is used for the Qt C++ code generation
    pub pointer: bool,
    /// If the type is of Regular it can be either a class or pod type
    pub class_type: bool,
}

///
/// Default implementation for Variable
///
impl Default for Variable {
    fn default() -> Self {
        Variable {
            name: String::new(),
            doc_comments: String::new(),
            def_file: String::new(),
            vtype: VariableType::None,
            type_name: String::new(),
            enum_sub_type: String::new(),
            qt_type_name: String::new(),
            array: false,
            optional: false,
            pointer: false,
            class_type: false,
        }
    }
}

///
/// Function type
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FunctionType {
    /// This is a regular function in a Qt Class
    Regular,
    /// Event functions maps to a virtual override in a Qt Class
    Event,
    /// Signal functions maps to a signal in Qt
    Signal,
    /// Static function is that doesn't belong to a class
    Static,
}

///
/// Holds the data for a function. Name, function_args, return_type, etc
///
#[derive(Debug, Clone)]
pub struct Function {
    /// Documentation
    pub doc_comments: String,
    /// Which def file this function comes from
    pub def_file: String,
    /// Name of the function
    pub name: String,
    /// This is the C++ name of the function. Most of the time it will
    /// be the same as name but it can vary
    pub cpp_name: String,
    /// Function argumnts
    pub function_args: Vec<Variable>,
    /// Return value
    pub return_val: Option<Variable>,
    /// Type of function. See FunctionType descrition for more info
    pub func_type: FunctionType,
    /// If the function is manually implemented (not auto-generated)
    pub is_manual: bool,
}

///
/// Default implementation for Function
///
impl Default for Function {
    fn default() -> Self {
        Function {
            doc_comments: String::new(),
            name: String::new(),
            def_file: String::new(),
            cpp_name: String::new(),
            function_args: Vec::new(),
            return_val: None,
            func_type: FunctionType::Regular,
            is_manual: false,
        }
    }
}

///
/// Holds the data for a struct
///
#[derive(Debug, Default)]
pub struct Struct {
    /// Docummentanion
    pub doc_comments: String,
    /// Name
    pub name: String,
    /// Which def file this struct comes from
    pub def_file: String,
    /// Name for the C++ class in the generation
    pub cpp_name: String,
    /// Name of the Qt Class/Struct
    pub qt_name: String,
    /// Variables in the struct
    pub variables: Vec<Variable>,
    /// Functions for the struct
    pub functions: Vec<Function>,
    /// Attributes of thu struct
    pub attributes: Vec<String>,
    /// Traits
    pub traits: Vec<String>,
    /// If the struct inherits another
    pub inherit: Option<Vec<String>>,
    /// The full inherit chain
    pub full_inherit: Vec<String>,
    /// If the struct is a widget or not
    pub is_widget: bool,
    /// If there are other structs that inherits this one
    /// we need to generate a trait implementation for it
    pub should_generate_trait: bool,
}

///
/// C/C++ style enum
///
#[derive(Debug)]
pub struct EnumEntry {
    /// Name of the enum entry
    pub name: String,
    /// Value of the enum entry
    pub value: u64,
}

///
/// Enums in C++ can have same value for different enum ids. This isn't supported in Rust.
/// Also Rust doesn't support that your "or" enums flags so we need to handle that.
///
#[derive(Debug)]
pub enum EnumType {
    /// All values are in sequantial order and no overlap
    Regular,
    /// This enum is constructed with bitflags due to being power of two or overlapping values
    Bitflags,
}

impl Default for EnumType {
    fn default() -> EnumType {
        EnumType::Regular
    }
}

///
/// Maps to a C++ enum
///
#[derive(Debug, Default)]
pub struct Enum {
    /// Name of the enum
    pub name: String,
    /// The file this enum is present in (notice if it's qnamespace we change it to rute_enums)
    pub def_file: String,
    /// Type of enum
    pub enum_type: EnumType,
    /// Qt supports having a flags macro on enums being type checked with an extra name
    pub flags_name: String,
    /// Original class name (like Qt, QAccesibility)
    pub original_class_name: String,
    /// All the enem entries
    pub entries: Vec<EnumEntry>,
}

///
/// Api definition for a file
///
#[derive(Debug, Default)]
pub struct ApiDef {
    /// full filename path
    pub filename: String,
    /// Base filename (such as foo/file/some_name.def) is some_name
    pub base_filename: String,
    /// Structs that only holds data (such as Rect, NOTE: currently not in use)
    pub pod_structs: Vec<Struct>,
    /// Structs that holds functions (such as QPushButton)
    pub class_structs: Vec<Struct>,
    /// Enums
    pub enums: Vec<Enum>,
}

///
/// Checks if name is a primitive
///
fn is_primitve(name: &str) -> bool {
    PRMITIVE_TYPES.iter().any(|&type_name| type_name == name)
}

#[derive(Parser)]
#[grammar = "api.pest"]
pub struct ApiParser;

///
/// Build struct info for a parsed API def file
///
impl ApiParser {
    pub fn parse_file<P: AsRef<Path>>(path: P, api_def: &mut ApiDef) {
        let mut buffer = String::new();

        let pathname = path.as_ref().to_str().unwrap();

        let mut f = File::open(pathname)
            .unwrap_or_else(|e| panic!("ApiParser: Unable to open {}: {}", pathname, e));
        f.read_to_string(&mut buffer)
            .unwrap_or_else(|e| panic!("ApiParser: Unable to read from {}: {}", pathname, e));

        Self::parse_string(&buffer, &pathname, api_def);
    }

    pub fn parse_string(buffer: &str, filename: &str, api_def: &mut ApiDef) {
        let chunks = ApiParser::parse(Rule::chunk, buffer)
            .unwrap_or_else(|e| panic!("APiParser: {} {}", filename, e));

        let base_filename = Path::new(filename).file_name().unwrap().to_str().unwrap();
        let base_filename = &base_filename[..base_filename.len() - 4];
        let mut struct_comments = String::new();

        api_def.filename = filename.to_owned();
        api_def.base_filename = base_filename.to_owned();

        for chunk in chunks {
            match chunk.as_rule() {
                Rule::structdef => {
                    let sdef = Self::fill_struct(chunk, &struct_comments, &api_def.base_filename);
                    struct_comments.clear();

                    // If we have some variables in the struct we push it to pod_struct
                    if !sdef.variables.is_empty() {
                        api_def.pod_structs.push(sdef);
                    } else {
                        api_def.class_structs.push(sdef);
                    }
                }

                Rule::doc_comment => {
                    struct_comments.push_str(chunk.as_str());
                    struct_comments.push_str("\n");
                }

                Rule::enumdef => {
                    let mut enum_def = Enum::default();

                    // Global enums are stored in qnamespace in Qt but we have them in rute_enums
                    // so we change the name here to reflect that
                    if base_filename == "qnamespace" {
                        enum_def.def_file = "rute_enums".to_owned();
                    } else {
                        enum_def.def_file = base_filename.to_owned();
                    }

                    for entry in chunk.into_inner() {
                        match entry.as_rule() {
                            Rule::name => enum_def.name = entry.as_str().to_owned(),
                            Rule::fieldlist => enum_def.entries = Self::fill_field_list_enum(entry),
                            Rule::org_name => {
                                enum_def.original_class_name = entry
                                    .into_inner()
                                    .next()
                                    .map(|e| e.as_str())
                                    .unwrap()
                                    .to_owned();
                            }
                            Rule::enum_flags => {
                                enum_def.flags_name = entry
                                    .into_inner()
                                    .next()
                                    .map(|e| e.as_str())
                                    .unwrap()
                                    .to_owned();
                            }
                            _ => (),
                        }
                    }

                    // Figure out enum type
                    let enum_type = Self::determine_enum_type(&enum_def);
                    enum_def.enum_type = enum_type;

                    api_def.enums.push(enum_def);
                }

                _ => (),
            }
        }
    }

    ///
    /// Check if the enum values overlaps
    ///
    fn check_overlapping(enum_def: &Enum) -> bool {
        let mut values = HashSet::<u64>::new();

        for v in &enum_def.entries {
            if values.contains(&v.value) {
                return true;
            } else {
                values.insert(v.value);
            }
        }

        false
    }

    ///
    /// check if an enum only has power of two values in it. This function calculate in percent how
    /// many values that happens to be power of two and returns true if it's a above a certain
    /// threshold. The reason for this is that some enums also combinations of other values
    /// so it's not possible to *only* check for single power of two values.
    ///
    fn check_power_of_two(enum_def: &Enum) -> bool {
        if enum_def.entries.is_empty() {
            return false;
        }

        let power_of_two_count: u32 = enum_def.entries
            .iter()
            .filter(|e| e.value.is_power_of_two())
            .map(|_v| 1)
            .sum();

        // if we have >= 50% of power of two values assume this enum is being used as bitflags
        let percent = enum_def.entries.len() as f32 / power_of_two_count as f32;
        percent >= 0.5
    }

    ///
    /// Figures out the type of enum
    ///
    fn determine_enum_type(enum_def: &Enum) -> EnumType {
        // if all numbers aren't overlapping
        let overlapping = Self::check_overlapping(enum_def);
        // check if all values are power of two
        let power_of_two = Self::check_power_of_two(enum_def);

        // if all values are power of two we assume this should be used as bitfield
        // or has overlapping values we
        if power_of_two || overlapping {
            EnumType::Bitflags
        } else {
            EnumType::Regular
        }
    }

    ///
    /// Fill struct def
    ///
    fn fill_struct(chunk: Pair<Rule>, doc_comments: &str, def_file: &str) -> Struct {
        let mut sdef = Struct::default();

        sdef.doc_comments = doc_comments.to_owned();
        sdef.def_file = def_file.to_owned();

        for entry in chunk.into_inner() {
            match entry.as_rule() {
                Rule::name => sdef.name = entry.as_str().to_owned(),
                Rule::derive => sdef.inherit = Some(Self::get_derive_list(entry)),
                Rule::attributes => sdef.attributes = Self::get_attrbutes(entry),
                Rule::traits => sdef.traits = Self::get_attrbutes(entry),
                Rule::fieldlist => {
                    let (var_entries, func_entries) = Self::fill_field_list(entry);
                    sdef.variables = var_entries;
                    sdef.functions = func_entries;
                }

                _ => (),
            }
        }

        // Set the name we use in C++
        if sdef.should_gen_wrap_class() {
            sdef.cpp_name = format!("WR{}", sdef.name);
        } else if sdef.cpp_name.is_empty() {
            sdef.cpp_name = format!("Q{}", sdef.name);
        }

        // TODO: We need to support the user to override this name if wanted
        // this is to allow supporting generation of code that isn't in the qt
        // strandard library but may be a user class not starting with Q
        if sdef.qt_name.is_empty() {
            sdef.qt_name = format!("Q{}", sdef.name);
        }

        sdef
    }

    ///
    /// Get attributes for a struct
    ///
    fn get_attrbutes(rule: Pair<Rule>) -> Vec<String> {
        let mut attribs = Vec::new();
        for entry in rule.into_inner() {
            if entry.as_rule() == Rule::namelist {
                attribs = Self::get_namelist_list(entry);
            }
        }

        attribs
    }

    ///
    /// Get attributes for a struct
    ///
    fn get_derive_list(rule: Pair<Rule>) -> Vec<String> {
        let mut attribs = Vec::new();
        for entry in rule.into_inner() {
            if entry.as_rule() == Rule::namelist {
                attribs = Self::get_namelist_list(entry);
            }
        }

        attribs
    }

    ///
    /// collect namelist (array) of strings
    ///
    fn get_namelist_list(rule: Pair<Rule>) -> Vec<String> {
        rule.into_inner().map(|e| e.as_str().to_owned()).collect()
    }

    ///
    /// Fill the entries in a struct
    ///
    /// Returns tuple with two ararys for variables and functions
    ///
    fn fill_field_list(rule: Pair<Rule>) -> (Vec<Variable>, Vec<Function>) {
        let mut var_entries = Vec::new();
        let mut func_entries = Vec::new();
        let mut doc_comments = String::new();

        for entry in rule.into_inner() {
            match entry.as_rule() {
                Rule::field => {
                    let field = entry.clone().into_inner().next().unwrap();

                    match field.as_rule() {
                        Rule::var => {
                            var_entries.push(Self::get_variable(field, &doc_comments));
                            doc_comments.clear();
                        }
                        Rule::function => {
                            func_entries.push(Self::get_function(field, &doc_comments));
                            doc_comments.clear();
                        }
                        _ => (),
                    }
                }

                Rule::doc_comment => {
                    doc_comments.push_str("    ");
                    doc_comments.push_str(entry.as_str());
                    doc_comments.push_str("\n");
                }

                _ => (),
            }
        }

        (var_entries, func_entries)
    }

    ///
    /// Get data for function declaration
    ///
    fn get_function(rule: Pair<Rule>, doc_comments: &str) -> Function {
        let mut function = Function {
            doc_comments: doc_comments.to_owned(),
            ..Function::default()
        };

        for entry in rule.into_inner() {
            match entry.as_rule() {
                Rule::name => function.name = entry.as_str().to_owned(),
                Rule::signal => function.func_type = FunctionType::Signal,
                Rule::event => function.func_type = FunctionType::Event,
                Rule::static_typ => function.func_type = FunctionType::Static,
                Rule::varlist => function.function_args = Self::get_variable_list(entry),
                Rule::retexp => function.return_val = Some(Self::get_variable(entry, "")),
                Rule::org_name => {
                    function.cpp_name = entry
                        .into_inner()
                        .next()
                        .map(|e| e.as_str())
                        .unwrap()
                        .to_owned();
                }

                Rule::manual => {
                    function.is_manual = true;
                    function.func_type = FunctionType::Regular;
                }
                _ => (),
            }
        }

        // if we don't have any function args we add self as first argument as we always have that
        if function.function_args.is_empty() {
            function.function_args.push(Variable {
                name: "self_c".to_owned(),
                vtype: VariableType::SelfType,
                type_name: "self".to_owned(),
                ..Variable::default()
            });
        }

        if function.cpp_name.is_empty() {
            function.cpp_name = function.name.to_owned();
        }

        function
    }

    ///
    /// Gather variable list
    ///
    fn get_variable_list(rule: Pair<Rule>) -> Vec<Variable> {
        let mut variables = Vec::new();

        variables.push(Variable {
            name: "self_c".to_owned(),
            vtype: VariableType::SelfType,
            ..Variable::default()
        });

        for entry in rule.into_inner() {
            variables.push(Self::get_variable(entry, ""));
        }

        variables
    }

    ///
    /// Get variable
    ///
    fn get_variable(rule: Pair<Rule>, doc_comments: &str) -> Variable {
        let mut vtype = Rule::var;
        let mut var = Variable::default();
        let mut type_name = String::new();

        var.doc_comments = doc_comments.to_owned();

        for entry in rule.into_inner() {
            match entry.as_rule() {
                Rule::name => var.name = entry.as_str().to_owned(),
                Rule::refexp => vtype = Rule::refexp,
                Rule::pointer_exp => vtype = Rule::pointer_exp,
                Rule::optional => var.optional = true,
                Rule::vtype => type_name = entry.as_str().to_owned(),
                Rule::enum_use => {
                    let name = entry.as_str();
                    var.enum_sub_type = name[2..].to_owned();
                    vtype = Rule::enum_use;
                }

                Rule::array => {
                    // Get the type if we have an array
                    for entry in entry.into_inner() {
                        match entry.as_rule() {
                            Rule::vtype => type_name = entry.as_str().to_owned(),
                            Rule::refexp => vtype = Rule::refexp,
                            Rule::pointer_exp => vtype = Rule::pointer_exp,
                            _ => (),
                        }
                    }
                    var.array = true;
                }

                _ => (),
            }
        }

        // match up with the correct type
        let var_type = match vtype {
            Rule::refexp => VariableType::Reference,
            Rule::pointer_exp => VariableType::Reference,
            Rule::enum_use => VariableType::Enum,
            _ => {
                if type_name == "String" {
                    VariableType::Str
                } else if is_primitve(&type_name) {
                    VariableType::Primitive
                } else {
                    VariableType::Regular
                }
            }
        };

        if vtype == Rule::pointer_exp {
            var.pointer = true;
        }

        // TODO: We assume regular is class type now but this will change
        // when we have POD structs
        if var_type == VariableType::Regular {
            var.class_type = true;
        }

        var.type_name = type_name;
        var.vtype = var_type;
        var
    }

    ///
    /// Get array of enums
    ///
    fn fill_field_list_enum(rule: Pair<Rule>) -> Vec<EnumEntry> {
        let mut entries = Vec::new();

        for entry in rule.into_inner() {
            if entry.as_rule() == Rule::field {
                let field = entry.clone().into_inner().next().unwrap();

                if field.as_rule() == Rule::enum_type {
                    entries.push(Self::get_enum(field));
                }
            }
        }

        entries
    }

    ///
    /// Get enum
    ///
    fn get_enum(rule: Pair<Rule>) -> EnumEntry {
        let mut name = String::new();
        let mut assign = None;

        for entry in rule.into_inner() {
            match entry.as_rule() {
                Rule::name => name = entry.as_str().to_owned(),
                Rule::enum_assign => assign = Some(Self::get_enum_assign(entry).parse::<u64>().unwrap()),
                _ => (),
            }
        }

        if let Some(value) = assign {
            EnumEntry { name, value }
        } else {
            panic!("Should not be here")
        }
    }

    ///
    /// Get enum asign
    ///
    fn get_enum_assign(rule: Pair<Rule>) -> String {
        let mut name_or_num = String::new();

        for entry in rule.into_inner() {
            if entry.as_rule() == Rule::name_or_num {
                name_or_num = entry.as_str().to_owned();
                break;
            }
        }

        name_or_num
    }

    ///
    /// Recursive get the structs
    ///
    fn recursive_get_inherit_structs(
        name: &str,
        include_self: RecurseIncludeSelf,
        lookup: &HashMap<String, Vec<String>>,
        out_structs: &mut Vec<String>,
    ) {
        if let Some(values) = lookup.get(name) {
            for v in values {
                Self::recursive_get_inherit_structs(
                    v,
                    RecurseIncludeSelf::Yes,
                    lookup,
                    out_structs,
                );
            }
        }

        if include_self == RecurseIncludeSelf::Yes {
            out_structs.push(name.to_owned());
        }
    }

    ///
    /// Get a list of all the traits
    ///
    fn get_inherit_structs(
        name: &str,
        include_self: RecurseIncludeSelf,
        lookup: &HashMap<String, Vec<String>>,
    ) -> Vec<String> {
        let mut out_structs = Vec::new();

        Self::recursive_get_inherit_structs(name, include_self, lookup, &mut out_structs);

        out_structs
    }

    fn update_variable(arg: &mut Variable,
        struct_name_map: &HashMap<String, String>,
        type_def_file: &HashMap<String, String>,
        enum_def_file: &HashMap<String, String>)
    {
        if let Some(qt_name) = struct_name_map.get(&arg.type_name) {
            arg.qt_type_name = qt_name.to_owned();
        }

        match arg.vtype {
            VariableType::Enum => {
                if let Some(def_file) = enum_def_file.get(&arg.enum_sub_type) {
                    arg.def_file = def_file.to_owned();
                } else {
                    println!("--> enum {} wasn't found in lookup", arg.enum_sub_type);
                }
            },

            VariableType::Regular => {
                if let Some(def_file) = type_def_file.get(&Self::get_trait_name(&arg.type_name)) {
                    arg.def_file = def_file.to_owned();
                } else {
                    println!("--> type {:?} wasn't found in lookup", arg);
                }
            },

            VariableType::Reference => {
                if let Some(def_file) = type_def_file.get(&Self::get_trait_name(&arg.type_name)) {
                    arg.def_file = def_file.to_owned();
                } else {
                    println!("--> type {:?} wasn't found in lookup", arg);
                }
            },

            _ => (),
        }
    }

    fn get_trait_name(name: &str) -> String {
        if name.ends_with("Type") {
            format!("{}Trait", &name[..name.len() - 4])
        } else {
            name.to_owned()
        }
    }

    pub fn second_pass(api_defs: &mut [ApiDef]) {
        // Build a hash_set of all classes that are inherited
        let mut inherited_classes = HashMap::new();

        for api_def in api_defs.iter() {
            api_def.class_structs.iter().for_each(|s| {
                s.inherit.as_ref().map_or((), |i| {
                    let mut in_values = Vec::new();
                    for v in i {
                        in_values.push(v.trim().to_owned());
                    }
                    // Using vec here to support multiple classes later
                    inherited_classes.insert(s.name.to_owned(), in_values);
                })
            });
        }

        // Fill all
        for api_def in api_defs.iter_mut() {
            api_def.class_structs.iter_mut().for_each(|s| {
                if s.inherit.is_some() {
                    s.full_inherit = Self::get_inherit_structs(
                        &s.name,
                        RecurseIncludeSelf::Yes,
                        &inherited_classes,
                    );
                } else {
                    s.full_inherit = vec![s.name.to_owned()];
                }
            });
        }

        // Build a hash map of all type and their QtName
        // and we also build two hashmaps for all types and which modules they belong into
        // and they are separate for structs and enums
        let mut struct_name_map = HashMap::new();
        let mut type_def_file = HashMap::new();
        let mut enum_def_file = HashMap::new();

        for api_def in api_defs.iter() {
            api_def.class_structs.iter().for_each(|s| {
                struct_name_map.insert(s.name.to_owned(), s.cpp_name.to_owned());
                type_def_file.insert(s.name.to_owned(), s.def_file.to_owned());
                type_def_file.insert(format!("{}Trait",s.name), s.def_file.to_owned());
            });

            api_def.enums.iter().for_each(|s| {
                enum_def_file.insert(s.name.to_owned(), s.def_file.to_owned());

                if !s.flags_name.is_empty() {
                    enum_def_file.insert(s.flags_name.to_owned(), s.def_file.to_owned());
                }
            });
        }

        // Patch up the qt names/def_file in the function arguments
        for func in api_defs
            .iter_mut()
            .flat_map(|api| api.class_structs.iter_mut())
            .flat_map(|s| s.functions.iter_mut())
        {
            for arg in func.function_args.iter_mut() {
                Self::update_variable(arg, &struct_name_map, &type_def_file, &enum_def_file);
            }

            if let Some(ref mut ret_val) = func.return_val {
                Self::update_variable(ret_val, &struct_name_map, &type_def_file, &enum_def_file);
            }
        }
    }
}

///
/// Use if self should be included when finding all the structs
///
#[derive(Copy, Clone, PartialEq)]
pub enum RecurseIncludeSelf {
    Yes,
}

///
/// ReturnType bool
///
#[derive(PartialEq, Clone, Copy)]
pub enum IsReturnArg {
    //Yes,
    No,
}

///
/// Used when returning types that may differ if used as input or not
///
#[derive(Copy, Clone, PartialEq)]
pub enum IsReturnType {
    Yes,
    No,
}

///
/// Some helper functions for ApiDef
///
impl ApiDef {
    ///
    /// Get functions from all structs that matches the filter
    ///
    pub fn get_functions<'a>(&'a self, func_type: FunctionType) -> Vec<&'a Function> {
        self.class_structs
            .iter()
            .flat_map(|s| s.functions.iter())
            .filter(|f| f.func_type == func_type)
            .collect()
    }
}

///
/// Impl for struct. Mostly helper functions to make it easier to extract info
///
impl Struct {
    ///
    /// Check if the struct should has static functions
    ///
    pub fn has_static_functions(&self) -> bool {
        self.functions
            .iter()
            .any(|ref f| f.func_type == FunctionType::Static)
    }
    ///
    /// Check if the struct should have a create function
    ///
    pub fn should_have_create_func(&self) -> bool {
        self.attributes
            .iter()
            .find(|&s| s == ATTRIB_NO_CREATE)
            .is_none()
    }

    ///
    /// Check if the struct is manually created
    ///
    pub fn has_manual_create(&self) -> bool {
        self.attributes
            .iter()
            .any(|ref s| *s == ATTRIB_MANUAL_CREATE)
    }

    ///
    /// Check if no wrapping class should be generated
    ///
    pub fn should_gen_wrap_class(&self) -> bool {
        self.attributes
            .iter()
            .find(|&s| s == ATTRIB_NO_WRAP_CLASS)
            .is_none()
    }

    ///
    /// Check if no wrapping class should be generated
    ///
    pub fn supports_cpp_clone(&self) -> bool {
        self.attributes
            .iter()
            .find(|&s| s == ATTRIB_NO_CPP_CLONE)
            .is_none()
    }

    ///
    /// Check if no wrapping class should be generated
    ///
    pub fn should_generate_drop(&self) -> bool {
        self.attributes.iter().any(|ref s| *s == ATTRIB_DROP)
    }
}

///
/// Impl for Variable. Helper functions to make C and Rust generation easier
///
impl Variable {
    pub fn get_c_type(&self, is_ret_type: IsReturnType) -> Cow<str> {
        if self.array {
            return "struct RUArray".into();
        }

        let tname = self.type_name.as_str();

        match self.vtype {
            VariableType::SelfType => "struct RUBase*".into(),
            VariableType::Primitive => self.get_c_primitive_type(),
            VariableType::Reference => match is_ret_type {
                IsReturnType::Yes => format!("struct RU{}", tname).into(),
                IsReturnType::No => "struct RUBase*".into(),
            },

            VariableType::Regular => {
                if tname == "String" {
                    "const char*".into()
                } else {
                    format!("struct RU{}", tname).into()
                }
            }

            VariableType::Enum => "int".into(),
            VariableType::Str => "const char*".into(),

            _ => {
                println!("Should not be here {}", self.name);
                "<error>".into()
            }
        }
    }

    pub fn get_c_primitive_type(&self) -> Cow<str> {
        let tname = self.type_name.as_str();

        match tname {
            "f32" => "float".into(),
            "bool" => "bool".into(),
            "f64" => "double".into(),
            "i32" => "int".into(),
            _ => {
                if self.type_name.starts_with('u') {
                    format!("uint{}_t", &tname[1..]).into()
                } else {
                    format!("int{}_t", &tname[1..]).into()
                }
            }
        }
    }

    ///
    /// If the typename ends with "Type" return a name without it
    ///
    pub fn get_untyped_name(&self) -> &str {
        if self.type_name.ends_with("Type") {
            &self.type_name[..self.type_name.len() - 4]
        } else {
            &self.type_name
        }
    }
}

///
/// This is used to replace the type of the first argument (self)
///
pub enum FirstArgType {
    /// The agument as is
    Keep,
    // Remove the argument
    //Remove,
    // Replace the argument type with this
    //Replace(&'static str),
}

///
/// This is used to replace the name of the first argument (self)
///
pub enum FirstArgName {
    // Keep the name
    //Keep,
    /// Remove the name
    Remove,
    // Replace the name with this
    //Replace(&'static str),
}

///
/// Impl for Function. Helper functions to make C and Rust generation easier
///
impl Function {
    ///
    /// Takes a function definition and generates a C function def from it
    ///
    /// For example: "float test, uint32_t bar"
    ///
    pub fn generate_c_function_def(&self, replace_first: FirstArgType) -> String {
        let mut function_args = String::with_capacity(128);
        let len = self.function_args.len();

        // write arguments
        for (i, arg) in self.function_args.iter().enumerate() {
            if i == 0 {
                match replace_first {
                    FirstArgType::Keep => function_args.push_str(&arg.get_c_type(IsReturnType::No)),
                    //FirstArgType::Remove => continue,
                    //FirstArgType::Event(ref arg) => function_args.push_str(&arg),
                }
            } else {
                function_args.push_str(&arg.get_c_type(IsReturnType::No));
            }

            function_args.push_str(" ");
            function_args.push_str(&arg.name);

            if i != len - 1 {
                function_args.push_str(", ");
            }
        }

        function_args
    }

    ///
    /// Takes a function definition and generates a C function def from it
    ///
    /// For example: "self, test, bar"
    ///
    pub fn generate_invoke(&self, replace_first_arg: FirstArgName) -> String {
        let mut function_invoke = String::with_capacity(128);
        let len = self.function_args.len();

        // write arguments
        for (i, arg) in self.function_args.iter().enumerate() {
            if i == 0 {
                match replace_first_arg {
                    //FirstArgName::Keep => function_invoke.push_str(&arg.name),
                    FirstArgName::Remove => continue,
                    //FirstArgName::Event(ref name) => function_invoke.push_str(name),
                }
            } else {
                function_invoke.push_str(&arg.name);
            }

            if i != len - 1 {
                function_invoke.push_str(", ");
            }
        }

        function_invoke
    }

    ///
    /// This function allows to replace any of the types when generating a c function declaration
    /// Type replacement depends highly on where the function is being use.
    ///
    pub fn gen_c_def_filter<F>(&self, replace_first: Option<Option<Cow<str>>>, filter: F) -> String
    where
        F: Fn(usize, &Variable) -> Option<Cow<str>>,
    {
        let mut output = String::with_capacity(256);
        let arg_count = self.function_args.len();
        let mut skip_first = false;

        // This allows us to change the first parameter and it also supports to not have any parameter at all
        replace_first
            .map(|arg| {
                skip_first = true;
                arg
            }).and_then(|v| v)
            .map(|v| {
                if arg_count > 0 {
                    output.push_str(&format!("{} {}", v, self.function_args[0].name));
                }

                if arg_count > 1 {
                    output.push_str(", ");
                }
            });

        // iterater over all the parameters and run the filter

        for (i, arg) in self.function_args.iter().enumerate() {
            if i == 0 && skip_first {
                continue;
            }

            let filter_arg = filter(i, &arg);
            let current_arg = filter_arg.map_or_else(|| arg.get_c_type(IsReturnType::No), |v| v);

            output.push_str(&format!("{} {}", current_arg, arg.name));

            if i != arg_count - 1 {
                output.push_str(", ");
            }
        }

        output
    }

    ///
    /// This function allows to replace any of the parameter names when generating a c function
    /// definition
    ///
    pub fn gen_c_invoke_filter<F>(&self, replace_first: FirstArgName, filter: F) -> String
    where
        F: Fn(usize, &Variable) -> Option<Cow<str>>,
    {
        let mut output = String::with_capacity(256);
        let arg_count = self.function_args.len();

        // iterater over all the parameters and run the filter

        for (i, arg) in self.function_args.iter().enumerate() {
            if i == 0 {
                match replace_first {
                    FirstArgName::Remove => continue,
                }
            }

            let filter_arg = filter(i, &arg);
            let current_arg = filter_arg.map_or_else(|| arg.name.clone().into(), |v| v);

            output.push_str(&format!("{}", current_arg));

            if i != arg_count - 1 {
                output.push_str(", ");
            }
        }

        output
    }

    ///
    /// This is kinda of a special case function but is useful to have here as it's
    /// being used in various parts of the code. It will return the name of the function (snake
    /// cased) without the _event if it has that at the end
    ///
    pub fn get_name_skip_event(&self) -> &str {
        if self.name.ends_with("event") && self.name != "event" {
            &self.name[..self.name.len() - 6]
        } else {
            &self.name
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_primitve_ok() {
        assert_eq!(is_primitve("i32"), true);
    }

    #[test]
    fn test_primitve_false() {
        assert_eq!(is_primitve("dummy"), false);
    }

    ///
    /// Make sure parsing of "struct Widget { show() }"
    ///
    #[test]
    fn test_basic_class_struct() {
        let mut api_def = ApiDef::default();
        ApiParser::parse_string(
            "struct Widget { show() }",
            "dummy_filename.def",
            &mut api_def,
        );
        assert_eq!(api_def.pod_structs.is_empty(), true);
        assert_eq!(api_def.class_structs.is_empty(), false);

        let sdef = &api_def.class_structs[0];

        assert_eq!(sdef.name, "Widget");
        assert_eq!(sdef.functions.len(), 1);
        assert_eq!(sdef.functions[0].name, "show");
    }

    ///
    /// Tests that get_untyped_name() returs correct
    ///
    #[test]
    fn test_var_untyped_name_typed() {
        let var = Variable {
            type_name: "WidgetType".to_owned(),
            ..Variable::default()
        };

        assert_eq!(var.get_untyped_name(), "Widget");
    }

    ///
    /// Tests that get_untyped_name() returs correct
    ///
    #[test]
    fn test_var_untyped_name() {
        let var = Variable {
            type_name: "Widget".to_owned(),
            ..Variable::default()
        };

        assert_eq!(var.get_untyped_name(), "Widget");
    }
}
