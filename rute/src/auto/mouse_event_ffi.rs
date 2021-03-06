// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::event_ffi::*;
#[allow(unused_imports)]
use auto::input_event_ffi::*;
#[allow(unused_imports)]
use auto::point_f_ffi::RUPointF;
#[allow(unused_imports)]
use auto::point_ffi::RUPoint;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUMouseEventFuncs {
    pub pos: extern "C" fn(self_c: *const RUBase) -> RUPoint,
    pub global_pos: extern "C" fn(self_c: *const RUBase) -> RUPoint,
    pub x: extern "C" fn(self_c: *const RUBase) -> i32,
    pub y: extern "C" fn(self_c: *const RUBase) -> i32,
    pub global_x: extern "C" fn(self_c: *const RUBase) -> i32,
    pub global_y: extern "C" fn(self_c: *const RUBase) -> i32,
    pub local_pos: extern "C" fn(self_c: *const RUBase) -> RUPointF,
    pub window_pos: extern "C" fn(self_c: *const RUBase) -> RUPointF,
    pub screen_pos: extern "C" fn(self_c: *const RUBase) -> RUPointF,
    pub button: extern "C" fn(self_c: *const RUBase) -> u32,
    pub buttons: extern "C" fn(self_c: *const RUBase) -> u32,
    pub set_local_pos: extern "C" fn(self_c: *const RUBase, local_position: *const RUBase),
    pub source: extern "C" fn(self_c: *const RUBase) -> u32,
    pub flags: extern "C" fn(self_c: *const RUBase) -> u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUMouseEventAllFuncs {
    pub event_funcs: *const RUEventFuncs,
    pub input_event_funcs: *const RUInputEventFuncs,
    pub mouse_event_funcs: *const RUMouseEventFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUMouseEvent {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUMouseEventAllFuncs,
}
