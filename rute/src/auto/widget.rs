
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::ffi::{CString, CStr};
use rute_ffi_base::*;


use auto::*;
use auto::widget_ffi::*;

#[derive(Clone)]
pub struct Widget<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUWidgetAllFuncs,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

pub trait WidgetType {

    fn show(&self) -> &Self {

        let (obj_data, funcs) = self.get_widget_obj_funcs();
        unsafe {
            ((*funcs).show)(obj_data);
        }
        self
    }

    fn set_fixed_height(&self, width: i32) -> &Self {

        let (obj_data, funcs) = self.get_widget_obj_funcs();
        unsafe {
            ((*funcs).set_fixed_height)(obj_data, width);
        }
        self
    }

    fn set_fixed_width(&self, width: i32) -> &Self {

        let (obj_data, funcs) = self.get_widget_obj_funcs();
        unsafe {
            ((*funcs).set_fixed_width)(obj_data, width);
        }
        self
    }

    fn resize(&self, width: i32, height: i32) -> &Self {

        let (obj_data, funcs) = self.get_widget_obj_funcs();
        unsafe {
            ((*funcs).resize)(obj_data, width, height);
        }
        self
    }

    fn update(&self) -> &Self {

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
        unsafe {
            (obj, (*self.all_funcs).widget_funcs)
        }
    }
}
