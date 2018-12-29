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

/// **Notice these docs are heavy WIP and not very relevent yet**
#[derive(Clone)]
pub struct Screen<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUScreenAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Screen<'a> {
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUScreen) -> Screen<'a> {
        let data =
            unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) };
        let t = Screen {
            data: data.clone(),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        };

        // this is to allow us to clone inside instead of the outside in iterators and such
        // as this is always used in that context
        ::std::mem::forget(data);

        t
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUScreen) -> Screen<'a> {
        Screen {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUScreen) -> Screen<'a> {
        Screen {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    pub fn name(&self) -> String {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).name)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }
    pub fn manufacturer(&self) -> String {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).manufacturer)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }
    pub fn model(&self) -> String {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).model)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }
    pub fn serial_number(&self) -> String {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).serial_number)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }
    pub fn depth(&self) -> i32 {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).depth)(obj_data);
            ret_val
        }
    }
    pub fn physical_dots_per_inch_x(&self) -> f32 {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).physical_dots_per_inch_x)(obj_data);
            ret_val
        }
    }
    pub fn physical_dots_per_inch_y(&self) -> f32 {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).physical_dots_per_inch_y)(obj_data);
            ret_val
        }
    }
    pub fn physical_dots_per_inch(&self) -> f32 {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).physical_dots_per_inch)(obj_data);
            ret_val
        }
    }
    pub fn logical_dots_per_inch_x(&self) -> f32 {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).logical_dots_per_inch_x)(obj_data);
            ret_val
        }
    }
    pub fn logical_dots_per_inch_y(&self) -> f32 {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).logical_dots_per_inch_y)(obj_data);
            ret_val
        }
    }
    pub fn logical_dots_per_inch(&self) -> f32 {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).logical_dots_per_inch)(obj_data);
            ret_val
        }
    }
    pub fn device_pixel_ratio(&self) -> f32 {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).device_pixel_ratio)(obj_data);
            ret_val
        }
    }
    pub fn available_size(&self) -> Size {
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
    pub fn virtual_size(&self) -> Size {
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
    pub fn available_virtual_size(&self) -> Size {
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
    pub fn primary_orientation(&self) -> ScreenOrientation {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).primary_orientation)(obj_data);
            let ret_val = ScreenOrientation::from_bits_truncate(ret_val);
            ret_val
        }
    }
    pub fn angle_between(&self, a: ScreenOrientation, b: ScreenOrientation) -> i32 {
        let enum_a_1 = a.bits();
        let enum_b_2 = b.bits();

        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).angle_between)(obj_data, enum_a_1, enum_b_2);
            ret_val
        }
    }
    pub fn is_landscape(&self, orientation: ScreenOrientation) -> bool {
        let enum_orientation_1 = orientation.bits();

        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_landscape)(obj_data, enum_orientation_1);
            ret_val
        }
    }
    pub fn refresh_rate(&self) -> f32 {
        let (obj_data, funcs) = self.get_screen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).refresh_rate)(obj_data);
            ret_val
        }
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}

impl<'a> From<WrapperRcOwn> for Screen<'a> {
    fn from(t: WrapperRcOwn) -> Self {
        let mut data = RUScreen {
            qt_data: ::std::ptr::null(),
            host_data: ::std::ptr::null(),
            all_funcs: t.all_funcs as *const RUScreenAllFuncs,
        };

        if t.owned {
            data.host_data = t.data as *const RUBase;
            Screen::new_from_rc(data)
        } else {
            data.qt_data = t.data as *const RUBase;
            Screen::new_from_temporary(data)
        }
    }
}

pub trait ScreenTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_screen_obj_funcs(&self) -> (*const RUBase, *const RUScreenFuncs);
}

impl<'a> ScreenTrait<'a> for Screen<'a> {
    #[doc(hidden)]
    fn get_screen_obj_funcs(&self) -> (*const RUBase, *const RUScreenFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).screen_funcs) }
    }
}
