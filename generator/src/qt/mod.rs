use std::io;
use std::fs::File;
use std::io::{Read, Write};
use std::collections::{BTreeMap, HashMap};
use api_parser::*;
use c_api_gen::generate_c_function_args;
use c_api_gen::callback_fun_def_name;

use heck::SnakeCase;
use heck::MixedCase;

static HEADER: &'static [u8] = b"
static char s_temp_string_buffer[1024*1024];

struct PrivData {
    QWidget* parent;
};

struct RUWidget plugin_ui_get_parent(RUBase* plugin_ui_priv) {
    PrivData* priv_data = (PrivData*)plugin_ui_priv;

    // meh?

    struct RUWidget widget = {
        &s_widget_funcs,
        (RUBase*)priv_data->parent,
    };

    return widget;
}

\n\n";

static CREATE_WIDGET_TEMPLATE: &'static [u8] =
    b"template<typename T, typename F, typename QT> T create_widget_func(F* funcs, void* priv_data) {
    PrivData* data = (PrivData*)priv_data;
    QT* qt_obj = nullptr;
    if (data) {
        qt_obj = new QT(data->parent);
    } else {
        qt_obj = new QT(nullptr);
    }
    T ctl;
    ctl.funcs = funcs;
    ctl.priv_data = (struct RUBase*)qt_obj;
    return ctl;
}\n\n";

static CREATE_GENERIC_TEMPLATE: &'static [u8] =
    b"template<typename T, typename F, typename QT> T create_generic_func(F* funcs, void* priv_data) {
    QT* qt_obj = new QT();
    T ctl;
    ctl.funcs = funcs;
    ctl.priv_data = (struct RUBase*)qt_obj;
    return ctl;
}\n\n";

static DESTROY_TEMPLATE: &'static [u8] =
    b"template<typename QT> void destroy_generic(struct RUBase* qt_data) {
    QT* qt_obj = (QT*)qt_data;
    delete qt_obj;
}\n\n";

static PLUGIN_UI_CREATE: &'static [u8] = b"
struct RUPluginUI* create_plugin_ui(struct RUBase* user_data, struct RUBase* parent) {
    struct RUPluginUI* instance = new RUPluginUI;
    memcpy(instance, &s_plugin_ui, sizeof(RUPluginUI));
    PrivData* priv_data = new PrivData;
    priv_data->parent = (QWidget*)parent;
    instance->priv_data = (RUBase*)priv_data;
    return instance;
}

void destroy_plugin_ui(RUPluginUI* plugin_ui) {
    PrivData* priv_data = (PrivData*)plugin_ui->priv_data;
    delete priv_data;
    delete plugin_ui;
}
\n\n";

static FOOTER: &'static [u8] = b"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#ifdef _WIN32
extern \"C\" __declspec(dllexport) struct RU* rute_get() {
#else
extern \"C\" struct RU* rute_get() {
#endif
    return (RU*)&s_pu;
}
";

static SEPARATOR: &'static [u8] = b"///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////\n\n";

trait TypeHandler {
    fn match_type(&self) -> String;

    fn replace_arg(&self, arg: &Variable) -> (String, String) {
        (arg.name.to_owned(), arg.vtype.to_owned())
    }

    fn gen_body_return(&self, function_name: &String, f: &mut File) -> io::Result<()> {
        f.write_fmt(format_args!(
            "    return qt_data->{}()",
            function_name.to_mixed_case()
        ))
    }
}

fn generate_bind_info(info: &mut HashMap<String, Function>, func: &Function) {
    let mut input_args = String::new();

    for arg in &func.function_args {
        let tname = arg.get_c_type(true);
        input_args.push_str(&tname);
        input_args.push_str(", ");
    }

    input_args.push_str("void*"); // user_data

    info.entry(input_args.clone()).or_insert(func.clone());
}

// generates signal wrappers for the variatos depending on input parameters
pub fn build_signal_wrappers_info(info: &mut HashMap<String, Function>, api_def: &ApiDef) {
    for sdef in &api_def.entries {
        let mut functions = Vec::new();
        sdef.get_callback_functions(&mut functions);

        for func in &functions {
            generate_bind_info(info, &func);
        }
    }
}

