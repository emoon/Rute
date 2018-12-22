use crate::api_parser::*;
use heck::{CamelCase, SnakeCase};
use liquid::value::{Object, Value};
use liquid::{ParserBuilder, Template};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

use crate::rust_gen_templates::*;

///
/// Keeps track of all the type handlers and when to apply them
///
struct TypeHandler {
    pub mapping: HashMap<&'static str, Box<TypeHandlerTrait>>,
    // We use a separate type handler for traits as we don't match the types for it as we do a
    // basic string matching on the name instead of register all the types
    pub trait_handler: Box<TypeHandlerTrait>,
    // We also use a separate handler for enums with same motivation as above as we can just check
    // the type instead of matching all enum names
    pub enum_handler: Box<TypeHandlerTrait>,
}

///
/// The Rust generator
///
pub struct RustGenerator {
    rust_func_template: Template,
    callback_template: Template,
    callback_trampoline_template: Template,
    trait_impl_end_template: Template,
    trait_impl_template: Template,
    struct_impl_template: Template,
    drop_template: Template,
    type_handler: TypeHandler,
}

///
/// Trait for handling different types that needs to be overriden
///
trait TypeHandlerTrait {
    fn replace_type(&self, arg: &Variable, _is_return_value: IsReturnArg) -> Cow<str> {
        arg.type_name.clone().into()
    }

    fn gen_body_return(&self, _varible: &Variable) -> Cow<str> {
        "".into()
    }

    ///
    /// Generater from Rust to FFI call
    ///
    fn gen_body_to_ffi(&self, _arg: &Variable, _index: usize) -> (Cow<str>, Cow<str>);

    ///
    /// Generate from FFI call to Rust
    ///
    fn gen_body_to_rust(&self, _arg: &Variable, _index: usize) -> (Cow<str>, Cow<str>) {
        ("".into(), "".into())
    }
}

///
/// String type handler
///
#[derive(Clone)]
struct StringTypeHandler;

///
/// We need to handle strings in a special way. They need to be sent down using CString and the
/// pointer to it so have a generator for it
///
impl TypeHandlerTrait for StringTypeHandler {
    fn replace_type(&self, _arg: &Variable, is_ret_value: IsReturnArg) -> Cow<str> {
        match is_ret_value {
            IsReturnArg::Yes => "String".into(),
            IsReturnArg::No => "&str".into(),
        }
    }

    fn gen_body_return(&self, _varible: &Variable) -> Cow<str> {
        "CStr::from_ptr(ret_val).to_string_lossy().into_owned()".into()
    }

    fn gen_body_to_ffi(&self, arg: &Variable, index: usize) -> (Cow<str>, Cow<str>) {
        let arg_name = format!("str_in_{}_{}", arg.name, index);
        (
            format!("{}.as_ptr()", arg_name).into(),
            format!("let {} = CString::new({}).unwrap();\n", arg_name, arg.name).into(),
        )
    }

    fn gen_body_to_rust(&self, arg: &Variable, index: usize) -> (Cow<str>, Cow<str>) {
        let arg_name = format!("str_in_{}_{}", arg.name, index);
        (
            format!("{}.to_str().unwrap()", arg_name).into(),
            format!("let {} = CStr::from_ptr({});\n", arg_name, arg.name).into(),
        )
    }
}

///
/// String type handler
///
#[derive(Clone)]
struct TraitTypeHandler;

///
/// Handling of TraitTypes
///
impl TypeHandlerTrait for TraitTypeHandler {
    fn gen_body_to_ffi(&self, arg: &Variable, index: usize) -> (Cow<str>, Cow<str>) {
        let type_name = &arg.type_name[..arg.type_name.len() - 4];
        let arg_name = format!("obj_{}_{}", arg.name, index);
        (
            arg_name.to_owned().into(),
            //format!("{}.as_ptr()", arg_name).into(),
            format!(
                "let ({}, _funcs) = {}.get_{}_obj_funcs();\n",
                arg_name,
                arg.name,
                type_name.to_snake_case()
            ).into(),
        )
    }

    fn gen_body_to_rust(&self, arg: &Variable, index: usize) -> (Cow<str>, Cow<str>) {
        let type_name = &arg.type_name[..arg.type_name.len() - 4];
        let arg_name = format!("obj_{}_{}", arg.name, index);
        (
            format!("&{}", arg_name).into(),
            format!(
                "let {} = {}::new_from_temporary(*({} as *const RU{}));",
                arg_name, type_name, arg.name, type_name
            ).into(),
        )
    }
}

///
/// Enum type handler
///
#[derive(Clone)]
struct EnumTypeHandler;

