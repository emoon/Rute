use auto::application_ffi::*;
use auto::event_ffi::*;
use auto::font_ffi::*;
use auto::list_widget_ffi::*;
use auto::list_widget_item_ffi::*;
use auto::paint_event_ffi::*;
use auto::push_button_ffi::*;
use auto::rect_ffi::*;
use auto::size_ffi::*;
use auto::widget_ffi::*;
use rute_ffi_base::*;
use std::os::raw::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RuteFFI {
    pub create_application: extern "C" fn(priv_data: *const RUBase) -> RUApplication,
    pub get_application: extern "C" fn(priv_data: *const RUBase) -> RUApplication,
    pub create_font: extern "C" fn(priv_data: *const RUBase) -> RUFont,
    pub create_list_widget: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUListWidget,
    pub create_list_widget_item: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUListWidgetItem,
    pub create_push_button: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUPushButton,
    pub create_rect: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RURect,
    pub create_size: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUSize,
    pub create_widget: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUWidget,
}

extern "C" {
    pub fn rute_static_ffi_get() -> *const RuteFFI;
}
