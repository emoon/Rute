use heck::MixedCase;
use heck::SnakeCase;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use c_gen::*;

use api_parser::*;

#[derive(PartialEq)]
pub enum EventType {
    Callback,
    Event,
}

//
// Used to make CPP generated code a bit easier to read
//
static SEPARATOR: &'static [u8] = b"///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////\n\n";

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
    f.write_all(b"static void create_enum_mappings() {\n")?;

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

fn generate_event_setup_def<W: Write>(f: &mut W, class_name: &str, func: &Function) -> io::Result<()> {
    let event_type = &func.function_args[1];

    // Write virtual function def
    f.write_all(b"public:\n")?;

    f.write_fmt(format_args!(
        "    virtual void {}Event(Q{}* event);\n",
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

///
/// Generate wrapper classes for all the Widges. This allows us to override
/// virtual functions from the outside (in C and other langs)
/// defs
///
fn generate_wrapper_classes_defs<W: Write>(
    f: &mut W,
    struct_name_map: &HashMap<&str, &str>,
    api_def: &ApiDef,
) -> io::Result<()> {
    for sdef in api_def
        .class_structs
        .iter()
        .filter(|v| v.inherits_widget(api_def))
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
        //f.write_all(b"    Q_PROPERTY(void* userData READ userData WRITE setUserData DESIGNABLE false SCRIPTABLE false)\n")?;
        //f.write_all(b"    void setUserData(void* data) { m_qt_user_data = data; }\n")?;
        //f.write_all(b"    void* userData() { return m_qt_user_data; }\n")?;
        //f.write_all(b"    void* m_qt_user_data;\n\n")?;

        f.write_fmt(format_args!(
            "    WR{}(QWidget* widget) : Q{}(widget) {{ }}\n",
            struct_qt_name, struct_qt_name
        ))?;
        f.write_fmt(format_args!("    virtual ~WR{}() {{}}\n\n", struct_qt_name))?;

        let event_funcs = api_def.get_functions_recursive(&sdef, FunctionType::Event);

        for func in &event_funcs {
            generate_event_setup_def(f, &struct_qt_name, &func)?;
        }

        f.write_all(b"};\n\n")?;
    }

    Ok(())
}

///
/// Generates wrapping code fore "Events" (i.e virtual overrides on Qt objects)
///
fn generate_event_setup_impl<W: Write>(
    f: &mut W,
    struct_name: &str,
    class_name: &str,
    func: &Function,
) -> io::Result<()> {
    let event_type = &func.function_args[1];

    f.write_fmt(format_args!(
        "void WR{}::{}Event(Q{}* event) {{\n",
        struct_name,
        func.name.to_mixed_case(),
        event_type.type_name
    ))?;
    f.write_fmt(format_args!("    if (m_{}) {{\n", func.name))?;
    f.write_fmt(format_args!("        RU{} e;\n", event_type.type_name,))?;
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
fn generate_event_setup_funcs<W: Write>(
    f: &mut W,
    struct_qt_name: &str,
    func: &Function,
) -> io::Result<()> {
    let func_name = format!("{}_{}", struct_qt_name.to_snake_case(), func.name);

    f.write_fmt(format_args!(
        "{} {{\n",
        CapiGenerator::callback_fun_def_name(false, &func_name, func),
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
fn generate_wrapper_classes_impl<W: Write>(
    f: &mut W,
    struct_name_map: &HashMap<&str, &str>,
    api_def: &ApiDef,
) -> io::Result<()> {
    for sdef in api_def
        .class_structs
        .iter()
        .filter(|v| v.inherits_widget(api_def))
    {
        let struct_name = sdef.name.as_str();
        let struct_qt_name = struct_name_map
            .get(struct_name)
            .unwrap_or_else(|| &struct_name);

        // Get all the structs that thu current struct inherit from

        let event_funcs = api_def.get_functions_recursive(&sdef, FunctionType::Event);

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
            if event_type == EventType::Event {
                function_args.push_str("RUBase*, void*");
            } else {
                function_args.push_str("void* self_c, void* wrapped_func");
            }
        } else {
            function_args.push_str(&arg.get_c_type());
            function_args.push_str(" ");
            function_args.push_str(&arg.name);
        }

        if i != len - 1 {
            function_args.push_str(", ");
        }
    }

    if func.function_args.len() == 0 && event_type == EventType::Event {
        function_args.push_str("RUBase*");
    }

    function_args
}

///
/// Generate signal wrapper name.
///
/// Example:
///
/// Signal_self_i32_u32_void
///
fn signal_type_callback(func: &Function) -> String {
    let mut name_def = "Signal_".to_owned();

    for arg in &func.function_args {
        match arg.vtype {
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

/// Generate a signal wrapper that is in the style of this:
///
/// typedef void (*Signal_self_int_void)(void* self_c, void* wrapped_func, int row);
///
/// class QSlotWrapperSignal_self_int_void : public QObject {
///     Q_OBJECT
/// public:
///     QSlotWrapperSignal_self_int_void(void* data, Signal_self_int_void func, void* wrapped_func) {
///         m_func = func;
///         m_data = data;
///         m_wrapped_func = wrapped_func;
///     }
///
///     Q_SLOT void method(int row) {
///         m_func(m_data, m_wrapped_func, row);
///     }
/// private:
///     Signal_self_int_void m_func;
///     void* m_data;
///     void* m_wrapped_func;
/// };
///
pub fn generate_signal_wrappers<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
    // Sort the signals by their names to have stable generation
    let temp = build_signal_wrappers_info(api_def);
    let ordered: BTreeMap<_, _> = temp.iter().collect();

    for (signal_type_name, func) in ordered {
        f.write_fmt(format_args!(
            "typedef void (*{})({});\n\n",
            signal_type_name,
            generate_c_function_args_signal_wrapper(EventType::Callback, func)
        ))?;

        f.write_fmt(format_args!(
            "class QSlotWrapper{} : public QObject {{\n    Q_OBJECT\npublic:\n",
            signal_type_name
        ))?;
        f.write_fmt(format_args!(
            "    QSlotWrapper{}(void* data, {} func, void* wrapped_func) {{\n",
            signal_type_name, signal_type_name
        ))?;
        f.write_all(b"        m_func = func;\n")?;
        f.write_all(b"        m_data = data;\n")?;
        f.write_all(b"        m_wrapped_func = wrapped_func;\n")?;
        f.write_all(b"    }\n\n")?;

        //
        // Generate a function defitition where we replace all references
        // so the name starts with Q* (as these map to Qt events)
        // We also remove the first parameter (as it's user data) which
        // isn't used for the Qt method def
        //
        let func_def = func.gen_c_def_filter(Some(None), |_index, arg| match arg.vtype {
            VariableType::Reference => Some(format!("Q{}*", arg.type_name).into()),
            _ => None,
        });

        f.write_fmt(format_args!("    Q_SLOT void method({}) {{\n", func_def))?;

        // generate temporaries for the case of reference for funcs
        for (index, arg) in func.function_args.iter().enumerate() {
            if index == 0 {
                continue;
            }

            match arg.vtype {
                VariableType::Reference => {
                    let name = arg.type_name.as_str();
                    f.write_fmt(format_args!(
                        "        auto temp_arg_{} = RU{} {{ &s_{}_funcs, (struct RUBase*){} }};\n",
                        index,
                        name,
                        name.to_snake_case(),
                        arg.name.to_owned()
                    ))?;
                }
                _ => (),
            }
        }

        // Generate the function invokation in the callback. First parameter is always m_data for
        // the user_data that needs to be passed to the callback
        let func_invoke = func.gen_c_invoke_filter(
            Some("m_data, m_wrapped_func".into()),
            |index, arg| match arg.vtype {
                VariableType::Reference => {
                    (Some(format!("(struct RUBase*)&temp_arg_{}", index).into()))
                }
                _ => None,
            },
        );

        f.write_fmt(format_args!("        m_func({});\n", func_invoke))?;
        f.write_all(b"    }\n")?;

        f.write_all(b"private:\n")?;
        f.write_fmt(format_args!("    {} m_func;\n", signal_type_name))?;
        f.write_all(b"    void* m_data;\n")?;
        f.write_all(b"    void* m_wrapped_func;\n")?;
        f.write_all(b"};\n\n")?;
    }

    Ok(())
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

        //
        // We create wrapper widgets for all Qt widges so we can have our own functionallity there
        // So we prefix them with WR (WRapper) otherwise we assume it's a regular Q* class
        //
        let qt_type = match is_inherits_widget {
            true => "WR",
            false => "Q",
        };

        f.write_fmt(format_args!(
            "static struct RU{} create_{}(struct RUBase* priv_data) {{\n",
            sdef.name,
            sdef.name.to_snake_case()
        ))?;

        //
        // Get the name if we have remapped the name (for example we use Button while Qt uses
        // AbstractButton)
        //
        let struct_qt_name = struct_name_map
            .get(struct_name)
            .unwrap_or_else(|| &struct_name);

        //
        // Depending on if this is a widget we generate the code a bit differently and have
        // a generic create function otherwise a template function for creating widgets
        //
        if is_inherits_widget {
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

pub struct CppGenerator;

impl CppGenerator {
    pub fn generate(target_name: &str, api_def: &ApiDef) -> io::Result<()> {
        let header_path = format!("{}.h", target_name);
        let cpp_path = format!("{}.cpp", target_name);
        let mut struct_name_map = HashMap::new();

        println!("header file {:?}", header_path);

        struct_name_map.insert("Button", "AbstractButton");

        // Create the header and cpp out
        let mut h_out = BufWriter::new(File::create(header_path)?);
        let mut cpp_out = BufWriter::new(File::create(cpp_path)?);

        // Generate all the struct func forward declarations
        generate_forward_declare_struct_defs(&mut h_out, api_def)?;

        // Build the signals info used to generate the signal wrapper permutations
        generate_signal_wrappers(&mut h_out, api_def)?;

        // Generate the Matching for Qt enums to our enums
        generate_enum_mappings(&mut cpp_out, api_def)?;

        // Generate the wrapping classes declartion is used as for Qt.
        generate_wrapper_classes_defs(&mut cpp_out, &struct_name_map, api_def)?;

        // Generate the wrapping implementation that is used as for Qt.
        generate_wrapper_classes_impl(&mut cpp_out, &struct_name_map, api_def)?;

        // generate the create functions for all widgets
        generate_create_functions(&mut cpp_out, &struct_name_map, api_def)?;

        Ok(())
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
                vtype: VariableType::Primitive("i32".to_owned()),
                array: false,
            }],
            return_val: None,
            func_type: FunctionType::Regular,
            is_manual: false,
        };

        let signal_gen = signal_type_callback(&func);
        assert_eq!(signal_gen, "Signal_int_void");
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
                vtype: VariableType::Reference("DropEvent".to_owned()),
                array: false,
            }],
            return_val: None,
            func_type: FunctionType::Regular,
            is_manual: false,
        };

        let signal_gen = signal_type_callback(&func);
        assert_eq!(signal_gen, "Signal_DropEvent_void");
    }
}
