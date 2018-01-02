
// ***************************************************************
// AUTO-GENERATED! DO NOT EDIT!
// ***************************************************************

use ffi_gen::*;
use std::ffi::CString;

pub use ffi_gen::PURect as Rect;

#[derive(Clone, Debug)]
pub struct Widget {
    pub obj: Option<*const PUWidget>,
}

#[derive(Clone, Debug)]
pub struct PushButton {
    pub obj: Option<*const PUPushButton>,
}

#[derive(Clone, Debug)]
pub struct Painter {
    pub obj: Option<*const PUPainter>,
}

#[derive(Clone, Debug)]
pub struct ListWidgetItem {
    pub obj: Option<*const PUListWidgetItem>,
}

#[derive(Clone, Debug)]
pub struct ListWidget {
    pub obj: Option<*const PUListWidget>,
}

#[derive(Clone, Debug)]
pub struct Slider {
    pub obj: Option<*const PUSlider>,
}

#[derive(Clone, Debug)]
pub struct MainWindow {
    pub obj: Option<*const PUMainWindow>,
}

#[derive(Clone, Debug)]
pub struct Application {
    pub obj: Option<*const PUApplication>,
}

#[derive(Clone, Debug)]
pub struct PaintEvent {
    pub obj: Option<*const PUPaintEvent>,
}

pub trait PaintDevice {
    fn get_obj(&self) -> *const ::std::os::raw::c_void;
}

pub trait WidgetType {
    fn get_obj(&self) -> *const ::std::os::raw::c_void;
}

impl Widget {
    pub fn show(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).show)((*obj).privd)
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).resize)((*obj).privd, width, height)
        }
    }

}

impl Drop for Widget {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*(*obj).funcs).destroy)(obj as *const ::std::os::raw::c_void);
          self.obj = None;
       }
    }
}

impl PaintDevice for Widget {
    fn get_obj(&self) -> *const ::std::os::raw::c_void {
       unsafe {
           let obj = self.obj.unwrap();
           (*obj).privd as *const ::std::os::raw::c_void
       }
    }
}

impl WidgetType for Widget {
    fn get_obj(&self) -> *const ::std::os::raw::c_void {
       unsafe {
           let obj = self.obj.unwrap();
           (*obj).privd as *const ::std::os::raw::c_void
       }
    }
}

impl PushButton {
    pub fn show(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).show)((*obj).privd)
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).resize)((*obj).privd, width, height)
        }
    }

    pub fn set_text(&self, text: &str) {
        let str_in_text_1 = CString::new(text).unwrap();
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).set_text)((*obj).privd, str_in_text_1.as_ptr())
        }
    }

    pub fn set_flat(&self, flat: bool) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).set_flat)((*obj).privd, flat)
        }
    }

}

impl Drop for PushButton {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*(*obj).funcs).destroy)(obj as *const ::std::os::raw::c_void);
          self.obj = None;
       }
    }
}

impl PaintDevice for PushButton {
    fn get_obj(&self) -> *const ::std::os::raw::c_void {
       unsafe {
           let obj = self.obj.unwrap();
           (*obj).privd as *const ::std::os::raw::c_void
       }
    }
}

impl WidgetType for PushButton {
    fn get_obj(&self) -> *const ::std::os::raw::c_void {
       unsafe {
           let obj = self.obj.unwrap();
           (*obj).privd as *const ::std::os::raw::c_void
       }
    }
}

impl Painter {
    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).draw_line)((*obj).privd, x1, y1, x2, y2)
        }
    }

}

impl Drop for Painter {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*(*obj).funcs).destroy)(obj as *const ::std::os::raw::c_void);
          self.obj = None;
       }
    }
}

impl ListWidgetItem {
    pub fn set_text(&self, text: &str) {
        let str_in_text_1 = CString::new(text).unwrap();
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).set_text)((*obj).privd, str_in_text_1.as_ptr())
        }
    }

}

impl Drop for ListWidgetItem {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*(*obj).funcs).destroy)(obj as *const ::std::os::raw::c_void);
          self.obj = None;
       }
    }
}

impl ListWidget {
    pub fn show(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).show)((*obj).privd)
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).resize)((*obj).privd, width, height)
        }
    }

    pub fn add_item(&self, text: &str) {
        let str_in_text_1 = CString::new(text).unwrap();
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).add_item)((*obj).privd, str_in_text_1.as_ptr())
        }
    }

    pub fn add_widget_item(&self, item: &ListWidgetItem) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).add_widget_item)((*obj).privd, (*item.obj.unwrap()).privd as *const PUListWidgetItem)
        }
    }

}

impl Drop for ListWidget {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*(*obj).funcs).destroy)(obj as *const ::std::os::raw::c_void);
          self.obj = None;
       }
    }
}

