use crate::c_gen::*;
use heck::{CamelCase, MixedCase, SnakeCase};
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::api_parser::*;
use crate::qt_gen_templates::*;

use liquid::value::{Object, Value};
use liquid::{Parser, ParserBuilder, Template};

#[derive(PartialEq)]
pub enum EventType {
    Callback,
    Event,
}

///
/// Keeps track of all the type handlers and when to apply them.
/// Notice that we have something similar in rust_gen also but they arent' exatly the same
/// as the generation differes a bit so we have a own version here instead
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
/// Trait for handling different types that needs to be overriden
///
trait TypeHandlerTrait {
    fn replace_arg(&self, arg: &Variable, _is_return_value: IsReturnArg) -> String {
        arg.type_name.clone().into()
    }

    fn gen_body_return(&self, _varible: &Variable) -> String {
        "".into()
    }

    fn gen_body(&self, _arg: &Variable, _index: usize) -> (String, String) {
        ("".into(), "".into())
    }
}
///
/// String type handler
///
#[derive(Clone)]
struct StringTypeHandler;

impl TypeHandlerTrait for StringTypeHandler {
    fn replace_arg(&self, arg: &Variable, _is_return_value: IsReturnArg) -> String {
        format!("QString::fromUtf8({})", &arg.name).into()
    }
}

///
/// Enum type handler
///
#[derive(Clone)]
struct EnumTypeHandler;

impl TypeHandlerTrait for EnumTypeHandler {
    fn replace_arg(&self, arg: &Variable, _is_return_value: IsReturnArg) -> String {
        let mut base_name = arg.type_name.as_str();
        let mut prefix_char = "";

        if arg.type_name == "Rute" {
            base_name = "Qt";
        } else {
            prefix_char = "Q";
        }

        format!(
            "({}{}::{}){}",
            prefix_char,
            base_name,
            arg.enum_sub_type,
            &arg.name
        ).into()
    }
}

///
/// String type handler
///
struct TraitTypeHandler {
    pub template: Template,
}

impl TypeHandlerTrait for TraitTypeHandler {
    fn replace_arg(&self, arg: &Variable, _is_return_value: IsReturnArg) -> String {
        if arg.pointer {
            format!("({}*){}", arg.qt_type_name, &arg.name).into()
        } else {
            format!("*(({}*){})", arg.qt_type_name, &arg.name).into()
        }
    }

