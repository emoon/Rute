
// This file is auto-generated by rute_gen. DO NOT EDIT.
use std::os::raw::c_void;

#[repr(C)]
#[derive(Default, Copy, Clone, Debug)]
pub struct RUBase {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RUArray {
    pub priv_data: *const c_void,
    pub elements: *const c_void,
    pub count: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUApplicationFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub set_style: extern "C" fn(self_c: *const RUBase, style: *const ::std::os::raw::c_char),
    pub exec: extern "C" fn(self_c: *const RUBase) -> i32,
    pub font: extern "C" fn(self_c: *const RUBase) ->  RUFont,
    pub set_about_to_quit_event: extern "C" fn(object: *const RUBase, user_data: *const c_void, trampoline_func: *const c_void,
                                            callback: *const c_void),
    pub beep: extern "C" fn(self_c: *const RUBase),
    pub about_qt: extern "C" fn(self_c: *const RUBase),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUApplication {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub extension: *const RUBase,
    pub application_funcs: *const RUApplicationFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUWidgetFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub show: extern "C" fn(self_c: *const RUBase),
    pub set_fixed_height: extern "C" fn(self_c: *const RUBase, width: i32),
    pub set_fixed_width: extern "C" fn(self_c: *const RUBase, width: i32),
    pub resize: extern "C" fn(self_c: *const RUBase, width: i32, height: i32),
    pub update: extern "C" fn(self_c: *const RUBase),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUWidget {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub extension: *const RUBase,
    pub widget_funcs: *const RUWidgetFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUListWidgetItemFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub set_text: extern "C" fn(self_c: *const RUBase, text: *const ::std::os::raw::c_char),
    pub text: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUListWidgetItem {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub extension: *const RUBase,
    pub list_widget_item_funcs: *const RUListWidgetItemFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUListWidgetFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub clear: extern "C" fn(self_c: *const RUBase),
    pub current_item: extern "C" fn(self_c: *const RUBase) ->  RUListWidgetItem,
    pub current_row: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_current_row: extern "C" fn(self_c: *const RUBase, index: i32),
    pub count: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_drag_enabled: extern "C" fn(self_c: *const RUBase, state: bool),
    pub set_drop_indicator_shown: extern "C" fn(self_c: *const RUBase, state: bool),
    pub set_item_activated_event: extern "C" fn(object: *const RUBase, user_data: *const c_void, trampoline_func: *const c_void,
                                            callback: *const c_void),
    pub set_current_row_changed_event: extern "C" fn(object: *const RUBase, user_data: *const c_void, trampoline_func: *const c_void,
                                            callback: *const c_void),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUListWidget {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub extension: *const RUBase,
    pub widget_funcs: *const RUWidgetFuncs,
    pub list_widget_funcs: *const RUListWidgetFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUFontFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub set_pixel_size: extern "C" fn(self_c: *const RUBase, size: i32),
    pub pixel_size: extern "C" fn(self_c: *const RUBase) -> i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUFont {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub extension: *const RUBase,
    pub font_funcs: *const RUFontFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RuteFFI {
    pub create_application: extern "C" fn(priv_data: *const RUBase) -> RUApplication,
    pub get_application: extern "C" fn(priv_data: *const RUBase) -> RUApplication,
    pub create_widget: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void) -> RUWidget,
    pub create_list_widget_item: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void) -> RUListWidgetItem,
    pub create_list_widget: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void) -> RUListWidget,
    pub create_font: extern "C" fn(priv_data: *const RUBase) -> RUFont,
}


extern "C" {
    pub fn rute_get() -> *const RuteFFI;
}
