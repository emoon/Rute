
///
/// In order to figure out what combination of of SignalWrappers we need to generate
/// we go over all the the registered callback function and create a hash for the
/// function arguments. This way we get one unique function wrapper in the cases
/// where several wrapers has the same input
///
pub fn build_signal_wrappers_info<'a>(api_def: &'a ApiDef) -> HashMap<String, &'a Function> {
    let mut wrapper_info = HashMap::new();

    api_def
        .get_functions(FunctionType::Callback)
        .map(|func| {
            let mut input_args = String::with_capacity(100);

            // this will construct a string that looks something like:
            // uint32_t, QEvent, void*

            for arg in &func.function_args {
                let tname = arg.get_c_type(true);
                input_args.push_str(&tname);
                input_args.push_str(", ");
            }

            input_args.push_str("void*"); // user_data
            wrapper_info.entry(input_args).or_insert(func);
        });

    wrapper_info
}