    fn gen_body(&self, arg: &Variable, index: usize) -> (String, String) {
        let arg_name = format!("obj_in_{}_{}", arg.name, index);
        let mut object = Object::new();

        object.insert("input_var".into(), Value::scalar(&arg.name));

        object.insert(
            "type_name".into(),
            Value::scalar(arg.get_untyped_name().to_owned()),
        );
        object.insert("var_name".into(), Value::scalar(&arg_name));
        object.insert(
            "type_name_snake".into(),
            Value::scalar(arg.get_untyped_name().to_snake_case()),
        );

        // Render using the QT_TRAIT_TYPE_TEMPLATE template
        (
            format!("(struct RUBase*)&{}", arg_name).into(),
            self.template.render(&object).unwrap().into(),
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
    fn new(parser: &Parser) -> TypeHandler {
        TypeHandler {
            mapping: HashMap::new(),
            enum_handler: Box::new(EnumTypeHandler {}),
            trait_handler: Box::new(TraitTypeHandler {
                template: parser.parse(QT_TRAIT_TYPE_TEMPLATE).unwrap(),
            }),
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
/// Adds some extra functionallity to Struct to make some checks easier
///
impl Struct {
    fn inherits_widget(&self) -> bool {
        self.full_inherit.iter().any(|i| i == "Widget")
    }
}
///
/// Generate a string in this style;
///
/// void* self_c, void* wrapped_func, int row
///
pub fn generate_c_function_args_signal_wrapper(event_type: EventType, func: &Function) -> String {
    let mut function_args = String::new();
    let len = func.function_args.len();

    // write arguments
    for (i, arg) in func.function_args.iter().enumerate() {
        if i == 0 {
            /*
               if event_type == EventType::Event {
               function_args.push_str("RUBase*, void*");
               } else {
               function_args.push_str("void* self_c, void* trampoline_func");
               }
               */
        } else {
            // References are passed as pointers to the C code
            match arg.vtype {
                VariableType::Reference => function_args.push_str("struct RUBase*"),
                _ => function_args.push_str(&arg.get_c_type(IsReturnType::No)),
            }

            function_args.push_str(" ");
            function_args.push_str(&arg.name);
        }

        if i != len - 1 {
            function_args.push_str(", ");
        }
    }

    if func.function_args.is_empty() && event_type == EventType::Event {
        function_args.push_str("RUBase*");
    }

    function_args
}

///
/// Generate includes from all the structs which are non-pod
///
fn generate_includes<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
    // There is a small hack here where we include generating includes for the static
    // funcs structs as it doesn't map to a qt class
    for sdef in api_def
        .class_structs
        .iter()
        .filter(|s| s.name != "StaticFuncs")
    {
        f.write_fmt(format_args!("#include <{}>\n", sdef.qt_name))?;
        f.write_fmt(format_args!(
            "#include \"{}_ffi.h\"\n",
            sdef.name.to_snake_case()
        ))?;
    }

    f.write_all(b"\n")
}

///
/// Generate signal wrapper name.
///
/// Example:
///
/// self_i32_u32_void
///
fn signal_type_callback(func: &Function) -> String {
    let mut name_def = String::with_capacity(128);

    for arg in &func.function_args {
        match arg.vtype {
            VariableType::SelfType => name_def.push_str("self"),
            VariableType::Reference => name_def.push_str(&arg.type_name),
            VariableType::Str => name_def.push_str("string"),
            _ => name_def.push_str(&arg.get_c_type(IsReturnType::No)),
        }

        name_def.push_str("_")
    }

    name_def.push_str("void");
    name_def
}

///
/// In order to figure out what combination of of SignalWrappers we need to generate
/// we go over all the the registered callback function and create a hash for the
/// function arguments. This way we get one unique function wrapper in the cases
/// where several wrapers has the same input
///
fn build_signal_wrappers_info<'a>(api_defs: &'a [ApiDef]) -> HashMap<String, &'a Function> {
    let mut wrapper_info = HashMap::new();
    let mut funcs = Vec::new();

    for api_def in api_defs {
        for t in api_def.get_functions(FunctionType::Signal) {
            funcs.push(t);
        }
    }

    funcs.iter().for_each(|func| {
        let input_args = signal_type_callback(func);
        wrapper_info.entry(input_args).or_insert(*func);
    });

    wrapper_info
}

fn function_name(struct_name: &str, func: &Function) -> String {
    format!("{}_{}", struct_name.to_snake_case(), func.name)
}
///
/// Generate code for returing array
///
///    int count = ret_value.size();
///     RUArray array = { 0 };
///     if (count > 0) {
///         RUListWidgetItem* elements = new RUListWidgetItem[count];
///         for (int i = 0; i < count; ++i) {
///             elements[i].funcs = &s_list_widget_item_funcs;
///             elements[i].priv_data = (struct RUBase*)data.at(i);
///         }
///         array.elements = (void*)elements;
///         array.count = int(count);
///     }
///     return array;

/*
   fn generate_return_array<W: Write>(f: &mut W, ret_val: &Variable) -> io::Result<()> {
// TODO: Use templates

let vtype = &ret_val.type_name;

// Reference in this case means that the data from Qt is by pointer
f.write_all(b"    int count = ret_value.size();\n")?;
f.write_all(b"    RUArray array = { 0 };\n")?;
f.write_all(b"    if (count > 0) {\n")?;
f.write_fmt(format_args!("        RU{}* elements = new RU{}[count];\n", vtype, vtype))?;
f.write_all(b"        for (int i = 0; i < count; ++i) {\n")?;
f.write_fmt(format_args!("            elements[i].funcs = &s_{}_funcs;\n", vtype.to_snake_case()))?;
if ret_val.vtype == VariableType::Reference {
f.write_all(b"            elements[i].priv_data = (struct RUBase*)ret_value.at(i);\n")?;
} else {
// this is hacky as it leaks memory and needs to be fixed
f.write_fmt(format_args!("            Q{}* temp = new Q{}(ret_value.at(i));\n", vtype, vtype))?;
f.write_all(b"            elements[i].priv_data = (struct RUBase*)temp;\n")?;
//f.write_all(b"            elements[i].priv_data = (struct RUBase*)&ret_value.at(i);\n")?;
}

f.write_all(b"       }\n")?;
f.write_all(b"       array.elements = (void*)elements;\n")?;
f.write_all(b"       array.count = int(count);\n")?;
f.write_all(b"   }\n")?;
f.write_all(b"   return array;\n")?;

Ok(())
}
*/

///
/// Get type for Qt Ref
///
fn get_qt_ref_type(var: &Variable) -> Cow<str> {
    let t_name = var.get_untyped_name();

    if var.pointer {
        format!("Q{}*", t_name).into()
    } else {
        format!("*(Q{})*", t_name).into()
    }
}

///
/// Get a Qt parameter for a variable
///
fn get_qt_parameter_type(var: &Variable) -> Cow<str> {
    match var.vtype {
        VariableType::Reference => get_qt_ref_type(var),
        VariableType::Str => "QString".into(),
        _ => var.get_c_type(IsReturnType::No),
    }
}

///
/// Generate a Qt function declaration
///
fn generate_qt_func_def(func: &Function) -> String {
    func.gen_c_def_filter(Some(None), |_index, arg| match arg.vtype {
        VariableType::Reference => Some(get_qt_ref_type(&arg)),
        VariableType::Str => Some("QString&".into()),
        _ => None,
    })
}

///
/// Geerate a declaration with only function types. Used for the SIGNAL/SLOT macros
///
fn generate_func_def_input_parms_only(func: &Function) -> String {
    let mut function_args = String::with_capacity(128);
    let len = func.function_args.len() - 1;

    // write arguments
    for (i, arg) in func.function_args[1..].iter().enumerate() {
        function_args.push_str(&get_qt_parameter_type(&arg));

        if i != len - 1 {
            function_args.push_str(", ");
        }
    }

    function_args
}

///
/// Generate get functions to be used with for static functions
///
fn generate_static_get_functions<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
    // TODO: Template here
    //
    // Generate create functions for all structs that are flagged with create function
    // and doesn't have manual create selected on them
    //
    for sdef in api_def
        .class_structs
        .iter()
        .filter(|s| s.has_static_functions())
    {
        f.write_all(SEPARATOR)?;
        let snake_name = sdef.name.to_snake_case();

        //
        // Get the name if we have remapped the name (for example we use Button while Qt uses
        // AbstractButton)
        //
        f.write_fmt(format_args!(
            "static struct RU{} get_{}(struct RUBase* priv_data) {{\n",
            sdef.name, snake_name
        ))?;
        f.write_all(b"    (void)priv_data;\n")?;
        f.write_fmt(format_args!("    RU{} ctl;\n", sdef.name))?;
        f.write_all(b"    ctl.qt_data = nullptr;\n")?;
        f.write_all(b"    ctl.host_data = nullptr;\n")?;
        f.write_fmt(format_args!(
            "    ctl.all_funcs = &s_{}_all_funcs;\n",
            snake_name
        ))?;
        f.write_all(b"    return ctl;\n}\n\n")?;
    }

    Ok(())
}

///
/// Generates the create functions
///
fn generate_create_functions<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
    //
    // Generate create functions for all structs that are flagged with create function
    // and doesn't have manual create selected on them
    //
    for sdef in api_def
        .class_structs
        .iter()
        .filter(|s| s.should_have_create_func() && !s.has_manual_create())
    {
        let snake_name = sdef.name.to_snake_case();
        let struct_name = sdef.name.as_str();
        let struct_qt_name = &sdef.cpp_name;
        let is_inherits_widget = sdef.inherits_widget();
        f.write_all(SEPARATOR)?;

        //
        // We create wrapper widgets for all Qt widges so we can have our own functionallity there
        // So we prefix them with WR (WRapper) otherwise we assume it's a regular Q* class
        //
        let create_func = if is_inherits_widget {
            "create_widget_func"
        } else if sdef.should_gen_wrap_class() {
            "generic_create_func_with_delete"
        } else {
            "generic_create_func"
        };

        //
        // Get the name if we have remapped the name (for example we use Button while Qt uses
        // AbstractButton)
        //

        if sdef.should_gen_wrap_class() {
            f.write_fmt(format_args!(
                "static struct RU{} create_{}(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{{\n",
                sdef.name, snake_name,
            ))?;

            f.write_fmt(format_args!(
                                "    auto ctl = {}<struct RU{}, {}>(priv_data, delete_callback, private_user_data);\n", create_func,
                                struct_name,
                                struct_qt_name,
                                ))?;
        } else {
            f.write_fmt(format_args!(
                "static struct RU{} create_{}(struct RUBase* priv_data) {{\n",
                sdef.name,
                sdef.name.to_snake_case()
            ))?;

            f.write_fmt(format_args!(
                "    auto ctl = {}<struct RU{}, {}>(priv_data);\n",
                create_func, struct_name, struct_qt_name,
            ))?;
        }

        f.write_fmt(format_args!(
            "    ctl.all_funcs = &s_{}_all_funcs;\n",
            snake_name
        ))?;
        f.write_all(b"    return ctl;\n}\n\n")?;

        f.write_all(SEPARATOR)?;

        //
        // Generate destroy functions as well
        //
        f.write_fmt(format_args!(
            "static void destroy_{}(struct RUBase* priv_data) {{\n",
            sdef.name.to_snake_case()
        ))?;
        f.write_fmt(format_args!(
            "    destroy_generic<{}>(priv_data);\n}}\n\n",
            struct_qt_name
        ))?;
    }

