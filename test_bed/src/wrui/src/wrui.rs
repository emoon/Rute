
// ***************************************************************
// AUTO-GENERATED! DO NOT EDIT!
// ***************************************************************

use ffi_gen::*;
pub use ffi_gen::PUBase as PUBase;


use std::ffi::CString;

pub use ffi_gen::PURect as Rect;

#[derive(Clone)]
pub struct Widget {
    pub obj: Option<PUWidget>,
}

#[derive(Clone)]
pub struct PushButton {
    pub obj: Option<PUPushButton>,
}

#[derive(Clone)]
pub struct Painter {
    pub obj: Option<PUPainter>,
}

#[derive(Clone)]
pub struct ListWidgetItem {
    pub obj: Option<PUListWidgetItem>,
}

#[derive(Clone)]
pub struct ListWidget {
    pub obj: Option<PUListWidget>,
}

#[derive(Clone)]
pub struct Slider {
    pub obj: Option<PUSlider>,
}

#[derive(Clone)]
pub struct MainWindow {
    pub obj: Option<PUMainWindow>,
}

#[derive(Clone)]
pub struct Action {
    pub obj: Option<PUAction>,
}

#[derive(Clone)]
pub struct Menu {
    pub obj: Option<PUMenu>,
}

#[derive(Clone)]
pub struct MenuBar {
    pub obj: Option<PUMenuBar>,
}

#[derive(Clone)]
pub struct Application {
    pub obj: Option<PUApplication>,
}

#[derive(Clone)]
pub struct PaintEvent {
    pub obj: Option<PUPaintEvent>,
}

#[derive(Clone)]
pub struct Layout {
    pub obj: Option<PULayout>,
}

#[derive(Clone)]
pub struct VBoxLayout {
    pub obj: Option<PUVBoxLayout>,
}

#[derive(Clone)]
pub struct HBoxLayout {
    pub obj: Option<PUHBoxLayout>,
}

pub trait LayoutType {
    fn get_obj(&self) -> *const PUBase;
}

pub trait PaintDevice {
    fn get_obj(&self) -> *const PUBase;
}

pub trait WidgetType {
    fn get_obj(&self) -> *const PUBase;
}

impl Widget {
    pub fn show(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).show)(obj.privd)
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).resize)(obj.privd, width, height)
        }
    }

    pub fn set_layout(&self, layout: &LayoutType) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).set_layout)(obj.privd, layout.get_obj() as *const PUBase)
        }
    }

}

impl Drop for Widget {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*obj.funcs).destroy)(obj.privd);
          self.obj = None;
       }
    }
}

impl PaintDevice for Widget {
    fn get_obj(&self) -> *const PUBase {
       let obj = self.obj.unwrap();
       obj.privd as *const PUBase
    }
}

impl WidgetType for Widget {
    fn get_obj(&self) -> *const PUBase {
       let obj = self.obj.unwrap();
       obj.privd as *const PUBase
    }
}

impl PushButton {
    pub fn show(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).show)(obj.privd)
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).resize)(obj.privd, width, height)
        }
    }

    pub fn set_layout(&self, layout: &LayoutType) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).set_layout)(obj.privd, layout.get_obj() as *const PUBase)
        }
    }

    pub fn set_text(&self, text: &str) {
        let str_in_text_1 = CString::new(text).unwrap();
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).set_text)(obj.privd, str_in_text_1.as_ptr())
        }
    }

    pub fn set_flat(&self, flat: bool) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).set_flat)(obj.privd, flat)
        }
    }

}

impl Drop for PushButton {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*obj.funcs).destroy)(obj.privd);
          self.obj = None;
       }
    }
}

impl PaintDevice for PushButton {
    fn get_obj(&self) -> *const PUBase {
       let obj = self.obj.unwrap();
       obj.privd as *const PUBase
    }
}

impl WidgetType for PushButton {
    fn get_obj(&self) -> *const PUBase {
       let obj = self.obj.unwrap();
       obj.privd as *const PUBase
    }
}

impl Painter {
    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).draw_line)(obj.privd, x1, y1, x2, y2)
        }
    }

}

impl Drop for Painter {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*obj.funcs).destroy)(obj.privd);
          self.obj = None;
       }
    }
}

impl ListWidgetItem {
    pub fn set_text(&self, text: &str) {
        let str_in_text_1 = CString::new(text).unwrap();
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).set_text)(obj.privd, str_in_text_1.as_ptr())
        }
    }

}

impl Drop for ListWidgetItem {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*obj.funcs).destroy)(obj.privd);
          self.obj = None;
       }
    }
}

