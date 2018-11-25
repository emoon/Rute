// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::line_ffi::RULine;
#[allow(unused_imports)]
use auto::point_f_ffi::RUPointF;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RULineFFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub from_polar: extern "C" fn(self_c: *const RUBase, length: f32, angle: f32) -> RULineF,
    pub is_null: extern "C" fn(self_c: *const RUBase) -> bool,
    pub p1: extern "C" fn(self_c: *const RUBase) -> RUPointF,
    pub p2: extern "C" fn(self_c: *const RUBase) -> RUPointF,
    pub x1: extern "C" fn(self_c: *const RUBase) -> f32,
    pub y1: extern "C" fn(self_c: *const RUBase) -> f32,
    pub x2: extern "C" fn(self_c: *const RUBase) -> f32,
    pub y2: extern "C" fn(self_c: *const RUBase) -> f32,
    pub dx: extern "C" fn(self_c: *const RUBase) -> f32,
    pub dy: extern "C" fn(self_c: *const RUBase) -> f32,
    pub length: extern "C" fn(self_c: *const RUBase) -> f32,
    pub set_length: extern "C" fn(self_c: *const RUBase, len: f32),
    pub angle: extern "C" fn(self_c: *const RUBase) -> f32,
    pub set_angle: extern "C" fn(self_c: *const RUBase, angle: f32),
    pub angle_to: extern "C" fn(self_c: *const RUBase, l: *const RUBase) -> f32,
    pub unit_vector: extern "C" fn(self_c: *const RUBase) -> RULineF,
    pub normal_vector: extern "C" fn(self_c: *const RUBase) -> RULineF,
    pub intersect:
        extern "C" fn(self_c: *const RUBase, l: *const RUBase, intersection_point: *const RUBase)
            -> i32,
    pub angle_2: extern "C" fn(self_c: *const RUBase, l: *const RUBase) -> f32,
    pub point_at: extern "C" fn(self_c: *const RUBase, t: f32) -> RUPointF,
    pub center: extern "C" fn(self_c: *const RUBase) -> RUPointF,
    pub set_p1: extern "C" fn(self_c: *const RUBase, p1: *const RUBase),
    pub set_p2: extern "C" fn(self_c: *const RUBase, p2: *const RUBase),
    pub set_points: extern "C" fn(self_c: *const RUBase, p1: *const RUBase, p2: *const RUBase),
    pub set_line: extern "C" fn(self_c: *const RUBase, x1: f32, y1: f32, x2: f32, y2: f32),
    pub to_line: extern "C" fn(self_c: *const RUBase) -> RULine,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RULineFAllFuncs {
    pub line_f_funcs: *const RULineFFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RULineF {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RULineFAllFuncs,
}