use std::io;
use std::fs::File;
use std::io::{Write, Read};
use std::collections::{BTreeMap, HashMap};
use api_parser::*;
use c_api_gen::generate_c_function_args;
use c_api_gen::callback_fun_def_name;

use heck::SnakeCase;
use heck::MixedCase;

static HEADER: &'static [u8] = b"
struct PrivData {
    QWidget* parent;
    void* user_data;
};\n\n";

static CREATE_WIDGET_TEMPLATE: &'static [u8] =
    b"template<typename T, typename QT> T* create_widget_func(T* struct_data, void* priv_data) {
    PrivData* data = (PrivData*)priv_data;
    QT* qt_obj = nullptr;
    if (data) {
        qt_obj = new QT(data->parent);
    } else {
        qt_obj = new QT(nullptr);
    }
    T* ctl = new T;
    memcpy(ctl, struct_data, sizeof(T));
    ctl->priv_data = qt_obj;
    return ctl;
}\n\n";

static CREATE_GENERIC_TEMPLATE: &'static [u8] =
    b"template<typename T, typename QT> T* create_generic_func(T* struct_data, void* priv_data) {
    QT* qt_obj = new QT();
    T* ctl = new T;
    memcpy(ctl, struct_data, sizeof(T));
    ctl->priv_data = qt_obj;
    return ctl;
}\n\n";

static DESTROY_TEMPLATE: &'static [u8] =
    b"template<typename T, typename QT> void destroy_generic(void* struct_data) {
    assert(struct_data);
    T* t = (T*)struct_data;
    QT* qt_obj = (QT*)t->priv_data;
    delete qt_obj;
    delete t;
}\n\n";

static FOOTER: &'static [u8] = b"
struct PU* PU_create_instance(void* user_data, QWidget* parent) {
    struct PU* instance = new PU;
    memcpy(instance, &s_pu, sizeof(PU));
    PrivData* priv_data = new PrivData;
    priv_data->parent = parent;
    priv_data->user_data = user_data;
    return instance;
}\n\n

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern \"C\" struct PU* wrui_get() {
    return (PU*)&s_pu;
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
        let tname = arg.get_c_type();
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

