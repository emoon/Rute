use std::io;
use std::fs::File;
use std::io::Write;
use std::io::{Error, ErrorKind};
use api_parser::*;
use std::collections::HashMap;
use heck::{CamelCase, SnakeCase};
use liquid::*;

use rust_templates::*;

///
/// Handles the generation of the Rust code
///
///
struct RustGenerator {
    callback_template: Template,
    event_template: Template,
    //type_handlers: Vec<Box<TypeHandler>>,
    output: File,
}

///
/// Trait for handling different types that needs to be overriden
///
trait TypeHandler {
    fn match_type(&self) -> String;

    fn replace_arg(&self, arg: &Variable) -> (String, String) {
        (arg.name.to_owned(), arg.vtype.to_owned())
    }

    fn gen_body_return(&self, _varible: &Variable, _f: &mut File) -> io::Result<()> {
        Ok(())
    }

    fn gen_body(&self, _arg: &str, _f: &mut File, _index: usize) -> String;
}

#[derive(Clone)]
struct StringTypeHandler;

#[derive(Clone)]
struct RectTypeHandler;

///
/// We need to handle strings in a special way. They need to be sent down using CString and the
/// pointer to it so have a generator for it
///
impl TypeHandler for StringTypeHandler {
    fn match_type(&self) -> String {
        "String".to_owned()
    }

    fn replace_arg(&self, arg: &Variable) -> (String, String) {
        (arg.name.to_owned(), "&str".to_owned())
    }

    fn gen_body(&self, arg: &str, f: &mut File, index: usize) -> String {
        let arg_name = format!("str_in_{}_{}", arg, index);
        f.write_fmt(format_args!(
            "        let {} = CString::new({}).unwrap();\n",
            arg_name, arg
        )).unwrap();
        format!("{}.as_ptr()", arg_name)
    }
}

///
/// We need to handle strings in a special way. They need to be sent down using CString and the
/// pointer to it so have a generator for it
///
impl TypeHandler for RectTypeHandler {
    fn match_type(&self) -> String {
        "Rect".to_owned()
    }

    fn gen_body(&self, _arg: &str, _f: &mut File, _index: usize) -> String {
        String::new()
    }

    fn gen_body_return(&self, _value: &Variable, f: &mut File) -> io::Result<()> {
        f.write_all(b"            ret_val\n")
    }
}


///
/// Type handler for traits as the function arguments when using a trait needs to use "get_obj"
///
struct TraitTypeHandler {
    name: String,
}

impl TypeHandler for TraitTypeHandler {
    fn match_type(&self) -> String {
        self.name.clone()
    }

    fn replace_arg(&self, arg: &Variable) -> (String, String) {
        (arg.name.to_owned(), format!("&{}", arg.vtype.to_owned()))
    }

    fn gen_body(&self, arg_name: &str, _f: &mut File, _index: usize) -> String {
        format!(
            "{}.get_{}_obj() as *const PUBase",
            arg_name,
            self.name.to_snake_case()
        )
    }
}

impl RustGenerator {
    ///
    /// Generate the traits implementations
    ///
    fn generate_traits(&mut self, type_handlers: &mut Vec<Box<TypeHandler>>, api_def: &ApiDef) -> io::Result<()> {
        let traits = api_def.get_all_traits();

        for trait_name in traits {
            self.output.write_fmt(format_args!("pub trait {} {{\n", trait_name))?;
            self.output.write_fmt(format_args!(
                "    fn get_{}_obj(&self) -> *const PUBase;\n}}\n\n",
                trait_name.to_snake_case()
            ))?;

            type_handlers.push(Box::new(TraitTypeHandler {
                name: trait_name.clone(),
            }));
        }

        Ok(())
    }

    ///
    ///
    ///
    fn generate_struct(&mut self, structs: &Vec<Struct>) -> io::Result<()> {
        for sdef in structs {
            if sdef.is_pod() {
                // for pod structs we re-use the FFI implementation
                self.output.write_fmt(format_args!(
                    "pub use ffi_gen::PU{} as {};\n\n",
                    sdef.name, sdef.name
                ))?;
            } else {
                self.output.write_all(b"#[derive(Clone)]\n")?;
                self.output.write_fmt(format_args!("pub struct {} {{\n", sdef.name))?;

                if sdef.is_pod() {
                    for entry in &sdef.entries {
                        match *entry {
                            StructEntry::Var(ref var) => {
                                self.output.write_fmt(format_args!("    pub {}: {},\n", var.name, var.vtype))?
                            }
                            _ => (),
                        }
                    }
                } else {
                    // Assume for non-pod that we only use the FFI interface to do stuff.
                    self.output.write_fmt(format_args!("    pub obj: Option<PU{}>,\n", sdef.name))?;
                }

                self.output.write_all(b"}\n\n")?;
            }
        }

        Ok(())
    }

