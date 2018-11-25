// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::point_f_ffi::RUPointF;
#[allow(unused_imports)]
use auto::rect_ffi::RURect;
#[allow(unused_imports)]
use auto::size_f_ffi::RUSizeF;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RURectFFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub is_null: extern "C" fn(self_c: *const RUBase) -> bool,
    pub is_empty: extern "C" fn(self_c: *const RUBase) -> bool,
    pub is_valid: extern "C" fn(self_c: *const RUBase) -> bool,
    pub normalized: extern "C" fn(self_c: *const RUBase) -> RURectF,
    pub left: extern "C" fn(self_c: *const RUBase) -> f32,
    pub top: extern "C" fn(self_c: *const RUBase) -> f32,
    pub right: extern "C" fn(self_c: *const RUBase) -> f32,
    pub bottom: extern "C" fn(self_c: *const RUBase) -> f32,
    pub x: extern "C" fn(self_c: *const RUBase) -> f32,
    pub y: extern "C" fn(self_c: *const RUBase) -> f32,
    pub set_left: extern "C" fn(self_c: *const RUBase, pos: f32),
    pub set_top: extern "C" fn(self_c: *const RUBase, pos: f32),
    pub set_right: extern "C" fn(self_c: *const RUBase, pos: f32),
    pub set_bottom: extern "C" fn(self_c: *const RUBase, pos: f32),
    pub set_x: extern "C" fn(self_c: *const RUBase, pos: f32),
    pub set_y: extern "C" fn(self_c: *const RUBase, pos: f32),
    pub top_left: extern "C" fn(self_c: *const RUBase) -> RUPointF,
    pub bottom_right: extern "C" fn(self_c: *const RUBase) -> RUPointF,
    pub top_right: extern "C" fn(self_c: *const RUBase) -> RUPointF,
    pub bottom_left: extern "C" fn(self_c: *const RUBase) -> RUPointF,
    pub center: extern "C" fn(self_c: *const RUBase) -> RUPointF,
    pub set_top_left: extern "C" fn(self_c: *const RUBase, p: *const RUBase),
    pub set_bottom_right: extern "C" fn(self_c: *const RUBase, p: *const RUBase),
    pub set_top_right: extern "C" fn(self_c: *const RUBase, p: *const RUBase),
    pub set_bottom_left: extern "C" fn(self_c: *const RUBase, p: *const RUBase),
    pub move_left: extern "C" fn(self_c: *const RUBase, pos: f32),
    pub move_top: extern "C" fn(self_c: *const RUBase, pos: f32),
    pub move_right: extern "C" fn(self_c: *const RUBase, pos: f32),
    pub move_bottom: extern "C" fn(self_c: *const RUBase, pos: f32),
    pub move_top_left: extern "C" fn(self_c: *const RUBase, p: *const RUBase),
    pub move_bottom_right: extern "C" fn(self_c: *const RUBase, p: *const RUBase),
    pub move_top_right: extern "C" fn(self_c: *const RUBase, p: *const RUBase),
    pub move_bottom_left: extern "C" fn(self_c: *const RUBase, p: *const RUBase),
    pub move_center: extern "C" fn(self_c: *const RUBase, p: *const RUBase),
    pub move_to: extern "C" fn(self_c: *const RUBase, x: f32, y: f32),
    pub move_to_2: extern "C" fn(self_c: *const RUBase, p: *const RUBase),
    pub set_rect: extern "C" fn(self_c: *const RUBase, x: f32, y: f32, w: f32, h: f32),
    pub set_coords: extern "C" fn(self_c: *const RUBase, x1: f32, y1: f32, x2: f32, y2: f32),
    pub adjust: extern "C" fn(self_c: *const RUBase, x1: f32, y1: f32, x2: f32, y2: f32),
    pub adjusted:
        extern "C" fn(self_c: *const RUBase, x1: f32, y1: f32, x2: f32, y2: f32) -> RURectF,
    pub size: extern "C" fn(self_c: *const RUBase) -> RUSizeF,
    pub width: extern "C" fn(self_c: *const RUBase) -> f32,
    pub height: extern "C" fn(self_c: *const RUBase) -> f32,
    pub set_width: extern "C" fn(self_c: *const RUBase, w: f32),
    pub set_height: extern "C" fn(self_c: *const RUBase, h: f32),
    pub set_size: extern "C" fn(self_c: *const RUBase, s: *const RUBase),
    pub contains: extern "C" fn(self_c: *const RUBase, r: *const RUBase) -> bool,
    pub contains_2: extern "C" fn(self_c: *const RUBase, p: *const RUBase) -> bool,
    pub contains_3: extern "C" fn(self_c: *const RUBase, x: f32, y: f32) -> bool,
    pub united: extern "C" fn(self_c: *const RUBase, other: *const RUBase) -> RURectF,
    pub intersected: extern "C" fn(self_c: *const RUBase, other: *const RUBase) -> RURectF,
    pub intersects: extern "C" fn(self_c: *const RUBase, r: *const RUBase) -> bool,
    pub to_rect: extern "C" fn(self_c: *const RUBase) -> RURect,
    pub to_aligned_rect: extern "C" fn(self_c: *const RUBase) -> RURect,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RURectFAllFuncs {
    pub rect_f_funcs: *const RURectFFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RURectF {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RURectFAllFuncs,
}
