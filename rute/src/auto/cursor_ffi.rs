// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::bitmap_ffi::RUBitmap;
#[allow(unused_imports)]
use auto::pixmap_ffi::RUPixmap;
#[allow(unused_imports)]
use auto::point_ffi::RUPoint;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUCursorFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub swap: extern "C" fn(self_c: *const RUBase, other: *const RUBase),
    pub shape: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_shape: extern "C" fn(self_c: *const RUBase, new_shape: i32),
    pub bitmap: extern "C" fn(self_c: *const RUBase) -> RUBitmap,
    pub mask: extern "C" fn(self_c: *const RUBase) -> RUBitmap,
    pub pixmap: extern "C" fn(self_c: *const RUBase) -> RUPixmap,
    pub hot_spot: extern "C" fn(self_c: *const RUBase) -> RUPoint,
    pub pos: extern "C" fn(self_c: *const RUBase) -> RUPoint,
    pub pos_2: extern "C" fn(self_c: *const RUBase, screen: *const RUBase) -> RUPoint,
    pub set_pos: extern "C" fn(self_c: *const RUBase, x: i32, y: i32),
    pub set_pos_2: extern "C" fn(self_c: *const RUBase, screen: *const RUBase, x: i32, y: i32),
    pub set_pos_3: extern "C" fn(self_c: *const RUBase, p: *const RUBase),
    pub set_pos_4: extern "C" fn(self_c: *const RUBase, screen: *const RUBase, p: *const RUBase),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUCursorAllFuncs {
    pub cursor_funcs: *const RUCursorFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUCursor {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUCursorAllFuncs,
}