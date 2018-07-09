use pest::Parser;
use pest::iterators::Pair;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashSet;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("api.pest");

///
/// Current primitive types
///
static PRMITIVE_TYPES: &[&'static str] =
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
    Enum(String),
    /// Struct/other type
    Regular(String),
    /// Prmitive type (such as i32,u64,etc)
    Primitive(String),
    /// Reference type
    Reference(String),
    /// Array type
    Array(String),
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
    PRMITIVE_TYPES.iter().find(|&&type_name| type_name == name).is_some()
}

///
/// Impl for struct. Mostly helper functions to make it easier to extract info
///
impl Struct {
    ///
    /// Get iterator for all callback functions
    ///
    pub fn get_callback_functions<'a>(&'a self) -> Box<Iterator<Item = &'a Function> + 'a> {
        Box::new(self.functions.iter().filter(|func| func.func_type == FunctionType::Callback))
    }

    ///
    /// Check if the struct should have a create function
    ///
    pub fn should_have_create_func(&self) -> bool {
        self.attributes.iter().find(|&s| s == ATTRIB_NO_CREATE).is_some()
    }
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

		//
		// Open and read file to memory
		//
		let mut f = File::open(pathname).unwrap_or_else(|e| panic!("ApiParser: Unable to open {}: {}", pathname, e));
		f.read_to_string(&mut buffer).unwrap_or_else(|e| panic!("ApiParser: Unable to read from {}: {}", pathname, e));

        let chunks =
            ApiParser::parse(Rule::chunk, &buffer).unwrap_or_else(|e| panic!("APiParser: {} {}", pathname, e));

        for chunk in chunks {
            match chunk.as_rule() {
                Rule::structdef => {
                    let sdef = Self::fill_struct(chunk);

                    // If we have some variables in the struct we push it to pod_struct
                    if sdef.variables.len() > 0 {
                        api_def.pod_structs.push(sdef);
                    } else {
                        api_def.class_structs.push(sdef);
                    }
                }

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
            match entry.as_rule() {
                Rule::namelist => attribs = Self::get_namelist_list(entry),
                _ => (),
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
            match entry.as_rule() {
                Rule::field => {
                    let field = entry.clone().into_inner().next().unwrap();

                    match field.as_rule() {
                        Rule::var => var_entries.push(Self::get_variable(field)),
                        Rule::function => func_entries.push(Self::get_function(field)),
                        _ => (),
                    }
                }

                _ => (),
            }
        }

        (var_entries, func_entries)
    }

    ///
    ///
    ///
    fn get_name(rule: Pair<Rule>) -> String {
        let mut name = String::new();

        for entry in rule.into_inner() {
            match entry.as_rule() {
                Rule::name => name = entry.as_str().to_owned(),
                _ => (),
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
        if function.function_args.len() == 0 {
            function.function_args.push(Variable {
                name: "self".to_owned(),
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
            name: "self".to_owned(),
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

}

///
/// Some helper functions for ApiDef
///
impl ApiDef {
    ///
    /// Get a list of all the traits
    ///
    pub fn get_all_traits(&self) -> Vec<String> {
        let mut traits = HashSet::new();

        for sdef in &self.class_structs {
            for t in &sdef.traits {
                traits.insert(t.clone());
            }
        }

        // TODO: There is likely a way better way to do this

        let mut sorted_traits = Vec::new();
        let mut sorted_list = traits.iter().collect::<Vec<(&String)>>();
        sorted_list.sort();

        for entry in sorted_list {
            println!("trait {}", entry.clone());
            sorted_traits.push(entry.clone());
        }

        sorted_traits
    }
}
