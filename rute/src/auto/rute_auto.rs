
use std::cell::Cell;
use std::marker::PhantomData;
use std::mem::transmute;
use std::os::raw::{c_void, c_char};
use std::cell::RefCell;
use std::rc::Rc;
use std::ffi::CString;
use std::ffi::CStr;
use auto::rute_auto_ffi::*;

unsafe extern "C" fn rute_object_delete_callback<T>(data: *const c_void) {
    let d = Rc::from_raw(data as *const Cell<Option<T>>);
    d.set(None);
}


#[derive(Clone)]
pub struct Application<'a> {
    data: Rc<Cell<Option<RUApplication>>>,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

#[derive(Clone)]
pub struct Widget<'a> {
    data: Rc<Cell<Option<RUWidget>>>,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

#[derive(Clone)]
pub struct ListWidgetItem<'a> {
    data: Rc<Cell<Option<RUListWidgetItem>>>,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

#[derive(Clone)]
pub struct ListWidget<'a> {
    data: Rc<Cell<Option<RUListWidget>>>,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

#[derive(Clone)]
pub struct Font<'a> {
    data: Rc<Cell<Option<RUFont>>>,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

#[derive(Clone)]
pub struct ApplicationStatic<'a> {
    data: RUApplication,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Application<'a> {
    pub fn style(&self) -> Option<Style> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).style)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn set_style(&self, arg0: &StyleType<'a>) -> &Application<'a> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            ((*funcs).set_style)(obj_data, arg0);
        }
        self
    
    }

    pub fn set_style(&self, arg0: &str) -> Option<Style> {
        let str_in_arg0_1 = CString::new(arg0).unwrap();

        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).set_style)(obj_data, str_in_arg0_1.as_ptr());
        
            ret_val
          
        }
    
    }

    pub fn color_spec(&self) -> i32 {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).color_spec)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn set_color_spec(&self, arg0: i32) -> &Application<'a> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            ((*funcs).set_color_spec)(obj_data, arg0);
        }
        self
    
    }

    pub fn palette(&self, arg0: &WidgetType<'a>) -> Palette {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).palette)(obj_data, arg0);
        
            Palette {
                data: Rc::new(Cell::new(Some(ret_val))),
                _marker: PhantomData,
            }
          
        }
    
    }

    pub fn palette(&self, class_name: &char<'a>) -> Palette {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).palette)(obj_data, class_name);
        
            Palette {
                data: Rc::new(Cell::new(Some(ret_val))),
                _marker: PhantomData,
            }
          
        }
    
    }

    pub fn set_palette(&self, arg0: &PaletteType<'a>, class_name: &char<'a>) -> &Application<'a> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            ((*funcs).set_palette)(obj_data, arg0, class_name);
        }
        self
    
    }

    pub fn font(&self) -> Font {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).font)(obj_data);
        
            Font {
                data: Rc::new(Cell::new(Some(ret_val))),
                _marker: PhantomData,
            }
          
        }
    
    }

    pub fn font(&self, arg0: &WidgetType<'a>) -> Font {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).font)(obj_data, arg0);
        
            Font {
                data: Rc::new(Cell::new(Some(ret_val))),
                _marker: PhantomData,
            }
          
        }
    
    }

    pub fn font(&self, class_name: &char<'a>) -> Font {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).font)(obj_data, class_name);
        
            Font {
                data: Rc::new(Cell::new(Some(ret_val))),
                _marker: PhantomData,
            }
          
        }
    
    }

    pub fn set_font(&self, arg0: &FontType<'a>, class_name: &char<'a>) -> &Application<'a> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            ((*funcs).set_font)(obj_data, arg0, class_name);
        }
        self
    
    }

    pub fn set_window_icon(&self, icon: &IconType<'a>) -> &Application<'a> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            ((*funcs).set_window_icon)(obj_data, icon);
        }
        self
    
    }

    pub fn window_icon(&self) -> Icon {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).window_icon)(obj_data);
        
            Icon {
                data: Rc::new(Cell::new(Some(ret_val))),
                _marker: PhantomData,
            }
          
        }
    
    }

    pub fn all_widgets(&self) -> WidgetList {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).all_widgets)(obj_data);
        
            WidgetList {
                data: Rc::new(Cell::new(Some(ret_val))),
                _marker: PhantomData,
            }
          
        }
    
    }

    pub fn top_level_widgets(&self) -> WidgetList {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).top_level_widgets)(obj_data);
        
            WidgetList {
                data: Rc::new(Cell::new(Some(ret_val))),
                _marker: PhantomData,
            }
          
        }
    
    }

    pub fn desktop(&self) -> Option<DesktopWidget> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).desktop)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn active_popup_widget(&self) -> Option<Widget> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).active_popup_widget)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn active_modal_widget(&self) -> Option<Widget> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).active_modal_widget)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn focus_widget(&self) -> Option<Widget> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).focus_widget)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn active_window(&self) -> Option<Widget> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).active_window)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn set_active_window(&self, act: &WidgetType<'a>) -> &Application<'a> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            ((*funcs).set_active_window)(obj_data, act);
        }
        self
    
    }

    pub fn widget_at(&self, p: &PointType<'a>) -> Option<Widget> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).widget_at)(obj_data, p);
        
            ret_val
          
        }
    
    }

    pub fn widget_at(&self, x: i32, y: i32) -> Option<Widget> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).widget_at)(obj_data, x, y);
        
            ret_val
          
        }
    
    }

    pub fn top_level_at(&self, p: &PointType<'a>) -> Option<Widget> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).top_level_at)(obj_data, p);
        
            ret_val
          
        }
    
    }

    pub fn top_level_at(&self, x: i32, y: i32) -> Option<Widget> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).top_level_at)(obj_data, x, y);
        
            ret_val
          
        }
    
    }

    pub fn beep(&self) -> &Application<'a> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            ((*funcs).beep)(obj_data);
        }
        self
    
    }

    pub fn alert(&self, widget: &WidgetType<'a>, duration: i32) -> &Application<'a> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            ((*funcs).alert)(obj_data, widget, duration);
        }
        self
    
    }

    pub fn set_cursor_flash_time(&self, arg0: i32) -> &Application<'a> {
        
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

    pub fn set_double_click_interval(&self, arg0: i32) -> &Application<'a> {
        
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

    pub fn set_keyboard_input_interval(&self, arg0: i32) -> &Application<'a> {
        
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

    pub fn set_wheel_scroll_lines(&self, arg0: i32) -> &Application<'a> {
        
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

    pub fn set_start_drag_time(&self, ms: i32) -> &Application<'a> {
        
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

    pub fn set_start_drag_distance(&self, l: i32) -> &Application<'a> {
        
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

    pub fn is_effect_enabled(&self, arg0: Rute::UIEffect) -> bool {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).is_effect_enabled)(obj_data, arg0);
        
            ret_val
          
        }
    
    }

    pub fn set_effect_enabled(&self, arg0: Rute::UIEffect, enable: bool) -> &Application<'a> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            ((*funcs).set_effect_enabled)(obj_data, arg0, enable);
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

    unsafe extern "C" fn focus_changed_trampoline_ud<T>(
        user_data: *const c_void,
        func: *const c_void,
        old: WidgetType, now: WidgetType
    ) {
        let f: &&(Fn(&T, WidgetType, WidgetType) + 'static) = transmute(func);
        let data = user_data as *const T;
        f(&*data, old, now);
    }

    pub fn set_focus_changed_event_ud<F, T>(&self, data: &'a T, func: F) -> &Application<'a>
    where
        F: Fn(&T, WidgetType, WidgetType) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_application_obj_funcs();

        let f: Box<Box<Fn(&T, WidgetType, WidgetType) + 'a>> = Box::new(Box::new(func));
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
        old: WidgetType, now: WidgetType
    ) {
        let f: &&(Fn(WidgetType, WidgetType) + 'static) = transmute(func);
        f(old, now);
    }

    pub fn set_focus_changed_event<F>(&self, func: F) -> &Application<'a>
    where
        F: Fn(WidgetType, WidgetType) + 'a,
    {
        let (obj_data, funcs) = self.get_application_obj_funcs();
        let f: Box<Box<Fn(WidgetType, WidgetType) + 'a>> = Box::new(Box::new(func));

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

    pub fn style_sheet(&self) -> &str {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).style_sheet)(obj_data);
        
           CStr::from_ptr(ret_val).to_string_lossy().into_owned()
          
        }
    
    }

    pub fn set_style_sheet(&self, sheet: &str) -> &Application<'a> {
        let str_in_sheet_1 = CString::new(sheet).unwrap();

        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            ((*funcs).set_style_sheet)(obj_data, str_in_sheet_1.as_ptr());
        }
        self
    
    }

    pub fn set_auto_sip_enabled(&self, enabled: bool) -> &Application<'a> {
        
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

    pub fn close_all_windows(&self) -> &Application<'a> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            ((*funcs).close_all_windows)(obj_data);
        }
        self
    
    }

    pub fn about_qt(&self) -> &Application<'a> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            ((*funcs).about_qt)(obj_data);
        }
        self
    
    }
    fn get_application_obj_funcs(&self) -> (*const RUBase, *const RUApplicationFuncs) {
        let obj = self.data.get().unwrap();
        (obj.privd, obj.application_funcs)
    }
}
impl<'a> ApplicationStatic<'a> {
    pub fn style(&self) -> Option<Style> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).style)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn set_style(&self, arg0: &StyleType<'a>) -> &ApplicationStatic<'a> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            ((*funcs).set_style)(obj_data, arg0);
        }
        self
    
    }

    pub fn set_style(&self, arg0: &str) -> Option<Style> {
        let str_in_arg0_1 = CString::new(arg0).unwrap();

        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).set_style)(obj_data, str_in_arg0_1.as_ptr());
        
            ret_val
          
        }
    
    }

    pub fn color_spec(&self) -> i32 {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).color_spec)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn set_color_spec(&self, arg0: i32) -> &ApplicationStatic<'a> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            ((*funcs).set_color_spec)(obj_data, arg0);
        }
        self
    
    }

    pub fn palette(&self, arg0: &WidgetType<'a>) -> Palette {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).palette)(obj_data, arg0);
        
            Palette {
                data: Rc::new(Cell::new(Some(ret_val))),
                _marker: PhantomData,
            }
          
        }
    
    }

    pub fn palette(&self, class_name: &char<'a>) -> Palette {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).palette)(obj_data, class_name);
        
            Palette {
                data: Rc::new(Cell::new(Some(ret_val))),
                _marker: PhantomData,
            }
          
        }
    
    }

    pub fn set_palette(&self, arg0: &PaletteType<'a>, class_name: &char<'a>) -> &ApplicationStatic<'a> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            ((*funcs).set_palette)(obj_data, arg0, class_name);
        }
        self
    
    }

    pub fn font(&self) -> Font {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).font)(obj_data);
        
            Font {
                data: Rc::new(Cell::new(Some(ret_val))),
                _marker: PhantomData,
            }
          
        }
    
    }

    pub fn font(&self, arg0: &WidgetType<'a>) -> Font {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).font)(obj_data, arg0);
        
            Font {
                data: Rc::new(Cell::new(Some(ret_val))),
                _marker: PhantomData,
            }
          
        }
    
    }

    pub fn font(&self, class_name: &char<'a>) -> Font {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).font)(obj_data, class_name);
        
            Font {
                data: Rc::new(Cell::new(Some(ret_val))),
                _marker: PhantomData,
            }
          
        }
    
    }

    pub fn set_font(&self, arg0: &FontType<'a>, class_name: &char<'a>) -> &ApplicationStatic<'a> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            ((*funcs).set_font)(obj_data, arg0, class_name);
        }
        self
    
    }

    pub fn set_window_icon(&self, icon: &IconType<'a>) -> &ApplicationStatic<'a> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            ((*funcs).set_window_icon)(obj_data, icon);
        }
        self
    
    }

    pub fn window_icon(&self) -> Icon {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).window_icon)(obj_data);
        
            Icon {
                data: Rc::new(Cell::new(Some(ret_val))),
                _marker: PhantomData,
            }
          
        }
    
    }

    pub fn all_widgets(&self) -> WidgetList {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).all_widgets)(obj_data);
        
            WidgetList {
                data: Rc::new(Cell::new(Some(ret_val))),
                _marker: PhantomData,
            }
          
        }
    
    }

    pub fn top_level_widgets(&self) -> WidgetList {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).top_level_widgets)(obj_data);
        
            WidgetList {
                data: Rc::new(Cell::new(Some(ret_val))),
                _marker: PhantomData,
            }
          
        }
    
    }

    pub fn desktop(&self) -> Option<DesktopWidget> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).desktop)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn active_popup_widget(&self) -> Option<Widget> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).active_popup_widget)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn active_modal_widget(&self) -> Option<Widget> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).active_modal_widget)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn focus_widget(&self) -> Option<Widget> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).focus_widget)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn active_window(&self) -> Option<Widget> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).active_window)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn set_active_window(&self, act: &WidgetType<'a>) -> &ApplicationStatic<'a> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            ((*funcs).set_active_window)(obj_data, act);
        }
        self
    
    }

    pub fn widget_at(&self, p: &PointType<'a>) -> Option<Widget> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).widget_at)(obj_data, p);
        
            ret_val
          
        }
    
    }

    pub fn widget_at(&self, x: i32, y: i32) -> Option<Widget> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).widget_at)(obj_data, x, y);
        
            ret_val
          
        }
    
    }

    pub fn top_level_at(&self, p: &PointType<'a>) -> Option<Widget> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).top_level_at)(obj_data, p);
        
            ret_val
          
        }
    
    }

    pub fn top_level_at(&self, x: i32, y: i32) -> Option<Widget> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).top_level_at)(obj_data, x, y);
        
            ret_val
          
        }
    
    }

    pub fn beep(&self) -> &ApplicationStatic<'a> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            ((*funcs).beep)(obj_data);
        }
        self
    
    }

    pub fn alert(&self, widget: &WidgetType<'a>, duration: i32) -> &ApplicationStatic<'a> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            ((*funcs).alert)(obj_data, widget, duration);
        }
        self
    
    }

    pub fn set_cursor_flash_time(&self, arg0: i32) -> &ApplicationStatic<'a> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            ((*funcs).set_cursor_flash_time)(obj_data, arg0);
        }
        self
    
    }

    pub fn cursor_flash_time(&self) -> i32 {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).cursor_flash_time)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn set_double_click_interval(&self, arg0: i32) -> &ApplicationStatic<'a> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            ((*funcs).set_double_click_interval)(obj_data, arg0);
        }
        self
    
    }

    pub fn double_click_interval(&self) -> i32 {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).double_click_interval)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn set_keyboard_input_interval(&self, arg0: i32) -> &ApplicationStatic<'a> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            ((*funcs).set_keyboard_input_interval)(obj_data, arg0);
        }
        self
    
    }

    pub fn keyboard_input_interval(&self) -> i32 {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).keyboard_input_interval)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn set_wheel_scroll_lines(&self, arg0: i32) -> &ApplicationStatic<'a> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            ((*funcs).set_wheel_scroll_lines)(obj_data, arg0);
        }
        self
    
    }

    pub fn wheel_scroll_lines(&self) -> i32 {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).wheel_scroll_lines)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn set_start_drag_time(&self, ms: i32) -> &ApplicationStatic<'a> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            ((*funcs).set_start_drag_time)(obj_data, ms);
        }
        self
    
    }

    pub fn start_drag_time(&self) -> i32 {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).start_drag_time)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn set_start_drag_distance(&self, l: i32) -> &ApplicationStatic<'a> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            ((*funcs).set_start_drag_distance)(obj_data, l);
        }
        self
    
    }

    pub fn start_drag_distance(&self) -> i32 {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).start_drag_distance)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn is_effect_enabled(&self, arg0: Rute::UIEffect) -> bool {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).is_effect_enabled)(obj_data, arg0);
        
            ret_val
          
        }
    
    }

    pub fn set_effect_enabled(&self, arg0: Rute::UIEffect, enable: bool) -> &ApplicationStatic<'a> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            ((*funcs).set_effect_enabled)(obj_data, arg0, enable);
        }
        self
    
    }

    pub fn exec(&self) -> i32 {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).exec)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn close_all_windows(&self) -> &ApplicationStatic<'a> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            ((*funcs).close_all_windows)(obj_data);
        }
        self
    
    }

    pub fn about_qt(&self) -> &ApplicationStatic<'a> {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            ((*funcs).about_qt)(obj_data);
        }
        self
    
    }
    fn get_application_static_obj_funcs(&self) -> (*const RUBase, *const RUApplicationFuncs) {
        let obj = self.data;
        (obj.privd, obj.application_funcs)
    }
}
impl<'a> Widget<'a> {
    pub fn show(&self) -> &Widget<'a> {
        
