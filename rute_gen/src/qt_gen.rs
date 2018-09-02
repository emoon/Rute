use c_gen::*;
use heck::MixedCase;
use heck::SnakeCase;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

use api_parser::*;
use qt_gen_templates::*;

use liquid::{Template, ParserBuilder, Object, Value};

#[derive(PartialEq)]
pub enum EventType {
    Callback,
    Event,
}

///
/// Type handlers allows us to override certain types that gets passed into functions.
/// Examples are structs such as Rect, Color and Strings
///
trait TypeHandler {
    fn match_type(&self) -> &str;

    fn replace_arg(&self, arg: &Variable) -> String {
        arg.name.clone()
    }

    fn gen_body_return(&self, function_name: &str) -> String {
        format!(
            "    return qt_data->{}()",
            function_name.to_mixed_case())
    }
}


///
/// Adds some extra functionallity to Struct to make some checks easier
///
impl Struct {
    fn inherits_widget(&self, api_def: &ApiDef) -> bool {
        if self.name == "Widget" {
            return true;
        }

        if let Some(ref inherit_name) = self.inherit {
            if inherit_name == "Widget" {
                return true;
            }

            for sdef in &api_def.class_structs {
                if sdef.inherits_widget(api_def) == true {
                    return true;
                }
            }
        }

        false
    }
}

///
/// Generate enum remappings from rute enums to Qt
///
fn generate_enum_mappings<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
    // write enum mapping defs

    f.write_all(SEPARATOR)?;
    f.write_all(b"\n")?;
    f.write_all(b"struct KeyVal { int val, key; };\n")?;

    for enum_def in &api_def.enums {
        f.write_fmt(format_args!(
            "static std::map<int, int> s_{}_lookup;\n",
            enum_def.name.to_snake_case()
        ))?;
    }

    f.write_all(b"\n")?;
    f.write_all(SEPARATOR)?;
    f.write_all(b"void create_enum_mappings() {\n")?;

    for enum_def in &api_def.enums {
        let enum_name = enum_def.name.to_snake_case();

        f.write_fmt(format_args!(
            "    static KeyVal {}_vals[] = {{\n",
            enum_name
        ))?;

        for (i, entry) in enum_def.entries.iter().enumerate() {
            match *entry {
                EnumEntry::Enum(ref value) => {
                    let name = &value;
                    f.write_fmt(format_args!("        {{ (int)Qt::{}, {} }},\n", name, i))?;
                }
                _ => (),
            }
        }

        f.write_all(b"    };\n\n")?;

        f.write_fmt(format_args!(
            "    for (int i = 0; i < {}; ++i) {{\n",
            enum_def.entries.len()
        ))?;
        f.write_fmt(format_args!(
            "        s_{}_lookup[{}_vals[i].key] = {}_vals[i].val;\n",
            enum_name, enum_name, enum_name
        ))?;
        f.write_all(b"    };\n")?;
    }

    f.write_all(b"}\n\n")
}
///
/// Genarate event setup code. It will look something like this
///
///  void Class::paintEvent(QPaintEvent* event) {
///        if (m_paint_event) {
///            RUPainteEvent e;
///            memcpy(&e, s_paint_event, sizeof(e));
///            e.priv_data = event;
///            m_paint_event((RUPainteEvent*)&e, m_paint_user_data);
///        } else {
///            QWidget::paintEvent(event);
///        }
///    }
///
///    RUPaintEventFunc m_paint_event = nullptr;
///    void* m_paint_user_data = nullptr;

/*
fn generate_event_setup_def<W: Write>(f: &mut W, func: &Function) -> io::Result<()> {
    let event_type = &func.function_args[1];

    // Write virtual function def
    f.write_fmt(format_args!(
        "\n    virtual void {}Event(Q{}* event);\n",
        func.name.to_mixed_case(),
        event_type.type_name
    ))?;

    f.write_fmt(format_args!(
        "    void (*m_{})({})",
        func.name,
        generate_c_function_args_signal_wrapper(EventType::Event, func)
    ))?;

    //func.write_c_func_def(f, |_, arg| (arg.get_c_type(), arg.name.to_owned()))?;

    f.write_fmt(format_args!(" = nullptr;\n"))?;
    f.write_fmt(format_args!(
        "    void* m_{}_user_data = nullptr;\n",
        func.name
    ))?;

    Ok(())
}
*/

