pub static HEADER: &'static [u8] = b"
// ***************************************************************
// AUTO-GENERATED! DO NOT EDIT!
// ***************************************************************

use ffi_gen::*;
use std::slice;
pub use ffi_gen::PUBase as PUBase;\n\n
use std::ffi::CString;\n\n";

pub static UI_HEADER: &'static [u8] = b"pub struct Ui {
    pu: *const PU
}

impl Ui {
    pub fn new(pu: *const PU) -> Ui { Ui { pu: pu } }
\n";

///
/// Method for destroying an object
///

pub static DESTROY_TEMPLATE : &'static [u8] =
b"    pub fn destroy(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*obj.funcs).destroy)(obj.privd);
          self.obj = None;
       }
    }
";

///
///
///

pub static CALLBACK_TEMPLATE: &str = "
#[macro_export]
macro_rules! set_{{name}}_event {
  ($sender:expr, $data:expr, $call_type:ident, $callback:path) => {
    {
      extern \"C\" fn temp_call({{ ffi_args }}) {
          unsafe {
              let app = self_c as *mut $call_type;
              $callback({{ rust_args }});
          }
      }
      fn get_data_ptr(val: &$call_type) -> *const c_void {
         let t: *const c_void = unsafe { ::std::mem::transmute(val) };
         t
      }

      unsafe {
          let obj = $sender.obj.unwrap();
         ((*obj.funcs).set_{{name}}_event)(obj.privd, get_data_ptr($data), temp_call);
      }
    }
} }

";

pub static EVENT_TEMPLATE: &str = "
#[macro_export]
macro_rules! set_{{name}}_event {
  ($sender:expr, $data:expr, $call_type:ident, $callback:path) => {
    {
      extern \"C\" fn temp_call(self_c: *const ::std::os::raw::c_void, event: *const wrui::wrui::PUBase) {
          unsafe {
              let app = self_c as *mut $call_type;
              let event = {{event_type}}Event { obj: Some(*(event as *const wrui::ffi_gen::PU{{event_type}}Event)) };
              $callback(&mut *app, &event);
          }
      }
      fn get_data_ptr(val: &$call_type) -> *const c_void {
         let t: *const c_void = unsafe { ::std::mem::transmute(val) };
         t
      }

      unsafe {
          let obj = $sender.obj.unwrap();
         ((*obj.funcs).set_{{name}}_event)(obj.privd, get_data_ptr($data), temp_call);
      }
    }
} }

";

pub static RUST_FUNC_IMPL_TEMPLATE: &str = "
    pub fn {{func_name}} {{ function_def }} {
        {{ body_setup }}
        unsafe {
            let obj = self.obj.unwrap();
        {% if return_value %}
            let ret_val = ((*obj.funcs).{{func_name}})({{function_args}});
          {% case return_type %}
          {% when 'replaced' %}
           {{replaced_return}}
          {% when 'optional' %}
            if ret_val.privd.is_null() {
                None
            } else {
                Some({{rust_return_type}} { obj: Some(ret_val) })
            }
          {% when 'array' %}
            if ret_val.count == 0 {
                Vec::new()
            } else {
                let mut data = Vec::with_capacity(ret_val.count as usize);
                let slice = slice::from_raw_parts(ret_val.elements as *const PU{{rust_return_type}}, ret_val.count as usize);

                for item in slice {
                    data.push({{rust_return_type}} { obj: Some(*item) });
                }

                data
            }
          {% else %}
            ret_val
          {% endcase %}
        {% else %}
            ((*obj.funcs).{{func_name}})({{function_args}});
        {% endif %}
        }
    }
";




