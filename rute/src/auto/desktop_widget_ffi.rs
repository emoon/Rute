// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::object_ffi::*;
#[allow(unused_imports)]
use auto::paint_device_ffi::*;
#[allow(unused_imports)]
use auto::widget_ffi::RUWidget;
#[allow(unused_imports)]
use auto::widget_ffi::*;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUDesktopWidgetFuncs {
    pub screen_number: extern "C" fn(self_c: *const RUBase, widget: *const RUBase) -> i32,
    pub is_virtual_desktop: extern "C" fn(self_c: *const RUBase) -> bool,
    pub num_screens: extern "C" fn(self_c: *const RUBase) -> i32,
    pub screen_count: extern "C" fn(self_c: *const RUBase) -> i32,
    pub primary_screen: extern "C" fn(self_c: *const RUBase) -> i32,
    pub screen_number_2: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase) -> i32,
    pub screen: extern "C" fn(self_c: *const RUBase, screen: i32) -> RUWidget,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUDesktopWidgetAllFuncs {
    pub object_funcs: *const RUObjectFuncs,
    pub paint_device_funcs: *const RUPaintDeviceFuncs,
    pub widget_funcs: *const RUWidgetFuncs,
    pub desktop_widget_funcs: *const RUDesktopWidgetFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUDesktopWidget {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUDesktopWidgetAllFuncs,
}