///
/// Enums are being handled separately as we need to cast them between Rust and native for C++
///
impl TypeHandlerTrait for EnumTypeHandler {
    fn gen_body_return(&self, varible: &Variable) -> Cow<str> {
        format!("{{ transmute::<i32, {}>(ret_val) }}", varible.enum_sub_type).into()
    }

    fn gen_body_to_ffi(&self, arg: &Variable, index: usize) -> (Cow<str>, Cow<str>) {
        let arg_name = format!("enum_{}_{}", arg.name, index);
        (
            arg_name.to_owned().into(),
            format!("let {} = {} as i32;\n", arg_name, arg.name).into(),
        )
    }
}

///
///
///
impl TypeHandler {
    ///
    /// Create a new instance of the handle
    ///
    fn new() -> TypeHandler {
        TypeHandler {
            mapping: HashMap::new(),
            trait_handler: Box::new(TraitTypeHandler {}),
            enum_handler: Box::new(EnumTypeHandler {}),
        }
    }

    ///
    /// Get a handler for a given type. This code will special case if the type ends with "Type"
    /// such as "WidgetType". In that case it will return a TraitHandler as "*Type" is expected to
    /// always be traits. If no handler was found None is returned
    ///
    fn get(&self, arg: &Variable) -> Option<&Box<TypeHandlerTrait>> {
        let type_name = arg.type_name.as_str();

        if arg.vtype == VariableType::Enum {
            Some(&self.enum_handler)
        } else if type_name.ends_with("Type") {
            Some(&self.trait_handler)
        } else {
            self.mapping.get(type_name)
        }
    }
}

///
/// Used for creating single letter assignments for generics
///
struct GenericLabelAssign<'a> {
    lookup: BTreeMap<&'a str, u32>,
}

impl<'a> GenericLabelAssign<'a> {
    pub fn new() -> GenericLabelAssign<'a> {
        GenericLabelAssign {
            lookup: BTreeMap::new(),
        }
    }

    ///
    /// Returns a new char for a type and tries to use the starting
    /// type name, if fails goes from A,B,C,... and will panic if it runs out
    ///
    pub fn get(&mut self, gen_type: &'a str) -> Option<char> {
        if !gen_type.ends_with("Type") {
            return None;
        }

        // If we have the type we just return it
        if let Some(label) = self.lookup.get(gen_type) {
            return Some(*label as u8 as char);
        }

        let mut current_char = gen_type.chars().next().unwrap() as u32;

        for _c in 0..27 {
            let found = self.lookup.iter().any(|(_, c)| *c == current_char);

            if !found {
                self.lookup.insert(gen_type, current_char);
                return Some(current_char as u8 as char);
            }

            current_char += 1;

            if current_char >= b'W' as u32 {
                current_char = b'A' as u32;
            }
        }

        // This should never happen (unless we have > 27 unique TypeParameters which we wont)
        panic!("Unable to find a character to use for {}", gen_type);
    }
}

///
/// ReturnType bool
///
#[derive(PartialEq, Clone, Copy)]
enum IsReturnArg {
    Yes,
    No,
}

///
/// Setup the type handlers
///

fn setup_type_handlers() -> TypeHandler {
    let mut type_handler = TypeHandler::new();
    type_handler
        .mapping
        .insert("String", Box::new(StringTypeHandler {}));
    type_handler
}

fn get_trait_name(name: &str) -> String {
    if name.ends_with("Type") {
        format!("{}Trait", &name[..name.len() - 4])
    } else {
        name.to_owned()
    }
}

fn get_real_type(name: &str) -> &str {
    if name.ends_with("Type") {
        &name[..name.len() - 4]
    } else {
        &name
    }
}

impl RustGenerator {
    pub fn new() -> RustGenerator {
        let parser = ParserBuilder::with_liquid().build();

        RustGenerator {
            type_handler: setup_type_handlers(),
            rust_func_template: parser.parse(RUST_FUNC_IMPL_TEMPLATE).unwrap(),
            drop_template: parser.parse(RUST_DROP_TEMPLATE).unwrap(),
            callback_template: parser.parse(RUST_CALLBACK_TEMPLATE).unwrap(),
            callback_trampoline_template: parser.parse(RUST_CALLBACK_TRAMPOLINE_TEMPLATE).unwrap(),
            trait_impl_template: parser.parse(RUST_IMPL_TRAIT_TEMPLATE).unwrap(),
            struct_impl_template: parser.parse(RUST_STRUCT_IMPL_TEMPLATE).unwrap(),
            trait_impl_end_template: parser.parse(RUST_IMPL_TRAIT_END_TEMPLATE).unwrap(),
        }
    }

