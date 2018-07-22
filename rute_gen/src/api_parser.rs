use pest::Parser;
use pest::iterators::Pair;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::borrow::Cow;
use std::io::Write;
use std::io;

#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("api.pest");

///
/// Current primitive types
///
const PRMITIVE_TYPES: &[&str] =
    &["i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64", "bool", "f32", "f64"];

///
/// Attribute tagged on struct of they shouldn't have a create function in the main
/// exported Rute struct
///
static ATTRIB_NO_CREATE: &'static str = "NoCreate";

///
/// Variable type
///
#[derive(Debug, Clone)]
pub enum VariableType {
    None,
    /// Self (aka this pointer in C++ and self in Rust)
    SelfType,
    /// Enum type
    //Enum(String),
    /// Struct/other type
    Regular(String),
    /// Prmitive type (such as i32,u64,etc)
    Primitive(String),
    /// Reference type
    Reference(String),
    /// Optional type
    Optional(String),
}

///
/// Holds the data for a variable. It's name and it's type
///
#[derive(Debug, Clone)]
pub struct Variable {
    /// Name of the variable
    pub name: String,
    /// Type of the variable
    pub vtype: VariableType,
    /// If variable is an array
    pub array: bool,
}

///
/// Default implementation for Variable
///
impl Default for Variable {
    fn default() -> Self {
        Variable {
            name: String::new(),
            vtype: VariableType::None,
            array: false,
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
    /// Evet functions maps to a virtual override in a Qt Class
    Event,
    /// Callback functions maps to a slot/signal in Qt
    Callback,
    /// Static function is that doesn't belong to a class
    Static,
}

///
/// Holds the data for a function. Name, function_args, return_type, etc
///
#[derive(Debug, Clone)]
pub struct Function {
    /// Name of the function
    pub name: String,
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
            name: String::new(),
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
    /// Name
    pub name: String,
    /// Variables in the struct
    pub variables: Vec<Variable>,
    /// Functions for the struct
    pub functions: Vec<Function>,
    /// Attributes of thu struct
    pub attributes: Vec<String>,
    /// Traits
    pub traits: Vec<String>,
    /// If the struct inherits another
    pub inherit: Option<String>,
    /// If the struct is a widget or not
    pub is_widget: bool,
}

///
/// C/C++ style enum
///
#[derive(Debug)]
pub enum EnumEntry {
    /// Enum in the style of: EnumEntry,
    Enum(String),
    /// Enum in the style of: EnumEntry = SomeValue,
    EnumValue(String, String),
}

///
/// Maps to a C++ enum
///
#[derive(Debug, Default)]
pub struct Enum {
    /// Name of the enum
    pub name: String,
    /// All the enem entries
    pub entries: Vec<EnumEntry>,
}

///
/// Api definition for a file
///
#[derive(Debug, Default)]
pub struct ApiDef {
    pub filename: String,
    /// Structs that only holds data (such as Rect)
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

        let mut f = File::open(pathname).unwrap_or_else(|e| panic!("ApiParser: Unable to open {}: {}", pathname, e));
        f.read_to_string(&mut buffer).unwrap_or_else(|e| panic!("ApiParser: Unable to read from {}: {}", pathname, e));

        Self::parse_string(&buffer, &pathname, api_def);
    }

    pub fn parse_string(buffer: &str, filename: &str, api_def: &mut ApiDef) {
        let chunks =
            ApiParser::parse(Rule::chunk, buffer).unwrap_or_else(|e| panic!("APiParser: {} {}", filename, e));

        for chunk in chunks {
            match chunk.as_rule() {
                Rule::structdef => {
                    let sdef = Self::fill_struct(chunk);

                    // If we have some variables in the struct we push it to pod_struct
                    if !sdef.variables.is_empty() {
                        api_def.pod_structs.push(sdef);
                    } else {
                        api_def.class_structs.push(sdef);
                    }
                },

                Rule::enumdef => {
                    let mut enum_def = Enum::default();

                    for entry in chunk.into_inner() {
                        match entry.as_rule() {
                            Rule::name => enum_def.name = entry.as_str().to_owned(),
                            Rule::fieldlist => enum_def.entries = Self::fill_field_list_enum(entry),
                            _ => (),
                        }
                    }

                    api_def.enums.push(enum_def);
                },

                _ => (),
            }
        }
    }

    ///
    /// Fill struct def
    ///
    fn fill_struct(chunk: Pair<Rule>) -> Struct {
        let mut sdef = Struct::default();

        for entry in chunk.into_inner() {
            match entry.as_rule() {
                Rule::name => sdef.name = entry.as_str().to_owned(),
                Rule::derive => sdef.inherit = Some(Self::get_name(entry)),
                Rule::attributes => sdef.attributes = Self::get_attrbutes(entry),
                Rule::traits => sdef.traits = Self::get_attrbutes(entry),
                Rule::fieldlist => {
                    let (var_entries, func_entries) = Self::fill_field_list(entry);
                    sdef.variables = var_entries;
                    sdef.functions = func_entries;
                },

                _ => (),
            }
        }

        sdef
    }

    ///
    /// Get attributes for a struct
    ///
    fn get_attrbutes(rule: Pair<Rule>) -> Vec<String> {
        /*
        rule.into_inner()
            .filter(|e| e.as_rule() == Rule::namelist)
            .map(|e| e.as_str().to_owned())
            .collect()
        */

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
        rule.into_inner()
            .map(|e| e.as_str().to_owned())
            .collect()
    }

    ///
    /// Fill the entries in a struct
    ///
    /// Returns tuple with two ararys for variables and functions
    ///
    fn fill_field_list(rule: Pair<Rule>) -> (Vec<Variable>, Vec<Function>) {
        let mut var_entries = Vec::new();
        let mut func_entries = Vec::new();

        for entry in rule.into_inner() {
            if entry.as_rule() == Rule::field {
                let field = entry.clone().into_inner().next().unwrap();

                match field.as_rule() {
                    Rule::var => var_entries.push(Self::get_variable(field)),
                    Rule::function => func_entries.push(Self::get_function(field)),
                    _ => (),
                }
            }
        }

        (var_entries, func_entries)
    }

    ///
    /// Get the name for a rule
    ///
    fn get_name(rule: Pair<Rule>) -> String {
        let mut name = String::new();

        for entry in rule.into_inner() {
            if entry.as_rule() == Rule::name {
                name = entry.as_str().to_owned();
                break;
            }
        }

        name
    }

    ///
    /// Get data for function declaration
    ///
    fn get_function(rule: Pair<Rule>) -> Function {
        let mut function = Function::default();

        for entry in rule.into_inner() {
            match entry.as_rule() {
                Rule::name => function.name = entry.as_str().to_owned(),
                Rule::callback => function.func_type = FunctionType::Callback,
                Rule::event => function.func_type = FunctionType::Event,
                Rule::static_typ => function.func_type = FunctionType::Static,
                Rule::varlist => function.function_args = Self::get_variable_list(entry),
                Rule::retexp => function.return_val = Some(Self::get_variable(entry)),
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
                .. Variable::default()
            });
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
            .. Variable::default()
        });

