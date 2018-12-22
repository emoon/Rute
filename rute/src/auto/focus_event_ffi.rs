// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::event_ffi::*;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUFocusEventFuncs {
    pub got_focus: extern "C" fn(self_c: *const RUBase) -> bool,
    pub lost_focus: extern "C" fn(self_c: *const RUBase) -> bool,
    pub reason: extern "C" fn(self_c: *const RUBase) -> u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUFocusEventAllFuncs {
    pub event_funcs: *const RUEventFuncs,
    pub focus_event_funcs: *const RUFocusEventFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUFocusEvent {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUFocusEventAllFuncs,
}