    ///
    ///
    ///
    fn get_arg(arg: &Variable, type_handlers: &Vec<Box<TypeHandler>>) -> (String, String) {
        for handler in type_handlers.iter() {
            if handler.match_type() == arg.vtype {
                return handler.replace_arg(&arg);
            }
        }

        if arg.vtype == "self" {
            ("&self".to_owned(), "".to_owned())
        } else if arg.primitive {
            (arg.name.to_owned(), arg.vtype.clone())
        } else if arg.reference {
            (arg.name.clone(), format!("&{}", arg.vtype.to_owned()))
        } else if arg.optional {
            (
                arg.name.clone(),
                format!("Option<{}>", arg.vtype.to_owned()),
            )
        } else {
            (arg.name.clone(), arg.vtype.to_owned())
        }
    }

    fn generate_func_impl(&mut self, func: &Function, type_handlers: &Vec<Box<TypeHandler>>) -> io::Result<()> {
        self.output.write_fmt(format_args!("    pub fn {}", func.name))?;

        func.write_func_def_full(&mut self.output, |_, arg| Self::get_arg(arg, &type_handlers))?;

        self.output.write_all(b" {\n")?;

        let mut name_remap = HashMap::new();

        for (i, arg) in func.function_args.iter().enumerate() {
            for handler in type_handlers.iter() {
                if arg.vtype == handler.match_type() {
                    let arg_name = handler.gen_body(&arg.name, &mut self.output, i);
                    name_remap.insert(i, arg_name);
                    break;
                }
            }
        }

        self.output.write_all(b"        unsafe {\n")?;
        self.output.write_all(b"            let obj = self.obj.unwrap();\n")?;

        if func.return_val.is_some() {
            self.output.write_fmt(format_args!(
                "            let ret_val = ((*obj.funcs).{})(",
                func.name
            ))?;
        } else {
            self.output.write_fmt(format_args!("            ((*obj.funcs).{})(", func.name))?;
        }

        // TODO: Clean this up

        func.write_func_def(&mut self.output, |index, arg| {
            if index == 0 {
                ("obj.privd".to_owned(), String::new())
            } else if !arg.primitive {
                if let Some(name) = name_remap.get(&index) {
                    (name.to_owned(), String::new())
                } else if arg.reference {
                    (format!("{}.obj.unwrap().privd", arg.name), String::new())
                } else {
                    (arg.name.to_owned(), String::new())
                }
            } else if arg.reference {
                (format!("{}.obj.unwrap().privd", arg.name), String::new())
            } else {
                (arg.name.to_owned(), String::new())
            }
        })?;

        self.output.write_all(b")")?;

        // Handle if we have a return value

        if let Some(ref ret_val) = func.return_val {
            self.output.write_all(b";\n")?;
            let mut skip_return_gen = false;

            for handler in type_handlers.iter() {
                if ret_val.vtype == handler.match_type() {
                    handler.gen_body_return(&ret_val, &mut self.output)?;
                    skip_return_gen = true;
                    break;
                }
            }

            if !skip_return_gen {
                if ret_val.primitive {
                    self.output.write_fmt(format_args!("            ret_val\n"))?;
                } else if ret_val.optional {
                    self.output.write_fmt(format_args!("            if ret_val.privd.is_null() {{\n"))?;
                    self.output.write_fmt(format_args!("                None\n"))?;
                    self.output.write_fmt(format_args!("            }} else {{\n"))?;
                    self.output.write_fmt(format_args!(
                        "                Some({} {{ obj: Some(ret_val) }})\n",
                        ret_val.vtype
                    ))?;
                    self.output.write_fmt(format_args!("            }}\n"))?;
                } else {
                    self.output.write_fmt(format_args!(
                        "            {} {{ obj: Some(ret_val) }}\n",
                        ret_val.vtype
                    ))?;
                }
            }
        } else {
            self.output.write_all(b"\n")?;
        }

        self.output.write_all(b"        }\n")?;
        self.output.write_all(b"    }\n\n")?;

        Ok(())
    }