        let (obj_data, funcs) = self.get_widget_obj_funcs();
    
        unsafe {
            ((*funcs).show)(obj_data);
        }
        self
    
    }

    pub fn set_fixed_height(&self, width: i32) -> &Widget<'a> {
        
        let (obj_data, funcs) = self.get_widget_obj_funcs();
    
        unsafe {
            ((*funcs).set_fixed_height)(obj_data, width);
        }
        self
    
    }

    pub fn set_fixed_width(&self, width: i32) -> &Widget<'a> {
        
        let (obj_data, funcs) = self.get_widget_obj_funcs();
    
        unsafe {
            ((*funcs).set_fixed_width)(obj_data, width);
        }
        self
    
    }

    pub fn resize(&self, width: i32, height: i32) -> &Widget<'a> {
        
        let (obj_data, funcs) = self.get_widget_obj_funcs();
    
        unsafe {
            ((*funcs).resize)(obj_data, width, height);
        }
        self
    
    }

    pub fn update(&self) -> &Widget<'a> {
        
        let (obj_data, funcs) = self.get_widget_obj_funcs();
    
        unsafe {
            ((*funcs).update)(obj_data);
        }
        self
    
    }
    fn get_widget_obj_funcs(&self) -> (*const RUBase, *const RUWidgetFuncs) {
        let obj = self.data.get().unwrap();
        (obj.privd, obj.widget_funcs)
    }
}
impl<'a> ListWidgetItem<'a> {
    pub fn set_text(&self, text: &str) -> &ListWidgetItem<'a> {
        let str_in_text_1 = CString::new(text).unwrap();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
    
