pub static HEADER: &'static [u8] = b"
// This file is auto-generated by rute_gen. DO NOT EDIT.
use std::cell::Cell;
use std::rc::Rc;

#[allow(unused_imports)]
use std::marker::PhantomData;

#[allow(unused_imports)]
use std::os::raw::c_void;

#[allow(unused_imports)]
use std::mem::transmute;

#[allow(unused_imports)]
use std::ffi::{CString, CStr};

use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::*;
\n";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUTE_IMPL_HEADER: &'static [u8] = b"

use auto::rute_ffi::*;

// This file is auto-generated by rute_gen. DO NOT EDIT.

// This is actually a pointer to *const RuteFFI but used as u64 because
// of Sync/Send compile errors (we never use this threaded anyway)
static mut G_RUTE_FFI: u64 = 0;

pub fn rute_ffi_get() -> *const RuteFFI {
    unsafe {
        if G_RUTE_FFI == 0 {
            panic!(\"Global FFI hasn't been setup. This is usually caused by Rute::new() not being called first. Otherwise report a issue at the issue tracker in the README.\");
        }

        G_RUTE_FFI as *const RuteFFI
    }
}

pub(crate) unsafe extern \"C\" fn rute_object_delete_callback(data: *const c_void) {
    let d = Rc::from_raw(data as *const Cell<Option<RUBase>>);
    d.set(None);
}

pub struct Rute;

impl Rute {
    #[cfg(not(feature = \"plugin-compatible\"))]
    pub fn new() {
        unsafe { G_RUTE_FFI = rute_static_ffi_get() as u64 };
    }
    #[cfg(feature = \"plugin-compatible\")]
    pub fn new(rute_ffi: *const RuteFFI) -> Rute {
        unsafe { G_RUTE_FFI = rute_ffi as u64 };
    }
}
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_STRUCT_IMPL_TEMPLATE: &str = "#[derive(Clone)]
pub struct {{struct_name}}<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RU{{struct_name}}AllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl <'a>{{struct_name}}<'a> {
{%- if has_new_method %}
    pub fn new() -> {{struct_name}}<'a> {
    {%- if wrap_create %}
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_{{snake_struct_name}})(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void)
        };

        data.set(Some(ffi_data.qt_data));

        {{struct_name}} {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    {%- else %}
        let ffi_data = unsafe { ((*rute_ffi_get()).create_{{snake_struct_name}})(::std::ptr::null()) };
        {{struct_name}} {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    {% endif %}
    }
{%- endif %}
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RU{{struct_name}}) -> {{struct_name}}<'a> {
        {{struct_name}} {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RU{{struct_name}}) -> {{struct_name}}<'a> {
        {{struct_name}} {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RU{{struct_name}}) -> {{struct_name}}<'a> {
        {{struct_name}} {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    {{-event_funcs}}

    pub fn build(&self) -> Self { self.clone() }
}
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_DROP_TEMPLATE: &str = "
impl<'a> Drop for {{type_name}}<'a> {
    fn drop(&mut self) {
        if self.owned && Rc::strong_count(&self.data) == 1 {
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

pub static RUST_FUNC_IMPL_TEMPLATE: &str =
"    pub fn {{func_name_header}}{{function_def}} {
{{ body_setup }}
    {%- if static_func %}
        let (obj_data, funcs) = unsafe {
            (::std::ptr::null(),
            (*((*rute_ffi_get()).get_{{obj_funcs_name}})(::std::ptr::null()).all_funcs).{{obj_funcs_name}}_funcs)
        };

    {%- else %}
        let (obj_data, funcs) = self.get_{{obj_funcs_name}}_obj_funcs();
    {%- endif %}
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
          {%- when 'primitive_array' %}
            let ret_val = PrimitiveArray::<{{return_vtype}}>::new(ret_val);
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
    {%- unless static_func %}
        self
    {%- endunless %}
    {%- endif %}
    }
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_IMPL_TRAIT_END_TEMPLATE: &str = "
    #[inline]
    #[doc(hidden)]
    fn get_{{type_name_snake}}_obj_funcs(&self) -> (*const RUBase, *const RU{{type_name}}Funcs);
}
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_IMPL_TRAIT_TEMPLATE: &str = "
impl<'a> {{trait_name}}Trait<'a> for {{target_name}}<'a> {
    #[doc(hidden)]
    fn get_{{target_name_snake}}_obj_funcs(&self) -> (*const RUBase, *const RU{{type_name}}Funcs) {
        let obj = self.data.get().unwrap();
        unsafe {
            (obj, (*self.all_funcs).{{target_name_snake_org}}_funcs)
        }
    }
}
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_CALLBACK_TRAMPOLINE_TEMPLATE: &str = "
pub(crate) unsafe extern \"C\" fn {{widget_snake_name}}_{{event_name}}_trampoline_ud<T>{{function_arguments}} {
    let f: &&(Fn(&T, {{function_arg_types}}) + 'static) = transmute(func);
    {{body_setup}}
    let data = self_c as *const T;
    f(&*data, {{function_params}});
}

#[allow(unused_variables)]
pub(crate) unsafe extern \"C\" fn {{widget_snake_name}}_{{event_name}}_trampoline{{function_arguments}} {
    let f: &&(Fn({{function_arg_types}}) + 'static) = transmute(func);
    {{body_setup}}
    f({{function_params}});
}

";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_CALLBACK_TEMPLATE: &str =
"   pub fn set_{{event_name}}_event_ud<F, T>(&self, data: &'a T, func: F) -> &Self
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
                Box::into_raw(f) as *const _,
                transmute({{widget_snake_name}}_{{event_name}}_trampoline_ud::<T> as usize),
            );
        }

        self
    }

    pub fn set_{{event_name}}_event<F>(&self, func: F) -> &Self
    where
        F: Fn({{function_arg_types}}) + 'a,
    {
        let (obj_data, funcs) = self.get_{{widget_snake_name}}_obj_funcs();
        let f: Box<Box<Fn({{function_arg_types}}) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_{{event_name}}_event)(
                obj_data,
                ::std::ptr::null(),
                Box::into_raw(f) as *const _,
                transmute({{widget_snake_name}}_{{event_name}}_trampoline as usize),
            );
        }

        self
    }
";