//
// Generates wrapping code fore "Events" (i.e virtual overrides on Qt objects)
//
/*
fn generate_event_setup_impl<W: Write>(
    f: &mut W,
    sdef: &Struct,
    class_name: &str,
    func: &Function,
    api_def: &ApiDef,
) -> io::Result<()> {
    let event_type = &func.function_args[1];

    f.write_all(SEPARATOR)?;

    f.write_fmt(format_args!(
        "void WR{}::{}Event(Q{}* event) {{\n",
        sdef.name,
        func.name.to_mixed_case(),
        event_type.type_name
    ))?;
    f.write_fmt(format_args!("    if (m_{}) {{\n", func.name))?;
    f.write_fmt(format_args!("        RU{} e;\n", event_type.type_name,))?;
    f.write_fmt(format_args!(
        "        e.{}_event_funcs = &s_{}_event_funcs;\n",
        func.name,
        func.name
    ))?;
    f.write_fmt(format_args!(
        "        e.priv_data = (struct RUBase*)event;\n"
    ))?;

    f.write_fmt(format_args!("        RU{} w;\n", sdef.name))?;

    // TODO: We should do a use a lookup here to find the object for this instead. The reason is
    // that we are tracking the destruction of this object so we can't access bad data on the Rust
    // side. The way we use this here that would still be possible to fail

    for s in api_def.get_inherit_structs(sdef, RecurseIncludeSelf::Yes) {
        let snake_name = s.name.to_snake_case();

        f.write_fmt(format_args!(
            "        w.{}_funcs = &s_{}_funcs;\n",
            snake_name,
            snake_name
        ))?;
    }

    f.write_fmt(format_args!(
        "        w.priv_data = (struct RUBase*)this;\n"
    ))?;

    f.write_fmt(format_args!(
        "        m_{}((struct RUBase*)&w, m_{}_user_data, (struct RUBase*)&e);\n",
        func.name, func.name,
    ))?;
    f.write_fmt(format_args!("    }} else {{\n"))?;
    f.write_fmt(format_args!(
        "        Q{}::{}Event(event);\n",
        class_name,
        func.name.to_mixed_case()
    ))?;
    f.write_fmt(format_args!("    }}\n"))?;
    f.write_fmt(format_args!("}}\n\n"))?;

    Ok(())
}
*/
///
/// Generate something like this
///
/// static void set_paint_event(void* object, void* user_data, void (*event)(void* self_c, struct RUBase* event)) {
///     WRWidget* qt_object = (WRWidget*)object;
///     qt_object->m_paint_user_data = user_data;
///     qt_object->m_paint = event;
/// }
///
/*
fn generate_event_setup_funcs<W: Write>(
    f: &mut W,
    struct_qt_name: &str,
    func: &Function,
) -> io::Result<()> {
    let func_name = format!("{}_{}", struct_qt_name.to_snake_case(), func.name);

    let mut func_def = String::with_capacity(128);

    CapiHeaderGen::callback_fun_def_name(&mut func_def, false, &func_name, func);

    f.write_fmt(format_args!( "{} {{\n", func_def))?;

    f.write_fmt(format_args!(
        "    WR{}* qt_object = (WR{}*)object;\n",
        struct_qt_name, struct_qt_name
    ))?;
    f.write_fmt(format_args!(
        "    qt_object->m_{}_user_data = user_data;\n",
        func.name
    ))?;
    f.write_fmt(format_args!("    qt_object->m_{} = event;\n", func.name))?;
    f.write_all(b"};\n\n")
}
*/

