use std::fs::File;
use std::io::Write;
use std::io;
use std::path::Path;

use std::rc::Rc;

use pest::Parser;
use pest::inputs::{FileInput, Input};
use pest::iterators::Pair;

#[derive(Debug, Clone, Default)]
pub struct Variable {
    pub name: String,
    pub vtype: String,
    pub primitive: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Function {
    pub name: String,
    pub function_args: Vec<Variable>,
    pub return_val: Option<Variable>,
    pub callback: bool,
    pub event: bool,
}

#[derive(Debug)]
pub enum StructEntry {
    Var(Variable),
    Function(Function),
}

#[derive(Debug, Default)]
pub struct Struct {
    pub name: String,
    pub inherit: Option<String>,
    pub entries: Vec<StructEntry>,
}

#[derive(Debug, Default)]
pub struct ApiDef {
    pub entries: Vec<Struct>,
}

fn is_primitve(name: &str) -> bool {
    if name == "u8" || name == "u8" || name == "i16" || name == "u16" ||
       name == "i32" || name == "u32" || name == "i64" || name == "u64" ||
       name == "f32" || name == "f64" || name == "bool" {
        true
    } else {
        false
    }
}

#[derive(Clone, Copy)]
enum FuncCollectionType {
    All,
    Callback,
    Regular,
}

impl Struct {
    pub fn is_pod(&self) -> bool {
        self.entries
            .iter()
            .all(|e| match *e {
                     StructEntry::Var(ref _var) => true,
                     _ => false,
                 })
    }

    pub fn get_functions<F>(&self, funcs: &mut Vec<Function>, filter: F)
        where F: Fn(&Function) -> bool {
        for entry in &self.entries {
            match *entry {
                StructEntry::Function(ref func) =>
                    if filter(func) {
                        funcs.push(func.clone());
                    },
                _ => (),
            }
        }
    }

    pub fn get_callback_functions(&self, funcs: &mut Vec<Function>) {
        Self::get_functions(self, funcs, |func| func.callback)
    }

    pub fn get_regular_functions(&self, funcs: &mut Vec<Function>) {
        Self::get_functions(self, funcs, |func| !func.callback)
    }

    pub fn get_all_functions(&self, funcs: &mut Vec<Function>) {
        Self::get_functions(self, funcs, |_| true)
    }
}

impl Variable {
    pub fn get_c_type(&self) -> String {
        let tname = &self.vtype;
        let primitve = self.primitive;

        if tname == "String" {
            return "const char*".to_owned();
        }

        if tname == "self" {
            return "void*".to_owned();
        }

        if primitve {
            if tname == "f32" {
                return "float".to_owned();
            } else if tname == "bool" {
                return "bool".to_owned();
            } else if tname == "f64" {
                return "double".to_owned();
            } else if tname == "i32" {
                return "int".to_owned();
            } else {
                // here we will have u8/i8,u32/etc
                if tname.starts_with("u") {
                    return format!("uint{}_t", &tname[1..]);
                } else {
                    return format!("int{}_t", &tname[1..]);
                }
            }
        } else {
            // Unknown type here, we always assume to use a struct Type*
            format!("struct PU{}*", tname)
        }
    }
}

impl Function {
    pub fn write_c_func_def<F>(&self, f: &mut File, filter: F) -> io::Result<()>
        where F: Fn(usize, &Variable) -> (String, String)
    {
        let arg_count = self.function_args.len();

        for (i, arg) in self.function_args.iter().enumerate() {
            let filter_arg = filter(i, &arg);
            let mut write_next = true;

            if filter_arg.1 == "" {
                if filter_arg.0 != "" {
                    f.write_fmt(format_args!("{}", filter_arg.0))?;
                } else {
                    write_next = false;
                }
            } else {
                f.write_fmt(format_args!("{} {}", filter_arg.0, filter_arg.1))?;
            }

            if (i != arg_count - 1) && write_next == true {
                f.write_all(b", ")?;
            }
        }

        f.write_all(b")")?;

        if let Some(ref ret_var) = self.return_val {
            let filter_arg = filter(arg_count, &ret_var);
            if filter_arg.1 != "" {
                f.write_fmt(format_args!(" -> {}", filter_arg.1))?;
            }
        }

        Ok(())
    }