    Ok(())
}

///
/// Generate the function struct defs in the following style
///
/// struct PUMimeDataFuncs s_mime_data_funcs = {
///    mime_data_has_color,
///    mime_data_urls,
///    ...
/// };
///
fn generate_struct_defs<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
    for sdef in &api_def.class_structs {
        f.write_all(SEPARATOR)?;

        let struct_name = &sdef.name;

        f.write_fmt(format_args!(
            "struct RU{}Funcs s_{}_funcs = {{\n",
            struct_name,
            struct_name.to_snake_case()
        ))?;

        if sdef.should_have_create_func() {
            f.write_fmt(format_args!(
                "    destroy_{},\n",
                struct_name.to_snake_case()
            ))?;
        }

        for func in &sdef.functions {
            match func.func_type {
                FunctionType::Regular => {
                    f.write_fmt(format_args!("    {},\n", function_name(struct_name, func)))?;
                }

                FunctionType::Static => {
                    f.write_fmt(format_args!("    {},\n", function_name(struct_name, func)))?;
                }

                FunctionType::Signal => {
                    f.write_fmt(format_args!(
                        "    set_{}_event,\n",
                        function_name(struct_name, func)
                    ))?;
                }

                FunctionType::Event => {
                    f.write_fmt(format_args!(
                        "    set_{},\n",
                        function_name(struct_name, func)
                    ))?;

                    f.write_fmt(format_args!(
                        "    remove_{},\n",
                        function_name(struct_name, func)
                    ))?;
                }
            }
        }

        f.write_all(b"};\n\n")?;

        f.write_all(SEPARATOR)?;
        // generate the alla funcs

        for sdef in &api_def.class_structs {
            f.write_fmt(format_args!(
                "struct RU{}AllFuncs s_{}_all_funcs = {{\n",
                struct_name,
                struct_name.to_snake_case()
            ))?;

            for name in &sdef.full_inherit {
                f.write_fmt(format_args!("    &s_{}_funcs,\n", name.to_snake_case()))?;
            }

            f.write_all(b"};\n\n")?;
        }
    }

    Ok(())
}

///
/// QtGenerator
///
pub struct QtGenerator {
    wrapper_template: Template,
    signal_wrapper_template: Template,
    func_def_template: Template,
    set_signal_template: Template,
    wrap_event_template: Template,
    regular_func_template: Template,
    set_event_template: Template,
    type_handler: TypeHandler,
}