    ///
    /// Takes in a varibale and generates a Rust function variable argument
    ///
    fn generate_arg_type(
        &self,
        dest: &mut String,
        var: &Variable,
        return_arg: IsReturnArg,
        need_lifetime: bool,
        real_type: bool,
    ) {
        dest.clear();

        // Run code to replace type if neeed
        let type_name = {
            match self.type_handler.get(var) {
                Some(handler) => handler.replace_type(var, return_arg),
                None => var.type_name.clone().into(),
            }
        };

        if var.optional {
            dest.push_str("Option<");
        }

        match var.vtype {
            VariableType::None => dest.push_str("<None>"),
            VariableType::SelfType => dest.push_str("&self"),
            VariableType::Primitive => dest.push_str(&type_name),
            VariableType::Enum => dest.push_str(&var.enum_sub_type),
            VariableType::Regular => {
                if real_type {
                    dest.push_str(get_real_type(&type_name));
                } else {
                    dest.push_str(&get_trait_name(&type_name));
                }

                if need_lifetime {
                    dest.push_str("<'a>");
                }
            }
            VariableType::Str => {
                if return_arg == IsReturnArg::Yes {
                    dest.push_str("String");
                } else {
                    dest.push_str("&str");
                }
            }

            VariableType::Reference => {
                if return_arg == IsReturnArg::No {
                    dest.push('&');
                }

                if real_type {
                    dest.push_str(get_real_type(&type_name));
                } else {
                    dest.push_str(&get_trait_name(&type_name));
                }

                if need_lifetime {
                    dest.push_str("<'a>");
                }
            }
        }

        if var.optional {
            dest.push('>');
        }
    }

    ///
    /// Generate a function implementation
    ///
    fn generate_func_def(&self, func: &Function, _struct_name: &str) -> (bool, String) {
        let mut temp_str = String::with_capacity(128);
        let mut func_imp = String::with_capacity(128);
        let mut gen_labels = GenericLabelAssign::new();
        let is_static_func = func.func_type == FunctionType::Static;
        let mut has_generic_params = false;

        // Check if we have any generic types, then we need to start with constructing the labels
        // for it

        for arg in &func.function_args[1..] {
            gen_labels.get(&arg.type_name);
        }

        // if we have some types we need to generated the generic setup
        if !gen_labels.lookup.is_empty() {
            has_generic_params = true;
            let len = gen_labels.lookup.len() - 1;

            func_imp.push('<');

            /*
            if func.func_type == FunctionType::Static {
                func_imp.push_str("'a, ");
            }
            */

            for (i, (name, label)) in gen_labels.lookup.iter().enumerate() {
                func_imp.push_str(&format!("{}: {}<'a>", *label as u8 as char, get_trait_name(&name)));

                if len != i {
                    func_imp.push_str(", ");
                }
            }

            func_imp.push('>');
        }

        if is_static_func {
            func_imp.push_str("(");
        } else {
            func_imp.push_str("(&self");
        }

        for (index, arg) in func.function_args[1..].iter().enumerate() {
            if let Some(label) = gen_labels.get(&arg.type_name) {
                temp_str.clear();
                temp_str.push('&');
                temp_str.push(label);
            } else {
                self.generate_arg_type(&mut temp_str, &arg, IsReturnArg::No, is_static_func, false);
            }

            if !(is_static_func && index == 0) {
                func_imp.push_str(", ");
            }

            func_imp.push_str(&arg.name);
            func_imp.push_str(": ");

            if arg.array {
                func_imp.push_str(&format!("&[{}]", temp_str));
            } else {
                func_imp.push_str(&temp_str);
            }
        }

        func_imp.push(')');

        //
        // If we don't have any return value we alwayes return self
        //
        if func.return_val.is_none() {
            if !is_static_func {
                func_imp.push_str(" -> &Self");
            }
        //func_imp.push_str(" -> &");
        //func_imp.push_str(struct_name);
        //func_imp.push_str("<'a>");
        } else {
            let ret_val = func.return_val.as_ref().unwrap();

            self.generate_arg_type(&mut temp_str, ret_val, IsReturnArg::Yes, is_static_func, false);
            func_imp.push_str(" -> ");

            if ret_val.array {
                if ret_val.vtype == VariableType::Primitive {
                    func_imp.push_str("PrimitiveArray<");
                    func_imp.push_str(&temp_str);
                    func_imp.push_str(">");
                } else {
                    func_imp.push_str("RefArray<");
                    func_imp.push_str(&temp_str);
                    func_imp.push_str(", WrapperRcOwn>");
                }
            } else {
                func_imp.push_str(&temp_str);
            }
        }

        (has_generic_params, func_imp)
    }