impl ListWidget {
    pub fn show(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).show)(obj.privd)
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).resize)(obj.privd, width, height)
        }
    }

    pub fn set_layout(&self, layout: &LayoutType) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).set_layout)(obj.privd, layout.get_obj() as *const PUBase)
        }
    }

    pub fn add_item(&self, text: &str) {
        let str_in_text_1 = CString::new(text).unwrap();
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).add_item)(obj.privd, str_in_text_1.as_ptr())
        }
    }

    pub fn item(&self, index: i32) -> Option<ListWidgetItem> {
        unsafe {
            let obj = self.obj.unwrap();
            let ret_val = ((*obj.funcs).item)(obj.privd, index);
            if ret_val.privd.is_null() {
                None
            } else {
                Some(ListWidgetItem { obj: Some(ret_val) })
            }
        }
    }

    pub fn set_drag_enabled(&self, state: bool) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).set_drag_enabled)(obj.privd, state)
        }
    }

    pub fn set_drop_indicator_shown(&self, state: bool) {
        unsafe {
            let obj = self.obj.unwrap();
            let ret_val = ((*obj.funcs).set_drop_indicator_shown)(obj.privd, state);
             { obj: Some(ret_val) }
        }
    }

    pub fn set_accept_drops(&self, state: bool) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).set_accept_drops)(obj.privd, state)
        }
    }

    pub fn add_widget_item(&self, item: &ListWidgetItem) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).add_widget_item)(obj.privd, item.obj.unwrap().privd)
        }
    }

}

impl Drop for ListWidget {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*obj.funcs).destroy)(obj.privd);
          self.obj = None;
       }
    }
}

impl PaintDevice for ListWidget {
    fn get_obj(&self) -> *const PUBase {
       let obj = self.obj.unwrap();
       obj.privd as *const PUBase
    }
}

impl WidgetType for ListWidget {
    fn get_obj(&self) -> *const PUBase {
       let obj = self.obj.unwrap();
       obj.privd as *const PUBase
    }
}

impl Slider {
    pub fn show(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).show)(obj.privd)
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).resize)(obj.privd, width, height)
        }
    }

    pub fn set_layout(&self, layout: &LayoutType) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).set_layout)(obj.privd, layout.get_obj() as *const PUBase)
        }
    }

}

impl Drop for Slider {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*obj.funcs).destroy)(obj.privd);
          self.obj = None;
       }
    }
}

impl PaintDevice for Slider {
    fn get_obj(&self) -> *const PUBase {
       let obj = self.obj.unwrap();
       obj.privd as *const PUBase
    }
}

impl WidgetType for Slider {
    fn get_obj(&self) -> *const PUBase {
       let obj = self.obj.unwrap();
       obj.privd as *const PUBase
    }
}

impl MainWindow {
    pub fn show(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).show)(obj.privd)
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).resize)(obj.privd, width, height)
        }
    }

    pub fn set_layout(&self, layout: &LayoutType) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).set_layout)(obj.privd, layout.get_obj() as *const PUBase)
        }
    }

    pub fn is_animated(&self) -> bool {
        unsafe {
            let obj = self.obj.unwrap();
            let ret_val = ((*obj.funcs).is_animated)(obj.privd);
            ret_val
        }
    }

    pub fn menu_bar(&self) -> MenuBar {
        unsafe {
            let obj = self.obj.unwrap();
            let ret_val = ((*obj.funcs).menu_bar)(obj.privd);
            MenuBar { obj: Some(ret_val) }
        }
    }

    pub fn set_central_widget(&self, widget: &WidgetType) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).set_central_widget)(obj.privd, widget.get_obj() as *const PUBase)
        }
    }

}

impl Drop for MainWindow {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*obj.funcs).destroy)(obj.privd);
          self.obj = None;
       }
    }
}

impl PaintDevice for MainWindow {
    fn get_obj(&self) -> *const PUBase {
       let obj = self.obj.unwrap();
       obj.privd as *const PUBase
    }
}

impl WidgetType for MainWindow {
    fn get_obj(&self) -> *const PUBase {
       let obj = self.obj.unwrap();
       obj.privd as *const PUBase
    }
}

impl Action {
    pub fn is_enabled(&self) -> bool {
        unsafe {
            let obj = self.obj.unwrap();
            let ret_val = ((*obj.funcs).is_enabled)(obj.privd);
            ret_val
        }
    }

    pub fn set_text(&self, text: &str) {
        let str_in_text_1 = CString::new(text).unwrap();
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).set_text)(obj.privd, str_in_text_1.as_ptr())
        }
    }

}

impl Drop for Action {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*obj.funcs).destroy)(obj.privd);
          self.obj = None;
       }
    }
}

impl Menu {
    pub fn show(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).show)(obj.privd)
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).resize)(obj.privd, width, height)
        }
    }

    pub fn set_layout(&self, layout: &LayoutType) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).set_layout)(obj.privd, layout.get_obj() as *const PUBase)
        }
    }

    pub fn add_action_text(&self, text: &str) {
        let str_in_text_1 = CString::new(text).unwrap();
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).add_action_text)(obj.privd, str_in_text_1.as_ptr())
        }
    }

    pub fn add_action(&self, action: &Action) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).add_action)(obj.privd, action.obj.unwrap().privd)
        }
    }

    pub fn set_title(&self, title: &str) {
        let str_in_title_1 = CString::new(title).unwrap();
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).set_title)(obj.privd, str_in_title_1.as_ptr())
        }
    }

}

impl Drop for Menu {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*obj.funcs).destroy)(obj.privd);
          self.obj = None;
       }
    }
}