impl QtGenerator {
    pub fn new() -> QtGenerator {
        let parser = ParserBuilder::with_liquid().build();
        let mut type_handler = TypeHandler::new(&parser);

        // Insert handler for strings
        type_handler
            .mapping
            .insert("String", Box::new(StringTypeHandler {}));

        QtGenerator {
            wrapper_template: parser.parse(QT_GEN_WRAPPER_TEMPLATE).unwrap(),
            signal_wrapper_template: parser.parse(SIGNAL_WRAPPER_TEMPLATE).unwrap(),
            func_def_template: parser.parse(QT_FUNC_DEF_TEMPLATE).unwrap(),
            set_signal_template: parser.parse(SET_SIGNAL_TEMPLATE).unwrap(),
            wrap_event_template: parser.parse(WRAP_EVENT_TEMPLATE).unwrap(),
            regular_func_template: parser.parse(QT_REGULAR_FUNC_DEF_TEMPLATE).unwrap(),
            set_event_template: parser.parse(QT_SET_EVENT_TEMPLATE).unwrap(),
            type_handler: type_handler,
        }
    }

    ///
    /// Generate the set/remove functions for events
    ///
    fn generate_event_remove_set<W: Write>(
        &self,
        dest: &mut W,
        sdef: &Struct,
        func: &Function,
    ) -> io::Result<()> {
        let mut object = Object::new();

        object.insert("type_name".into(), Value::scalar(sdef.name.to_snake_case()));
        object.insert("event_type_snake".into(), Value::scalar(&func.name));
        object.insert("event_type".into(), Value::scalar(&sdef.name));
        object.insert(
            "event_name".into(),
            Value::scalar(func.name.to_snake_case()),
        );
        object.insert(
            "event_args".into(),
            Value::scalar(&func.gen_c_def_filter(Some(Some("void*, void*".into())), |_, _| None)),
        );

        let res = self.set_event_template.render(&object).unwrap();
        dest.write_all(res.as_bytes())
    }

    ///
    /// Generation function def for callbacks/slots
    ///
    fn generate_func_callback<W: Write>(
        &self,
        dest: &mut W,
        struct_name: &str,
        func: &Function,
    ) -> io::Result<()> {
        let signal_type_name = signal_type_callback(&func);
        let func_name = function_name(struct_name, func);

        let mut callback_def = String::with_capacity(128);
        CapiHeaderGen::callback_fun_def_name(&mut callback_def, false, &func_name, "_event", func);

        let func_def = generate_func_def_input_parms_only(func);
        let mut object = Object::new();

        object.insert("event_def".into(), Value::scalar(&callback_def));
        object.insert("signal_type_name".into(), Value::scalar(&signal_type_name));
        object.insert(
            "qt_signal_name".into(),
            Value::scalar(func.name.to_mixed_case()),
        );
        object.insert("func_def".into(), Value::scalar(&func_def));

        let res = self.set_signal_template.render(&object).unwrap();
        dest.write_all(res.as_bytes())
    }

    ///
    /// Shared handling of return value
    ///
    fn generate_func_ret_value(object: &mut Object, func: &Function) {
        // Setup return value
        let ret_value = func
            .return_val
            .as_ref()
            .map_or("void".into(), |v| v.get_c_type(IsReturnType::Yes));

		object.insert("c_primitive_type".into(), Value::scalar(""));

        if let Some(ref ret_val) = func.return_val {
            object.insert("array_return".into(), Value::scalar(ret_val.array));
            object.insert("qt_ret_value".into(), Value::scalar("ret_value"));
            object.insert("funcs_name".into(), Value::scalar(""));
            object.insert("enum_type_name".into(), Value::scalar(""));
            object.insert(
                "qt_return_type".into(),
                Value::scalar(&ret_val.qt_type_name),
            );

            match ret_val.vtype {
                VariableType::Primitive => {
                    object.insert("return_type".into(), Value::scalar("primitive"));
					object.insert("c_primitive_type".into(), Value::scalar(
						ret_val.get_c_primitive_type().into_owned()))
                }
                VariableType::Str => object.insert("return_type".into(), Value::scalar("string")),
                VariableType::Regular => {
                    object.insert(
                        "funcs_name".into(),
                        Value::scalar(ret_val.type_name.to_snake_case()),
                    );
                    object.insert("return_type".into(), Value::scalar("regular"))
                }
                VariableType::Reference => {
                    object.insert(
                        "funcs_name".into(),
                        Value::scalar(ret_val.type_name.to_snake_case()),
                    );
                    object.insert("return_type".into(), Value::scalar("reference"))
                }
                VariableType::Enum => {
                    object.insert("return_type".into(), Value::scalar("enum_type"));
                    object.insert(
                        "enum_type_name".into(),
                        Value::scalar(ret_val.enum_sub_type.to_snake_case()),
                    )
                }
                _ => object.insert("return_type".into(), Value::scalar("<illegal>")),
            };
        } else {
            object.insert("array_return".into(), Value::scalar(false));
            object.insert("return_type".into(), Value::scalar(""));
            object.insert("qt_ret_value".into(), Value::scalar(""));
        }

		object.insert(
            "c_return_type".into(),
            Value::scalar(ret_value.into_owned()),
        );
    }

