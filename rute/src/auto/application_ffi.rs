// This file is auto-generated by rute_gen. DO NOT EDIT.
use std::os::raw::c_void;
use rute::auto::base_ffi::*;

use rute::auto::desktop_widget_ffi::RUDesktopWidget;
use rute::auto::font_ffi::RUFont;
use rute::auto::icon_ffi::RUIcon;
use rute::auto::palette_ffi::RUPalette;
use rute::auto::style_ffi::RUStyle;
use rute::auto::widget_ffi::RUWidget;
use rute::auto::widget_list_ffi::RUWidgetList;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUApplicationFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub style: extern "C" fn(self_c: *const RUBase) ->  RUStyle,
    pub set_style: extern "C" fn(self_c: *const RUBase, arg0: i32),
    pub set_style: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase),
    pub set_style: extern "C" fn(self_c: *const RUBase, arg0: *const ::std::os::raw::c_char) ->  RUStyle,
    pub color_spec: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_color_spec: extern "C" fn(self_c: *const RUBase, arg0: i32),
    pub palette: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase) ->  RUPalette,
    pub palette: extern "C" fn(self_c: *const RUBase, class_name: *const ::std::os::raw::c_char) ->  RUPalette,
    pub set_palette: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase, class_name: *const ::std::os::raw::c_char),
    pub font: extern "C" fn(self_c: *const RUBase) ->  RUFont,
    pub font: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase) ->  RUFont,
    pub font: extern "C" fn(self_c: *const RUBase, class_name: *const ::std::os::raw::c_char) ->  RUFont,
    pub set_font: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase, class_name: *const ::std::os::raw::c_char),
    pub set_window_icon: extern "C" fn(self_c: *const RUBase, icon: *const RUBase),
    pub window_icon: extern "C" fn(self_c: *const RUBase) ->  RUIcon,
    pub all_widgets: extern "C" fn(self_c: *const RUBase) ->  RUWidgetList,
    pub top_level_widgets: extern "C" fn(self_c: *const RUBase) ->  RUWidgetList,
    pub desktop: extern "C" fn(self_c: *const RUBase) ->  RUDesktopWidget,
    pub active_popup_widget: extern "C" fn(self_c: *const RUBase) ->  RUWidget,
    pub active_modal_widget: extern "C" fn(self_c: *const RUBase) ->  RUWidget,
    pub focus_widget: extern "C" fn(self_c: *const RUBase) ->  RUWidget,
    pub active_window: extern "C" fn(self_c: *const RUBase) ->  RUWidget,
    pub set_active_window: extern "C" fn(self_c: *const RUBase, act: *const RUBase),
    pub widget_at: extern "C" fn(self_c: *const RUBase, p: *const RUBase) ->  RUWidget,
    pub widget_at: extern "C" fn(self_c: *const RUBase, x: i32, y: i32) ->  RUWidget,
    pub top_level_at: extern "C" fn(self_c: *const RUBase, p: *const RUBase) ->  RUWidget,
    pub top_level_at: extern "C" fn(self_c: *const RUBase, x: i32, y: i32) ->  RUWidget,
    pub beep: extern "C" fn(self_c: *const RUBase),
    pub alert: extern "C" fn(self_c: *const RUBase, widget: *const RUBase, duration: i32),
    pub set_cursor_flash_time: extern "C" fn(self_c: *const RUBase, arg0: i32),
    pub cursor_flash_time: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_double_click_interval: extern "C" fn(self_c: *const RUBase, arg0: i32),
    pub double_click_interval: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_keyboard_input_interval: extern "C" fn(self_c: *const RUBase, arg0: i32),
    pub keyboard_input_interval: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_wheel_scroll_lines: extern "C" fn(self_c: *const RUBase, arg0: i32),
    pub wheel_scroll_lines: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_start_drag_time: extern "C" fn(self_c: *const RUBase, ms: i32),
    pub start_drag_time: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_start_drag_distance: extern "C" fn(self_c: *const RUBase, l: i32),
    pub start_drag_distance: extern "C" fn(self_c: *const RUBase) -> i32,
    pub is_effect_enabled: extern "C" fn(self_c: *const RUBase, arg0: i32) -> bool,
    pub set_effect_enabled: extern "C" fn(self_c: *const RUBase, arg0: i32, enable: bool),
    pub exec: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_focus_changed_event: extern "C" fn(object: *const RUBase, user_data: *const c_void, trampoline_func: *const c_void,
                                            callback: *const c_void),

    pub style_sheet: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub set_style_sheet: extern "C" fn(self_c: *const RUBase, sheet: *const ::std::os::raw::c_char),
    pub set_auto_sip_enabled: extern "C" fn(self_c: *const RUBase, enabled: bool),
    pub auto_sip_enabled: extern "C" fn(self_c: *const RUBase) -> bool,
    pub close_all_windows: extern "C" fn(self_c: *const RUBase),
    pub about_qt: extern "C" fn(self_c: *const RUBase),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUApplicationAllFuncs {
    pub application_funcs: *const RUApplicationFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUApplication {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUFontAllFuncs,
}


extern "C" {
    pub fn rute_get() -> *const RuteFFI;
}