impl PaintDevice for Menu {
    fn get_obj(&self) -> *const PUBase {
       let obj = self.obj.unwrap();
       obj.privd as *const PUBase
    }
}

impl WidgetType for Menu {
    fn get_obj(&self) -> *const PUBase {
       let obj = self.obj.unwrap();
       obj.privd as *const PUBase
    }
}

impl MenuBar {
    pub fn show(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).show)(obj.privd)
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).resize)(obj.privd, width, height)
        }
    }

    pub fn set_layout(&self, layout: &LayoutType) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).set_layout)(obj.privd, layout.get_obj() as *const PUBase)
        }
    }

    pub fn add_menu(&self, menu: &Menu) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).add_menu)(obj.privd, menu.obj.unwrap().privd)
        }
    }

}

impl Drop for MenuBar {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*obj.funcs).destroy)(obj.privd);
          self.obj = None;
       }
    }
}

impl PaintDevice for MenuBar {
    fn get_obj(&self) -> *const PUBase {
       let obj = self.obj.unwrap();
       obj.privd as *const PUBase
    }
}

impl WidgetType for MenuBar {
    fn get_obj(&self) -> *const PUBase {
       let obj = self.obj.unwrap();
       obj.privd as *const PUBase
    }
}

impl Application {
    pub fn set_style(&self, style: &str) {
        let str_in_style_1 = CString::new(style).unwrap();
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).set_style)(obj.privd, str_in_style_1.as_ptr())
        }
    }

    pub fn exec(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).exec)(obj.privd)
        }
    }

}

impl Drop for Application {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*obj.funcs).destroy)(obj.privd);
          self.obj = None;
       }
    }
}

impl PaintEvent {
    pub fn rect(&self) -> Rect {
        unsafe {
            let obj = self.obj.unwrap();
            let ret_val = ((*obj.funcs).rect)(obj.privd);
            ret_val
        }
    }

}

impl Layout {
    pub fn add_widget(&self, widget: &WidgetType) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).add_widget)(obj.privd, widget.get_obj() as *const PUBase)
        }
    }

}

impl LayoutType for Layout {
    fn get_obj(&self) -> *const PUBase {
       let obj = self.obj.unwrap();
       obj.privd as *const PUBase
    }
}

impl VBoxLayout {
    pub fn add_widget(&self, widget: &WidgetType) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).add_widget)(obj.privd, widget.get_obj() as *const PUBase)
        }
    }

    pub fn update(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).update)(obj.privd)
        }
    }

}

impl Drop for VBoxLayout {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*obj.funcs).destroy)(obj.privd);
          self.obj = None;
       }
    }
}

impl LayoutType for VBoxLayout {
    fn get_obj(&self) -> *const PUBase {
       let obj = self.obj.unwrap();
       obj.privd as *const PUBase
    }
}

impl HBoxLayout {
    pub fn add_widget(&self, widget: &WidgetType) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).add_widget)(obj.privd, widget.get_obj() as *const PUBase)
        }
    }

    pub fn update(&self) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).update)(obj.privd)
        }
    }

}

impl Drop for HBoxLayout {
    fn drop(&mut self) {
       unsafe {
          let obj = self.obj.unwrap();
          ((*obj.funcs).destroy)(obj.privd);
          self.obj = None;
       }
    }
}

impl LayoutType for HBoxLayout {
    fn get_obj(&self) -> *const PUBase {
       let obj = self.obj.unwrap();
       obj.privd as *const PUBase
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
         ((*obj.funcs).set_current_row_changed_event)(obj.privd, ::std::mem::transmute($data), temp_call);
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
         ((*obj.funcs).set_released_event)(obj.privd, ::std::mem::transmute($data), temp_call);
      }
    }
}}

#[macro_export]
macro_rules! set_triggered_event {
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
         ((*obj.funcs).set_triggered_event)(obj.privd, ::std::mem::transmute($data), temp_call);
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
         ((*obj.funcs).set_value_changed_event)(obj.privd, ::std::mem::transmute($data), temp_call);
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

    pub fn create_action(&self) -> Action {
        Action { obj: Some(unsafe { ((*self.pu).create_action)((*self.pu).privd) }) }
    }

    pub fn create_menu(&self) -> Menu {
        Menu { obj: Some(unsafe { ((*self.pu).create_menu)((*self.pu).privd) }) }
    }

    pub fn create_menu_bar(&self) -> MenuBar {
        MenuBar { obj: Some(unsafe { ((*self.pu).create_menu_bar)((*self.pu).privd) }) }
    }

    pub fn create_application(&self) -> Application {
        Application { obj: Some(unsafe { ((*self.pu).create_application)((*self.pu).privd) }) }
    }

    pub fn create_v_box_layout(&self) -> VBoxLayout {
        VBoxLayout { obj: Some(unsafe { ((*self.pu).create_v_box_layout)((*self.pu).privd) }) }
    }

    pub fn create_h_box_layout(&self) -> HBoxLayout {
        HBoxLayout { obj: Some(unsafe { ((*self.pu).create_h_box_layout)((*self.pu).privd) }) }
    }

}