    ///
    /// Generate function def for a wrapping call to Qt. Some examples:
    ///
    ///  resize(width, height);
    ///
    ///  auto ret = getData();
    ///  return ret;
    ///
    fn generate_func_def(
        &self,
        object: &mut Object,
        sdef: &Struct,
        func: &Function,
        instance_call: &'static str,
        extra_args: &str,
    ) -> String {
        object.insert("qt_instance_call".into(), Value::scalar(instance_call));
        object.insert("extra_args".into(), Value::scalar(extra_args.to_owned()));

        // Build function args
        let func_def = func.gen_c_invoke_filter(FirstArgName::Remove, |_index, arg| {
            match self.type_handler.get(&arg) {
                Some(handler) => Some(handler.replace_arg(&arg, IsReturnArg::No).into()),
                _ => None,
            }
        });

        object.insert(
            "func_name".into(),
            Value::scalar(function_name(&sdef.name, func)),
        );
        object.insert(
            "func_def".into(),
            Value::scalar(func.generate_c_function_def(FirstArgType::Keep)),
        );
        object.insert(
            "qt_func_name".into(),
            Value::scalar(func.cpp_name.to_mixed_case()),
        );
        object.insert("cpp_type_name".into(), Value::scalar(&sdef.cpp_name));
        object.insert("qt_func_args".into(), Value::scalar(func_def));
        object.insert(
            "type_snake_name".into(),
            Value::scalar(func.name.to_snake_case()),
        );

        Self::generate_func_ret_value(object, func);

        // Render the output data
        self.func_def_template.render(object).unwrap()
    }

    ///
    /// Generate function def for a wrapping call to Qt. Some examples:
    ///
    ///  resize(width, height);
    ///
    ///  auto ret = getData();
    ///  return ret;
    ///
    fn generate_event_func_def(
        &self,
        object: &mut Object,
        sdef: &Struct,
        func: &Function,
    ) -> String {
        let func_snake_name = func.name.to_snake_case();

        // Setup event name. Notice these are not part of any template in order to keep func def
        // separate and not put inside two places
        let event_name = format!("m_{}", func_snake_name);
        let user_data = format!("m_{}_user_data", func_snake_name);
        let wrapped_func = format!("m_{}_wrapped_func", func_snake_name);
        let extra_args = format!("{}, {}", user_data, wrapped_func);

        object.insert("qt_instance_call".into(), Value::scalar("        "));
        object.insert("extra_args".into(), Value::scalar(extra_args));

        let mut body_init = String::with_capacity(4096);
        let mut func_args = String::with_capacity(128);
        let mut function_args = Vec::with_capacity(func.function_args.len());

        for (i, arg) in func.function_args.iter().enumerate() {
            match self.type_handler.get(&arg) {
                Some(handler) => {
                    let (gen_arg, body) = handler.gen_body(&arg, i);
                    function_args.push((true, gen_arg));
                    body_init += &body;
                }
                None => {
                    function_args.push((false, arg.name.clone().into()));
                }
            }
        }

        // Generate the function call
        for (index, arg) in func.function_args.iter().enumerate() {
            match arg.vtype {
                VariableType::SelfType => (),
                _ => {
                    func_args.push_str(", ");
                    func_args.push_str(&function_args[index].1);
                }
            }
        }

        object.insert("body_init".into(), Value::scalar(body_init));
        object.insert("qt_func_args".into(), Value::scalar(func_args));

        object.insert(
            "func_name".into(),
            Value::scalar(function_name(&sdef.name, func)),
        );
        object.insert(
            "func_def".into(),
            Value::scalar(func.generate_c_function_def(FirstArgType::Keep)),
        );
        object.insert("qt_func_name".into(), Value::scalar(event_name));
        object.insert("cpp_type_name".into(), Value::scalar(&sdef.cpp_name));
        object.insert("type_snake_name".into(), Value::scalar(func_snake_name));

        Self::generate_func_ret_value(object, func);

        // Render the output data
        self.func_def_template.render(object).unwrap()
    }

    ///
    /// Generate function wrappers such as:
    ///
    /// static void widget_resize(struct PUBase* self_c, int width, int height) {
    ///     WRWidget* qt_data = (WRWidget*)self_c;
    ///     qt_data->resize(width, height);
    /// }
    ///
    fn generate_function_wrappers<W: Write>(
        &self,
        dest: &mut W,
        api_def: &ApiDef,
    ) -> io::Result<()> {
        for sdef in &api_def.class_structs {
            for func in sdef.functions.iter().filter(|func| !func.is_manual) {
                dest.write_all(SEPARATOR)?;

                match func.func_type {
                    FunctionType::Static => {
                        let mut object = Object::new();
                        let func_output =
                            self.generate_func_def(&mut object, &sdef, func, "qt_value->", "");
                        object.insert("body_setup".into(), Value::scalar(func_output));
                        let output = self.regular_func_template.render(&object).unwrap();
                        dest.write_all(output.as_bytes())?;
                    }
                    FunctionType::Regular => {
                        let mut object = Object::new();
                        let func_output =
                            self.generate_func_def(&mut object, &sdef, func, "qt_value->", "");
                        object.insert("body_setup".into(), Value::scalar(func_output));
                        let output = self.regular_func_template.render(&object).unwrap();
                        dest.write_all(output.as_bytes())?;
                    }
                    FunctionType::Signal => self.generate_func_callback(dest, &sdef.name, func)?,
                    FunctionType::Event => self.generate_event_remove_set(dest, &sdef, func)?,
                }
            }
        }

        Ok(())
    }

