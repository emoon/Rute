
use ffi_gen::*;
use std::ffi::CString;

pub use ffi_gen::PURect as Rect;

pub struct Widget {
    pub obj: Option<*const PUWidget>,
}

pub struct PushButton {
    pub obj: Option<*const PUPushButton>,
}

pub struct Slider {
    pub obj: Option<*const PUSlider>,
}

pub struct Application {
    pub obj: Option<*const PUApplication>,
}

pub struct PaintEvent {
    pub obj: Option<*const PUPaintEvent>,
}

pub struct Painter {
    pub obj: Option<*const PUPainter>,
}

impl Widget {
    pub fn show(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj).show)((*obj).privd)
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj).resize)((*obj).privd, width, height)
        }
    }

}

impl Drop for Widget {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*obj).destroy)(obj as *const ::std::os::raw::c_void);
          self.obj = None;
       }
    }
}

impl PushButton {
    pub fn show(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj).show)((*obj).privd)
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj).resize)((*obj).privd, width, height)
        }
    }

    pub fn set_text(&self, text: &str) {
        let str_in_text_1 = CString::new(text).unwrap();
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj).set_text)((*obj).privd, str_in_text_1.as_ptr())
        }
    }

    pub fn set_flat(&self, flat: bool) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj).set_flat)((*obj).privd, flat)
        }
    }

}

impl Drop for PushButton {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*obj).destroy)(obj as *const ::std::os::raw::c_void);
          self.obj = None;
       }
    }
}

impl Slider {
    pub fn show(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj).show)((*obj).privd)
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj).resize)((*obj).privd, width, height)
        }
    }

}

impl Drop for Slider {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*obj).destroy)(obj as *const ::std::os::raw::c_void);
          self.obj = None;
       }
    }
}

impl Application {
    pub fn set_style(&self, style: &str) {
        let str_in_style_1 = CString::new(style).unwrap();
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj).set_style)((*obj).privd, str_in_style_1.as_ptr())
        }
    }

    pub fn exec(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj).exec)((*obj).privd)
        }
    }

}

impl Drop for Application {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*obj).destroy)(obj as *const ::std::os::raw::c_void);
          self.obj = None;
       }
    }
}

impl PaintEvent {
    pub fn rect(&self) -> Rect {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj).rect)((*obj).privd)
        }
    }

}

impl Painter {
    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj).draw_line)((*obj).privd, x1, y1, x2, y2)
        }
    }

}

impl Drop for Painter {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*obj).destroy)(obj as *const ::std::os::raw::c_void);
          self.obj = None;
       }
    }
}

#[macro_export]
macro_rules! set_released_event {
  ($sender:expr, $data:expr, $call_type:ident, $callback:path) => {
    {
      extern "C" fn temp_call(self_c: *const ::std::os::raw::c_void) {
          unsafe {
              let app = self_c as *mut $call_type;
              $callback(&mut *app);
          }
      }
      unsafe {
          let obj = $sender.obj.unwrap();
         ((*obj).set_released_event)((*obj).privd, ::std::mem::transmute($data), temp_call);
      }
    }
}}

#[macro_export]
macro_rules! set_value_changed_event {
  ($sender:expr, $data:expr, $call_type:ident, $callback:path) => {
    {
      extern "C" fn temp_call(self_c: *const ::std::os::raw::c_void, value: i32) {
          unsafe {
              let app = self_c as *mut $call_type;
              $callback(&mut *app, value);
          }
      }
      unsafe {
          let obj = $sender.obj.unwrap();
         ((*obj).set_value_changed_event)((*obj).privd, ::std::mem::transmute($data), temp_call);
      }
    }
}}

pub struct Ui {
    pu: *const PU
}

impl Ui {
    pub fn new(pu: *const PU) -> Ui { Ui { pu: pu } }

    pub fn create_widget(&self) -> Widget {
        Widget { obj: Some(unsafe { ((*self.pu).create_widget)((*self.pu).privd) }) }
    }

    pub fn create_push_button(&self) -> PushButton {
        PushButton { obj: Some(unsafe { ((*self.pu).create_push_button)((*self.pu).privd) }) }
    }

    pub fn create_slider(&self) -> Slider {
        Slider { obj: Some(unsafe { ((*self.pu).create_slider)((*self.pu).privd) }) }
    }

    pub fn create_application(&self) -> Application {
        Application { obj: Some(unsafe { ((*self.pu).create_application)((*self.pu).privd) }) }
    }

    pub fn create_painter(&self) -> Painter {
        Painter { obj: Some(unsafe { ((*self.pu).create_painter)((*self.pu).privd) }) }
    }

}