    fn get_function_args(func: &Function) -> String {
        let mut args = String::new();

        for arg in &func.function_args {
            args.push_str(&arg.vtype);
            args.push_str(", ");
        }

        args
    }

    ///
    /// Generate something that looks like this
    ///
    /// macro_rules! set_released_event {
    ///     ($sender:expr, $data:expr, $call_type:ident, $callback:path) => {
    ///         {
    ///             extern "C" fn temp_call(target: *mut std::os::raw::c_void) {
    ///                 unsafe {
    ///                     let app = target as *mut $call_type;
    ///                     $callback(&mut *app);
    ///                 }
    ///             }
    ///
    ///             unsafe {
    ///                 let obj = $sender.obj.unwrap();
    ///                 ((*obj).set_value_changed_event)((*obj).privd, ::std::mem::transmute($data), temp_call);
    ///             }
    ///         }
    ///     }
    /// }
    fn generate_set_event_impl(&mut self, connect_funcs: &Vec<(&String, &Function)>) -> io::Result<()> {
        for funcs in connect_funcs {
            let mut template_data = Object::new();

            let ffi_args = funcs.1.gen_func_def(|index, arg| {
                if index == 0 {
                    (arg.name.to_owned(), "*const ::std::os::raw::c_void".to_owned())
                } else {
                    (arg.name.to_owned(), arg.get_rust_ffi_type())
                }
            });

            let rust_args = funcs.1.gen_func_def(|index, arg| {
                if index == 0 {
                    ("&mut *app".to_owned(), String::new())
                } else {
                    (arg.name.to_owned(), String::new())
                }
            });

            template_data.insert("name".to_owned(), Value::Str(funcs.0.to_owned()));
            template_data.insert("ffi_args".to_owned(), Value::Str(ffi_args));
            template_data.insert("rust_args".to_owned(), Value::Str(rust_args));

            let output = self.callback_template.render(&template_data).unwrap();

            self.output.write_all(output.as_bytes())?;
        }

        Ok(())
    }

    ///
    /// This code assumes that the connection name has the same number of args
    ///
    fn generate_set_event(&mut self, api_def: &ApiDef) -> io::Result<()> {
        let mut event_names: HashMap<String, Function> = HashMap::new();

        for sdef in api_def.entries.iter().filter(|s| !s.is_pod()) {
            let funcs = api_def.collect_callback_functions(&sdef);

            for func in funcs
                .iter()
                .filter(|s| s.func_type == FunctionType::Callback)
            {
                let args = Self::get_function_args(&func);
                let mut found = true;

                if let Some(ref f) = event_names.get(&func.name) {
                    let current_args = Self::get_function_args(f);
                    if &current_args != &args {
                        println!(
                            "Signal: {} - has versions with diffrent args {} - {}",
                            func.name, current_args, args
                        );
                        return Err(Error::new(ErrorKind::Other, "Fail"));
                    }
                } else {
                    found = false;
                }

                if !found {
                    event_names.insert(func.name.clone(), func.clone());
                }
            }
        }

        let mut event_list = event_names.iter().collect::<Vec<(&String, &Function)>>();
        event_list.sort_by(|a, b| a.0.cmp(b.0));

        // println!("{:?}", event_list);

        self.generate_set_event_impl(&event_list)
    }

