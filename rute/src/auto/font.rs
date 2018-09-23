
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

#[allow(unused_imports)]
use std::ffi::{CString, CStr};

use rute_ffi_base::*;


#[allow(unused_imports)]use auto::*;
use auto::font_ffi::*;

#[derive(Clone)]
pub struct Font<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUFontAllFuncs,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

pub trait FontType {

    fn set_pixel_size(&self, size: i32) -> &Self {

        let (obj_data, funcs) = self.get_font_obj_funcs();
        unsafe {
            ((*funcs).set_pixel_size)(obj_data, size);
        }
        self
    }

    fn pixel_size(&self) -> i32 {

        let (obj_data, funcs) = self.get_font_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).pixel_size)(obj_data);
            ret_val
        }
    }

    fn get_font_obj_funcs(&self) -> (*const RUBase, *const RUFontFuncs);
}

impl<'a> FontType for Font<'a> {
    fn get_font_obj_funcs(&self) -> (*const RUBase, *const RUFontFuncs) {
        let obj = self.data.get().unwrap();
        unsafe {
            (obj, (*self.all_funcs).font_funcs)
        }
    }
}

impl<'a> Drop for Font<'a> {
    fn drop(&mut self) {
        if Rc::strong_count(&self.data) == 1 {
            let obj = self.data.get().unwrap();
            unsafe {
                ((*(*self.all_funcs).font_funcs).destroy)(obj);
            }

            self.data.set(None);
        }
    }
}