fn signal_type_callback(func: &Function) -> String {
    let mut name_def = "Signal_".to_owned();

    for arg in &func.function_args {
        name_def.push_str(&arg.vtype);
        name_def.push_str("_")
    }

    name_def.push_str("void");
    name_def
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
            generate_c_function_args(func)
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
                (arg.get_c_type(), arg.name.to_owned())
            }
        })?;

        f.write_all(b" {\n        m_func(")?;

        func.write_c_func_def(f, |index, arg| {
            if index == 0 {
                ("m_data".to_owned(), "".to_owned())
            } else {
                (arg.name.to_owned(), "".to_owned())
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
        .map_or("void".to_owned(), |v| v.get_c_type());

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

        f.write_fmt(format_args!(
            "    return qt_data->{}()",
            func.name.to_mixed_case()
        ))?;
    } else {
        f.write_fmt(format_args!("    qt_data->{}(", func.name.to_mixed_case()))?;

        func.write_c_func_def(f, |index, arg| {
            if index == 0 {
                ("".to_owned(), "".to_owned())
            } else if arg.vtype == "String" {
                (format!("QString::fromLatin1({})", &arg.name), "".to_owned())
            } else {
                for handler in type_handlers.iter() {
                    if arg.vtype == handler.match_type() {
                        return handler.replace_arg(&arg);
                    }
                }

                (arg.name.clone(), "".to_owned())
            }
        })?;
    }

    f.write_all(b";\n")?;
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

    println!("struct_name {}", struct_name);

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
            ("".to_owned(), "".to_owned())
        } else {
            (arg.get_c_type(), "".to_owned())
        }
    })?;

    f.write_all(b"), wrap, SLOT(method(")?;

    //func.write_c_func_def(f, |_, arg| (arg.get_c_type(), "".to_owned()))?;

    func.write_c_func_def(f, |index, arg| {
        if index == 0 {
            ("".to_owned(), "".to_owned())
        } else {
            (arg.get_c_type(), "".to_owned())
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
                        generate_func_def(f, name, func, struct_name_map, type_handlers, is_widget)?
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
///  virtual void paintEvent(QPaintEvent* event) {
///        if (m_paint_event) {
///            PUPainteEvent e;
///            memcpy(&e, s_paint_event, sizeof(e));
///            e.priv_data = event;
///            m_paint_event((PUPainteEvent*)&e, m_paint_user_data);
///        } else {
///            QWidget::paintEvent(event);
///        }
///    }
///
///    PUPaintEventFunc m_paint_event = nullptr;
///    void* m_paint_user_data = nullptr;

fn generate_event_setup(f: &mut File, class_name: &str, func: &Function) -> io::Result<()> {
    let event_type = &func.function_args[1];

    // Write virtual function def

    f.write_fmt(format_args!(
        "    virtual void {}(Q{}* event) {{\n",
        func.name.to_mixed_case(),
        event_type.vtype
    ))?;
    f.write_fmt(format_args!("        if (m_{}) {{\n", func.name))?;
    f.write_fmt(format_args!(
        "            PU{} e = s_{};\n",
        event_type.vtype, func.name
    ))?;
    f.write_fmt(format_args!("            e.priv_data = event;\n"))?;
    f.write_fmt(format_args!(
        "            m_{}(m_{}_user_data, (PU{}*)&e);\n",
        func.name, func.name, event_type.vtype
    ))?;
    f.write_fmt(format_args!("        }} else {{\n"))?;
    f.write_fmt(format_args!(
        "            Q{}::{}(event);\n",
        class_name,
        func.name.to_mixed_case()
    ))?;
    f.write_fmt(format_args!("        }}\n"))?;
    f.write_fmt(format_args!("    }}\n\n"))?;

    // write data

    f.write_fmt(format_args!("    void (*m_{})(", func.name))?;

    func.write_c_func_def(f, |_, arg| (arg.get_c_type(), arg.name.to_owned()))?;

    f.write_fmt(format_args!(" = nullptr;\n"))?;
    f.write_fmt(format_args!(
        "    void* m_{}_user_data = nullptr;\n",
        func.name
    ))?;

    Ok(())
}

///
/// Generate wrapper classes for all the Widges. This allows us to override
/// virtual functions from the outside (in C and other langs)
///
fn generate_wrapper_classes(
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
        f.write_all(b"public:\n")?;
        f.write_fmt(format_args!(
            "    WR{}(QWidget* widget) : Q{}(widget) {{}}\n",
            struct_qt_name, struct_qt_name
        ))?;
        f.write_fmt(format_args!("    virtual ~WR{}() {{}}\n\n", struct_qt_name))?;

        let mut event_funcs = Vec::new();
        sdef.get_event_functions(&mut event_funcs);

        for func in event_funcs {
            generate_event_setup(f, &struct_qt_name, &func)?;
        }

        f.write_all(b"};\n\n")?;
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
///
///
///
///
fn generate_struct_defs(f: &mut File, api_def: &ApiDef) -> io::Result<()> {
    for sdef in api_def.entries.iter().filter(|s| !s.is_pod()) {
        f.write_all(SEPARATOR)?;
        f.write_fmt(format_args!(
            "struct PU{} s_{} = {{\n",
            sdef.name,
            sdef.name.to_snake_case()
        ))?;

        if !sdef.is_pod() && sdef.should_have_create_func() {
            f.write_fmt(format_args!("    destroy_{},\n", sdef.name.to_snake_case()))?;
        }

        generate_struct_def(f, &sdef.name, api_def, sdef)?;

        f.write_all(b"    0,\n")?;
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
            "extern struct PU{} s_{};\n",
            sdef.name,
            sdef.name.to_snake_case()
        ))?;
    }

    f.write_all(b"\n")
}

// struct PUWidget* create_widget(void* priv_data) {
//    PrivData* data = (PrivData*)priv_data;
//    QWidget* qt_obj = new QWidget(data->parent);
//    PUWidget* ctl = new PUWidget;
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
            "static struct PU{}* create_{}(void* priv_data) {{\n",
            sdef.name,
            sdef.name.to_snake_case()
        ))?;

        let struct_qt_name = struct_name_map
            .get(struct_name)
            .unwrap_or_else(|| &struct_name);

        if inherits_widget(&sdef, api_def) {
            f.write_fmt(format_args!(
                "    return create_widget_func<struct PU{}, WR{}>(&s_{}, priv_data);\n}}\n\n",
                struct_name,
                struct_qt_name,
                struct_name.to_snake_case()
            ))?;
        } else {
            f.write_fmt(format_args!(
                "    return create_generic_func<struct PU{}, Q{}>(&s_{}, priv_data);\n}}\n\n",
                struct_name,
                struct_qt_name,
                struct_name.to_snake_case()
            ))?;
        }

        if !sdef.is_pod() && sdef.should_have_create_func() {
            f.write_all(SEPARATOR)?;

            f.write_fmt(format_args!(
                "static void destroy_{}(void* priv_data) {{\n",
                sdef.name.to_snake_case()
            ))?;
            f.write_fmt(format_args!(
                "    destroy_generic<struct PU{}, {}{}>(priv_data);\n}}\n\n",
                struct_name, qt_type, struct_qt_name
            ))?;
        }
    }

    Ok(())
}

