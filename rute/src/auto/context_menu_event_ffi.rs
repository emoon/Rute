// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::event_ffi::*;
#[allow(unused_imports)]
use auto::input_event_ffi::*;
#[allow(unused_imports)]
use auto::point_ffi::RUPoint;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUContextMenuEventFuncs {
    pub x: extern "C" fn(self_c: *const RUBase) -> i32,
    pub y: extern "C" fn(self_c: *const RUBase) -> i32,
    pub global_x: extern "C" fn(self_c: *const RUBase) -> i32,
    pub global_y: extern "C" fn(self_c: *const RUBase) -> i32,
    pub pos: extern "C" fn(self_c: *const RUBase) -> RUPoint,
    pub global_pos: extern "C" fn(self_c: *const RUBase) -> RUPoint,
    pub reason: extern "C" fn(self_c: *const RUBase) -> u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUContextMenuEventAllFuncs {
    pub event_funcs: *const RUEventFuncs,
    pub input_event_funcs: *const RUInputEventFuncs,
    pub context_menu_event_funcs: *const RUContextMenuEventFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUContextMenuEvent {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUContextMenuEventAllFuncs,
}