    /// Generate something that looks like this
    ///
    /// macro_rules! set_released_event {
    ///     ($sender:expr, $data:expr, $call_type:ident, $callback:path) => {
    ///         {
    ///             extern "C" fn temp_call(target: *mut std::os::raw::c_void, event: *const PUBase) {
    ///                 unsafe {
    ///                     let app = target as *mut $call_type;
    ///                     let event = EventType { obj: Some(*event) };
    ///                     $callback(&mut *app, &event);
    ///                 }
    ///             }
    ///
    ///             unsafe {
    ///                 let obj = $sender.obj.unwrap();
    ///                 ((*obj.funcs).set_<something>_event)((*obj).privd, ::std::mem::transmute($data), temp_call);
    ///             }
    ///         }
    ///     }
    /// }
    ///
    /// Generate the "virtual" set events that connects overriden functions in the Qt class
    ///
    fn generate_virt_set_event(&mut self, api_def: &ApiDef) -> io::Result<()> {
        let mut event_names: HashMap<String, Function> = HashMap::new();

        for sdef in api_def.entries.iter().filter(|s| !s.is_pod()) {
            let mut funcs = Vec::new();
            sdef.get_event_functions(&mut funcs);

            for func in &funcs {
                event_names.insert(func.name.clone(), func.clone());
            }
        }

        let mut event_list = event_names.iter().collect::<Vec<(&String, &Function)>>();
        event_list.sort_by(|a, b| a.0.cmp(b.0));

        for func in &event_list {
            let mut template_data = Object::new();

            let event_type = func.0.to_camel_case();

            template_data.insert("name".to_owned(), Value::Str(func.0.to_owned()));
            template_data.insert("event_type".to_owned(), Value::Str(event_type));

            let output = self.event_template.render(&template_data).unwrap();

            self.output.write_all(output.as_bytes())?;
        }

        Ok(())
    }

    ///
    ///
    ///
    ///
    fn generate_impl(&mut self, type_handlers: &Vec<Box<TypeHandler>>, api_def: &ApiDef) -> io::Result<()> {
        for sdef in api_def.entries.iter().filter(|s| !s.is_pod()) {
            self.output.write_fmt(format_args!("impl {} {{\n", sdef.name))?;

            // If we have a create function we also have a destroy one
            if sdef.should_have_create_func() {
                self.output.write_all(DESTROY_TEMPLATE)?;
            }

            for func in api_def.collect_regular_functions(&sdef) {
                self.generate_func_impl(&func, type_handlers)?;
            }

            self.output.write_all(b"}\n\n")?;

            for trait_name in api_def.get_traits(&sdef) {
                self.output.write_fmt(format_args!("impl {} for {} {{\n", trait_name, sdef.name))?;
                self.output.write_fmt(format_args!(
                    "    fn get_{}_obj(&self) -> *const PUBase {{\n",
                    trait_name.to_snake_case()
                ))?;
                self.output.write_all(b"       let obj = self.obj.unwrap();\n")?;
                self.output.write_all(b"       obj.privd as *const PUBase\n")?;
                self.output.write_all(b"    }\n")?;
                self.output.write_all(b"}\n\n")?;
            }
        }

        Ok(())
    }

    fn generate_ui_impl(&mut self, api_def: &ApiDef) -> io::Result<()> {
        self.output.write_all(UI_HEADER)?;

        for sdef in api_def
            .entries
            .iter()
            .filter(|s| !s.is_pod() && s.should_have_create_func())
        {
            let snake_name = sdef.name.to_snake_case();

            self.output.write_fmt(format_args!(
                "    pub fn create_{}(&self) -> {} {{\n",
                snake_name, sdef.name
            ))?;
            self.output.write_fmt(format_args!(
                "        {} {{ obj: Some(unsafe {{ ((*self.pu).create_{})((*self.pu).privd) }}) }}\n",
                sdef.name, snake_name
            ))?;
            self.output.write_all(b"    }\n\n")?;
        }

        self.output.write_all(b"}\n")?;

        Ok(())
    }

    fn new(filename: &str) -> RustGenerator {
        let parser = ParserBuilder::with_liquid().build();

        RustGenerator {
            callback_template: parser.parse(CALLBACK_TEMPLATE).unwrap(),
            event_template: parser.parse(EVENT_TEMPLATE).unwrap(),
            output: File::create(filename).unwrap(),
        }
    }
}

pub fn generate_rust_bindings(filename: &str, api_def: &ApiDef) -> io::Result<()> {
    let mut rust_gen = RustGenerator::new(filename);

    //let mut f = File::create(filename)?;
    let mut type_handlers: Vec<Box<TypeHandler>> = Vec::new();

    type_handlers.push(Box::new(StringTypeHandler {}));
    type_handlers.push(Box::new(RectTypeHandler {}));

    rust_gen.output.write_all(HEADER)?;

    rust_gen.generate_struct(&api_def.entries)?;
    rust_gen.generate_traits(&mut type_handlers, &api_def)?;
    rust_gen.generate_impl(&type_handlers, &api_def)?;
    rust_gen.generate_set_event(&api_def)?;
    rust_gen.generate_virt_set_event(&api_def)?;

    rust_gen.generate_ui_impl(&api_def)
}