///
/// Generate the PU structure
///
fn generate_pu_struct(f: &mut File, api_def: &ApiDef) -> io::Result<()> {
    f.write_all(b"static struct PU s_pu = {\n")?;

    for sdef in api_def
        .entries
        .iter()
        .filter(|s| !s.is_pod() && s.should_have_create_func())
    {
        f.write_fmt(format_args!("    create_{},\n", sdef.name.to_snake_case()))?;
    }

    f.write_all(b"    0,\n\n")?;
    f.write_all(b"};\n\n")?;
    f.write_all(SEPARATOR)
}

///
/// Type handler for traits as the function arguments when using a trait needs to use "get_obj"
///
/*
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

    fn gen_body(&self, fuction_name: &str, f: &mut File) -> io::Result<()>  {
    fn gen_body(&self, arg_name: &str, _f: &mut File, _index: usize) -> String {
        format!("{}.get_obj() as *const PU{}", arg_name, self.name)
    }
}
*/

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
        f.write_fmt(format_args!("    return PURect {{ .x = t.x(), .y = t.y(), .width = t.width(), .height = t.height() }}"))
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
            (format!("(QWidget*){}", &arg.name), "".to_owned())
        } else {
            (format!("(Q{}*){}", self.0, &arg.name), "".to_owned())
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

    for trait_name in api_def.get_all_traits() {
        type_handlers.push(Box::new(TraitTypeHandler(trait_name.clone())));
    }

    struct_name_map.insert("Button", "AbstractButton");

    f.write_all(b"#include \"c_api.h\"\n")?;
    f.write_all(b"#include \"qt_api_gen.h\"\n")?;
    f.write_all(b"#include <assert.h>\n")?;

    generate_includes(&mut f, &struct_name_map, api_def)?;

    f.write_all(HEADER)?;

    build_signal_wrappers_info(&mut signals_info, api_def);

    header_file.write_all(b"#pragma once\n#include <QObject>\n\n")?;
    generate_signal_wrappers(&mut header_file, &signals_info)?;

    generate_forward_declare_struct_defs(&mut f, api_def)?;

    generate_wrapper_classes(&mut f, &struct_name_map, api_def)?;

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
    generate_pu_struct(&mut f, api_def)?;

    f.write_all(FOOTER)
}