// generates signal wrappers for the variations depending on input parameters
pub fn generate_signal_wrappers_includes(f: &mut File, api_def: &ApiDef) -> io::Result<()> {
    let mut info = HashMap::new();

    for sdef in &api_def.entries {
        let mut functions = Vec::new();
        sdef.get_callback_functions(&mut functions);

        for func in &functions {
            for arg in &func.function_args {
                info.entry(arg.vtype.clone()).or_insert(arg.clone());
            }
        }
    }

    let ordered: BTreeMap<_, _> = info.iter().collect();

    // Generate includes for all refreance types. We assume that these maps to Qt types.

    for (_, arg) in ordered.iter().filter(|&(_, arg)| arg.reference) {
        f.write_fmt(format_args!("#include <Q{}>\n", arg.vtype))?;
    }

    f.write_all(b"\n")?;

    // Generate the ext refs for the static funcs

    for (_, arg) in ordered.iter().filter(|&(_, arg)| arg.reference) {
        f.write_fmt(format_args!("extern struct RU{}Funcs s_{}_funcs;\n", arg.vtype, arg.vtype.to_snake_case()))?;
    }

    f.write_all(b"\n")?;

    Ok(())
}


fn signal_type_callback(func: &Function) -> String {
    let mut name_def = "Signal_".to_owned();

    for arg in &func.function_args {
        name_def.push_str(&arg.vtype);
        name_def.push_str("_")
    }

    name_def.push_str("void");
    name_def
}

pub fn generate_c_function_args_signal_wrapper(event_type: bool, func: &Function) -> String {
    let mut function_args = String::new();
    let len = func.function_args.len();

    // write arguments
    for (i, arg) in func.function_args.iter().enumerate() {
        if i == 0 {
            if event_type {
                function_args.push_str("RUBase*, void*");
            } else {
                function_args.push_str("void*");
            }
        } else {
            function_args.push_str(&arg.get_c_type(false));
        }

        function_args.push_str(" ");
        function_args.push_str(&arg.name);

        if i != len - 1 {
            function_args.push_str(", ");
        }
    }

    if func.function_args.len() == 0 && event_type {
        function_args.push_str("RUBase*");
    }

    function_args
}