///
/// Generate wrapper classes for all the Widges. This allows us to override
/// virtual functions from the outside (in C and other langs)
/// defs
///
fn generate_wrapper_classes_impl<W: Write>(
    _f: &mut W,
    _struct_name_map: &HashMap<&str, &str>,
    api_def: &ApiDef,
) -> io::Result<()> {
    for _sdef in api_def
        .class_structs
        .iter()
        .filter(|v| v.inherits_widget(api_def))
    {
        /*
        let struct_name = sdef.name.as_str();
        let struct_qt_name = struct_name_map
            .get(struct_name)
            .unwrap_or_else(|| &struct_name);
        */

        /*
        let iterator = sdef
            .functions
            .iter()
            .filter(|func| func.func_type == FunctionType::Event);
        */

        // Get all the structs that thu current struct inherit from

        /*
        for func in iterator.clone() {
            generate_event_setup_impl(f, &sdef, &struct_qt_name, &func, api_def)?;
        }

        // Generate the setup functions for events
        for func in iterator {
            f.write_all(SEPARATOR)?;
            generate_event_setup_funcs(f, &struct_qt_name, &func)?;
        }
        */
    }

    Ok(())
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
                _ => function_args.push_str(&arg.get_c_type()),
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
fn generate_includes<W: Write>(
    f: &mut W,
    struct_name_map: &HashMap<&str, &str>,
    api_def: &ApiDef,
) -> io::Result<()> {
    // There is a small hack here where we include generating includes for the static
    // funcs structs as it doesn't map to a qt class
    for sdef in api_def
        .class_structs
        .iter()
        .filter(|s| s.name != "StaticFuncs")
    {
        let struct_name = sdef.name.as_str();
        let struct_qt_name = struct_name_map
            .get(struct_name)
            .unwrap_or_else(|| &struct_name);
        f.write_fmt(format_args!("#include <Q{}>\n", struct_qt_name))?;
    }

    Ok(())
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
            _ => name_def.push_str(&arg.get_c_type()),
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
fn build_signal_wrappers_info<'a>(api_def: &'a ApiDef) -> HashMap<String, &'a Function> {
    let mut wrapper_info = HashMap::new();
    let funcs = api_def.get_functions(FunctionType::Callback);

    funcs.iter().for_each(|func| {
        let input_args = signal_type_callback(func);
        wrapper_info.entry(input_args).or_insert(*func);
    });

    wrapper_info
}


///
/// Generate all forward declarations function pointer structs
///
/// Example output:
///
/// extern struct RUWidgetFuncts s_widget_funcs;
/// extern struct RUListFuncts s_list_funcs;
///
fn generate_forward_declare_struct_defs<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
    for s in &api_def.class_structs {
        f.write_fmt(format_args!(
            "extern struct RU{}Funcs s_{}_funcs;\n",
            s.name,
            s.name.to_snake_case()
        ))?;
    }

    f.write_all(b"\n")
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
/// Generate code for returing string
/// We currently use a static buffer here to reduce mallocs. This isn't thread safe but it's
/// only allowed to use QtData from one thread anyway.
///
///   QByteArray ba = ret_value.toUtf8();
///   const char* c_str = ba.data();
///   assert(ba.size() < sizeof(s_temp_string_buffer));
///   memcpy(s_temp_string_buffer, c_str, ba.size());
///   return s_temp_string_buffer;
///

fn generate_return_string<W: Write>(f: &mut W) -> io::Result<()> {
    f.write_all(b"    QByteArray ba = ret_value.toUtf8();\n")?;
    f.write_all(b"    const char* c_str = ba.data();\n")?;
    f.write_all(b"    assert((ba.size() + 1) < sizeof(s_temp_string_buffer));\n")?;
    f.write_all(b"    memcpy(s_temp_string_buffer, c_str, ba.size() + 1);\n")?;
    f.write_all(b"    return s_temp_string_buffer;\n")?;

    Ok(())
}