    ///
    /// Generates the implementations for the structs
    ///
    fn generate_trait_impl<W: Write>(&self, dest: &mut W, sdef: &Struct) -> io::Result<()> {
        writeln!(dest, "pub trait {}Trait<'a> {{", sdef.name)?;

        let mut template_data = Object::new();
        template_data.insert("type_name".into(), Value::scalar(&sdef.name));
        template_data.insert(
            "type_name_snake".into(),
            Value::scalar(sdef.name.to_snake_case()),
        );

        let out = self.trait_impl_end_template.render(&template_data).unwrap();
        dest.write_all(out.as_bytes())?;

        for name in &sdef.full_inherit {
            let mut template_data = Object::new();
            template_data.insert("trait_name".into(), Value::scalar(name));
            template_data.insert("target_name".into(), Value::scalar(&sdef.name));
            template_data.insert("type_name".into(), Value::scalar(name));
            template_data.insert(
                "target_name_snake".into(),
                Value::scalar(name.to_snake_case()),
            );
            template_data.insert(
                "target_name_snake_org".into(),
                Value::scalar(name.to_snake_case()),
            );

            let out = self.trait_impl_template.render(&template_data).unwrap();
            dest.write_all(out.as_bytes())?;
        }

        Ok(())
    }

    //
    //
    //
    fn generate_callback(&self, func: &Function, struct_name: &str, template: &Template) -> String {
        let mut template_data = Object::new();
        //let mut function_params = String::with_capacity(128);
        let mut function_arg_types = String::with_capacity(128);
        let mut temp_str = String::with_capacity(128);

        let ffi_func_def = func.rust_func_def(
            true,
            Some("*const c_void, func: *const c_void"),
            |arg, is_ret| arg.get_rust_ffi_type(is_ret).into(),
        );

        let len = func.function_args[1..].len().wrapping_sub(1);
        for (index, arg) in func.function_args[1..].iter().enumerate() {
            self.generate_arg_type(&mut temp_str, &arg, IsReturnArg::No, false, true);
            function_arg_types.push_str(&temp_str);

            if index != len {
                function_arg_types.push_str(", ");
            }
        }

        let mut function_args = Vec::with_capacity(func.function_args.len());
        let mut body_setup = String::with_capacity(4096);

        // Setup the input args
        for (i, arg) in func.function_args[1..].iter().enumerate() {
            match self.type_handler.get(&arg) {
                Some(handler) => {
                    let (gen_arg, body) = handler.gen_body_to_rust(&arg, i);
                    function_args.push((true, gen_arg));
                    body_setup.push_str("        ");
                    body_setup += &body;
                }
                None => {
                    function_args.push((false, arg.name.clone().into()));
                }
            }
        }

        template_data.insert("body_setup".into(), Value::scalar(body_setup));

        let mut func_args = String::with_capacity(128);

        let len = function_args.len().wrapping_sub(1);
        // Generate the function call
        for (index, _arg) in function_args.iter().enumerate() {
            func_args.push_str(&function_args[index].1);

            if index != len {
                func_args.push_str(", ");
            }
        }

        template_data.insert(
            "event_name".into(),
            Value::scalar(func.get_name_skip_event().to_owned()),
        );
        template_data.insert("function_arguments".into(), Value::scalar(ffi_func_def));
        template_data.insert("function_params".into(), Value::scalar(func_args));
        template_data.insert(
            "function_arg_types".into(),
            Value::scalar(function_arg_types),
        );
        template_data.insert("widget_name".into(), Value::scalar(struct_name.to_owned()));
        template_data.insert(
            "widget_snake_name".into(),
            Value::scalar(struct_name.to_snake_case()),
        );

        template.render(&template_data).unwrap()
    }

