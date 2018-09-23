
use std::cell::Cell;
use std::marker::PhantomData;
use std::mem::transmute;
use std::os::raw::{c_void, c_char};
use std::cell::RefCell;
use std::rc::Rc;
use std::ffi::CString;
use std::ffi::CStr;
use rute_ffi_base::*;


use auto::*;
use auto::application_ffi::*;

#[derive(Clone)]
pub struct Application<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUApplicationAllFuncs,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

#[derive(Clone)]
pub struct ApplicationStatic<'a> {
    data: RUApplication,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

pub trait ApplicationType {

    fn set_style_sheet(&self, sheet: &str) -> &Self {
        let str_in_sheet_1 = CString::new(sheet).unwrap();

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).set_style_sheet)(obj_data, str_in_sheet_1.as_ptr());
        }
        self
    }

    fn set_auto_sip_enabled(&self, enabled: bool) -> &Self {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).set_auto_sip_enabled)(obj_data, enabled);
        }
        self
    }

    fn auto_sip_enabled(&self) -> bool {

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
        unsafe {
            (obj, (*self.all_funcs).application_funcs)
        }
    }
}
pub trait ApplicationStaticType {

    fn color_spec(&self) -> i32 {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).color_spec)(obj_data);
            ret_val
        }
    }

    fn set_color_spec(&self, arg0: i32) -> &Self {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
        unsafe {
            ((*funcs).set_color_spec)(obj_data, arg0);
        }
        self
    }

    fn active_popup_widget(&self) -> Option<Widget> {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
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
                    all_funcs: t.all_funcs,
                    _marker: PhantomData,
                };
            } else {
                ret_val = Widget {
                    data: Rc::new(Cell::new(Some(t.qt_data as *const RUBase))),
                    all_funcs: t.all_funcs,
                    _marker: PhantomData,
                };
            }
            Some(ret_val)
        }
    }

    fn active_modal_widget(&self) -> Option<Widget> {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
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
                    all_funcs: t.all_funcs,
                    _marker: PhantomData,
                };
            } else {
                ret_val = Widget {
                    data: Rc::new(Cell::new(Some(t.qt_data as *const RUBase))),
                    all_funcs: t.all_funcs,
                    _marker: PhantomData,
                };
            }
            Some(ret_val)
        }
    }

    fn focus_widget(&self) -> Option<Widget> {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
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
                    all_funcs: t.all_funcs,
                    _marker: PhantomData,
                };
            } else {
                ret_val = Widget {
                    data: Rc::new(Cell::new(Some(t.qt_data as *const RUBase))),
                    all_funcs: t.all_funcs,
                    _marker: PhantomData,
                };
            }
            Some(ret_val)
        }
    }

    fn active_window(&self) -> Option<Widget> {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
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
                    all_funcs: t.all_funcs,
                    _marker: PhantomData,
                };
            } else {
                ret_val = Widget {
                    data: Rc::new(Cell::new(Some(t.qt_data as *const RUBase))),
                    all_funcs: t.all_funcs,
                    _marker: PhantomData,
                };
            }
            Some(ret_val)
        }
    }

    fn widget_at(&self, x: i32, y: i32) -> Option<Widget> {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
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
                    all_funcs: t.all_funcs,
                    _marker: PhantomData,
                };
            } else {
                ret_val = Widget {
                    data: Rc::new(Cell::new(Some(t.qt_data as *const RUBase))),
                    all_funcs: t.all_funcs,
                    _marker: PhantomData,
                };
            }
            Some(ret_val)
        }
    }

    fn top_level_at(&self, x: i32, y: i32) -> Option<Widget> {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
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
                    all_funcs: t.all_funcs,
                    _marker: PhantomData,
                };
            } else {
                ret_val = Widget {
                    data: Rc::new(Cell::new(Some(t.qt_data as *const RUBase))),
                    all_funcs: t.all_funcs,
                    _marker: PhantomData,
                };
            }
            Some(ret_val)
        }
    }

    fn beep(&self) -> &Self {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
        unsafe {
            ((*funcs).beep)(obj_data);
        }
        self
    }

    fn set_cursor_flash_time(&self, arg0: i32) -> &Self {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
        unsafe {
            ((*funcs).set_cursor_flash_time)(obj_data, arg0);
        }
        self
    }

    fn cursor_flash_time(&self) -> i32 {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).cursor_flash_time)(obj_data);
            ret_val
        }
    }

    fn set_double_click_interval(&self, arg0: i32) -> &Self {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
        unsafe {
            ((*funcs).set_double_click_interval)(obj_data, arg0);
        }
        self
    }

    fn double_click_interval(&self) -> i32 {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).double_click_interval)(obj_data);
            ret_val
        }
    }

    fn set_keyboard_input_interval(&self, arg0: i32) -> &Self {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
        unsafe {
            ((*funcs).set_keyboard_input_interval)(obj_data, arg0);
        }
        self
    }

    fn keyboard_input_interval(&self) -> i32 {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).keyboard_input_interval)(obj_data);
            ret_val
        }
    }

    fn set_wheel_scroll_lines(&self, arg0: i32) -> &Self {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
        unsafe {
            ((*funcs).set_wheel_scroll_lines)(obj_data, arg0);
        }
        self
    }

    fn wheel_scroll_lines(&self) -> i32 {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).wheel_scroll_lines)(obj_data);
            ret_val
        }
    }

    fn set_start_drag_time(&self, ms: i32) -> &Self {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
        unsafe {
            ((*funcs).set_start_drag_time)(obj_data, ms);
        }
        self
    }

    fn start_drag_time(&self) -> i32 {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).start_drag_time)(obj_data);
            ret_val
        }
    }

    fn set_start_drag_distance(&self, l: i32) -> &Self {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
        unsafe {
            ((*funcs).set_start_drag_distance)(obj_data, l);
        }
        self
    }

    fn start_drag_distance(&self) -> i32 {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).start_drag_distance)(obj_data);
            ret_val
        }
    }

    fn exec(&self) -> i32 {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).exec)(obj_data);
            ret_val
        }
    }

    fn close_all_windows(&self) -> &Self {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
        unsafe {
            ((*funcs).close_all_windows)(obj_data);
        }
        self
    }

    fn about_qt(&self) -> &Self {

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
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
        unsafe {
            (obj, (*self.all_funcs).application_funcs)
        }
    }
}
