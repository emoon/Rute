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
pub struct Font<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUFontAllFuncs,
    pub owned: bool,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Font<'a> {
    pub fn new() -> Font<'a> {
        let ffi_data = unsafe { ((*rute_ffi_get()).create_font)(::std::ptr::null()) };
        Font {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    pub fn new_from_rc(ffi_data: RUFont) -> Font<'a> {
        Font {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    pub fn new_from_owned(ffi_data: RUFont) -> Font<'a> {
        Font {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
}
pub trait FontType<'a> {
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

impl<'a> FontType<'a> for Font<'a> {
    fn get_font_obj_funcs(&self) -> (*const RUBase, *const RUFontFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).font_funcs) }
    }
}

impl<'a> Drop for Font<'a> {
    fn drop(&mut self) {
        if self.owned && Rc::strong_count(&self.data) == 1 {
            let obj = self.data.get().unwrap();
            unsafe {
                ((*(*self.all_funcs).font_funcs).destroy)(obj);
            }

            self.data.set(None);
        }
    }
}
