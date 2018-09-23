
use std::cell::Cell;
use std::marker::PhantomData;
use std::mem::transmute;
use std::os::raw::{c_void, c_char};
use std::cell::RefCell;
use std::rc::Rc;
use std::ffi::CString;
use std::ffi::CStr;
use auto::rute_enums::*;


#[derive(Clone)]
pub struct Application<'a> {
    data: Rc<Cell<Option<RUApplication>>>,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

#[derive(Clone)]
pub struct ApplicationStatic<'a> {
    data: RUApplication,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

pub trait ApplicationType {

    pub fn style_sheet(&self) -> &str {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).style_sheet)(obj_data);
           let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }

    pub fn set_style_sheet(&self, sheet: &str) -> &Self {
        let str_in_sheet_1 = CString::new(sheet).unwrap();

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).set_style_sheet)(obj_data, str_in_sheet_1.as_ptr());
        }
        self
    }

    pub fn set_auto_sip_enabled(&self, enabled: bool) -> &Self {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).set_auto_sip_enabled)(obj_data, enabled);
        }
        self
    }

    pub fn auto_sip_enabled(&self) -> bool {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).auto_sip_enabled)(obj_data);
            ret_val
        }
    }

    fn get_application_obj_funcs(&self) -> (*const RUBase, *const RUApplicationFuncs);
}

impl<'a> ApplicationType for Application<'a> {
    fn get_application_obj_funcs(&self) -> (*const RUBase, *const RUApplicationFuncs) {
        let obj = self.data.get().unwrap();
        (obj, self.all_funcs.application_funcs)
    }
}
pub trait ApplicationStaticType {

    pub fn color_spec(&self) -> i32 {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).color_spec)(obj_data);
            ret_val
        }
    }

    pub fn set_color_spec(&self, arg0: i32) -> &Self {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).set_color_spec)(obj_data, arg0);
        }
        self
    }

    pub fn active_popup_widget(&self) -> Option<Widget> {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).active_popup_widget)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Widget {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<*const RUBase>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = Widget {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            Some(ret_val)
        }
    }

    pub fn active_modal_widget(&self) -> Option<Widget> {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).active_modal_widget)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Widget {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<*const RUBase>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = Widget {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            Some(ret_val)
        }
    }

    pub fn focus_widget(&self) -> Option<Widget> {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).focus_widget)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Widget {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<*const RUBase>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = Widget {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            Some(ret_val)
        }
    }

    pub fn active_window(&self) -> Option<Widget> {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).active_window)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Widget {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<*const RUBase>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = Widget {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            Some(ret_val)
        }
    }

    pub fn set_active_window(&self, act: &WidgetType) -> &Self {
        let (obj_act_1, _funcs) = act.get_widget_obj_funcs();

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).set_active_window)(obj_data, obj_act_1);
        }
        self
    }

    pub fn widget_at(&self, x: i32, y: i32) -> Option<Widget> {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).widget_at)(obj_data, x, y);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Widget {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<*const RUBase>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = Widget {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            Some(ret_val)
        }
    }

    pub fn top_level_at(&self, x: i32, y: i32) -> Option<Widget> {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).top_level_at)(obj_data, x, y);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Widget {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<*const RUBase>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = Widget {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            Some(ret_val)
        }
    }

    pub fn beep(&self) -> &Self {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).beep)(obj_data);
        }
        self
    }

    pub fn alert(&self, widget: &WidgetType, duration: i32) -> &Self {
        let (obj_widget_1, _funcs) = widget.get_widget_obj_funcs();

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).alert)(obj_data, obj_widget_1, duration);
        }
        self
    }

    pub fn set_cursor_flash_time(&self, arg0: i32) -> &Self {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).set_cursor_flash_time)(obj_data, arg0);
        }
        self
    }

    pub fn cursor_flash_time(&self) -> i32 {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).cursor_flash_time)(obj_data);
            ret_val
        }
    }

    pub fn set_double_click_interval(&self, arg0: i32) -> &Self {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).set_double_click_interval)(obj_data, arg0);
        }
        self
    }

    pub fn double_click_interval(&self) -> i32 {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).double_click_interval)(obj_data);
            ret_val
        }
    }

    pub fn set_keyboard_input_interval(&self, arg0: i32) -> &Self {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).set_keyboard_input_interval)(obj_data, arg0);
        }
        self
    }

    pub fn keyboard_input_interval(&self) -> i32 {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).keyboard_input_interval)(obj_data);
            ret_val
        }
    }

    pub fn set_wheel_scroll_lines(&self, arg0: i32) -> &Self {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).set_wheel_scroll_lines)(obj_data, arg0);
        }
        self
    }

    pub fn wheel_scroll_lines(&self) -> i32 {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).wheel_scroll_lines)(obj_data);
            ret_val
        }
    }

    pub fn set_start_drag_time(&self, ms: i32) -> &Self {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).set_start_drag_time)(obj_data, ms);
        }
        self
    }

    pub fn start_drag_time(&self) -> i32 {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).start_drag_time)(obj_data);
            ret_val
        }
    }

    pub fn set_start_drag_distance(&self, l: i32) -> &Self {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).set_start_drag_distance)(obj_data, l);
        }
        self
    }

    pub fn start_drag_distance(&self) -> i32 {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).start_drag_distance)(obj_data);
            ret_val
        }
    }

    pub fn exec(&self) -> i32 {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).exec)(obj_data);
            ret_val
        }
    }

    pub fn close_all_windows(&self) -> &Self {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).close_all_windows)(obj_data);
        }
        self
    }

    pub fn about_qt(&self) -> &Self {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).about_qt)(obj_data);
        }
        self
    }

    fn get_application_static_obj_funcs(&self) -> (*const RUBase, *const RUApplicationFuncs);
}

impl<'a> ApplicationStaticType for Application<'a> {
    fn get_application_static_obj_funcs(&self) -> (*const RUBase, *const RUApplicationFuncs) {
        let obj = self.data.get().unwrap();
        (obj, self.all_funcs.application_funcs)
    }
}