    pub fn write_func_def<F>(&self, f: &mut File, filter: F) -> io::Result<()>
        where F: Fn(usize, &Variable) -> (String, String)
    {
        let arg_count = self.function_args.len();

        for (i, arg) in self.function_args.iter().enumerate() {
            let filter_arg = filter(i, &arg);

            if filter_arg.1 == "" {
                f.write_fmt(format_args!("{}", filter_arg.0))?;
            } else {
                f.write_fmt(format_args!("{}: {}", filter_arg.0, filter_arg.1))?;
            }

            if i != arg_count - 1 {
                f.write_all(b", ")?;
            }
        }

        //f.write_all(b")")?;

        if let Some(ref ret_var) = self.return_val {
            let filter_arg = filter(arg_count, &ret_var);
            if filter_arg.1 != "" {
                f.write_fmt(format_args!(" -> {}", filter_arg.1))?;
            }
        }

        Ok(())
    }
}


impl ApiDef {
    fn collect_recursive(funcs: &mut Vec<Function>, api_def: &ApiDef, sdef: &Struct, coll_type: FuncCollectionType) {
        if let Some(ref inherit_name) = sdef.inherit {
            for sdef in &api_def.entries {
                if &sdef.name == inherit_name {
                    Self::collect_recursive(funcs, api_def, &sdef, coll_type);
                }
            }
        }

        for entry in &sdef.entries {
            match *entry {
                StructEntry::Function(ref func) => {
                    match coll_type {
                        FuncCollectionType::All => funcs.push(func.clone()),
                        FuncCollectionType::Callback => {
                            if func.callback {
                                funcs.push(func.clone());
                            }
                        },

                        FuncCollectionType::Regular => {
                            if !func.callback {
                                funcs.push(func.clone());
                            }
                        },
                    }
                }

                _ => (),
            }
        }
    }

    fn collect_functions(&self, sdef: &Struct, coll_type: FuncCollectionType) -> Vec<Function> {
        let mut funcs = Vec::new();
        Self::collect_recursive(&mut funcs, &self, sdef, coll_type);
        funcs
    }

    pub fn collect_all_functions(&self, sdef: &Struct) -> Vec<Function> {
        Self::collect_functions(&self, sdef, FuncCollectionType::All)
    }

    pub fn collect_callback_functions(&self, sdef: &Struct) -> Vec<Function> {
        Self::collect_functions(&self, sdef, FuncCollectionType::Callback)
    }

    pub fn collect_regular_functions(&self, sdef: &Struct) -> Vec<Function> {
        Self::collect_functions(&self, sdef, FuncCollectionType::Regular)
    }
}


#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("api.pest");

#[derive(Parser)]
#[grammar = "api.pest"]
struct ApiParser;

impl ApiDef {
    fn get_variable<I: Input>(rule: &Pair<Rule, I>) -> Variable {
        let mut var = Variable::default();

        for entry in rule.clone().into_inner() {
            match entry.as_rule() {
                Rule::name => var.name = entry.as_str().to_owned(),
                Rule::vtype => var.vtype = entry.as_str().to_owned(),
                _ => (),
            }
        }

        var.primitive = is_primitve(&var.vtype);
        var
    }

    fn get_variable_list<I: Input>(rule: &Pair<Rule, I>) -> Vec<Variable> {
        let mut variables = Vec::new();

        for entry in rule.clone().into_inner() {
            variables.push(Self::get_variable(&entry));
        }

        variables
    }

    fn get_function<I: Input>(rule: &Pair<Rule, I>) -> Function {
        let mut function = Function::default();

        for entry in rule.clone().into_inner() {
            match entry.as_rule() {
                Rule::name => function.name = entry.as_str().to_owned(),
                Rule::callback => function.callback = true,
                Rule::event => function.event = true,
                Rule::varlist => function.function_args = Self::get_variable_list(&entry),
                Rule::retexp => function.return_val = Some(Self::get_variable(&entry)),
                _ => (),
            }
        }

        function
    }

    fn get_derive<I: Input>(rule: &Pair<Rule, I>) -> String {
        let mut derive_type = String::new();

        for entry in rule.clone().into_inner() {
            match entry.as_rule() {
                Rule::name => derive_type = entry.as_str().to_owned(),
                _ => (),
            }
        }

        derive_type
    }

    fn fill_field_list<I: Input>(rule: &Pair<Rule, I>) -> Vec<StructEntry> {
        let mut entries = Vec::new();

        for entry in rule.clone().into_inner() {
            match entry.as_rule() {
                Rule::field => {
                    let field = entry.clone().into_inner().next().unwrap();

                    match field.as_rule() {
                        Rule::var => entries.push(StructEntry::Var(Self::get_variable(&field))),
                        Rule::function => entries.push(StructEntry::Function(Self::get_function(&field))),
                        _ => (),
                    }
                },

                _ => (),
            }
        }

        entries
    }

    pub fn new<P: AsRef<Path>>(path: P) -> ApiDef {
        let mut api_def = ApiDef::default();
        let file_input = FileInput::new(path).unwrap();

        let chunks = ApiParser::parse(Rule::chunk, Rc::new(file_input)).
            unwrap_or_else(|e| panic!("{}", e));

        for chunk in chunks {
            match chunk.as_rule() {
                Rule::structdef => {
                    let mut cur_struct = Struct::default();

                    for entry in chunk.into_inner() {
                        match entry.as_rule() {
                            Rule::name => cur_struct.name = entry.as_str().to_owned(),
                            Rule::derive => cur_struct.inherit = Some(Self::get_derive(&entry)),
                            Rule::fieldlist => cur_struct.entries = Self::fill_field_list(&entry),
                            _ => (),
                        }
                    }

                    api_def.entries.push(cur_struct);
                }

                _ => (),
            }
        }

        api_def
    }
}
