// This file is auto-generated by rute_gen. DO NOT EDIT.
use std::os::raw::c_void;
use rute::auto::base_ffi::*;


#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUListWidgetFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUListWidgetAllFuncs {
    pub list_widget_funcs: *const RUListWidgetFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUListWidget {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUFontAllFuncs,
}


extern "C" {
    pub fn rute_get() -> *const RuteFFI;
}