impl PaintDevice for ListWidget {
    fn get_obj(&self) -> *const ::std::os::raw::c_void {
       unsafe {
           let obj = self.obj.unwrap();
           (*obj).privd as *const ::std::os::raw::c_void
       }
    }
}

impl WidgetType for ListWidget {
    fn get_obj(&self) -> *const ::std::os::raw::c_void {
       unsafe {
           let obj = self.obj.unwrap();
           (*obj).privd as *const ::std::os::raw::c_void
       }
    }
}

impl Slider {
    pub fn show(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).show)((*obj).privd)
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).resize)((*obj).privd, width, height)
        }
    }

}

impl Drop for Slider {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*(*obj).funcs).destroy)(obj as *const ::std::os::raw::c_void);
          self.obj = None;
       }
    }
}

impl PaintDevice for Slider {
    fn get_obj(&self) -> *const ::std::os::raw::c_void {
       unsafe {
           let obj = self.obj.unwrap();
           (*obj).privd as *const ::std::os::raw::c_void
       }
    }
}

impl WidgetType for Slider {
    fn get_obj(&self) -> *const ::std::os::raw::c_void {
       unsafe {
           let obj = self.obj.unwrap();
           (*obj).privd as *const ::std::os::raw::c_void
       }
    }
}

impl MainWindow {
    pub fn show(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).show)((*obj).privd)
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).resize)((*obj).privd, width, height)
        }
    }

    pub fn is_animated(&self) -> bool {
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).is_animated)((*obj).privd)
        }
    }

    pub fn set_central_widget(&self, widget: &WidgetType) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).set_central_widget)((*obj).privd, widget.get_obj() as *const PUWidgetType)
        }
    }

}

impl Drop for MainWindow {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*(*obj).funcs).destroy)(obj as *const ::std::os::raw::c_void);
          self.obj = None;
       }
    }
}

impl PaintDevice for MainWindow {
    fn get_obj(&self) -> *const ::std::os::raw::c_void {
       unsafe {
           let obj = self.obj.unwrap();
           (*obj).privd as *const ::std::os::raw::c_void
       }
    }
}

impl WidgetType for MainWindow {
    fn get_obj(&self) -> *const ::std::os::raw::c_void {
       unsafe {
           let obj = self.obj.unwrap();
           (*obj).privd as *const ::std::os::raw::c_void
       }
    }
}

impl Application {
    pub fn set_style(&self, style: &str) {
        let str_in_style_1 = CString::new(style).unwrap();
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).set_style)((*obj).privd, str_in_style_1.as_ptr())
        }
    }

    pub fn exec(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).exec)((*obj).privd)
        }
    }

}

impl Drop for Application {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*(*obj).funcs).destroy)(obj as *const ::std::os::raw::c_void);
          self.obj = None;
       }
    }
}

impl PaintEvent {
    pub fn rect(&self) -> Rect {
        unsafe {
            let obj = self.obj.unwrap();
            ((*(*obj).funcs).rect)((*obj).privd)
        }
    }

}

#[macro_export]
macro_rules! set_current_row_changed_event {
  ($sender:expr, $data:expr, $call_type:ident, $callback:path) => {
    {
      extern "C" fn temp_call(self_c: *const ::std::os::raw::c_void, row: i32) {
          unsafe {
              let app = self_c as *mut $call_type;
              $callback(&mut *app, row);
          }
      }
      unsafe {
          let obj = $sender.obj.unwrap();
         ((*(*obj).funcs).set_current_row_changed_event)((*obj).privd, ::std::mem::transmute($data), temp_call);
      }
    }
}}

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
         ((*(*obj).funcs).set_released_event)((*obj).privd, ::std::mem::transmute($data), temp_call);
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
         ((*(*obj).funcs).set_value_changed_event)((*obj).privd, ::std::mem::transmute($data), temp_call);
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

    pub fn create_painter(&self) -> Painter {
        Painter { obj: Some(unsafe { ((*self.pu).create_painter)((*self.pu).privd) }) }
    }

    pub fn create_list_widget_item(&self) -> ListWidgetItem {
        ListWidgetItem { obj: Some(unsafe { ((*self.pu).create_list_widget_item)((*self.pu).privd) }) }
    }

    pub fn create_list_widget(&self) -> ListWidget {
        ListWidget { obj: Some(unsafe { ((*self.pu).create_list_widget)((*self.pu).privd) }) }
    }

    pub fn create_slider(&self) -> Slider {
        Slider { obj: Some(unsafe { ((*self.pu).create_slider)((*self.pu).privd) }) }
    }

    pub fn create_main_window(&self) -> MainWindow {
        MainWindow { obj: Some(unsafe { ((*self.pu).create_main_window)((*self.pu).privd) }) }
    }

    pub fn create_application(&self) -> Application {
        Application { obj: Some(unsafe { ((*self.pu).create_application)((*self.pu).privd) }) }
    }

}
