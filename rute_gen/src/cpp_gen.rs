use std::collections::HashMap;
use std::path::Path;
use std::io::{BufWriter, Write};
use std::fs::File;
use std::io;

use api_parser::*;

#[derive(PartialEq)]
pub enum EventType {
    Callback,
    Event,
}

///
///
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
                function_args.push_str("void*");
            }
        } else {
            function_args.push_str(arg.get_c_type());
        }

        function_args.push_str(" ");
        function_args.push_str(&arg.name);

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
/// Signal_i32_u32_void
///
fn signal_type_callback(func: &Function) -> String {
    let mut name_def = "Signal_".to_owned();

    for arg in &func.function_args {
        name_def.push_str(arg.vtype.get_type());
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
pub fn generate_signal_wrappers<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
    // Sort the signals by their names to have stable generation
    let ordered: BTreeMap<_, _> = build_signal_wrappers_info().iter().collect();

    for (signal_type_name, func) in ordered {
        let signal_type_name = signal_type_callback(func);

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
            "    QSlotWrapper{}(void* data, {} func) {{\n",
            signal_type_name, signal_type_name
        ))?;
        f.write_all(b"        m_func = func;\n")?;
        f.write_all(b"        m_data = data;\n")?;
        f.write_all(b"    }\n\n")?;

        f.write_all(b"    Q_SLOT void method(")?;
    }

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
    f.write_all(SEPARATOR)?;

    api_def.class_structs.iter().for_each(|s| {
        f.write_fmt(format_args!(
            "extern struct RU{}Funcs s_{}_funcs;\n",
            sdef.name,
            sdef.name.to_snake_case()
        ))?;
    });

    f.write_all(b"\n")
}



pub struct CppGenerator;

impl CppGenerator {
    pub fn generate(target_name: &str, api_def: &ApiDef) -> io::Result<()> {
        let header_path = Path::new(target_name).join(".h");
        let cpp_path = Path::new(target_name).join(".cpp");

        // Create the header and cpp out
        let mut h_out = BufWriter::new(File::create(header_path)?);
        let mut cpp_out = BufWriter::new(File::create(cpp_path)?);

        // Generate all the struct func forward declarations
        generate_forward_declare_struct_defs(&mut h_out, api_def)?;

        // Build the signals info used to generate the signal wrapper permutations
        let signal_info = build_signal_wrappers_info(api_def);

        Ok(())
    }
}