    //
    // Do the actual function generation
    //
    fn generate_function(&self, func: &Function, struct_name: &str, base_name: &str) -> String {
        let mut template_data = Object::new();
        let is_static_func = func.func_type == FunctionType::Static;

        // Generate function declaration
        let (has_generic_params, func_def) = self.generate_func_def(func, struct_name);

        template_data.insert("func_name".into(), Value::scalar(&func.name));

        // If it's a static func but doesn't have any generic parametres we need to append the
        // lifetime at the end of the function name

        if !has_generic_params && is_static_func {
            template_data.insert(
                "func_name_header".into(),
                Value::scalar(format!("{}", func.name)),
            );
        } else {
            template_data.insert("func_name_header".into(), Value::scalar(&func.name));
        }

        template_data.insert("static_func".into(), Value::scalar(is_static_func));
        template_data.insert("function_def".into(), Value::scalar(func_def));

        if is_static_func {
            template_data.insert(
                "obj_funcs_name".into(),
                Value::scalar(base_name.to_snake_case()),
            );
        } else {
            template_data.insert(
                "obj_funcs_name".into(),
                Value::scalar(struct_name.to_snake_case()),
            );
        }

        let mut function_args = Vec::with_capacity(func.function_args.len());
        let mut body_setup = String::with_capacity(4096);

        // Setup the input args
        for (i, arg) in func.function_args.iter().enumerate() {
            match self.type_handler.get(&arg) {
                Some(handler) => {
                    let (gen_arg, body) = handler.gen_body_to_ffi(&arg, i);
                    function_args.push((true, gen_arg));
                    body_setup.push_str("        ");
                    body_setup += &body;
                }
                None => {
                    function_args.push((false, arg.name.clone().into()));
                }
            }
        }

        template_data.insert("body_setup".into(), Value::scalar(body_setup));

        let mut func_args = String::with_capacity(128);

        // Generate the function call
        for (index, arg) in func.function_args.iter().enumerate() {
            match arg.vtype {
                VariableType::SelfType => func_args.push_str("obj_data"),
                _ => {
                    func_args.push_str(", ");
                    func_args.push_str(&function_args[index].1);
                }
            }
        }

        //
        // Setup the return part of the template
        //

        template_data.insert(
            "return_value".into(),
            Value::scalar(func.return_val.is_some()),
        );
        template_data.insert("function_args".into(), Value::scalar(func_args));

        func.return_val.as_ref().map(|ret_val| {
            template_data.insert("return_type".into(), Value::scalar("none"));
            template_data.insert("optional_return".into(), Value::scalar(ret_val.optional));

            match self.type_handler.get(&ret_val) {
                Some(handler) => {
                    let ret = handler.gen_body_return(&ret_val);
                    template_data.insert("return_type".into(), Value::scalar("replaced"));
                    template_data.insert("replaced_return".into(), Value::scalar(ret.into_owned()));
                }
                None => {
                    match ret_val.vtype {
                        VariableType::Regular => {
                            template_data
                                .insert("return_type".into(), Value::scalar("pointer_ref"));
                            template_data.insert(
                                "return_vtype".into(),
                                Value::scalar(get_trait_name(&ret_val.type_name)),
                            );
                        }

                        VariableType::Reference => {
                            template_data
                                .insert("return_type".into(), Value::scalar("pointer_ref"));
                            template_data.insert(
                                "return_vtype".into(),
                                Value::scalar(ret_val.type_name.to_owned()),
                            );
                        }

                        _ => (),
                    }

                    if ret_val.array && ret_val.vtype == VariableType::Primitive {
                        template_data.insert("return_type".into(), Value::scalar("primitive_array"));
                        template_data.insert("return_vtype".into(), Value::scalar(ret_val.type_name.to_owned()));
                    }
                }
            }
        });

        self.rust_func_template.render(&template_data).unwrap()
    }