    /// Generate a signal wrapper that is in the style of this:
    ///
    /// typedef void (*Signal_self_int_void)(void* self_c, void* wrapped_func, int row);
    ///
    /// class QSlotWrapperSignal_self_int_void : public QObject {
    ///     Q_OBJECT
    /// public:
    ///     QSlotWrapperSignal_self_int_void(void* data, Signal_self_int_void trampoline_func, void* wrapped_func) {
    ///         m_tampoline_func = trampoline_func;
    ///         m_data = data;
    ///         m_wrapped_func = wrapped_func;
    ///     }
    ///
    ///     Q_SLOT void method(int row) {
    ///         m_tampoline_func(m_data, m_wrapped_func, row);
    ///     }
    /// private:
    ///     Signal_self_int_void m_tampoline_func;
    ///     void* m_data;
    ///     void* m_wrapped_func;
    /// };
    ///
    pub fn generate_signal_wrappers<W: Write>(
        &self,
        f: &mut W,
        api_defs: &[ApiDef],
    ) -> io::Result<()> {
        // Sort the signals by their names to have stable generation
        let temp = build_signal_wrappers_info(api_defs);
        let ordered: BTreeMap<_, _> = temp.iter().collect();

        for (signal_type_name, func) in ordered {
            let mut template_data = Object::new();

            let c_args = generate_c_function_args_signal_wrapper(EventType::Callback, func);
            let c_call_args = func.generate_invoke(FirstArgName::Remove);

            template_data.insert(
                "signal_func_name".into(),
                Value::scalar(signal_type_name.clone()),
            );
            template_data.insert("c_args".into(), Value::scalar(c_args));

            // we need to add, at the front of the call args as this is being used inside a
            // function call argument already if we have some args
            if c_call_args != "" {
                template_data.insert(
                    "c_call_args".into(),
                    Value::scalar(format!(", {}", c_call_args)),
                );
            } else {
                template_data.insert("c_call_args".into(), Value::scalar(""));
            }

            let res = self.signal_wrapper_template.render(&template_data).unwrap();
            f.write_all(res.as_bytes())?;
        }

        Ok(())
    }

    ///
    /// Genarate event setup code. It will look something like this
    ///
    ///  void paintEvent(QPaintEvent* event) {
    ///        if (m_paint_event_trampoline) {
    ///            RUPainteEvent e;
    ///            e.qt_data = (struct RUBase*)event;
    ///            e.host_data = nullptr;
    ///            e.all_funcs = &s_paint_event_funcs;
    ///            m_paint_event_trampoline(m_paint_user_data,
    ///                                     m_paint_event_wrapped_func, (RUPainteEvent*)&e);
    ///        } else {
    ///            QWidget::paintEvent(event);
    ///        }
    ///    }
    ///
    ///    RUPaintEventFunc m_paint_event_trampoline = nullptr;
    ///    void* m_paint_user_data = nullptr;
    ///    void* m_paint_event_wrapped_func = nullptr;
    ///
    fn generate_event_setup_def(&self, sdef: &Struct, func: &Function) -> String {
        let mut object = Object::new();

        let body_setup = self.generate_event_func_def(&mut object, sdef, func);

        object.insert(
            "qt_event_name".into(),
            Value::scalar(func.cpp_name.to_mixed_case()),
        );
        object.insert(
            "qt_event_args".into(),
            Value::scalar(generate_qt_func_def(func)),
        );
        object.insert("qt_class_name".into(), Value::scalar(&sdef.name));
        object.insert("class_name".into(), Value::scalar(&sdef.name));
        object.insert("qt_class_name".into(), Value::scalar(&sdef.qt_name));
        object.insert(
            "event_args".into(),
            Value::scalar(func.generate_invoke(FirstArgName::Remove)),
        );
        object.insert("body_setup".into(), Value::scalar(body_setup));
        object.insert(
            "event_type".into(),
            Value::scalar(&func.name.to_camel_case()),
        );
        object.insert(
            "event_type_snake".into(),
            Value::scalar(&func.name.to_snake_case()),
        );
        object.insert(
            "c_event_args".into(),
            Value::scalar(&func.gen_c_def_filter(Some(Some("void*, void*".into())), |_, _| None)),
        );

        self.wrap_event_template.render(&object).unwrap()
    }

    ///
    /// Generate wrapper classes for all the Widges. This allows us to override
    /// virtual functions from the outside (in C and other langs)
    /// defs and add custom data that we need. This also supports a callback that is called
    /// upon destruction of the object from the C++ side. This allows the callback to tag
    /// the object (on the Rust side) as invalid. This allows for proper error handling if user
    /// tries to access a "deleted" object
    ///
    fn generate_wrapper_classes_defs<W: Write>(
        &self,
        f: &mut W,
        api_def: &ApiDef,
    ) -> io::Result<()> {
        for sdef in api_def
            .class_structs
            .iter()
            .filter(|s| s.should_gen_wrap_class())
        {
            let mut template_data = Object::new();
            let inherits_widget = sdef.inherits_widget();

            template_data.insert("struct_name".into(), Value::scalar(&sdef.name));
            template_data.insert("qt_name".into(), Value::scalar(&sdef.qt_name));
            template_data.insert("widget".into(), Value::scalar(inherits_widget));

            let mut events = String::with_capacity(64 * 1024);

            sdef.functions
                .iter()
                .filter(|func| func.func_type == FunctionType::Event)
                .for_each(|f| {
                    let setup = self.generate_event_setup_def(&sdef, &f);
                    events.push_str(&setup);
                });

            template_data.insert("events".into(), Value::scalar(events));
            template_data.insert(
                "supports_clone".into(),
                Value::scalar(sdef.supports_cpp_clone()),
            );

            let res = self.wrapper_template.render(&template_data).unwrap();
            f.write_all(res.as_bytes())?;
        }

        Ok(())
    }

