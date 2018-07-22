use heck::SnakeCase;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

use api_parser::*;

#[derive(PartialEq)]
pub enum EventType {
    Callback,
    Event,
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
            VariableType::SelfType => name_def.push_str("self"),
            VariableType::Reference(ref name) => name_def.push_str(name),
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
///    private:f
///        SignalNoArgs m_func;
///        void* m_data;
///    };
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
        let func_def = func.gen_c_def_filter(Some(None), |_index, arg| {
            match arg.vtype {
                VariableType::Reference(ref name) => {
                    Some(format!("Q{}*", name).into())
                },
                _ => None,
            }
        });

        f.write_fmt(format_args!("    Q_SLOT void method({}) {{\n", func_def))?;

        // generate temporaries for the case of reference for funcs
        for (index, arg) in func.function_args.iter().enumerate() {
            if index == 0 {
                continue;
            }

            match arg.vtype {
                VariableType::Reference(ref name) => {
                    f.write_fmt(format_args!("        auto temp_arg_{} = RU{} {{ &s_{}_funcs, (struct RUBase*){} }};\n",
                    index, name, name.to_snake_case(), arg.name.to_owned()))?;
                },
                _ => (),
            }
        }

        // Generate the function invokation in the callback. First parameter is always m_data for
        // the user_data that needs to be passed to the callback
        let func_invoke = func.gen_c_invoke_filter(Some("m_data, m_wrapped_func".into()), |index, arg| {
            match arg.vtype {
                VariableType::Reference(ref _name) => {
                    (Some(format!("(struct RUBase*)&temp_arg_{}", index).into()))
                },
                _ => None,
            }
        });

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

pub struct CppGenerator;

impl CppGenerator {
    pub fn generate(target_name: &str, api_def: &ApiDef) -> io::Result<()> {
        let header_path = format!("{}.h", target_name);
        //let cpp_path = Path::new(target_name).join(".cpp");

        println!("header file {:?}", header_path);

        // Create the header and cpp out
        let mut h_out = BufWriter::new(File::create(header_path)?);
        //let mut cpp_out = BufWriter::new(File::create(cpp_path)?);

        // Generate all the struct func forward declarations
        generate_forward_declare_struct_defs(&mut h_out, api_def)?;

        // Build the signals info used to generate the signal wrapper permutations
        generate_signal_wrappers(&mut h_out, api_def)
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