    fn find_api_def<'a>(name: &str, api_defs: &'a [ApiDef]) -> Option<&'a Struct> {
        api_defs
            .iter()
            .flat_map(|api_def| api_def.class_structs.iter())
            .find(|s| s.name == name)
    }

    fn generate_functions(&self, dest: &mut String, sdef: &Struct,
            gen_comments: bool, func_names_lookup: &mut HashSet<String>) {
        sdef.functions
            .iter()
            .for_each(|f| {
                let res;
                if !func_names_lookup.contains(&f.name) {
                    if gen_comments {
                        //dest.push("**Notice these docs are heavy WIP and not very relevent yet**");
                        dest.push_str(&f.doc_comments);
                    } else {
                        dest.push_str("    #[doc(hidden)]\n");
                    }

                    if f.func_type == FunctionType::Event || f.func_type == FunctionType::Signal {
                        res = self.generate_callback(&f, &sdef.name, &self.callback_template);
                    } else {
                        res = self.generate_function(&f, &sdef.name, &sdef.name);
                    }
                    dest.push_str(&res);
                    func_names_lookup.insert(f.name.clone());
                }
            });
    }

    ///
    /// Generate the structs. The structs will be generated in this style
    ///
    /// pub struct Application<'a> {
    ///     data: Rc<Cell<Option<RUApplication>>>,
    ///     _marker: PhantomData<::std::cell::Cell<&'a ()>>,
    /// }
    ///
    fn generate_structs<W: Write>(&self, dest: &mut W, api_def: &ApiDef, api_defs: &[ApiDef]) -> io::Result<()> {
        // generate the trampoline functions
        for sdef in &api_def.class_structs {
            sdef.functions
                .iter()
                .filter(|f| f.func_type == FunctionType::Signal || f.func_type == FunctionType::Event)
                .try_for_each(|func| {
                    let res =
                        self.generate_callback(&func, &sdef.name, &self.callback_trampoline_template);
                    dest.write_all(res.as_bytes())
            })?;

            let mut template_data = Object::new();
            template_data.insert("struct_name".into(), Value::scalar(&sdef.name));
            template_data.insert(
                "snake_struct_name".into(),
                Value::scalar(sdef.name.to_snake_case()),
            );
            template_data.insert(
                "wrap_create".into(),
                Value::scalar(sdef.should_gen_wrap_class()),
            );
            template_data.insert(
                "has_new_method".into(),
                Value::scalar(sdef.should_have_create_func()),
            );

            let mut event_funcs = String::with_capacity(512*1024);
            let mut func_lookup = HashSet::new();

            // Reverse order to start with highest inheritance to lowest
            for inh in sdef.full_inherit.iter().rev() {
                if let Some(ref def) = Self::find_api_def(&inh, api_defs) {
                    self.generate_functions(&mut event_funcs, def, def.name == sdef.name,
                        &mut func_lookup);
                }
            }

            template_data.insert("event_funcs".into(), Value::scalar(event_funcs));
            dest.write_all("/// **Notice these docs are heavy WIP and not very relevent yet**\n".as_bytes())?;
            dest.write_all(sdef.doc_comments.as_bytes())?;
            let output = self.struct_impl_template.render(&template_data).unwrap();
            dest.write_all(output.as_bytes())?;
        }

        Ok(())
    }

    ///
    /// Generates the implementations for the structs
    ///
    fn generate_traits_impl<W: Write>(&self, f: &mut W, api_def: &ApiDef) -> io::Result<()> {
        api_def.class_structs.iter().try_for_each(|s| {
            self.generate_trait_impl(f, s)?;

            // Implement drop for structs that needs it

            if s.should_generate_drop() {
                let mut template_data = Object::new();

                template_data.insert("type_name".into(), Value::scalar(s.name.clone()));
                template_data.insert(
                    "type_snake_name".into(),
                    Value::scalar(s.name.to_snake_case()),
                );

                let output = self.drop_template.render(&template_data).unwrap();

                f.write_all(output.as_bytes())?;
            }

            Ok(())
        })
    }

    ///
    ///
    ///
    pub fn generate_rute(&self, filename: &str, _api_defs: &[ApiDef]) -> io::Result<()> {
        let mut f = BufWriter::new(File::create(filename)?);

        // write header
        f.write_all(HEADER)?;

        // impl header
        f.write_all(RUTE_IMPL_HEADER)?;

        Ok(())
    }
    ///
    /// Generates the Rust mod file to import all FFI and Rust generated code
    ///
    pub fn generate_auto_mod(filename: &str, api_defs: &[ApiDef]) -> io::Result<()> {
        let mut dest = BufWriter::new(File::create(filename).unwrap());

        writeln!(
            dest,
            "// This file is auto-generated by rute_gen. DO NOT EDIT.\n"
        )?;

        // Collect all then struct names (snake case) and get a sorted list in the end
        let mut names = api_defs
            .iter()
            .flat_map(|api_def| api_def.class_structs.iter())
            .map(|s| s.name.to_snake_case())
            .collect::<Vec<String>>();

        // rute auto has some extra stuff used by all the files
        //names.push("rute_auto".to_owned());
        names.sort();

        for entry in &names {
            writeln!(dest, "pub mod {}_ffi;", entry)?;
            writeln!(dest, "pub mod {};", entry)?;
        }

        writeln!(dest, "pub mod rute_ffi;")?;
        writeln!(dest, "pub mod rute_enums;")?;
        writeln!(dest, "pub mod rute;\n")?;

        for entry in &names {
            // TODO: Fixme
            writeln!(dest, "pub use {}::*;", entry)?;
            writeln!(dest, "pub use {}_ffi::*;", entry)?;
        }

        writeln!(dest, "pub use rute_ffi::*;")?;
        writeln!(dest, "pub use rute_enums::*;")?;
        writeln!(dest, "pub use rute::*;")
    }

    pub fn generate(&self, filename: &str, api_def: &ApiDef, api_defs: &[ApiDef]) -> io::Result<()> {
        let mut dest = BufWriter::new(File::create(filename)?);

        // write header
        dest.write_all(HEADER)?;

        // write all the structs
        self.generate_structs(&mut dest, api_def, api_defs)?;

        // Generate the implementations for the structs
        self.generate_traits_impl(&mut dest, api_def)?;

        // generate the enums
        Self::generate_enums(&mut dest, api_def)
    }

    fn generate_enums<W: Write>(dest: &mut W, api_def: &ApiDef) -> io::Result<()> {
        for enum_def in &api_def.enums {
            writeln!(dest, "#[repr(u32)]")?;
            writeln!(dest, "pub enum {} {{", enum_def.name)?;

            for e in &enum_def.entries {
                writeln!(dest, "    {} = {},", e.name.to_camel_case(), e.value)?;
            }

            writeln!(dest, "}}\n")?;

            // if enum has a flag alias we need to generate that as well
            if !enum_def.flags_name.is_empty() {
                writeln!(dest, "pub type {} = {};\n", enum_def.flags_name, enum_def.name)?;
            }
        }

        Ok(())
    }

    ///
    /// Generate global enums
    ///
    pub fn generate_global_enums(&self, filename: &str, api_def: &ApiDef) -> io::Result<()> {
        let mut dest = BufWriter::new(File::create(filename)?);

        // write header
        writeln!(
            dest,
            "// This file is auto-generated by rute_gen. DO NOT EDIT.\n"
        )?;

        Self::generate_enums(&mut dest, api_def)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use crate::api_parser::*;

    //
    // Create a default function to reduce duplication a bit
    //
    fn build_default_func() -> Function {
        Function {
            name: "test".to_owned(),
            function_args: vec![Variable {
                name: "self".to_owned(),
                ..Variable::default()
            }],
            ..Function::default()
        }
    }

    //
    // Test function def with self only
    //
    #[test]
    fn test_function_self_only() {
        let rust_gen = RustGenerator::new();
        let func = build_default_func();
        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl.1, "(&self) -> &Self");
    }

    //
    // Test function def with one primitive
    //
    #[test]
    fn test_function_one_primitive() {
        let rust_gen = RustGenerator::new();
        let mut func = build_default_func();
        func.function_args.push(Variable {
            name: "foo".to_owned(),
            type_name: "i32".to_owned(),
            vtype: VariableType::Primitive,
            ..Variable::default()
        });

        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl.1, "(&self, foo: i32) -> &Self");
    }

    #[test]
    fn test_function_string() {
        let rust_gen = RustGenerator::new();
        let mut func = build_default_func();
        func.function_args.push(Variable {
            name: "foo".to_owned(),
            type_name: "String".to_owned(),
            vtype: VariableType::Regular,
            ..Variable::default()
        });

        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl.1, "(&self, foo: &str) -> &Self");
    }

    //
    // Test with one generic widget type
    //
    #[test]
    fn test_function_widget_type() {
        let rust_gen = RustGenerator::new();
        let mut func = build_default_func();
        func.function_args.push(Variable {
            name: "foo".to_owned(),
            type_name: "WidgetType".to_owned(),
            vtype: VariableType::Regular,
            ..Variable::default()
        });

        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl.1, "<W: WidgetTrait<'a>>(&self, foo: &W) -> &Self");
    }

    //
    // Test with one generic widget type
    //
    #[test]
    fn test_staic_function_widget_lifetime_type() {
        let rust_gen = RustGenerator::new();
        let mut func = build_default_func();
        func.function_args.push(Variable {
            name: "foo".to_owned(),
            type_name: "WidgetType".to_owned(),
            vtype: VariableType::Regular,
            ..Variable::default()
        });

        func.func_type = FunctionType::Static;
        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");

        assert_eq!(func_impl.0, true);
        assert_eq!(func_impl.1, "<W: WidgetTrait<'a>>(foo: &W)");
    }

    //
    // Test with two generic widget type
    //
    #[test]
    fn test_function_two_widget_type() {
        let rust_gen = RustGenerator::new();
        let mut func = build_default_func();
        func.function_args.push(Variable {
            name: "foo".to_owned(),
            type_name: "WidgetType".to_owned(),
            vtype: VariableType::Regular,
            ..Variable::default()
        });

        func.function_args.push(Variable {
            name: "bar".to_owned(),
            type_name: "WidgetType".to_owned(),
            vtype: VariableType::Regular,
            ..Variable::default()
        });

        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(
            func_impl.1,
            "<W: WidgetTrait<'a>>(&self, foo: &W, bar: &W) -> &Self"
        );
    }

    //
    // Test with two generic widget type
    //
    #[test]
    fn test_function_three_types_overlap_char() {
        let rust_gen = RustGenerator::new();
        let mut func = build_default_func();
        func.function_args.push(Variable {
            name: "foo".to_owned(),
            type_name: "WidgetType".to_owned(),
            vtype: VariableType::Regular,
            ..Variable::default()
        });

        func.function_args.push(Variable {
            name: "bar".to_owned(),
            type_name: "PaintType".to_owned(),
            vtype: VariableType::Regular,
            ..Variable::default()
        });

        func.function_args.push(Variable {
            name: "pole".to_owned(),
            type_name: "PoleType".to_owned(),
            vtype: VariableType::Regular,
            ..Variable::default()
        });

        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl.1, "<P: PaintTrait<'a>, Q: PoleTrait<'a>, W: WidgetTrait<'a>>(&self, foo: &W, bar: &P, pole: &Q) -> &Self");
    }

    //
    // Test function def with two primitives
    //
    #[test]
    fn test_function_two_primitive() {
        let mut func = build_default_func();
        let rust_gen = RustGenerator::new();

        func.function_args.push(Variable {
            name: "width".to_owned(),
            type_name: "i32".to_owned(),
            vtype: VariableType::Primitive,
            ..Variable::default()
        });
        func.function_args.push(Variable {
            name: "height".to_owned(),
            type_name: "f32".to_owned(),
            vtype: VariableType::Primitive,
            ..Variable::default()
        });

        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl.1, "(&self, width: i32, height: f32) -> &Self");
    }

    //
    // Test function def with self and primitive return
    //
    #[test]
    fn test_function_primitive_return() {
        let mut func = build_default_func();
        let rust_gen = RustGenerator::new();

        func.return_val = Some(Variable {
            type_name: "i32".to_owned(),
            vtype: VariableType::Primitive,
            ..Variable::default()
        });

        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl.1, "(&self) -> i32");
    }

    //
    // Test function def with self and optional return
    //
    #[test]
    fn test_function_primitive_optional_return() {
        let mut func = build_default_func();
        let rust_gen = RustGenerator::new();

        func.return_val = Some(Variable {
            type_name: "f32".to_owned(),
            vtype: VariableType::Primitive,
            optional: true,
            ..Variable::default()
        });

        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl.1, "(&self) -> Option<f32>");
    }

    //
    // Test function def with self and array return
    //
    #[test]
    fn test_function_primitive_array_return() {
        let mut func = build_default_func();
        let rust_gen = RustGenerator::new();

        func.return_val = Some(Variable {
            type_name: "f32".to_owned(),
            vtype: VariableType::Primitive,
            array: true,
            ..Variable::default()
        });

        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl.1, "(&self) -> PrimitiveArray<f32>");
    }

    //
    // Test function def with self, array input
    //
    #[test]
    fn test_function_array_input() {
        let mut func = build_default_func();
        let rust_gen = RustGenerator::new();

        func.function_args.push(Variable {
            name: "width".to_owned(),
            type_name: "i32".to_owned(),
            vtype: VariableType::Primitive,
            array: true,
            ..Variable::default()
        });

        let func_impl = rust_gen.generate_func_def(&func, "TestStruct");
        assert_eq!(func_impl.1, "(&self, width: &[i32]) -> &Self");
    }

    #[test]
    fn test_generic_label_incorrect_type() {
        let mut label_assign = GenericLabelAssign::new();
        assert_eq!(None, label_assign.get("i32"));
    }

    #[test]
    fn test_generic_label_assign_single() {
        let mut label_assign = GenericLabelAssign::new();
        assert_eq!('A', label_assign.get("AType").unwrap());
    }

    #[test]
    fn test_generic_label_assign_single_extra_check() {
        let mut label_assign = GenericLabelAssign::new();
        assert_eq!('A', label_assign.get("AType").unwrap());
        assert_eq!('A', label_assign.get("AType").unwrap());
    }

    #[test]
    fn test_generic_label_assign_double_new_char() {
        let mut label_assign = GenericLabelAssign::new();
        assert_eq!('A', label_assign.get("AType").unwrap());
        assert_eq!('B', label_assign.get("ANextType").unwrap());
    }

    #[test]
    fn test_generic_label_wrap() {
        let mut label_assign = GenericLabelAssign::new();
        assert_eq!('W', label_assign.get("WidgetType").unwrap());
        assert_eq!('A', label_assign.get("WillType").unwrap());
    }

    #[test]
    fn test_generic_many() {
        let mut label_assign = GenericLabelAssign::new();
        assert_eq!('W', label_assign.get("WidgetType").unwrap());
        assert_eq!('L', label_assign.get("LobsterType").unwrap());
        assert_eq!('A', label_assign.get("AstroType").unwrap());
        assert_eq!('B', label_assign.get("Astro2Type").unwrap());
        assert_eq!('C', label_assign.get("BrokenType").unwrap());
    }
}
