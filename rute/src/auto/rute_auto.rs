
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
    pub fn set_style(&self, style: &str) -> &Application<'a> {
        let str_in_style_1 = CString::new(style).unwrap();

        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            ((*funcs).set_style)(obj_data, str_in_style_1.as_ptr());
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

    unsafe extern "C" fn about_to_quit_trampoline<T>(
        user_data: *const c_void,
        func: *const c_void
        
    ) {
        let f: &&(Fn(&T) + 'static) = transmute(func);
        let data = user_data as *const T;
        f(&*data);
    }

    pub fn about_to_quit<F, T>(&self, data: &'a T, func: F) -> &Application<'a>
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
                transmute(Self::about_to_quit_trampoline::<T> as usize),
                Box::into_raw(f) as *const _,
            );
        }

        self
    }

    pub fn beep(&self) -> &Application<'a> {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            ((*funcs).beep)(obj_data);
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
    pub fn exec(&self) -> i32 {
        
        let (obj_data, funcs) = self.get_application_static_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).exec)(obj_data);
        
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

    pub fn text(&self) -> String {
        
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

    unsafe extern "C" fn current_row_changed_trampoline<T>(
        user_data: *const c_void,
        func: *const c_void
        , row: i32
    ) {
        let f: &&(Fn(&T, i32) + 'static) = transmute(func);
        let data = user_data as *const T;
        f(&*data, row);
    }

    pub fn current_row_changed<F, T>(&self, data: &'a T, func: F) -> &ListWidget<'a>
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
                transmute(Self::current_row_changed_trampoline::<T> as usize),
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
