
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
pub struct Widget<'a> {
    data: Rc<Cell<Option<RUWidget>>>,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

pub trait WidgetType {

    pub fn show(&self) -> &Self {

        let (obj_data, funcs) = self.get_widget_obj_funcs();
        unsafe {
            ((*funcs).show)(obj_data);
        }
        self
    }

    pub fn set_fixed_height(&self, width: i32) -> &Self {

        let (obj_data, funcs) = self.get_widget_obj_funcs();
        unsafe {
            ((*funcs).set_fixed_height)(obj_data, width);
        }
        self
    }

    pub fn set_fixed_width(&self, width: i32) -> &Self {

        let (obj_data, funcs) = self.get_widget_obj_funcs();
        unsafe {
            ((*funcs).set_fixed_width)(obj_data, width);
        }
        self
    }

    pub fn resize(&self, width: i32, height: i32) -> &Self {

        let (obj_data, funcs) = self.get_widget_obj_funcs();
        unsafe {
            ((*funcs).resize)(obj_data, width, height);
        }
        self
    }

    pub fn update(&self) -> &Self {

        let (obj_data, funcs) = self.get_widget_obj_funcs();
        unsafe {
            ((*funcs).update)(obj_data);
        }
        self
    }

    fn get_widget_obj_funcs(&self) -> (*const RUBase, *const RUWidgetFuncs);
}

impl<'a> WidgetType for Widget<'a> {
    fn get_widget_obj_funcs(&self) -> (*const RUBase, *const RUWidgetFuncs) {
        let obj = self.data.get().unwrap();
        (obj.privd, obj.widget_funcs)
    }
}
