



///
/// Go over all
///
fn generate_signal_wrappers_includes(f: &mut File, api_def: &ApiDef) -> io::Result<()> {
    let mut info = HashMap::new();

    api_def.class_structs
        .iter()
        .flat_map(|s| s.functions.iter())
        .filter(|f| f.func_type == FunctionType::Callback)
        .try_for_each(|func| Self::generate_function(&mut f, &func))?;

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

