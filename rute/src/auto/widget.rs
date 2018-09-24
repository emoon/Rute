
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

#[allow(unused_imports)]
use std::ffi::{CString, CStr};

use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::*;


#[derive(Clone)]
pub struct Widget<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUWidgetAllFuncs,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl <'a>Widget<'a> {
    pub fn new_from_rc(ffi_data: RUWidget) -> Widget<'a> {
        Widget {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            _marker: PhantomData,
        }
    }

    pub fn new_from_owned(ffi_data: RUWidget) -> Widget<'a> {
        Widget {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            _marker: PhantomData,
        }
    }
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
