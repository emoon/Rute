// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::layout_ffi::RULayout;
#[allow(unused_imports)]
use auto::size_ffi::RUSize;
#[allow(unused_imports)]
use auto::spacer_item_ffi::RUSpacerItem;
#[allow(unused_imports)]
use auto::widget_ffi::RUWidget;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RULayoutItemFuncs {
    pub size_hint: extern "C" fn(self_c: *const RUBase) -> RUSize,
    pub minimum_size: extern "C" fn(self_c: *const RUBase) -> RUSize,
    pub maximum_size: extern "C" fn(self_c: *const RUBase) -> RUSize,
    pub expanding_directions: extern "C" fn(self_c: *const RUBase) -> i32,
    pub is_empty: extern "C" fn(self_c: *const RUBase) -> bool,
    pub has_height_for_width: extern "C" fn(self_c: *const RUBase) -> bool,
    pub height_for_width: extern "C" fn(self_c: *const RUBase, arg0: i32) -> i32,
    pub minimum_height_for_width: extern "C" fn(self_c: *const RUBase, arg0: i32) -> i32,
    pub invalidate: extern "C" fn(self_c: *const RUBase),
    pub widget: extern "C" fn(self_c: *const RUBase) -> RUWidget,
    pub layout: extern "C" fn(self_c: *const RUBase) -> RULayout,
    pub spacer_item: extern "C" fn(self_c: *const RUBase) -> RUSpacerItem,
    pub alignment: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_alignment: extern "C" fn(self_c: *const RUBase, a: i32),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RULayoutItemAllFuncs {
    pub layout_item_funcs: *const RULayoutItemFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RULayoutItem {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RULayoutItemAllFuncs,
}
