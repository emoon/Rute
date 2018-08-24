pub static HEADER: &'static [u8] = b"
// ***************************************************************
// AUTO-GENERATED! DO NOT EDIT!
// ***************************************************************

use ffi_gen::*;
use std::ffi::CStr;
use std::slice;
pub use rust_auto_ffi::*;\n\n
use std::ffi::CString;\n\n";

pub static UI_HEADER: &'static [u8] =
b"
#[derive(Copy, Clone, Default)]
pub struct Ui {
    pu: Option<*const RU>
}

impl Ui {
    pub fn new(pu: *const RU) -> Ui { Ui { pu: Some(pu) } }

    pub fn get_c_api(&self) -> *const RU { self.pu.unwrap() }
\n";

pub static UI_FOOTER: &'static [u8] = b"    pub fn create_plugin_ui(&self, parent: &WidgetType) -> PluginUi {
        PluginUi { pu: Some(unsafe { ((*self.pu.unwrap()).create_plugin_ui)((*self.pu.unwrap()).privd, parent.get_widget_type_obj()) }) }
    }
}\n
\n";

pub static PLUGIN_UI_HEADER: &'static [u8] =
b"
#[derive(Copy, Clone, Default)]
pub struct PluginUi {
    pu: Option<*const RUPluginUI>
}

impl PluginUi {
    pub fn new(pu: *const RUPluginUI) -> PluginUi { PluginUi { pu: Some(pu) } }
    pub fn get_c_api(&self) -> *const RUPluginUI { self.pu.unwrap() }
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
macro_rules! set_{{func_name}}_event {
  ($sender:expr, $data:expr, $call_type:ident, $callback:path) => {
    {
      extern \"C\" fn temp_call({{ ffi_args }}) {
          unsafe {
              let app = self_c as *mut $call_type;
              $callback({{ rust_args }});
          }
      }
      fn get_data_ptr(val: &$call_type) -> *const ::std::os::raw::c_void {
         let t: *const ::std::os::raw::c_void = unsafe { ::std::mem::transmute(val) };
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
      extern \"C\" fn temp_call(widget: *const ::rute::rute::RUBase, self_c: *const ::std::os::raw::c_void, event: *const ::rute::rute::RUBase) {
          unsafe {
              let app = self_c as *mut $call_type;
              let w = {{widget_type}} { obj: Some(*(widget as *const ::rute::ffi_gen::RU{{widget_type}})) };
              let event = {{event_type}}Event { obj: Some(*(event as *const ::rute::ffi_gen::RU{{event_type}}Event)) };
              $callback(&mut *app, &w, &event);
          }
      }
      fn get_data_ptr(val: &$call_type) -> *const ::std::os::raw::c_void {
         let t: *const ::std::os::raw::c_void = unsafe { ::std::mem::transmute(val) };
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
                let slice = slice::from_raw_parts(ret_val.elements as *const RU{{rust_return_type}}, ret_val.count as usize);

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




