// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::clipboard_ffi::RUClipboard;
#[allow(unused_imports)]
use auto::core_application_ffi::*;
#[allow(unused_imports)]
use auto::cursor_ffi::RUCursor;
#[allow(unused_imports)]
use auto::font_ffi::RUFont;
#[allow(unused_imports)]
use auto::icon_ffi::RUIcon;
#[allow(unused_imports)]
use auto::object_ffi::RUObject;
#[allow(unused_imports)]
use auto::object_ffi::*;
#[allow(unused_imports)]
use auto::palette_ffi::RUPalette;
#[allow(unused_imports)]
use auto::screen_ffi::RUScreen;
#[allow(unused_imports)]
use auto::window_ffi::RUWindow;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUGuiApplicationFuncs {
    pub set_application_display_name:
        extern "C" fn(self_c: *const RUBase, name: *const ::std::os::raw::c_char),
    pub application_display_name:
        extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub set_desktop_file_name:
        extern "C" fn(self_c: *const RUBase, name: *const ::std::os::raw::c_char),
    pub desktop_file_name: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub top_level_at: extern "C" fn(self_c: *const RUBase, pos: *const RUBase) -> RUWindow,
    pub set_window_icon: extern "C" fn(self_c: *const RUBase, icon: *const RUBase),
    pub window_icon: extern "C" fn(self_c: *const RUBase) -> RUIcon,
    pub platform_name: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub modal_window: extern "C" fn(self_c: *const RUBase) -> RUWindow,
    pub focus_window: extern "C" fn(self_c: *const RUBase) -> RUWindow,
    pub focus_object: extern "C" fn(self_c: *const RUBase) -> RUObject,
    pub primary_screen: extern "C" fn(self_c: *const RUBase) -> RUScreen,
    pub screen_at: extern "C" fn(self_c: *const RUBase, point: *const RUBase) -> RUScreen,
    pub device_pixel_ratio: extern "C" fn(self_c: *const RUBase) -> f32,
    pub override_cursor: extern "C" fn(self_c: *const RUBase) -> RUCursor,
    pub set_override_cursor: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase),
    pub change_override_cursor: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase),
    pub restore_override_cursor: extern "C" fn(self_c: *const RUBase),
    pub font: extern "C" fn(self_c: *const RUBase) -> RUFont,
    pub set_font: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase),
    pub clipboard: extern "C" fn(self_c: *const RUBase) -> RUClipboard,
    pub palette: extern "C" fn(self_c: *const RUBase) -> RUPalette,
    pub set_palette: extern "C" fn(self_c: *const RUBase, pal: *const RUBase),
    pub keyboard_modifiers: extern "C" fn(self_c: *const RUBase) -> u32,
    pub query_keyboard_modifiers: extern "C" fn(self_c: *const RUBase) -> u32,
    pub mouse_buttons: extern "C" fn(self_c: *const RUBase) -> u32,
    pub set_layout_direction: extern "C" fn(self_c: *const RUBase, direction: u32),
    pub layout_direction: extern "C" fn(self_c: *const RUBase) -> u32,
    pub is_right_to_left: extern "C" fn(self_c: *const RUBase) -> bool,
    pub is_left_to_right: extern "C" fn(self_c: *const RUBase) -> bool,
    pub set_desktop_settings_aware: extern "C" fn(self_c: *const RUBase, on: bool),
    pub desktop_settings_aware: extern "C" fn(self_c: *const RUBase) -> bool,
    pub set_quit_on_last_window_closed: extern "C" fn(self_c: *const RUBase, quit: bool),
    pub quit_on_last_window_closed: extern "C" fn(self_c: *const RUBase) -> bool,
    pub application_state: extern "C" fn(self_c: *const RUBase) -> u32,
    pub exec: extern "C" fn(self_c: *const RUBase) -> i32,
    pub is_session_restored: extern "C" fn(self_c: *const RUBase) -> bool,
    pub session_id: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub session_key: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub is_saving_session: extern "C" fn(self_c: *const RUBase) -> bool,
    pub is_fallback_session_management_enabled: extern "C" fn(self_c: *const RUBase) -> bool,
    pub set_fallback_session_management_enabled: extern "C" fn(self_c: *const RUBase, arg0: bool),
    pub sync: extern "C" fn(self_c: *const RUBase),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUGuiApplicationAllFuncs {
    pub object_funcs: *const RUObjectFuncs,
    pub core_application_funcs: *const RUCoreApplicationFuncs,
    pub gui_application_funcs: *const RUGuiApplicationFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUGuiApplication {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUGuiApplicationAllFuncs,
}