    pub fn generate(&self, target_name: &str, api_def: &ApiDef) -> io::Result<()> {
        let header_path = format!("{}.h", target_name);
        let cpp_path = format!("{}.cpp", target_name);

        // Set up all the type handlers that are being used to do special
        // generation when needed
        //let type_handlers: Vec<Box<TypeHandler>> = Vec::new();

        // Create the header and cpp out
        let mut h_out = BufWriter::new(File::create(header_path)?);
        let mut cpp_out = BufWriter::new(File::create(cpp_path)?);

        // Write the header to cpp generated code
        h_out.write_all(b"#pragma once\n")?;
        h_out.write_all(HEADER)?;
        cpp_out.write_all(HEADER)?;

        // Generate includes for all non-POD structs(
        //generate_includes(&mut h_out, &api_def)?;
        generate_includes(&mut cpp_out, &api_def)?;

        // Generate the wrapping classes declartion is used as for Qt.
        //self.generate_wrapper_classes_defs(&mut h_out, api_def)?;

        // Generate wrapper functions for are regular defined functions
        self.generate_function_wrappers(&mut cpp_out, api_def)?;

        // generate the create functions for all widgets
        generate_create_functions(&mut cpp_out, api_def)?;

        // generate the static get functions that returns a struct to be used with static functions
        generate_static_get_functions(&mut cpp_out, api_def)?;

        // generate the structs that holds the wrapper functions
        generate_struct_defs(&mut cpp_out, api_def)?;

        Ok(())
    }

    ///
    /// Generate all the signal wrappers
    ///
    pub fn generate_all_signal_wrappers(
        &self,
        target_name: &str,
        api_defs: &[ApiDef],
    ) -> io::Result<()> {
        let mut dest = BufWriter::with_capacity(512 * 1024, File::create(target_name)?);

        // Write header first

        dest.write_all(b"#pragma once\n")?;
        dest.write_all(HEADER)?;

        for api_def in api_defs {
            generate_includes(&mut dest, &api_def)?;
        }

        for api_def in api_defs {
            self.generate_wrapper_classes_defs(&mut dest, api_def)?;
        }

        self.generate_signal_wrappers(&mut dest, api_defs)
    }

    ///
    /// Generate enum remappings from rute enums to Qt
    ///
    /*
    pub fn generate_enum_mappings(&self, target_name: &str, api_defs: &[ApiDef]) -> io::Result<()> {
        let mut dest = BufWriter::with_capacity(512 * 1024, File::create(target_name)?);
        let mut enums = BTreeMap::new();

        for api_def in api_defs {
            for enum_def in &api_def.enums {
                enums.insert(enum_def.name.clone(), enum_def);
            }
        }

        let mut enum_org_names = BTreeMap::new();

        for (_, enum_def) in &enums {
            enum_org_names.insert(&enum_def.original_class_name, ());
        }

        for (name, _) in enum_org_names {
            writeln!(dest, "#include <{}>", name)?;
        }

        dest.write_all(b"#include <map>\n\n")?;
        dest.write_all(b"struct KeyVal { unsigned int val, key; };\n\n")?;

        for (name, _) in &enums {
            dest.write_fmt(format_args!(
                "std::map<int, int> s_{}_lookup;\n",
                name.to_snake_case()
            ))?;
        }

        dest.write_all(b"\n")?;
        dest.write_all(SEPARATOR)?;
        dest.write_all(b"extern void create_enum_mappings() {\n")?;

        for (_, enum_def) in enums {
            let enum_name = enum_def.name.to_snake_case();
            let mut template_data = Object::new();

            template_data.insert("enum_name".into(), Value::scalar(enum_name));
            template_data.insert(
                "qt_class".into(),
                Value::scalar(&enum_def.original_class_name),
            );

            let mut values = Vec::with_capacity(enum_def.entries.len());

            for e in &enum_def.entries {
                let mut enum_data = Object::new();
                enum_data.insert("name".into(), Value::scalar(e.name.to_owned()));
                enum_data.insert("id".into(), Value::scalar(e.value.to_string()));
                values.push(Value::Object(enum_data));
            }

            template_data.insert("enums".into(), Value::Array(values));

            let res = self.enum_mapping_template.render(&template_data).unwrap();
            dest.write_all(res.as_bytes())?;
        }

        dest.write_all(b"}\n")
    }

    ///
    /// Generate enum remappings from rute enums to Qt
    ///
    pub fn generate_enum_mappings_header(
        &self,
        target_name: &str,
        api_defs: &[ApiDef],
    ) -> io::Result<()> {
        let mut dest = BufWriter::with_capacity(4 * 1024, File::create(target_name)?);
        let mut enums = BTreeMap::new();

        for api_def in api_defs {
            for enum_def in &api_def.enums {
                enums.insert(
                    (enum_def.name.clone(), enum_def.flags_name.clone()),
                    enum_def,
                );
            }
        }

        dest.write_all(b"#pragma once\n")?;
        dest.write_all(AUTO_GEN_HEADER)?;
        dest.write_all(b"#include <map>\n\n")?;

        for ((name, _), _) in &enums {
            dest.write_fmt(format_args!(
                "extern std::map<int, int> s_{}_lookup;\n",
                name.to_snake_case()
            ))?;
        }

        writeln!(dest, "")?;

        // forwarding define because sometimes Qt uses a macro for this
        for ((name, flags_name), _) in &enums {
            if !flags_name.is_empty() {
                dest.write_fmt(format_args!(
                    "#define s_{}_lookup s_{}_lookup\n",
                    flags_name.to_snake_case(),
                    name.to_snake_case()
                ))?;
            }
        }

        Ok(())
    }
    */

