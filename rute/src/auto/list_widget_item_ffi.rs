// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

use auto::list_widget_ffi::RUListWidget;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUListWidgetItemFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub list_widget: extern "C" fn(self_c: *const RUBase) -> RUListWidget,
    pub set_selected: extern "C" fn(self_c: *const RUBase, select: bool),
    pub is_selected: extern "C" fn(self_c: *const RUBase) -> bool,
    pub set_hidden: extern "C" fn(self_c: *const RUBase, hide: bool),
    pub is_hidden: extern "C" fn(self_c: *const RUBase) -> bool,
    pub text: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub set_text: extern "C" fn(self_c: *const RUBase, text: *const ::std::os::raw::c_char),
    pub status_tip: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub set_status_tip:
        extern "C" fn(self_c: *const RUBase, status_tip: *const ::std::os::raw::c_char),
    pub tool_tip: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub set_tool_tip: extern "C" fn(self_c: *const RUBase, tool_tip: *const ::std::os::raw::c_char),
    pub whats_this: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub set_whats_this:
        extern "C" fn(self_c: *const RUBase, whats_this: *const ::std::os::raw::c_char),
    pub text_alignment: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_text_alignment: extern "C" fn(self_c: *const RUBase, alignment: i32),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUListWidgetItemAllFuncs {
    pub list_widget_item_funcs: *const RUListWidgetItemFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUListWidgetItem {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUListWidgetItemAllFuncs,
}
