// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RURectFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub is_null: extern "C" fn(self_c: *const RUBase) -> bool,
    pub is_empty: extern "C" fn(self_c: *const RUBase) -> bool,
    pub is_valid: extern "C" fn(self_c: *const RUBase) -> bool,
    pub left: extern "C" fn(self_c: *const RUBase) -> i32,
    pub top: extern "C" fn(self_c: *const RUBase) -> i32,
    pub right: extern "C" fn(self_c: *const RUBase) -> i32,
    pub bottom: extern "C" fn(self_c: *const RUBase) -> i32,
    pub x: extern "C" fn(self_c: *const RUBase) -> i32,
    pub y: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_left: extern "C" fn(self_c: *const RUBase, pos: i32),
    pub set_top: extern "C" fn(self_c: *const RUBase, pos: i32),
    pub set_right: extern "C" fn(self_c: *const RUBase, pos: i32),
    pub set_bottom: extern "C" fn(self_c: *const RUBase, pos: i32),
    pub set_x: extern "C" fn(self_c: *const RUBase, x: i32),
    pub set_y: extern "C" fn(self_c: *const RUBase, y: i32),
    pub move_left: extern "C" fn(self_c: *const RUBase, pos: i32),
    pub move_top: extern "C" fn(self_c: *const RUBase, pos: i32),
    pub move_right: extern "C" fn(self_c: *const RUBase, pos: i32),
    pub move_bottom: extern "C" fn(self_c: *const RUBase, pos: i32),
    pub move_to: extern "C" fn(self_c: *const RUBase, x: i32, t: i32),
    pub width: extern "C" fn(self_c: *const RUBase) -> i32,
    pub height: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_width: extern "C" fn(self_c: *const RUBase, w: i32),
    pub set_height: extern "C" fn(self_c: *const RUBase, h: i32),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RURectAllFuncs {
    pub rect_funcs: *const RURectFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RURect {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RURectAllFuncs,
}