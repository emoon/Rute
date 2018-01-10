pub static HEADER: &'static [u8] = b"
// ***************************************************************
// AUTO-GENERATED! DO NOT EDIT!
// ***************************************************************

use ffi_gen::*;
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

