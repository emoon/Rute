// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::drop_event_ffi::*;
#[allow(unused_imports)]
use auto::event_ffi::*;
#[allow(unused_imports)]
use auto::rect_ffi::RURect;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUDragMoveEventFuncs {
    pub answer_rect: extern "C" fn(self_c: *const RUBase) -> RURect,
    pub accept: extern "C" fn(self_c: *const RUBase),
    pub ignore: extern "C" fn(self_c: *const RUBase),
    pub accept_2: extern "C" fn(self_c: *const RUBase, r: *const RUBase),
    pub ignore_2: extern "C" fn(self_c: *const RUBase, r: *const RUBase),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUDragMoveEventAllFuncs {
    pub event_funcs: *const RUEventFuncs,
    pub drop_event_funcs: *const RUDropEventFuncs,
    pub drag_move_event_funcs: *const RUDragMoveEventFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUDragMoveEvent {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUDragMoveEventAllFuncs,
}
