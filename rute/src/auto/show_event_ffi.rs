// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::event_ffi::*;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUShowEventFuncs {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUShowEventAllFuncs {
    pub event_funcs: *const RUEventFuncs,
    pub show_event_funcs: *const RUShowEventFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUShowEvent {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUShowEventAllFuncs,
}
