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
pub struct Screen<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUScreenAllFuncs,
    pub owned: bool,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Screen<'a> {
    pub fn new_from_rc(ffi_data: RUScreen) -> Screen<'a> {
        Screen {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    pub fn new_from_owned(ffi_data: RUScreen) -> Screen<'a> {
        Screen {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    pub fn new_from_temporary(ffi_data: RUScreen) -> Screen<'a> {
        Screen {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
}
pub trait ScreenType<'a> {
    fn name(&self) -> String {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).name)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }

    fn manufacturer(&self) -> String {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).manufacturer)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }

    fn model(&self) -> String {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).model)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }

    fn serial_number(&self) -> String {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).serial_number)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }

    fn depth(&self) -> i32 {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).depth)(obj_data);
            ret_val
        }
    }

    fn physical_dots_per_inch_x(&self) -> f32 {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).physical_dots_per_inch_x)(obj_data);
            ret_val
        }
    }

    fn physical_dots_per_inch_y(&self) -> f32 {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).physical_dots_per_inch_y)(obj_data);
            ret_val
        }
    }

    fn physical_dots_per_inch(&self) -> f32 {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).physical_dots_per_inch)(obj_data);
            ret_val
        }
    }

    fn logical_dots_per_inch_x(&self) -> f32 {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).logical_dots_per_inch_x)(obj_data);
            ret_val
        }
    }

    fn logical_dots_per_inch_y(&self) -> f32 {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).logical_dots_per_inch_y)(obj_data);
            ret_val
        }
    }

    fn logical_dots_per_inch(&self) -> f32 {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).logical_dots_per_inch)(obj_data);
            ret_val
        }
    }

    fn device_pixel_ratio(&self) -> f32 {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).device_pixel_ratio)(obj_data);
            ret_val
        }
    }

    fn available_size(&self) -> Size {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).available_size)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Size::new_from_rc(t);
            } else {
                ret_val = Size::new_from_owned(t);
            }
            ret_val
        }
    }

    fn virtual_size(&self) -> Size {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).virtual_size)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Size::new_from_rc(t);
            } else {
                ret_val = Size::new_from_owned(t);
            }
            ret_val
        }
    }

    fn available_virtual_size(&self) -> Size {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).available_virtual_size)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Size::new_from_rc(t);
            } else {
                ret_val = Size::new_from_owned(t);
            }
            ret_val
        }
    }

    fn angle_between(&self, a: ScreenOrientation, b: ScreenOrientation) -> i32 {
        let enum_a_1 = a as i32;
        let enum_b_2 = b as i32;

        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).angle_between)(obj_data, enum_a_1, enum_b_2);
            ret_val
        }
    }

    fn is_landscape(&self, orientation: ScreenOrientation) -> bool {
        let enum_orientation_1 = orientation as i32;

        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_landscape)(obj_data, enum_orientation_1);
            ret_val
        }
    }

    fn refresh_rate(&self) -> f32 {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).refresh_rate)(obj_data);
            ret_val
        }
    }

    fn get_screen_obj_funcs(&self) -> (*const RUBase, *const RUScreenFuncs);
}

impl<'a> ScreenType<'a> for Screen<'a> {
    fn get_screen_obj_funcs(&self) -> (*const RUBase, *const RUScreenFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).screen_funcs) }
    }
}
