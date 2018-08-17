pub static HEADER: &'static [u8] = b"
use std::cell::Cell;
use std::marker::PhantomData;
use std::mem::transmute;
use std::os::raw::{c_void, c_char};
use std::cell::RefCell;
use std::rc::Rc;
use std::ffi::CString;
use auto::rute_auto_ffi::*;

\n\n";

pub static RUTE_IMPL_HEADER: &'static [u8] = b"
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

pub static RUST_FUNC_IMPL_TEMPLATE: &str = "
    pub fn {{func_name}}{{function_def}} {
        {{ body_setup }}
        let (obj_data, funcs) = self.get_{{obj_funcs_name}}_obj_funcs();
    {% if return_value %}
        unsafe {
            let ret_val = ((*funcs).{{func_name}})({{function_args}});
        {% case return_type %}
          {% when 'replaced' %}
           {{replaced_return}}
          {% else %}
            ret_val
          {% endcase %}
        }
    {% else %}
        unsafe {
            ((*funcs).{{func_name}})({{function_args}});
        }
        self
    {% endif %}
    }
";


