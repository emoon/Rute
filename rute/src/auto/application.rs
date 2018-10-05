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
use std::ffi::{CStr, CString};

use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::*;

#[derive(Clone)]
pub struct Application<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUApplicationAllFuncs,
    pub owned: bool,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Application<'a> {
    pub fn new() -> Application<'a> {
        let ffi_data = unsafe { ((*rute_ffi_get()).create_application)(::std::ptr::null()) };
        Application {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    pub fn new_from_rc(ffi_data: RUApplication) -> Application<'a> {
        Application {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    pub fn new_from_owned(ffi_data: RUApplication) -> Application<'a> {
        Application {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
}
#[derive(Clone)]
pub struct ApplicationStatic<'a> {
    pub all_funcs: *const RUApplicationAllFuncs,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

unsafe extern "C" fn application_about_to_quit_trampoline_ud<T>(
    user_data: *const c_void,
    func: *const c_void,
) {
    let f: &&(Fn(&T) + 'static) = transmute(func);
    let data = user_data as *const T;
    f(&*data);
}

unsafe extern "C" fn application_about_to_quit_trampoline(
    _user_data: *const c_void,
    func: *const c_void,
) {
    let f: &&(Fn() + 'static) = transmute(func);
    f();
}

pub trait ApplicationType<'a> {
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

    fn set_about_to_quit_event_ud<F, T>(&self, data: &'a T, func: F) -> &Self
    where
        F: Fn(&T) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_application_obj_funcs();

        let f: Box<Box<Fn(&T) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_about_to_quit_event)(
                obj_data,
                user_data,
                transmute(application_about_to_quit_trampoline_ud::<T> as usize),
                Box::into_raw(f) as *const _,
            );
        }

        self
    }

    fn set_about_to_quit_event<F>(&self, func: F) -> &Self
    where
        F: Fn() + 'a,
    {
        let (obj_data, funcs) = self.get_application_obj_funcs();
        let f: Box<Box<Fn() + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_about_to_quit_event)(
                obj_data,
                ::std::ptr::null(),
                transmute(application_about_to_quit_trampoline as usize),
                Box::into_raw(f) as *const _,
            );
        }

        self
    }

    fn get_application_obj_funcs(&self) -> (*const RUBase, *const RUApplicationFuncs);
}

impl<'a> ApplicationType<'a> for Application<'a> {
    fn get_application_obj_funcs(&self) -> (*const RUBase, *const RUApplicationFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).application_funcs) }
    }
}
pub trait ApplicationStaticType {
    fn color_spec<'a>() -> i32 {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).color_spec)(obj_data);
            ret_val
        }
    }

    fn set_color_spec<'a>(arg0: i32) {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            ((*funcs).set_color_spec)(obj_data, arg0);
        }
    }

    fn get_font<'a>() -> Font<'a> {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).get_font)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Font::new_from_rc(t);
            } else {
                ret_val = Font::new_from_owned(t);
            }
            ret_val
        }
    }

    fn active_popup_widget<'a>() -> Option<Widget<'a>> {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).active_popup_widget)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Widget::new_from_rc(t);
            } else {
                ret_val = Widget::new_from_owned(t);
            }
            Some(ret_val)
        }
    }

    fn active_modal_widget<'a>() -> Option<Widget<'a>> {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).active_modal_widget)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Widget::new_from_rc(t);
            } else {
                ret_val = Widget::new_from_owned(t);
            }
            Some(ret_val)
        }
    }

    fn focus_widget<'a>() -> Option<Widget<'a>> {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).focus_widget)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Widget::new_from_rc(t);
            } else {
                ret_val = Widget::new_from_owned(t);
            }
            Some(ret_val)
        }
    }

    fn active_window<'a>() -> Option<Widget<'a>> {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).active_window)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Widget::new_from_rc(t);
            } else {
                ret_val = Widget::new_from_owned(t);
            }
            Some(ret_val)
        }
    }

    fn set_active_window<'a, W: WidgetType<'a>>(actor: &W) {
        let (obj_actor_1, _funcs) = actor.get_widget_obj_funcs();

        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            ((*funcs).set_active_window)(obj_data, obj_actor_1);
        }
    }

    fn widget_at<'a>(x: i32, y: i32) -> Option<Widget<'a>> {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).widget_at)(obj_data, x, y);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Widget::new_from_rc(t);
            } else {
                ret_val = Widget::new_from_owned(t);
            }
            Some(ret_val)
        }
    }

    fn top_level_at<'a>(x: i32, y: i32) -> Option<Widget<'a>> {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).top_level_at)(obj_data, x, y);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Widget::new_from_rc(t);
            } else {
                ret_val = Widget::new_from_owned(t);
            }
            Some(ret_val)
        }
    }

    fn beep<'a>() {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            ((*funcs).beep)(obj_data);
        }
    }

    fn set_cursor_flash_time<'a>(arg0: i32) {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            ((*funcs).set_cursor_flash_time)(obj_data, arg0);
        }
    }

    fn cursor_flash_time<'a>() -> i32 {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).cursor_flash_time)(obj_data);
            ret_val
        }
    }

    fn set_double_click_interval<'a>(arg0: i32) {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            ((*funcs).set_double_click_interval)(obj_data, arg0);
        }
    }

    fn double_click_interval<'a>() -> i32 {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).double_click_interval)(obj_data);
            ret_val
        }
    }

    fn set_keyboard_input_interval<'a>(arg0: i32) {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            ((*funcs).set_keyboard_input_interval)(obj_data, arg0);
        }
    }

    fn keyboard_input_interval<'a>() -> i32 {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).keyboard_input_interval)(obj_data);
            ret_val
        }
    }

    fn set_wheel_scroll_lines<'a>(arg0: i32) {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            ((*funcs).set_wheel_scroll_lines)(obj_data, arg0);
        }
    }

    fn wheel_scroll_lines<'a>() -> i32 {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).wheel_scroll_lines)(obj_data);
            ret_val
        }
    }

    fn set_start_drag_time<'a>(ms: i32) {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            ((*funcs).set_start_drag_time)(obj_data, ms);
        }
    }

    fn start_drag_time<'a>() -> i32 {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).start_drag_time)(obj_data);
            ret_val
        }
    }

    fn set_start_drag_distance<'a>(l: i32) {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            ((*funcs).set_start_drag_distance)(obj_data, l);
        }
    }

    fn start_drag_distance<'a>() -> i32 {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).start_drag_distance)(obj_data);
            ret_val
        }
    }

    fn exec<'a>() -> i32 {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).exec)(obj_data);
            ret_val
        }
    }

    fn close_all_windows<'a>() {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            ((*funcs).close_all_windows)(obj_data);
        }
    }

    fn about_qt<'a>() {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_application)(::std::ptr::null()).all_funcs)
                    .application_funcs,
            )
        };
        unsafe {
            ((*funcs).about_qt)(obj_data);
        }
    }
}

impl<'a> ApplicationStaticType for Application<'a> {}

impl<'a> ApplicationStaticType for ApplicationStatic<'a> {}