///
/// Generate function wrappers such as:
///
/// static void widget_resize(struct PUBase* self_c, int width, int height) {
///     WRWidget* qt_data = (WRWidget*)self_c;
///     qt_data->resize(width, height);
/// }
///
fn generate_func_def<W: Write>(
    f: &mut W,
    sdef: &Struct,
    _api_def: &ApiDef,
    func: &Function,
    struct_name_map: &HashMap<&str, &str>,
    type_handlers: &[Box<TypeHandler>],
    is_widget: bool,
) -> io::Result<()> {
    let ret_value = func.return_val
        .as_ref()
        .map_or("void".into(), |v| v.get_c_type());

    // write return value and function name
    f.write_fmt(format_args!(
        "static {} {}({}) {{ \n",
        ret_value,
        function_name(&sdef.name, func),
        func.generate_c_function_def(FirstArgType::Keep)
    ))?;

    let struct_name: &str = &sdef.name;

    let struct_qt_name = struct_name_map
        .get(struct_name)
        .unwrap_or_else(|| &struct_name);

    let qt_type = if is_widget { "WR" } else { "Q" };

    // (changed from snake_case to camelCase matches the Qt function)

    f.write_fmt(format_args!(
        "    {}{}* qt_data = ({}{}*)self_c;\n",
        qt_type, struct_qt_name, qt_type, struct_qt_name
    ))?;

    if let Some(ref ret_val) = func.return_val {
        for handler in type_handlers.iter() {
            if ret_val.type_name == handler.match_type() {
                let ret = handler.gen_body_return(&func.name);
                return f.write_fmt(format_args!("{};\n", ret));
            }
        }
    }

    if func.return_val.is_some() {
        f.write_fmt(format_args!(
            "    auto ret_value = qt_data->{}(",
            func.name.to_mixed_case()
        ))?;
    } else {
        f.write_fmt(format_args!("    qt_data->{}(", func.name.to_mixed_case()))?;
    }

    let func_def = func.gen_c_invoke_filter(FirstArgName::Remove, |_index, arg| {
        if arg.type_name == "String" {
            Some(format!("QString::fromUtf8({})", &arg.name).into())
        } else {
            for handler in type_handlers.iter() {
                if arg.type_name == handler.match_type() {
                    return Some(handler.replace_arg(&arg).into());
                }
            }

            if arg.vtype == VariableType::Reference {
                Some(format!("(Q{}*){}", &arg.type_name, &arg.name).into())
            } else {
                None
            }
        }
    });

    f.write_fmt(format_args!("{});\n", func_def))?;

    if let Some(ref ret_val) = func.return_val {
        if ret_val.vtype == VariableType::Primitive {
            f.write_all(b"    return ret_value;\n")?;
        //} else if ret_val.array {
            //generate_return_array(f, ret_val)?;
        } else if ret_val.type_name == "String" {
            generate_return_string(f)?;
        } else {
            // If we have a complex (currently assumed) Qt here we need to wrap it before we return
            f.write_fmt(format_args!("    RU{} ctl;\n", ret_val.type_name))?;

            // fill in the function ptr structs
            /*
            for s in api_def.get_inherit_structs(&sdef, RecurseIncludeSelf::Yes) {
                let snake_name = s.name.to_snake_case();

                f.write_fmt(format_args!(
                    "    ctl->{}_funcs = &s_{}_funcs;\n",
                    snake_name,
                    snake_name
                ))?;
            }
            */

            let name = ret_val.type_name.to_snake_case();

            f.write_fmt(format_args!(
               "    ctl.{}_funcs = &s_{}_funcs;\n", name, name
            ))?;

            if ret_val.vtype == VariableType::Regular {
                f.write_fmt(format_args!(
                    "    ctl.priv_data = (struct RUBase*)new Q{}(ret_value);\n", ret_val.type_name,
                ))?;
            } else {
                f.write_fmt(format_args!(
                    "    ctl.priv_data = (struct RUBase*)ret_value;\n"
                ))?;
            }

            f.write_all(b"    return ctl;\n")?;
        }
    }

    f.write_all(b"}\n\n")?;

    Ok(())
}

///
/// Geerate a declaration with only function types. Used for the SIGNAL/SLOT macros
///

fn generate_func_def_input_parms_only(func: &Function) -> String {
    let mut function_args = String::with_capacity(128);
    let len = func.function_args.len() - 1;

    // write arguments
    for (i, arg) in func.function_args[1..].iter().enumerate() {
        let a = match arg.vtype {
            VariableType::Reference => format!("Q{}*", arg.type_name).into(),
            _ => arg.get_c_type(),
        };

        function_args.push_str(&a);

        if i != len - 1 {
            function_args.push_str(", ");
        }
    }

    function_args
}

///
/// Generation function def for callbacks/slots
///

