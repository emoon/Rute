
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
pub struct Font<'a> {
    data: Rc<Cell<Option<RUFont>>>,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

pub trait FontType {

    pub fn set_pixel_size(&self, size: i32) -> &Self {

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

    fn get_font_obj_funcs(&self) -> (*const RUBase, *const RUFontFuncs);
}

impl<'a> FontType for Font<'a> {
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