        for entry in rule.into_inner() {
            variables.push(Self::get_variable(entry));
        }

        variables
    }

    ///
    /// Get variable
    ///
    fn get_variable(rule: Pair<Rule>) -> Variable {
        let mut vtype = Rule::var;
        let mut var = Variable::default();
        let mut type_name = String::new();

        for entry in rule.into_inner() {
            match entry.as_rule() {
                Rule::name => var.name = entry.as_str().to_owned(),
                Rule::refexp => vtype = Rule::refexp,
                Rule::optional => vtype = Rule::optional,
                Rule::vtype => type_name = entry.as_str().to_owned(),
                Rule::array => {
                    // Get the type if we have an array
                    for entry in entry.into_inner() {
                        match entry.as_rule() {
                            Rule::vtype => type_name = entry.as_str().to_owned(),
                            Rule::refexp => vtype = Rule::refexp,
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
            Rule::refexp => VariableType::Reference(type_name),
            Rule::optional => VariableType::Optional(type_name),
            _ => {
                if is_primitve(&type_name) {
                    VariableType::Primitive(type_name)
                } else {
                    VariableType::Regular(type_name)
                }
            }
        };

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
        let mut assign = String::new();

        for entry in rule.into_inner() {
            match entry.as_rule() {
                Rule::name => name = entry.as_str().to_owned(),
                Rule::enum_assign => assign =  Self::get_enum_assign(entry),
                _ => (),
            }
        }

        if assign.is_empty() {
            EnumEntry::Enum(name)
        } else {
            EnumEntry::EnumValue(name, assign)
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

}


///
/// Use if self should be included when finding all the structs
///
#[derive(Copy, Clone, PartialEq)]
pub enum RecurseIncludeSelf {
    Yes,
}

///
/// Some helper functions for ApiDef
///
impl ApiDef {
    ///
    /// Get a list of all the traits
    ///
    pub fn get_all_traits<'a>(&'a self) -> Vec<&'a String> {
        // Get all the traits name into a hashset so we can stort them afterwards
        let traits = self
            .class_structs
            .iter()
            .flat_map(|s| s.traits.iter())
            .map(|t| t)
            .collect::<HashSet<&'a String>>();

        let mut sorted_traits = Vec::with_capacity(traits.len());
        let mut sorted_list = traits.iter().collect::<Vec<&&'a String>>();

        sorted_list.sort();
        //sorted_list.collect()
        //sorted_list.collect();

        for entry in sorted_list {
            sorted_traits.push(*entry);
        }

        sorted_traits
    }

    ///
    /// Recursive get the structs
    ///
    fn recursive_get_inherit_structs<'a>(sdef: &'a Struct, api_def: &'a ApiDef, include_self: RecurseIncludeSelf, out_structs: &mut Vec<&'a Struct>) {
        sdef.inherit
            .as_ref()
            .map(|name| {
                api_def.class_structs.iter().find(|s| s.name == *name)
                    .map(|s| {
                        Self::recursive_get_inherit_structs(s, api_def, RecurseIncludeSelf::Yes, out_structs);
                    })
            }
        );

        if include_self == RecurseIncludeSelf::Yes {
            out_structs.push(sdef);
        }
    }

    ///
    /// Get a list of all the traits
    ///
    pub fn get_inherit_structs<'a>(&'a self, sdef: &'a Struct, include_self: RecurseIncludeSelf) -> Vec<&'a Struct> {
        let mut out_structs = Vec::new();

        Self::recursive_get_inherit_structs(sdef, self, include_self, &mut out_structs);

        out_structs
    }

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
    /// Check if the struct should have a create function
    ///
    pub fn should_have_create_func(&self) -> bool {
        self.attributes.iter().find(|&s| s == ATTRIB_NO_CREATE).is_none()
    }
}

///
/// Impl for Variable. Helper functions to make C and Rust generation easier
///
impl Variable {
    pub fn get_c_type(&self) -> Cow<str> {
        if self.array {
            return "struct RUArray".into();
        }

        match self.vtype {
            VariableType::SelfType => "struct RUBase*".into(),
            VariableType::Primitive(ref tname) => {
                match tname.as_str() {
                    "f32" => "float".into(),
                    "bool" => "bool".into(),
                    "f64" => "double".into(),
                    "i32" => "int".into(),
                    _ => {
                        if tname.starts_with('u') {
                            format!("uint{}_t", &tname[1..]).into()
                        } else {
                            format!("int{}_t", &tname[1..]).into()
                        }
                    }
                }
            }

            VariableType::Reference(ref _tname) => {
                format!("struct RU{}", _tname).into()
            }

            VariableType::Regular(ref tname) => {
                if tname == "String" {
                    "const char*".into()
                } else {
                    format!("struct RU{}", tname).into()
                }
            }

            VariableType::Optional(ref tname) => format!("struct RU{}", tname).into(),

            _ => {
                println!("Should not be here {}", self.name);
                "<error>".into()
            }
        }
    }
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
    pub fn generate_c_function_def(&self, replace_first: Option<&'static str>) -> String {
        let mut function_args = String::with_capacity(128);
        let len = self.function_args.len();

        // write arguments
        for (i, arg) in self.function_args.iter().enumerate() {
            if replace_first.is_some() && i == 0 {
                function_args.push_str(replace_first.unwrap());
            } else {
                function_args.push_str(&arg.get_c_type());
            }

            function_args.push_str(" ");
            function_args.push_str(&arg.name);

            if i != len - 1 {
                function_args.push_str(", ");
            }
        }

        if replace_first.is_some() && self.function_args.is_empty() {
            function_args.push_str(replace_first.unwrap());
        }

        function_args
    }

    ///
    /// Takes a function definition and generates a C function def from it
    ///
    /// For example: "self, test, bar"
    ///
    pub fn generate_invoke(&self,
        replace_first_arg: Option<&'static str>) -> String {
        let mut function_invoke = String::with_capacity(128);
        let len = self.function_args.len();

        // write arguments
        for (i, arg) in self.function_args.iter().enumerate() {
            if i == 0 && replace_first_arg.is_some() {
                function_invoke.push_str(replace_first_arg.unwrap());
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
    /// Allows to write out a C function def but to use a filter for the variable type
    ///
    /// TODO: Cleanup this code
    pub fn write_c_func_def<F, W>(&self, f: &mut W, filter: F) -> io::Result<()>
    where
        F: Fn(usize, &Variable) -> (Option<Cow<str>>, Option<Cow<str>>),
        W: Write,
    {
        let arg_count = self.function_args.len();

        for (i, arg) in self.function_args.iter().enumerate() {
            let filter_arg = filter(i, &arg);
            let mut write_next = true;

            if filter_arg.0.is_none() {
                if filter_arg.1.is_some() {
                    f.write_fmt(format_args!("{}", filter_arg.0.unwrap()))?;
                } else {
                    write_next = false;
                }
            } else {
                if filter_arg.0.is_none() {
                	f.write_fmt(format_args!("{}", filter_arg.1.unwrap()))?;
                } else {
                	f.write_fmt(format_args!("{} {}", filter_arg.0.unwrap(), filter_arg.1.unwrap()))?;
                }
            }

            if (i != arg_count - 1) && write_next == true {
                f.write_all(b", ")?;
            }
        }

        f.write_all(b")")?;

        if let Some(ref ret_var) = self.return_val {
            let filter_arg = filter(arg_count, &ret_var);
            if let Some(arg) = filter_arg.1 {
                f.write_fmt(format_args!(" -> {}", arg))?;
            }
        }

        Ok(())
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

    #[test]
    fn test_basic_pod_struct() {
        let mut api_def = ApiDef::default();
        ApiParser::parse_string("struct Foo { i32 test }", "", &mut api_def);
        assert_eq!(api_def.pod_structs.is_empty(), false);
        assert_eq!(api_def.class_structs.is_empty(), true);

        let sdef = &api_def.pod_structs[0];

        assert_eq!(sdef.name, "Foo");
        assert_eq!(sdef.variables.len(), 1);
        assert_eq!(sdef.functions.is_empty(), true);
        assert_eq!(sdef.variables[0].name, "test");
    }

    ///
    /// Make sure parsing of "struct Widget { show() }"
    ///
    #[test]
    fn test_basic_class_struct() {
        let mut api_def = ApiDef::default();
        ApiParser::parse_string("struct Widget { show() }", "", &mut api_def);
        assert_eq!(api_def.pod_structs.is_empty(), true);
        assert_eq!(api_def.class_structs.is_empty(), false);

        let sdef = &api_def.class_structs[0];

        assert_eq!(sdef.name, "Widget");
        assert_eq!(sdef.functions.len(), 1);
        assert_eq!(sdef.functions[0].name, "show");
    }



    // dumy

}
