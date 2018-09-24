pub static HEADER: &'static [u8] = b"
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

#[allow(unused_imports)]
use std::ffi::{CString, CStr};

use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::*;
\n\n";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUTE_IMPL_HEADER: &'static [u8] = b"
unsafe extern \"C\" fn rute_object_delete_callback(data: *const c_void) {
    let d = Rc::from_raw(data as *const Cell<Option<RUBase>>);
    d.set(None);
}

pub struct Rute<'a> {
    rute_ffi: *const RuteFFI,
    priv_data: *const c_void,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Rute<'a> {
    pub fn new() -> Rute<'a> {
        Rute {
            rute_ffi: unsafe { rute_get() },
            priv_data: ::std::ptr::null(),
            _marker: PhantomData,
        }
    }
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_NO_WRAP_TEMPLATE: &str = "
    pub fn create_{{widget_snake_name}}(&self) -> {{widget_name}}<'a> {
        let ffi_data = unsafe { ((*self.rute_ffi).create_{{widget_snake_name}})(::std::ptr::null()) };
        {{widget_name}} {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data))),
            all_funcs: ffi_data.all_funcs,
            _marker: PhantomData,
        }
    }
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_STRUCT_IMPL_TEMPLATE: &str =
"#[derive(Clone)]
pub struct {{struct_name}}<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RU{{struct_name}}AllFuncs,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl <'a>{{struct_name}}<'a> {
    pub fn new_from_rc(ffi_data: RU{{struct_name}}) -> {{struct_name}}<'a> {
        {{struct_name}} {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            _marker: PhantomData,
        }
    }

    pub fn new_from_owned(ffi_data: RU{{struct_name}}) -> {{struct_name}}<'a> {
        {{struct_name}} {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            _marker: PhantomData,
        }
    }
}

";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_GET_STATIC_TEMPLATE: &str = "
    pub fn {{widget_snake_name}}(&self) -> {{widget_name}}Static<'a> {
        let ffi_data = unsafe { ((*self.rute_ffi).get_{{widget_snake_name}})(::std::ptr::null()) };
        {{widget_name}}Static {
            data: ffi_data.qt_data,
            all_funcs: ffi_data.all_funcs,
            _marker: PhantomData,
        }
    }
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_CREATE_TEMPLATE: &str = "
    pub fn create_{{widget_snake_name}}(&self) -> {{widget_name}}<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*self.rute_ffi).create_{{widget_snake_name}})(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback::<RU{{widget_name}}> as usize),
                Rc::into_raw(data.clone()) as *const c_void)
        };

        data.set(Some(ffi_data.qt_data));

        {{widget_name}} {
            data,
            all_funcs: ffi_data.all_funcs,
            _marker: PhantomData,
        }
    }
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_DROP_TEMPLATE: &str = "
impl<'a> Drop for {{type_name}}<'a> {
    fn drop(&mut self) {
        if Rc::strong_count(&self.data) == 1 {
            let obj = self.data.get().unwrap();
            unsafe {
                ((*(*self.all_funcs).{{type_snake_name}}_funcs).destroy)(obj);
            }

            self.data.set(None);
        }
    }
}
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_FUNC_IMPL_TEMPLATE: &str = "
    fn {{func_name}}{{function_def}} {
{{ body_setup }}
        let (obj_data, funcs) = self.get_{{obj_funcs_name}}_obj_funcs();
    {%- if return_value %}
        unsafe {
            let ret_val = ((*funcs).{{func_name}})({{function_args}});
        {%- if optional_return %}
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
       {%- endif %}
        {%- case return_type %}
          {%- when 'replaced' %}
           let ret_val = {{replaced_return}};
          {%- when 'pointer_ref' %}
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = {{return_vtype}}::new_from_rc(t);
            } else {
                ret_val = {{return_vtype}}::new_from_owned(t);
            }
          {%- endcase %}
        {%- if optional_return %}
            Some(ret_val)
        {%- else %}
            ret_val
        {%- endif %}
        }
    {%- else %}
        unsafe {
            ((*funcs).{{func_name}})({{function_args}});
        }
        self
    {%- endif %}
    }
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_IMPL_TRAIT_END_TEMPLATE: &str = "
    fn get_{{type_name_snake}}_obj_funcs(&self) -> (*const RUBase, *const RU{{type_name}}Funcs);
}
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_IMPL_TRAIT_TEMPLATE: &str = "
impl<'a> {{trait_name}}Type for {{target_name}}<'a> {
    fn get_{{target_name_snake}}_obj_funcs(&self) -> (*const RUBase, *const RU{{type_name}}Funcs) {
        let obj = self.data.get().unwrap();
        unsafe {
            (obj, (*self.all_funcs).{{target_name_snake_org}}_funcs)
        }
    }
}
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_CALLBACK_TEMPLATE: &str = "
    unsafe extern \"C\" fn {{event_name}}_trampoline_ud<T>(
        user_data: *const c_void,
        func: *const c_void,
        {{function_arguments}}
    ) {
        let f: &&(Fn(&T, {{function_arg_types}}) + 'static) = transmute(func);
        let data = user_data as *const T;
        f(&*data, {{function_params}});
    }

    pub fn set_{{event_name}}_event_ud<F, T>(&self, data: &'a T, func: F) -> &{{widget_name}}<'a>
    where
        F: Fn(&T, {{function_arg_types}}) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_{{widget_snake_name}}_obj_funcs();

        let f: Box<Box<Fn(&T, {{function_arg_types}}) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_{{event_name}}_event)(
                obj_data,
                user_data,
                transmute(Self::{{event_name}}_trampoline_ud::<T> as usize),
                Box::into_raw(f) as *const _,
            );
        }

        self
    }

    unsafe extern \"C\" fn {{event_name}}_trampoline(
        user_data: *const c_void,
        func: *const c_void,
        {{function_arguments}}
    ) {
        let f: &&(Fn({{function_arg_types}}) + 'static) = transmute(func);
        f({{function_params}});
    }

    pub fn set_{{event_name}}_event<F>(&self, func: F) -> &{{widget_name}}<'a>
    where
        F: Fn({{function_arg_types}}) + 'a,
    {
        let (obj_data, funcs) = self.get_{{widget_snake_name}}_obj_funcs();
        let f: Box<Box<Fn({{function_arg_types}}) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_{{event_name}}_event)(
                obj_data,
                ::std::ptr::null(),
                transmute(Self::{{event_name}}_trampoline as usize),
                Box::into_raw(f) as *const _,
            );
        }

        self
    }
";