        unsafe {
            ((*funcs).set_text)(obj_data, str_in_text_1.as_ptr());
        }
        self
    
    }

    pub fn text(&self) -> &str {
        
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).text)(obj_data);
        
           CStr::from_ptr(ret_val).to_string_lossy().into_owned()
          
        }
    
    }
    fn get_list_widget_item_obj_funcs(&self) -> (*const RUBase, *const RUListWidgetItemFuncs) {
        let obj = self.data.get().unwrap();
        (obj.privd, obj.list_widget_item_funcs)
    }
}
impl<'a> ListWidget<'a> {
    pub fn clear(&self) -> &ListWidget<'a> {
        
        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
    
        unsafe {
            ((*funcs).clear)(obj_data);
        }
        self
    
    }

    pub fn current_item(&self) -> Option<ListWidgetItem> {
        
        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).current_item)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn current_row(&self) -> i32 {
        
        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).current_row)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn set_current_row(&self, index: i32) -> &ListWidget<'a> {
        
        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
    
        unsafe {
            ((*funcs).set_current_row)(obj_data, index);
        }
        self
    
    }

    pub fn count(&self) -> i32 {
        
        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).count)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn set_drag_enabled(&self, state: bool) -> &ListWidget<'a> {
        
        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
    
        unsafe {
            ((*funcs).set_drag_enabled)(obj_data, state);
        }
        self
    
    }

    pub fn set_drop_indicator_shown(&self, state: bool) -> &ListWidget<'a> {
        
        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
    
        unsafe {
            ((*funcs).set_drop_indicator_shown)(obj_data, state);
        }
        self
    
    }

    unsafe extern "C" fn item_activated_trampoline_ud<T>(
        user_data: *const c_void,
        func: *const c_void,
        item: ListWidgetItem
    ) {
        let f: &&(Fn(&T, ListWidgetItem) + 'static) = transmute(func);
        let data = user_data as *const T;
        f(&*data, item);
    }

    pub fn set_item_activated_event_ud<F, T>(&self, data: &'a T, func: F) -> &ListWidget<'a>
    where
        F: Fn(&T, ListWidgetItem) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_list_widget_obj_funcs();

        let f: Box<Box<Fn(&T, ListWidgetItem) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_item_activated_event)(
                obj_data,
                user_data,
                transmute(Self::item_activated_trampoline_ud::<T> as usize),
                Box::into_raw(f) as *const _,
            );
        }

        self
    }

    unsafe extern "C" fn item_activated_trampoline(
        user_data: *const c_void,
        func: *const c_void,
        item: ListWidgetItem
    ) {
        let f: &&(Fn(ListWidgetItem) + 'static) = transmute(func);
        f(item);
    }

    pub fn set_item_activated_event<F>(&self, func: F) -> &ListWidget<'a>
    where
        F: Fn(ListWidgetItem) + 'a,
    {
        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
        let f: Box<Box<Fn(ListWidgetItem) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_item_activated_event)(
                obj_data,
                ::std::ptr::null(),
                transmute(Self::item_activated_trampoline as usize),
                Box::into_raw(f) as *const _,
            );
        }

        self
    }

    unsafe extern "C" fn current_row_changed_trampoline_ud<T>(
        user_data: *const c_void,
        func: *const c_void,
        row: i32
    ) {
        let f: &&(Fn(&T, i32) + 'static) = transmute(func);
        let data = user_data as *const T;
        f(&*data, row);
    }

    pub fn set_current_row_changed_event_ud<F, T>(&self, data: &'a T, func: F) -> &ListWidget<'a>
    where
        F: Fn(&T, i32) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_list_widget_obj_funcs();

        let f: Box<Box<Fn(&T, i32) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_current_row_changed_event)(
                obj_data,
                user_data,
                transmute(Self::current_row_changed_trampoline_ud::<T> as usize),
                Box::into_raw(f) as *const _,
            );
        }

        self
    }

    unsafe extern "C" fn current_row_changed_trampoline(
        user_data: *const c_void,
        func: *const c_void,
        row: i32
    ) {
        let f: &&(Fn(i32) + 'static) = transmute(func);
        f(row);
    }

    pub fn set_current_row_changed_event<F>(&self, func: F) -> &ListWidget<'a>
    where
        F: Fn(i32) + 'a,
    {
        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
        let f: Box<Box<Fn(i32) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_current_row_changed_event)(
                obj_data,
                ::std::ptr::null(),
                transmute(Self::current_row_changed_trampoline as usize),
                Box::into_raw(f) as *const _,
            );
        }

        self
    }
    fn get_list_widget_obj_funcs(&self) -> (*const RUBase, *const RUListWidgetFuncs) {
        let obj = self.data.get().unwrap();
        (obj.privd, obj.list_widget_funcs)
    }
}
impl<'a> Font<'a> {
    pub fn set_pixel_size(&self, size: i32) -> &Font<'a> {
        