    ///
    /// Generate the function struct defs in the following style
    ///
    /// struct Rute s_mime_data_funcs = {
    ///    mime_data_has_color,
    ///    mime_data_urls,
    ///    ...
    /// };
    ///
    pub fn generate_rute_struct(filename: &str, api_defs: &[ApiDef]) -> io::Result<()> {
        let mut dest = BufWriter::new(File::create(filename)?);

        writeln!(dest, "{}", QT_MAIN_HEADER)?;

        for sdef in api_defs
            .iter()
            .flat_map(|d| d.class_structs.iter())
        {
            if sdef.should_have_create_func() {
                writeln!(dest, "    create_{},", sdef.name.to_snake_case())?;
            }

            if sdef.has_static_functions() {
                writeln!(dest, "    get_{},", sdef.name.to_snake_case())?;
            }
        }

        writeln!(dest, "{}", QT_MAIN_FOOTER)
    }

    ///
    /// Generate a big bulk file for cpp code for now to allow better code opt and easier to build
    /// We may need to change this if we run into memory issues on machines trying to compile it
    ///
    pub fn generate_bulk_cpp(filename: &str, api_defs: &[ApiDef]) -> io::Result<()> {
        let mut dest = BufWriter::new(File::create(filename).unwrap());

        writeln!(
            &mut dest,
            "// This file is auto-generated by rute_gen. DO NOT EDIT!\n"
        )?;

        let mut files = Vec::new();

        for sdef in api_defs {
            let base_filename = Path::new(&sdef.filename)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();
            let base_filename = &base_filename[..base_filename.len() - 4];
            // TODO: Fixme
            if base_filename != "qnamespace" {
                files.push(base_filename.to_owned());
            }
        }

        files.sort();

        for f in &files {
            writeln!(dest, "#include \"{}_ffi.h\"", f)?;
            writeln!(dest, "#include \"{}.h\"", f)?;
        }

        writeln!(dest, "#include \"rute_signal_wrappers.h\"")?;
        writeln!(dest, "#include \"qt_enum_mapping.h\"")?;
        writeln!(dest, "")?;

        for f in &files {
            writeln!(dest, "#include \"{}.cpp\"", f)?;
        }

        // include main file as last file
        writeln!(dest, "#include \"qt_enum_mapping.cpp\"")?;
        writeln!(dest, "#include \"qt_rute.cpp\"")
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    fn init_wrapper_default_sdef() -> (Struct, QtGenerator) {
        let generator = QtGenerator::new();
        let mut sdef = Struct::default();
        sdef.name = "Foo".to_owned();
        sdef.qt_name = "QFoo".to_owned();
        sdef.cpp_name = "WRFoo".to_owned();
        (sdef, generator)
    }

    fn init_default_func() -> Function {
        Function {
            name: "test".to_owned(),
            function_args: vec![Variable {
                name: "self_c".to_owned(),
                vtype: VariableType::SelfType,
                type_name: "struct RUBase".to_owned(),
                ..Variable::default()
            }],
            return_val: None,
            func_type: FunctionType::Regular,
            is_manual: false,
            cpp_name: "test".to_owned(),
        }
    }

    //
    // This will test that the signal_type_callback generates the correct output
    //
    #[test]
    fn test_signal_type_callback_0() {
        let func = Function {
            name: "test".to_owned(),
            function_args: vec![Variable {
                name: "var0".to_owned(),
                vtype: VariableType::Primitive,
                type_name: "i32".to_owned(),
                ..Variable::default()
            }],
            return_val: None,
            func_type: FunctionType::Regular,
            is_manual: false,
            cpp_name: "test".to_owned(),
        };

        let signal_gen = signal_type_callback(&func);
        assert_eq!(signal_gen, "int_void");
    }

    //
    // This will test that the signal_type_callback generates the correct output
    //
    #[test]
    fn test_signal_type_callback_1() {
        let func = Function {
            name: "test".to_owned(),
            function_args: vec![Variable {
                name: "var0".to_owned(),
                vtype: VariableType::Reference,
                type_name: "DropEvent".to_owned(),
                ..Variable::default()
            }],
            return_val: None,
            func_type: FunctionType::Regular,
            is_manual: false,
            cpp_name: "test".to_owned(),
        };

        let signal_gen = signal_type_callback(&func);
        assert_eq!(signal_gen, "DropEvent_void");
    }

    //
    // test and validate function generation
    //
    #[test]
    fn test_qt_wrap_func_gen_no_return() {
        let (sdef, gen) = init_wrapper_default_sdef();
        let func = init_default_func();
        let dest = gen.generate_func_def(&mut Object::new(), &sdef, &func, "qt_value->", "");
        assert_eq!(
            dest,
            "
    qt_value->test();"
        );
    }

    //
    // Test returing a 32-bit value
    //
    #[test]
    fn test_qt_wrap_func_i32_return() {
        let (sdef, gen) = init_wrapper_default_sdef();
        let mut func = init_default_func();

        func.return_val = Some(Variable {
            name: "ret".to_owned(),
            vtype: VariableType::Primitive,
            type_name: "i32".to_owned(),
            ..Variable::default()
        });

        let dest = gen.generate_func_def(&mut Object::new(), &sdef, &func, "qt_value->", "");
        assert_eq!(
            dest,
            "
    auto ret_value = qt_value->test();
    return ret_value;"
        );
    }

    //
    // Test returing a string value
    //
    #[test]
    fn test_qt_wrap_func_string_return() {
        let (sdef, gen) = init_wrapper_default_sdef();
        let mut func = init_default_func();

        func.return_val = Some(Variable {
            name: String::new(),
            vtype: VariableType::Str,
            type_name: String::new(),
            ..Variable::default()
        });

        let dest = gen.generate_func_def(&mut Object::new(), &sdef, &func, "qt_value->", "");
        assert_eq!(
            dest,
            "
    auto ret_value = qt_value->test();
    return q_string_to_const_char(ret_value);"
        );
    }

    #[test]
    fn test_generate_qt_func_def() {}
}
*/