/// Generate a signal wrapper that is in the style of this:
///
///    class QSlotWrapperNoArgs : public QObject {
///        Q_OBJECT
///    public:
///        QSlotWrapperNoArgs(void* data, SignalNoArgs func) {
///            m_func = func;
///            m_data = data;
///        }
///
///        Q_SLOT void method() {
///            m_func(m_data);
///        }
///
///    private:
///        SignalNoArgs m_func;
///        void* m_data;
///    };
///
pub fn generate_signal_wrappers(f: &mut File, info: &HashMap<String, Function>) -> io::Result<()> {
    let ordered: BTreeMap<_, _> = info.iter().collect();

    for (_, func) in ordered {
        let signal_type_name = signal_type_callback(func);

        f.write_all(SEPARATOR)?;

        f.write_fmt(format_args!(
            "typedef void (*{})({});\n\n",
            signal_type_name,
            generate_c_function_args_signal_wrapper(false, func)
        ))?;

        f.write_fmt(format_args!(
            "class QSlotWrapper{} : public QObject {{\n    Q_OBJECT\npublic:\n",
            signal_type_name
        ))?;
        f.write_fmt(format_args!(
            "    QSlotWrapper{}(void* data, {} func) {{\n",
            signal_type_name, signal_type_name
        ))?;
        f.write_all(b"        m_func = func;\n")?;
        f.write_all(b"        m_data = data;\n")?;
        f.write_all(b"    }\n\n")?;

        f.write_all(b"    Q_SLOT void method(")?;

        func.write_c_func_def(f, |index, arg| {
            if index == 0 {
                ("".to_owned(), "".to_owned())
            } else {
                if arg.reference {
                    (format!("Q{}*", arg.vtype), arg.name.to_owned())
                } else {
                    (arg.get_c_type(false), arg.name.to_owned())
                }
            }
        })?;

        f.write_all(b" {\n")?;

        // generate temporaries for the case of reference

        for (index, arg) in func.function_args.iter().enumerate() {
            if index == 0 {
                continue;
            }

            if arg.reference {
                f.write_fmt(format_args!("        auto temp_arg_{} = RU{} {{ &s_{}_funcs, (struct RUBase*){} }};\n",
                index, arg.vtype, arg.vtype.to_snake_case(), arg.name.to_owned()))?;
            }
        }

        f.write_all(b"        m_func(")?;

        func.write_c_func_def(f, |index, arg| {
            if index == 0 {
                ("m_data".to_owned(), String::new())
            } else {
                if arg.reference {
                    (format!("(struct RUBase*)&temp_arg_{}", index), String::new())
                } else {
                    (arg.name.to_owned(), String::new())
                }
            }
        })?;

        f.write_all(b";\n")?;
        f.write_all(b"    }\n")?;

        f.write_all(b"private:\n")?;
        f.write_fmt(format_args!("    {} m_func;\n", signal_type_name))?;
        f.write_all(b"    void* m_data;\n")?;
        f.write_all(b"};\n\n")?;
    }

    Ok(())
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

fn generate_return_array(f: &mut File, ret_val: &Variable) -> io::Result<()> {
    // TODO: Use templates

    let vtype = &ret_val.vtype;

    // Reference in this case means that the data from Qt is by pointer
    f.write_all(b"    int count = ret_value.size();\n")?;
    f.write_all(b"    RUArray array = { 0 };\n")?;
    f.write_all(b"    if (count > 0) {\n")?;
    f.write_fmt(format_args!("        RU{}* elements = new RU{}[count];\n", vtype, vtype))?;
    f.write_all(b"        for (int i = 0; i < count; ++i) {\n")?;
    f.write_fmt(format_args!("            elements[i].funcs = &s_{}_funcs;\n", ret_val.vtype.to_snake_case()))?;
    if ret_val.reference {
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

fn generate_return_string(f: &mut File) -> io::Result<()> {
    // TODO: Use templates

    f.write_all(b"    QByteArray ba = ret_value.toUtf8();\n")?;
    f.write_all(b"    const char* c_str = ba.data();\n")?;
    f.write_all(b"    assert((ba.size() + 1) < sizeof(s_temp_string_buffer));\n")?;
    f.write_all(b"    memcpy(s_temp_string_buffer, c_str, ba.size() + 1);\n")?;
    // f.write_all(b"    printf(\"temp string buffer %s\\n\", s_temp_string_buffer);\n")?;
    f.write_all(b"    return s_temp_string_buffer;\n")?;

    Ok(())
}


fn function_name(struct_name: &str, func: &Function) -> String {
    format!("{}_{}", struct_name.to_snake_case(), func.name)
}

fn generate_func_def(
    f: &mut File,
    struct_name: &str,
    func: &Function,
    struct_name_map: &HashMap<&str, &str>,
    type_handlers: &Vec<Box<TypeHandler>>,
    is_widget: bool,
) -> io::Result<()> {
    let ret_value = func.return_val
        .as_ref()
        .map_or("void".to_owned(), |v| v.get_c_type(false));

    // write return value and function name
    f.write_fmt(format_args!(
        "static {} {}({}) {{ \n",
        ret_value,
        function_name(struct_name, func),
        generate_c_function_args(func)
    ))?;

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
            if ret_val.vtype == handler.match_type() {
                handler.gen_body_return(&func.name, f)?;
                f.write_all(b";\n")?;
                return f.write_all(b"}\n\n");
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

    func.write_c_func_def(f, |index, arg| {
        if index == 0 {
            ("".to_owned(), "".to_owned())
        } else if arg.vtype == "String" {
            (format!("QString::fromUtf8({})", &arg.name), "".to_owned())
        } else {
            for handler in type_handlers.iter() {
                if arg.vtype == handler.match_type() {
                    return handler.replace_arg(&arg);
                }
            }

            if arg.reference {
                if arg.cpp_ref {
                    (format!("*(Q{}*){}", &arg.vtype, &arg.name), String::new())
                } else {
                    (format!("(Q{}*){}", &arg.vtype, &arg.name), String::new())
                }
            } else {
                (arg.name.clone(), String::new())
            }
        }
    })?;

    f.write_all(b";\n")?;

    if let Some(ref ret_val) = func.return_val {
        // if this is just a primitive type we can go ahead and return it

        if ret_val.primitive {
            f.write_all(b"    return ret_value;\n")?;
        } else if ret_val.array {
            generate_return_array(f, ret_val)?;
        } else if ret_val.vtype == "String" {
            generate_return_string(f)?;
        } else {
            // If we have a complex (currently assumed) QT here we need to wrap it before we return

            f.write_fmt(format_args!("    RU{} ctl;\n", ret_val.vtype))?;
            f.write_fmt(format_args!(
                "    ctl.funcs = &s_{}_funcs;\n",
                ret_val.vtype.to_snake_case()
            ))?;
            f.write_fmt(format_args!(
                "    ctl.priv_data = (struct RUBase*)ret_value;\n"
            ))?;
            f.write_all(b"    return ctl;\n")?;
        }
    }

    f.write_all(b"}\n\n")?;

    Ok(())
}

///
///
///
///

fn func_def_callback(f: &mut File, struct_name: &str, func: &Function) -> io::Result<()> {
    let signal_type_name = signal_type_callback(func);
    let func_name = function_name(struct_name, func);

    f.write_fmt(format_args!(
        "static {} {{\n",
        callback_fun_def_name(false, &func_name, func)
    ))?;

    //QSlotWrapperNoArgs* wrap = new QSlotWrapperNoArgs(reciver, (SignalNoArgs)callback);
    f.write_fmt(format_args!(
        "    QSlotWrapper{}* wrap = new QSlotWrapper{}(user_data, ({})event);\n",
        signal_type_name, signal_type_name, signal_type_name
    ))?;
    f.write_all(b"    QObject* q_obj = (QObject*)object;\n")?;

    f.write_fmt(format_args!(
        "    QObject::connect(q_obj, SIGNAL({}(",
        func.name.to_mixed_case()
    ))?;

    func.write_c_func_def(f, |index, arg| {
        if index == 0 {
            (String::new(), String::new())
        } else {
            if arg.reference {
                (format!("Q{}*", arg.vtype), String::new())
            } else {
                (arg.get_c_type(false), String::new())
            }
        }
    })?;

    f.write_all(b"), wrap, SLOT(method(")?;

    //func.write_c_func_def(f, |_, arg| (arg.get_c_type(), "".to_owned()))?;

    func.write_c_func_def(f, |index, arg| {
        if index == 0 {
            ("".to_owned(), "".to_owned())
        } else {
            if arg.reference {
                (format!("Q{}*", arg.vtype), String::new())
            } else {
                (arg.get_c_type(false), String::new())
            }
        }
    })?;

    f.write_all(b"));\n")?;
    f.write_all(b"}\n\n")?;

    Ok(())
}

///
///
///
///
fn generate_struct_body_recursive(
    f: &mut File,
    name: &str,
    sdef: &Struct,
    struct_name_map: &HashMap<&str, &str>,
    type_handlers: &Vec<Box<TypeHandler>>,
    api_def: &ApiDef,
) -> io::Result<()> {
    if let Some(ref inherit_name) = sdef.inherit {
        for sdef in &api_def.entries {
            if &sdef.name == inherit_name {
                generate_struct_body_recursive(
                    f,
                    name,
                    &sdef,
                    struct_name_map,
                    type_handlers,
                    api_def,
                )?;
            }
        }
    }

    let is_widget = inherits_widget(sdef, api_def);

    for entry in &sdef.entries {
        match *entry {
            StructEntry::Function(ref func) => {
                f.write_all(SEPARATOR)?;

                match func.func_type {
                    FunctionType::Regular => {
                        if !func.is_manual {
                            generate_func_def(
                                f,
                                name,
                                func,
                                struct_name_map,
                                type_handlers,
                                is_widget,
                            )?
                        }
                    }
                    FunctionType::Callback => func_def_callback(f, name, func)?,
                    _ => (),
                }
            }

            _ => (),
        }
    }

    Ok(())
}

fn inherits_widget(sdef: &Struct, api_def: &ApiDef) -> bool {
    if sdef.name == "Widget" {
        return true;
    }

    if let Some(ref inherit_name) = sdef.inherit {
        if inherit_name == "Widget" {
            return true;
        }

        for sdef in &api_def.entries {
            if inherits_widget(&sdef, api_def) == true {
                return true;
            }
        }
    }

    false
}

///
/// Generate includes from all the structs which are non-pod
///
fn generate_includes(
    f: &mut File,
    struct_name_map: &HashMap<&str, &str>,
    api_def: &ApiDef,
) -> io::Result<()> {
    for sdef in api_def.entries.iter().filter(|v| !v.is_pod()) {
        let struct_name = sdef.name.as_str();
        let struct_qt_name = struct_name_map
            .get(struct_name)
            .unwrap_or_else(|| &struct_name);
        f.write_fmt(format_args!("#include <Q{}>\n", struct_qt_name))?;
    }

    Ok(())
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

fn generate_event_setup_def(f: &mut File, class_name: &str, func: &Function) -> io::Result<()> {
    let event_type = &func.function_args[1];

    // Write virtual function def
    f.write_all(b"public:\n")?;

    f.write_fmt(format_args!(
        "    virtual void {}Event(Q{}* event);\n",
        func.name.to_mixed_case(),
        event_type.vtype
    ))?;



    f.write_fmt(format_args!(
        "    void (*m_{})({})",
        func.name,
        generate_c_function_args_signal_wrapper(true, &func)
    ))?;

    //func.write_c_func_def(f, |_, arg| (arg.get_c_type(), arg.name.to_owned()))?;

    f.write_fmt(format_args!(" = nullptr;\n"))?;
    f.write_fmt(format_args!(
        "    void* m_{}_user_data = nullptr;\n",
        func.name
    ))?;

    Ok(())
}

fn generate_event_setup_impl(f: &mut File, struct_name: &str, class_name: &str, func: &Function) -> io::Result<()> {
    let event_type = &func.function_args[1];

    f.write_fmt(format_args!(
        "void WR{}::{}Event(Q{}* event) {{\n",
        struct_name,
        func.name.to_mixed_case(),
        event_type.vtype
    ))?;
    f.write_fmt(format_args!("    if (m_{}) {{\n", func.name))?;
    f.write_fmt(format_args!("        RU{} e;\n", event_type.vtype,))?;
    f.write_fmt(format_args!(
        "        e.funcs = &s_{}_event_funcs;\n",
        func.name
    ))?;
    f.write_fmt(format_args!(
        "        e.priv_data = (struct RUBase*)event;\n"
    ))?;

    f.write_fmt(format_args!("        RU{} w;\n", struct_name))?;
    f.write_fmt(format_args!(
        "        w.funcs = &s_{}_funcs;\n",
        struct_name.to_snake_case(),
    ))?;
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


///
/// Generate something like this
///
/// static void set_paint_event(void* object, void* user_data, void (*event)(void* self_c, struct RUBase* event)) {
///     WRWidget* qt_object = (WRWidget*)object;
///     qt_object->m_paint_user_data = user_data;
///     qt_object->m_paint = event;
/// }
///
fn generate_event_setup_funcs(
    f: &mut File,
    struct_qt_name: &str,
    func: &Function,
) -> io::Result<()> {
    let func_name = format!("{}_{}", struct_qt_name.to_snake_case(), func.name);

    f.write_fmt(format_args!(
        "{} {{\n",
        callback_fun_def_name(false, &func_name, func)
    ))?;

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

///
/// Generate wrapper classes for all the Widges. This allows us to override
/// virtual functions from the outside (in C and other langs)
/// defs
///
fn generate_wrapper_classes_defs(
    f: &mut File,
    struct_name_map: &HashMap<&str, &str>,
    api_def: &ApiDef,
) -> io::Result<()> {
    for sdef in api_def
        .entries
        .iter()
        .filter(|v| !v.is_pod() && inherits_widget(&v, api_def))
    {
        let struct_name = sdef.name.as_str();
        let struct_qt_name = struct_name_map
            .get(struct_name)
            .unwrap_or_else(|| &struct_name);

        f.write_all(SEPARATOR)?;
        f.write_fmt(format_args!(
            "class WR{} : public Q{} {{\n",
            struct_qt_name, struct_qt_name
        ))?;
        f.write_all(b"    Q_OBJECT\n")?;
        f.write_all(b"public:\n")?;
        f.write_all(b"    Q_PROPERTY(QString persistData READ persistData WRITE setPersistData DESIGNABLE false SCRIPTABLE false)\n")?;
        f.write_all(b"    void setPersistData(const QString& data) { m_persistData = data; }\n")?;
        f.write_all(b"    QString persistData() { return m_persistData; }\n")?;
        f.write_all(b"    QString m_persistData;\n\n")?;

        f.write_fmt(format_args!(
            "    WR{}(QWidget* widget) : Q{}(widget) {{  setObjectName(QStringLiteral(\"Test\")); setPersistData(QStringLiteral(\"SomeData\")); }}\n",
            //"    WR{}(QWidget* widget) : Q{}(widget) {{ }}\n",
            struct_qt_name, struct_qt_name
        ))?;
        f.write_fmt(format_args!("    virtual ~WR{}() {{}}\n\n", struct_qt_name))?;

        let mut event_funcs = Vec::new();
        sdef.get_event_functions(&mut event_funcs);

        for func in &event_funcs {
            generate_event_setup_def(f, &struct_qt_name, &func)?;
        }

        f.write_all(b"};\n\n")?;
    }

    Ok(())
}

///
/// Generate wrapper classes for all the Widges. This allows us to override
/// virtual functions from the outside (in C and other langs)
/// defs
///
fn generate_wrapper_classes_impl(
    f: &mut File,
    struct_name_map: &HashMap<&str, &str>,
    api_def: &ApiDef,
) -> io::Result<()> {
    for sdef in api_def
        .entries
        .iter()
        .filter(|v| !v.is_pod() && inherits_widget(&v, api_def))
    {
        let struct_name = sdef.name.as_str();
        let struct_qt_name = struct_name_map
            .get(struct_name)
            .unwrap_or_else(|| &struct_name);

        let mut event_funcs = Vec::new();
        sdef.get_event_functions(&mut event_funcs);

        for func in &event_funcs {
            generate_event_setup_impl(f, &struct_name, &struct_qt_name, &func)?;
        }

        // Generate the setup functions for events

        for func in event_funcs {
            f.write_all(SEPARATOR)?;
            generate_event_setup_funcs(f, &struct_qt_name, &func)?;
        }
    }

    Ok(())
}

///
/// Generate the struct defs
///
fn generate_struct_def(
    f: &mut File,
    struct_name: &str,
    api_def: &ApiDef,
    sdef: &Struct,
) -> io::Result<()> {
    if let Some(ref inherit_name) = sdef.inherit {
        for sdef in &api_def.entries {
            if &sdef.name == inherit_name {
                generate_struct_def(f, struct_name, api_def, &sdef)?;
            }
        }
    }

    for entry in &sdef.entries {
        match *entry {
            StructEntry::Function(ref func) => match func.func_type {
                FunctionType::Regular => {
                    f.write_fmt(format_args!("    {},\n", function_name(struct_name, func)))?
                }
                FunctionType::Callback => f.write_fmt(format_args!(
                    "    set_{}_event,\n",
                    function_name(struct_name, func)
                ))?,
                _ => (),
            },

            _ => (),
        }
    }

    Ok(())
}

///
/// Generate the struct body
///
fn generate_struct_events(f: &mut File, sdef: &Struct) -> io::Result<()> {
    for entry in &sdef.entries {
        match *entry {
            StructEntry::Function(ref func) => match func.func_type {
                FunctionType::Event => f.write_fmt(format_args!(
                    "    set_{}_event,\n",
                    function_name(&sdef.name, func)
                ))?,
                _ => (),
            },

            _ => (),
        }
    }

    Ok(())
}

///
///
///
///
///
fn generate_struct_defs(f: &mut File, api_def: &ApiDef) -> io::Result<()> {
    for sdef in api_def.entries.iter().filter(|s| !s.is_pod()) {
        f.write_all(SEPARATOR)?;
        f.write_fmt(format_args!(
            "struct RU{}Funcs s_{}_funcs = {{\n",
            sdef.name,
            sdef.name.to_snake_case()
        ))?;

        if !sdef.is_pod() && sdef.should_have_create_func() {
            f.write_fmt(format_args!("    destroy_{},\n", sdef.name.to_snake_case()))?;
        }

        generate_struct_def(f, &sdef.name, api_def, sdef)?;
        generate_struct_events(f, &sdef)?;

        f.write_all(b"};\n\n")?;
    }

    Ok(())
}

///
///
///
///
///
fn generate_forward_declare_struct_defs(f: &mut File, api_def: &ApiDef) -> io::Result<()> {
    f.write_all(SEPARATOR)?;

    for sdef in api_def.entries.iter().filter(|s| !s.is_pod()) {
        f.write_fmt(format_args!(
            "extern struct RU{}Funcs s_{}_funcs;\n",
            sdef.name,
            sdef.name.to_snake_case()
        ))?;
    }

    f.write_all(b"\n")
}

///
/// Generate enum remappings from rute enums to Qt
///
fn generate_enum_mappings(f: &mut File, api_def: &ApiDef) -> io::Result<()> {
    // write enum mapping defs

    f.write_all(SEPARATOR)?;
    f.write_all(b"\n")?;
    f.write_all(b"struct KeyVal { int val, key; };\n")?;

    for enum_def in &api_def.enums {
        f.write_fmt(format_args!("static std::map<int, int> s_{}_lookup;\n", enum_def.name.to_snake_case()))?;
    }

    f.write_all(b"\n")?;
    f.write_all(SEPARATOR)?;
    f.write_all(b"static void create_enum_mappings() {\n")?;

    for enum_def in &api_def.enums {
        let enum_name = enum_def.name.to_snake_case();

        f.write_fmt(format_args!("    static KeyVal {}_vals[] = {{\n", enum_name))?;

        for (i, entry) in enum_def.entries.iter().enumerate() {
            match *entry {
                EnumEntry::Enum(ref value) => {
                    let name = &value;
                    f.write_fmt(format_args!("        {{ (int)Qt::{}, {} }},\n", name, i))?;
                },
                _ => (),
            }
        }

        f.write_all(b"    };\n\n")?;

        f.write_fmt(format_args!("    for (int i = 0; i < {}; ++i) {{\n", enum_def.entries.len()))?;
        f.write_fmt(format_args!("        s_{}_lookup[{}_vals[i].key] = {}_vals[i].val;\n",
            enum_name, enum_name, enum_name))?;
        f.write_all(b"    };\n")?;
    }

    f.write_all(b"}\n\n")
}

// struct RUWidget* create_widget(void* priv_data) {
//    PrivData* data = (PrivData*)priv_data;
//    QWidget* qt_obj = new QWidget(data->parent);
//    RUWidget* ctl = new RUWidget;
//    memcpy(ctl, s_widget, sizeof(s_widget);
//    ctl->priv_data = qt_object;
//    return ctl;
// }

///
/// Generates the create functions
///
fn generate_create_functions(
    f: &mut File,
    struct_name_map: &HashMap<&str, &str>,
    api_def: &ApiDef,
) -> io::Result<()> {
    f.write_all(SEPARATOR)?;
    f.write_all(CREATE_WIDGET_TEMPLATE)?;
    f.write_all(SEPARATOR)?;
    f.write_all(CREATE_GENERIC_TEMPLATE)?;
    f.write_all(SEPARATOR)?;
    f.write_all(DESTROY_TEMPLATE)?;

    for sdef in api_def
        .entries
        .iter()
        .filter(|s| !s.is_pod() && s.should_have_create_func() && !s.has_manual_create())
    {
        let struct_name = sdef.name.as_str();

        let qt_type = match inherits_widget(&sdef, api_def) {
            true => "WR",
            false => "Q",
        };

        f.write_all(SEPARATOR)?;

        f.write_fmt(format_args!(
            "static struct RU{} create_{}(struct RUBase* priv_data) {{\n",
            sdef.name,
            sdef.name.to_snake_case()
        ))?;

        let struct_qt_name = struct_name_map
            .get(struct_name)
            .unwrap_or_else(|| &struct_name);

        if inherits_widget(&sdef, api_def) {
            f.write_fmt(format_args!(
                "    return create_widget_func<struct RU{}, struct RU{}Funcs, WR{}>(&s_{}_funcs, priv_data);\n}}\n\n",
                struct_name,
                struct_name,
                struct_qt_name,
                struct_name.to_snake_case()
            ))?;
        } else {
            f.write_fmt(format_args!(
                "    return create_generic_func<struct RU{}, struct RU{}Funcs, Q{}>(&s_{}_funcs, priv_data);\n}}\n\n",
                struct_name,
                struct_name,
                struct_qt_name,
                struct_name.to_snake_case()
            ))?;
        }

        if !sdef.is_pod() && sdef.should_have_create_func() {
            f.write_all(SEPARATOR)?;

            f.write_fmt(format_args!(
                "static void destroy_{}(struct RUBase* priv_data) {{\n",
                sdef.name.to_snake_case()
            ))?;
            f.write_fmt(format_args!(
                "    destroy_generic<{}{}>(priv_data);\n}}\n\n",
                qt_type, struct_qt_name
            ))?;
        }
    }

    Ok(())
}

///
/// Generate the RU structure
///
fn generate_pu_struct(f: &mut File, api_def: &ApiDef) -> io::Result<()> {
    f.write_all(b"static struct RU s_pu = {\n")?;

    for sdef in api_def
        .entries
        .iter()
        .filter(|s| !s.is_pod() && s.should_have_create_func())
    {
        f.write_fmt(format_args!("    create_{},\n", sdef.name.to_snake_case()))?;
    }

    f.write_all(b"    0,")?;    // hack, should be open_files
    f.write_all(b"    create_plugin_ui,\n")?;
    f.write_all(b"    destroy_plugin_ui,\n")?;
    f.write_all(b"    0,\n\n")?;
    f.write_all(b"};\n\n")?;
    f.write_all(SEPARATOR)
}
///
/// Generate the RUPluginUI structure
///
fn generate_plugin_ui_struct(f: &mut File, api_def: &ApiDef) -> io::Result<()> {
    f.write_all(b"static struct RUPluginUI s_plugin_ui = {\n")?;

    for sdef in api_def
        .entries
        .iter()
        .filter(|s| !s.is_pod() && s.should_have_create_func_plugin())
    {
        f.write_fmt(format_args!("    create_{},\n", sdef.name.to_snake_case()))?;
    }

    f.write_all(b"    0,\n\n")?;
    f.write_all(b"    plugin_ui_get_parent,\n")?;
    f.write_all(b"    0,\n\n")?;
    f.write_all(b"};\n\n")?;
    f.write_all(SEPARATOR)?;
    f.write_all(PLUGIN_UI_CREATE)
}

///
/// Handling for Rect
///
struct RectTypeHandler;

impl TypeHandler for RectTypeHandler {
    fn match_type(&self) -> String {
        "Rect".to_owned()
    }

    fn replace_arg(&self, arg: &Variable) -> (String, String) {
        (
            format!(
                "QRect({}->x, {}->y, {}->width, {}->height)",
                &arg.name, &arg.name, &arg.name, &arg.name
            ),
            "".to_owned(),
        )
    }

    fn gen_body_return(&self, function_name: &String, f: &mut File) -> io::Result<()> {
        f.write_fmt(format_args!(
            "    const auto& t = qt_data->{}();\n",
            function_name.to_mixed_case()
        ))?;
        f.write_fmt(format_args!("    return RURect {{ t.x(), t.y(), t.width(), t.height() }}"))
    }
}
///
/// Handling for Color
///
struct ColorTypeHandler;

impl TypeHandler for ColorTypeHandler {
    fn match_type(&self) -> String {
        "Color".to_owned()
    }

    fn replace_arg(&self, arg: &Variable) -> (String, String) {
        (
            format!(
                "QColor({}->r, {}->g, {}->b, {}->a)",
                &arg.name, &arg.name, &arg.name, &arg.name
            ),
            "".to_owned(),
        )
    }

    fn gen_body_return(&self, function_name: &String, f: &mut File) -> io::Result<()> {
        f.write_fmt(format_args!(
            "    const auto& t = qt_data->{}();\n",
            function_name.to_mixed_case()
        ))?;
        f.write_fmt(format_args!("    return RUColor {{ .r = uint16_t(t.red()), .g = uint16_t(t.green()), .b = uint16_t(t.blue()), .a = uint16_t(t.alpha()) }}"))
    }
}


///
/// Handling for Traits
///
struct TraitTypeHandler(String);

impl TypeHandler for TraitTypeHandler {
    fn match_type(&self) -> String {
        self.0.to_owned()
    }

    fn replace_arg(&self, arg: &Variable) -> (String, String) {
        // This is a bit hacky but not sure how to solve this right now
        if self.0 == "WidgetType" {
            (format!("(QWidget*){}", &arg.name), String::new())
        } else if self.0 == "LayoutType" {
            (format!("(QLayout*){}", &arg.name), String::new())
        } else {
            (
                format!("dynamic_cast<Q{}*>((QObject*){})", self.0, &arg.name),
                String::new(),
            )
            //(format!("(Q{}*){}", self.0, &arg.name), "".to_owned())
        }
    }
}

///
/// This is the main entry for generating the C/C++ buindings for Qt. The current API mimics Qt
/// fairly closely when it comes to names but at the start there is a map setup that used used
/// to translate between some struct names to fit the Qt version. If more structs gets added that
/// needs to be translated into another Qt name they needs to be added here as well
///
pub fn generate_qt_bindings(
    filename: &str,
    header_filename: &str,
    manual_cpp_code: &str,
    api_def: &ApiDef,
) -> io::Result<()> {
    let mut f = File::create(filename)?;
    let mut header_file = File::create(header_filename)?;
    let mut cpp_manual = File::open(manual_cpp_code)?;
    let mut signals_info = HashMap::new();
    let mut struct_name_map = HashMap::new();
    let mut type_handlers: Vec<Box<TypeHandler>> = Vec::new();
    let mut cpp_manual_data = Vec::new();

    cpp_manual.read_to_end(&mut cpp_manual_data)?;

    type_handlers.push(Box::new(RectTypeHandler {}));
    type_handlers.push(Box::new(ColorTypeHandler {}));

    for trait_name in api_def.get_all_traits() {
        type_handlers.push(Box::new(TraitTypeHandler(trait_name.clone())));
    }

    struct_name_map.insert("Button", "AbstractButton");

    f.write_all(b"#include \"c_api.h\"\n")?;
    f.write_all(b"#include \"qt_api_gen.h\"\n")?;
    f.write_all(b"#include <assert.h>\n")?;

    build_signal_wrappers_info(&mut signals_info, api_def);

    header_file.write_all(b"#pragma once\n\n#include \"c_api.h\"\n#include <QObject>\n")?;
    generate_includes(&mut header_file, &struct_name_map, api_def)?;
    generate_includes(&mut f, &struct_name_map, api_def)?;

    generate_forward_declare_struct_defs(&mut header_file, api_def)?;
    generate_signal_wrappers_includes(&mut header_file, api_def)?;
    generate_signal_wrappers(&mut header_file, &signals_info)?;

    //generate_forward_declare_struct_defs(&mut f, api_def)?;
    generate_enum_mappings(&mut f, api_def)?;

    f.write_all(HEADER)?;

    generate_wrapper_classes_defs(&mut header_file, &struct_name_map, api_def)?;
    generate_wrapper_classes_impl(&mut f, &struct_name_map, api_def)?;

    // generate wrapper functions

    for sdef in &api_def.entries {
        generate_struct_body_recursive(
            &mut f,
            &sdef.name,
            &sdef,
            &struct_name_map,
            &type_handlers,
            api_def,
        )?;
    }

    generate_create_functions(&mut f, &struct_name_map, api_def)?;


    f.write_all(&cpp_manual_data)?;

    generate_struct_defs(&mut f, api_def)?;
    generate_plugin_ui_struct(&mut f, api_def)?;
    generate_pu_struct(&mut f, api_def)?;

    f.write_all(FOOTER)
}
