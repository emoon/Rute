
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

impl<'a> Application<'a> {
    unsafe extern "C" fn focus_changed_trampoline_ud<T>(
        user_data: *const c_void,
        func: *const c_void,
        old: Widget, now: Widget
    ) {
        let f: &&(Fn(&T, Widget, Widget) + 'static) = transmute(func);
        let data = user_data as *const T;
        f(&*data, old, now);
    }

    pub fn set_focus_changed_event_ud<F, T>(&self, data: &'a T, func: F) -> &Application<'a>
    where
        F: Fn(&T, Widget, Widget) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_application_obj_funcs();

        let f: Box<Box<Fn(&T, Widget, Widget) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_focus_changed_event)(
                obj_data,
                user_data,
                transmute(Self::focus_changed_trampoline_ud::<T> as usize),
                Box::into_raw(f) as *const _,
            );
        }

        self
    }

    unsafe extern "C" fn focus_changed_trampoline(
        user_data: *const c_void,
        func: *const c_void,
        old: Widget, now: Widget
    ) {
        let f: &&(Fn(Widget, Widget) + 'static) = transmute(func);
        f(old, now);
    }

    pub fn set_focus_changed_event<F>(&self, func: F) -> &Application<'a>
    where
        F: Fn(Widget, Widget) + 'a,
    {
        let (obj_data, funcs) = self.get_application_obj_funcs();
        let f: Box<Box<Fn(Widget, Widget) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_focus_changed_event)(
                obj_data,
                ::std::ptr::null(),
                transmute(Self::focus_changed_trampoline as usize),
                Box::into_raw(f) as *const _,
            );
        }

        self
    }
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
        (obj.privd, obj.application_funcs)
    }
}
pub trait ApplicationStaticType {

    pub fn style(&self) -> Option<Style> {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).style)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Style {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUStyle>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = Style {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            Some(ret_val)
        }
    }

    pub fn set_style(&self, arg0: test) -> &Self {
        let enum_arg0_1 = arg0 as i32;

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).set_style)(obj_data, enum_arg0_1);
        }
        self
    }

    pub fn set_style(&self, arg0: &StyleType) -> &Self {
        let (obj_arg0_1, _funcs) = arg0.get_style_obj_funcs();

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).set_style)(obj_data, obj_arg0_1);
        }
        self
    }

    pub fn set_style(&self, arg0: &str) -> Option<Style> {
        let str_in_arg0_1 = CString::new(arg0).unwrap();

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).set_style)(obj_data, str_in_arg0_1.as_ptr());
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Style {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUStyle>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = Style {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            Some(ret_val)
        }
    }

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

    pub fn palette(&self, arg0: &WidgetType) -> Palette {
        let (obj_arg0_1, _funcs) = arg0.get_widget_obj_funcs();

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).palette)(obj_data, obj_arg0_1);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Palette {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUPalette>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = Palette {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            ret_val
        }
    }

    pub fn palette(&self, class_name: &str) -> Palette {
        let str_in_class_name_1 = CString::new(class_name).unwrap();

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).palette)(obj_data, str_in_class_name_1.as_ptr());
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Palette {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUPalette>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = Palette {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            ret_val
        }
    }

    pub fn set_palette(&self, arg0: &PaletteType, class_name: &str) -> &Self {
        let (obj_arg0_1, _funcs) = arg0.get_palette_obj_funcs();
        let str_in_class_name_2 = CString::new(class_name).unwrap();

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).set_palette)(obj_data, obj_arg0_1, str_in_class_name_2.as_ptr());
        }
        self
    }

    pub fn font(&self) -> Font {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).font)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Font {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUFont>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = Font {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            ret_val
        }
    }

    pub fn font(&self, arg0: &WidgetType) -> Font {
        let (obj_arg0_1, _funcs) = arg0.get_widget_obj_funcs();

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).font)(obj_data, obj_arg0_1);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Font {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUFont>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = Font {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            ret_val
        }
    }

    pub fn font(&self, class_name: &str) -> Font {
        let str_in_class_name_1 = CString::new(class_name).unwrap();

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).font)(obj_data, str_in_class_name_1.as_ptr());
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Font {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUFont>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = Font {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            ret_val
        }
    }

    pub fn set_font(&self, arg0: &FontType, class_name: &str) -> &Self {
        let (obj_arg0_1, _funcs) = arg0.get_font_obj_funcs();
        let str_in_class_name_2 = CString::new(class_name).unwrap();

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).set_font)(obj_data, obj_arg0_1, str_in_class_name_2.as_ptr());
        }
        self
    }

    pub fn set_window_icon(&self, icon: &IconType) -> &Self {
        let (obj_icon_1, _funcs) = icon.get_icon_obj_funcs();

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).set_window_icon)(obj_data, obj_icon_1);
        }
        self
    }

    pub fn window_icon(&self) -> Icon {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).window_icon)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Icon {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUIcon>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = Icon {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            ret_val
        }
    }

    pub fn all_widgets(&self) -> WidgetList {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).all_widgets)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = WidgetList {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUWidgetList>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = WidgetList {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            ret_val
        }
    }

    pub fn top_level_widgets(&self) -> WidgetList {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).top_level_widgets)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = WidgetList {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUWidgetList>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = WidgetList {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            ret_val
        }
    }

    pub fn desktop(&self) -> Option<DesktopWidget> {

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).desktop)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = DesktopWidget {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUDesktopWidget>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = DesktopWidget {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            Some(ret_val)
        }
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
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUWidget>>),
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
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUWidget>>),
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
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUWidget>>),
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
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUWidget>>),
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

    pub fn widget_at(&self, p: &PointType) -> Option<Widget> {
        let (obj_p_1, _funcs) = p.get_point_obj_funcs();

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).widget_at)(obj_data, obj_p_1);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Widget {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUWidget>>),
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
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUWidget>>),
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

    pub fn top_level_at(&self, p: &PointType) -> Option<Widget> {
        let (obj_p_1, _funcs) = p.get_point_obj_funcs();

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).top_level_at)(obj_data, obj_p_1);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Widget {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUWidget>>),
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
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUWidget>>),
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

    pub fn is_effect_enabled(&self, arg0: UIEffect) -> bool {
        let enum_arg0_1 = arg0 as i32;

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_effect_enabled)(obj_data, enum_arg0_1);
            ret_val
        }
    }

    pub fn set_effect_enabled(&self, arg0: UIEffect, enable: bool) -> &Self {
        let enum_arg0_1 = arg0 as i32;

        let (obj_data, funcs) = self.get_application_obj_funcs();
        unsafe {
            ((*funcs).set_effect_enabled)(obj_data, enum_arg0_1, enable);
        }
        self
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

impl<'a> ApplicationStaticType for ApplicationStatic<'a> {
    fn get_application_static_obj_funcs(&self) -> (*const RUBase, *const RUApplicationFuncs) {
        let obj = self.data.get().unwrap();
        (obj.privd, obj.application_funcs)
    }
}

impl<'a> ApplicationStaticType for Application<'a> {
    fn get_application_static_obj_funcs(&self) -> (*const RUBase, *const RUApplicationFuncs) {
        let obj = self.data.get().unwrap();
        (obj.privd, obj.application_funcs)
    }
}