fn generate_func_callback<W: Write>(f: &mut W, struct_name: &str, func: &Function) -> io::Result<()> {
    let signal_type_name = signal_type_callback(&func);
    let func_name = function_name(struct_name, func);

    let mut callback_def = String::with_capacity(128);
    CapiHeaderGen::callback_fun_def_name(&mut callback_def, false, &func_name, func);

    f.write_fmt(format_args!("static {} {{\n", callback_def))?;

    //QSlotWrapperNoArgs* wrap = new QSlotWrapperNoArgs(reciver, (SignalNoArgs)callback);
    f.write_fmt(format_args!(
        "    QSlotWrapper{}* wrap = new QSlotWrapper{}(user_data, ({})trampoline_func, (void*)event);\n",
        signal_type_name, signal_type_name, signal_type_name
    ))?;
    f.write_all(b"    QObject* q_obj = (QObject*)object;\n")?;

    f.write_fmt(format_args!(
        "    QObject::connect(q_obj, SIGNAL({}(",
        func.name.to_mixed_case()
    ))?;

    let func_def = generate_func_def_input_parms_only(func);

    f.write_fmt(format_args!("{})), wrap, SLOT(method(", func_def))?;
    f.write_fmt(format_args!("{})));\n", func_def))?;
    f.write_all(b"}\n\n")?;

    Ok(())
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
    f: &mut W,
    struct_name_map: &HashMap<&str, &str>,
    type_handlers: &Vec<Box<TypeHandler>>,
    api_def: &ApiDef,
) -> io::Result<()> {
    for sdef in &api_def.class_structs {
        let is_widget = sdef.inherits_widget(api_def);

        for func in sdef
            .functions
            .iter()
            .filter(|func| !func.is_manual)
        {
            f.write_all(SEPARATOR)?;

            match func.func_type {
                FunctionType::Static => generate_func_def(f, &sdef, api_def, func, struct_name_map, type_handlers, is_widget)?,
                FunctionType::Regular => generate_func_def(f, &sdef, api_def, func, struct_name_map, type_handlers, is_widget)?,
                FunctionType::Callback => generate_func_callback(f, &sdef.name, func)?,
                _ => (),
            }
        }
    }

    Ok(())
}

///
/// Generate get functions to be used with for static functions
///
fn generate_static_get_functions<W: Write>(
    f: &mut W,
    api_def: &ApiDef,
) -> io::Result<()> {
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

        //
        // Get the name if we have remapped the name (for example we use Button while Qt uses
        // AbstractButton)
        //
        f.write_fmt(format_args!("static struct RU{} get_{}(struct RUBase* priv_data) {{\n", sdef.name, sdef.name.to_snake_case()))?;
        f.write_all(b"    (void)priv_data;\n")?;
        f.write_fmt(format_args!("    RU{} ctl;\n", sdef.name))?;
        f.write_all(b"    ctl.priv_data = nullptr;\n")?;

        // fill in the function ptr structs
        // TODO: Make to common function
        for s in api_def.get_inherit_structs(&sdef, RecurseIncludeSelf::Yes) {
            let snake_name = s.name.to_snake_case();

            f.write_fmt(format_args!(
                "    ctl.{}_funcs = &s_{}_funcs;\n",
                snake_name,
                snake_name
            ))?;
        }

        f.write_all(b"    return ctl;\n}\n\n")?;
    }

    Ok(())
}

///
/// Generates the create functions
///
fn generate_create_functions<W: Write>(
    f: &mut W,
    struct_name_map: &HashMap<&str, &str>,
    api_def: &ApiDef,
) -> io::Result<()> {
    //
    // Generate create functions for all structs that are flagged with create function
    // and doesn't have manual create selected on them
    //
    for sdef in api_def
        .class_structs
        .iter()
        .filter(|s| s.should_have_create_func() && !s.has_manual_create())
    {
        let struct_name = sdef.name.as_str();
        let is_inherits_widget = sdef.inherits_widget(api_def);
        f.write_all(SEPARATOR)?;

        //
        // We create wrapper widgets for all Qt widges so we can have our own functionallity there
        // So we prefix them with WR (WRapper) otherwise we assume it's a regular Q* class
        //
        let (qt_type, create_func) = if is_inherits_widget {
            ("WR", "create_widget_func")
        } else if sdef.should_gen_wrap_class() {
            ("WR", "generic_create_func_with_delete")
        } else {
            ("Q", "generic_create_func")
        };

        //
        // Get the name if we have remapped the name (for example we use Button while Qt uses
        // AbstractButton)
        //
        let struct_qt_name = struct_name_map
            .get(struct_name)
            .unwrap_or_else(|| &struct_name);

        if sdef.should_gen_wrap_class() {
            f.write_fmt(format_args!(
            "static struct RU{} create_{}(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{{\n",
                sdef.name,
                sdef.name.to_snake_case()
            ))?;

            f.write_fmt(format_args!(
                "    auto ctl = {}<struct RU{}, {}{}>(priv_data, delete_callback, private_user_data);\n", create_func,
                struct_name,
                qt_type,
                struct_qt_name,
            ))?;
        } else {
            f.write_fmt(format_args!(
                "static struct RU{} create_{}(struct RUBase* priv_data) {{\n", sdef.name, sdef.name.to_snake_case()
            ))?;

            f.write_fmt(format_args!(
                "    auto ctl = {}<struct RU{}, {}{}>(priv_data);\n", create_func,
                struct_name,
                qt_type,
                struct_qt_name,
            ))?;
        }

        // fill in the function ptr structs
        // TODO: Make to common function
        for s in api_def.get_inherit_structs(&sdef, RecurseIncludeSelf::Yes) {
            let snake_name = s.name.to_snake_case();

            f.write_fmt(format_args!(
                "    ctl.{}_funcs = &s_{}_funcs;\n",
                snake_name,
                snake_name
            ))?;
        }

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
            "    destroy_generic<{}{}>(priv_data);\n}}\n\n",
            qt_type, struct_qt_name
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
            f.write_fmt(format_args!("    destroy_{},\n", struct_name.to_snake_case()))?;
        }

        for func in &sdef.functions {
            match func.func_type {
                 FunctionType::Regular => {
                    f.write_fmt(format_args!("    {},\n", function_name(struct_name, func)))?;
                },

                 FunctionType::Static => {
                    f.write_fmt(format_args!("    {},\n", function_name(struct_name, func)))?;
                },

                FunctionType::Callback => {
                    f.write_fmt(format_args!( "    set_{}_event,\n", function_name(struct_name, func)))?;
                },

                FunctionType::Event => {
                    f.write_fmt(format_args!( "    set_{}_event,\n", function_name(struct_name, func)))?;
                },
            }
        }

        f.write_all(b"};\n\n")?;
    }

    Ok(())
}

