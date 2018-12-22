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
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct PaintDevice<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUPaintDeviceAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> PaintDevice<'a> {
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUPaintDevice) -> PaintDevice<'a> {
        PaintDevice {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUPaintDevice) -> PaintDevice<'a> {
        PaintDevice {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUPaintDevice) -> PaintDevice<'a> {
        PaintDevice {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    pub fn painting_active(&self) -> bool {
        let (obj_data, funcs) = self.get_paint_device_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).painting_active)(obj_data);
            ret_val
        }
    }
    pub fn paint_engine(&self) -> Option<PaintEngine> {
        let (obj_data, funcs) = self.get_paint_device_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).paint_engine)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = PaintEngine::new_from_rc(t);
            } else {
                ret_val = PaintEngine::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    pub fn width(&self) -> i32 {
        let (obj_data, funcs) = self.get_paint_device_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).width)(obj_data);
            ret_val
        }
    }
    pub fn height(&self) -> i32 {
        let (obj_data, funcs) = self.get_paint_device_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).height)(obj_data);
            ret_val
        }
    }
    pub fn logical_dpi_x(&self) -> i32 {
        let (obj_data, funcs) = self.get_paint_device_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).logical_dpi_x)(obj_data);
            ret_val
        }
    }
    pub fn logical_dpi_y(&self) -> i32 {
        let (obj_data, funcs) = self.get_paint_device_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).logical_dpi_y)(obj_data);
            ret_val
        }
    }
    pub fn physical_dpi_x(&self) -> i32 {
        let (obj_data, funcs) = self.get_paint_device_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).physical_dpi_x)(obj_data);
            ret_val
        }
    }
    pub fn physical_dpi_y(&self) -> i32 {
        let (obj_data, funcs) = self.get_paint_device_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).physical_dpi_y)(obj_data);
            ret_val
        }
    }
    pub fn device_pixel_ratio(&self) -> i32 {
        let (obj_data, funcs) = self.get_paint_device_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).device_pixel_ratio)(obj_data);
            ret_val
        }
    }
    pub fn device_pixel_ratio_f(&self) -> f32 {
        let (obj_data, funcs) = self.get_paint_device_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).device_pixel_ratio_f)(obj_data);
            ret_val
        }
    }
    pub fn color_count(&self) -> i32 {
        let (obj_data, funcs) = self.get_paint_device_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).color_count)(obj_data);
            ret_val
        }
    }
    pub fn depth(&self) -> i32 {
        let (obj_data, funcs) = self.get_paint_device_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).depth)(obj_data);
            ret_val
        }
    }
}
pub trait PaintDeviceTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_paint_device_obj_funcs(&self) -> (*const RUBase, *const RUPaintDeviceFuncs);
}

impl<'a> PaintDeviceTrait<'a> for PaintDevice<'a> {
    #[doc(hidden)]
    fn get_paint_device_obj_funcs(&self) -> (*const RUBase, *const RUPaintDeviceFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).paint_device_funcs) }
    }
}
#[repr(u32)]
pub enum PaintDeviceMetric {
    PdmWidth = 1,
    PdmHeight = 2,
    PdmWidthMm = 3,
    PdmHeightMm = 4,
    PdmNumColors = 5,
    PdmDepth = 6,
    PdmDpiX = 7,
    PdmDpiY = 8,
    PdmPhysicalDpiX = 9,
    PdmPhysicalDpiY = 10,
    PdmDevicePixelRatio = 11,
    PdmDevicePixelRatioScaled = 12,
}