        let (obj_data, funcs) = self.get_font_obj_funcs();
    
        unsafe {
            ((*funcs).set_pixel_size)(obj_data, size);
        }
        self
    
    }

    pub fn pixel_size(&self) -> i32 {
        
        let (obj_data, funcs) = self.get_font_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).pixel_size)(obj_data);
        
            ret_val
          
        }
    
    }
    fn get_font_obj_funcs(&self) -> (*const RUBase, *const RUFontFuncs) {
        let obj = self.data.get().unwrap();
        (obj.privd, obj.font_funcs)
    }
}

impl<'a> Drop for Font<'a> {
    fn drop(&mut self) {
        if Rc::strong_count(&self.data) == 1 {
            let obj = self.data.get().unwrap();
            unsafe {
                ((*obj.font_funcs).destroy)(obj.privd);
            }

            self.data.set(None);
        }
    }
}

pub struct Rute<'a> {
    rute_ffi: *const RuteFFI,
    priv_data: *const c_void,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Rute<'a> {
    pub fn new() -> Rute<'a> {
        Rute {
            rute_ffi: unsafe { rute_get() },
            priv_data: ::std::ptr::null(),
            _marker: PhantomData,
        }
    }

    pub fn create_application(&self) -> Application<'a> {
        let ffi_data = unsafe { ((*self.rute_ffi).create_application)(::std::ptr::null()) };
        Application {
            data: Rc::new(Cell::new(Some(ffi_data))),
            _marker: PhantomData,
        }
    }

    pub fn application(&self) -> ApplicationStatic<'a> {
        let ffi_data = unsafe { ((*self.rute_ffi).get_application)(::std::ptr::null()) };
        ApplicationStatic {
            data: ffi_data,
            _marker: PhantomData,
        }
    }

    pub fn create_widget(&self) -> Widget<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*self.rute_ffi).create_widget)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback::<RUWidget> as usize),
                Rc::into_raw(data.clone()) as *const c_void)
        };

        data.set(Some(ffi_data));

        Widget {
            data,
            _marker: PhantomData,
        }
    }

    pub fn create_list_widget_item(&self) -> ListWidgetItem<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*self.rute_ffi).create_list_widget_item)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback::<RUListWidgetItem> as usize),
                Rc::into_raw(data.clone()) as *const c_void)
        };

        data.set(Some(ffi_data));

        ListWidgetItem {
            data,
            _marker: PhantomData,
        }
    }

    pub fn create_list_widget(&self) -> ListWidget<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*self.rute_ffi).create_list_widget)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback::<RUListWidget> as usize),
                Rc::into_raw(data.clone()) as *const c_void)
        };

        data.set(Some(ffi_data));

        ListWidget {
            data,
            _marker: PhantomData,
        }
    }

    pub fn create_font(&self) -> Font<'a> {
        let ffi_data = unsafe { ((*self.rute_ffi).create_font)(::std::ptr::null()) };
        Font {
            data: Rc::new(Cell::new(Some(ffi_data))),
            _marker: PhantomData,
        }
    }
}