///
/// Generate the function struct defs in the following style
///
/// struct Rute s_mime_data_funcs = {
///    mime_data_has_color,
///    mime_data_urls,
///    ...
/// };
///

fn generate_rute_struct<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
    f.write_all(SEPARATOR)?;
    f.write_all(b"static struct Rute s_rute = { \n")?;
    f.write_all(b"    nullptr,\n")?;

    for sdef in api_def
        .class_structs
        .iter()
        .filter(|s| s.should_have_create_func())
    {
        f.write_fmt(format_args!(
            "    create_{},\n",
            sdef.name.to_snake_case()
        ))?;

        if sdef.has_static_functions() {
            f.write_fmt(format_args!(
                "    get_{},\n",
                sdef.name.to_snake_case()
            ))?;
        }
    }

    f.write_all(b"};\n")
}

///
/// Handling for Traits
///
struct TraitTypeHandler(String);

impl TypeHandler for TraitTypeHandler {
    fn match_type(&self) -> &str {
        self.0.as_str()
    }

    fn replace_arg(&self, arg: &Variable) -> String {
        // This is a bit hacky but not sure how to solve this right now
        if self.0 == "WidgetType" {
            format!("(QWidget*){}", &arg.name)
        } else if self.0 == "LayoutType" {
            format!("(QLayout*){}", &arg.name)
        } else {
            format!("dynamic_cast<Q{}*>((QObject*){})", self.0, &arg.name)
        }
    }
}

pub struct QtGenerator {
    wrapper_template: Template,
    signal_wrapper_template: Template,
}

