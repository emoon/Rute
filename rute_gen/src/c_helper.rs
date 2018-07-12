///
/// This file contains helper functions for generating C variables, functions, etc
/// We want to share this as c_gen and cpp_gen has many similar requirements
///
use api_parser::{Function, Variable, VariableType};

///
/// Used in the function bellow to make bool a bit clerar
///
#[derive(Copy, Clone, PartialEq)]
pub enum UseTypeRef {
    Yes,
    No,
}

///
/// This function takes in a generic variable and produces a C type for it
///
/// Conversions that take place
///
/// SelfType => struct RUBase*
/// Primitives such as f32,u32 gets translated to float,uint32_t,etc
/// Array => struct RUArray
///
pub fn get_c_type(var: &Variable, use_type_ref: UseTypeRef) -> String {
    if var.array {
        return "struct RUArray".to_owned();
    }

    match var.vtype {
        VariableType::SelfType => "struct RUBase*".to_owned(),
        //VariableType::Enum(ref tname) => format!("RU{}", tname),
        VariableType::Primitive(ref tname) => {
            if tname == "f32" {
                "float".to_owned()
            } else if tname == "bool" {
                "bool".to_owned()
            } else if tname == "f64" {
                "double".to_owned()
            } else if tname == "i32" {
                "int".to_owned()
            } else if tname.starts_with('u') {
                format!("uint{}_t", &tname[1..])
            } else {
                format!("int{}_t", &tname[1..])
            }
        }

        VariableType::Reference(ref _tname) => {
            if use_type_ref == UseTypeRef::Yes {
                format!("struct RU{}", _tname)
            } else {
                "struct RUBase*".to_owned()
            }
        }

        VariableType::Regular(ref tname) => {
            if tname == "String" {
                "const char*".to_owned()
            } else {
                format!("struct RU{}", tname)
            }
        }

        VariableType::Optional(ref tname) => format!("struct RU{}", tname),

        _ => {
            println!("Should not be here {}", var.name);
            String::new()
        }
    }
}

///
/// Takes a function definition and generates a C function def from it
///
/// For example: "float test, uint32_t bar"
///
pub fn generate_c_function_args(func: &Function) -> String {
    let mut function_args = String::new();
    let len = func.function_args.len();

    // write arguments
    for (i, arg) in func.function_args.iter().enumerate() {
        function_args.push_str(&get_c_type(&arg, UseTypeRef::No));
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
pub fn generate_c_function_invoke(
    func: &Function,
    replace_first_arg: Option<&'static str>,
) -> String {
    let mut function_invoke = String::new();
    let len = func.function_args.len();

    // write arguments
    for (i, arg) in func.function_args.iter().enumerate() {
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