impl QtGenerator {
    pub fn new() -> QtGenerator {
        let parser = ParserBuilder::with_liquid().build();

        QtGenerator {
            wrapper_template: parser.parse(QT_GEN_WRAPPER_TEMPLATE).unwrap(),
            signal_wrapper_template: parser.parse(SIGNAL_WRAPPER_TEMPLATE).unwrap(),
        }
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
    pub fn generate_signal_wrappers<W: Write>(&self, f: &mut W, api_def: &ApiDef) -> io::Result<()> {
        // Sort the signals by their names to have stable generation
        let temp = build_signal_wrappers_info(api_def);
        let ordered: BTreeMap<_, _> = temp.iter().collect();

        for (signal_type_name, func) in ordered {
            let mut template_data = Object::new();

            let c_args = generate_c_function_args_signal_wrapper(EventType::Callback, func);
            //let func_def = func.gen_c_def_filter(Some(None), |_, _| None);
            let c_call_args = func.generate_invoke(FirstArgName::Remove);

            template_data.insert("signal_func_name".to_owned(), Value::Str(signal_type_name.clone()));
            template_data.insert("c_args".to_owned(), Value::Str(c_args));

            // we need to add, at the front of the call args as this is being used inside a
            // function call argument already if we have some args
            if c_call_args != "" {
                template_data.insert("c_call_args".to_owned(), Value::Str(format!(", {}", c_call_args)));
            } else {
                template_data.insert("c_call_args".to_owned(), Value::str(""));
            }

            let res = self.signal_wrapper_template.render(&template_data).unwrap();
            f.write_all(res.as_bytes())?;
        }

        Ok(())
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
            let inherits_widget = sdef.inherits_widget(api_def);

            template_data.insert("struct_name".to_owned(), Value::Str(sdef.name.clone()));
            template_data.insert("cpp_name".to_owned(), Value::Str(sdef.cpp_name.clone()));
            template_data.insert("widget".to_owned(), Value::Bool(inherits_widget));

            /*
            for func in sdef
                .functions
                .iter()
                .filter(|func| func.func_type == FunctionType::Event)
            {
                self.generate_event_setup_def(f, &func)?;
            }
            */


            // TODO: Fix me
            template_data.insert("events".to_owned(), Value::str(""));

            let res = self.wrapper_template.render(&template_data).unwrap();
            f.write_all(res.as_bytes())?;
        }

        Ok(())
    }


    pub fn generate(&self, target_name: &str, api_def: &ApiDef) -> io::Result<()> {
        let header_path = format!("{}.h", target_name);
        let cpp_path = format!("{}.cpp", target_name);
        let mut struct_name_map = HashMap::new();

        // Remapping of the name Button -> AbstractButton for Qt
        struct_name_map.insert("Button", "AbstractButton");

        // Set up all the type handlers that are being used to do special
        // generation when needed
        let mut type_handlers: Vec<Box<TypeHandler>> = Vec::new();

        for trait_name in api_def.get_all_traits() {
            type_handlers.push(Box::new(TraitTypeHandler(trait_name.clone())));
        }

        // Create the header and cpp out
        let mut h_out = BufWriter::new(File::create(header_path)?);
        let mut cpp_out = BufWriter::new(File::create(cpp_path)?);

        // Write the header to cpp generated code
        h_out.write_all(b"#pragma once\n")?;
        h_out.write_all(HEADER)?;
        cpp_out.write_all(HEADER)?;

        // Generate includes for all non-POD structs(
        generate_includes(&mut h_out, &struct_name_map, &api_def)?;
        generate_includes(&mut cpp_out, &struct_name_map, &api_def)?;

        cpp_out.write_all(QT_HEADER)?;

        // Generate all the struct func forward declarations
        generate_forward_declare_struct_defs(&mut h_out, api_def)?;
        generate_forward_declare_struct_defs(&mut cpp_out, api_def)?;

        // Build the signals info used to generate the signal wrapper permutations
        self.generate_signal_wrappers(&mut h_out, api_def)?;

        // Generate the Matching for Qt enums to our enums
        generate_enum_mappings(&mut cpp_out, api_def)?;

        // Generate the wrapping classes declartion is used as for Qt.
        self.generate_wrapper_classes_defs(&mut h_out, api_def)?;

        // Generate the wrapping implementation that is used as for Qt.
        generate_wrapper_classes_impl(&mut cpp_out, &struct_name_map, api_def)?;

        // Generate wrapper functions for are regular defined functions
        generate_function_wrappers(&mut cpp_out, &struct_name_map, &type_handlers, api_def)?;

        // generate the create functions for all widgets
        generate_create_functions(&mut cpp_out, &struct_name_map, api_def)?;

        // generate the static get functions that returns a struct to be used with static functions
        generate_static_get_functions(&mut cpp_out, api_def)?;

        // generate the structs that holds the wrapper functions
        generate_struct_defs(&mut cpp_out, api_def)?;

        // Generate the main Rute struct definition
        generate_rute_struct(&mut cpp_out, api_def)?;

        // write the end footer of the file that is used for the export
        cpp_out.write_all(FOOTER)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
                array: false,
            }],
            return_val: None,
            func_type: FunctionType::Regular,
            is_manual: false,
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
                array: false,
            }],
            return_val: None,
            func_type: FunctionType::Regular,
            is_manual: false,
        };

        let signal_gen = signal_type_callback(&func);
        assert_eq!(signal_gen, "DropEvent_void");
    }
}