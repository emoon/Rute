use auto::application_ffi::*;
use auto::backing_store_ffi::*;
use auto::brush_ffi::*;
use auto::color_ffi::*;
use auto::event_ffi::*;
use auto::font_ffi::*;
use auto::gradient_ffi::*;
use auto::image_ffi::*;
use auto::list_widget_ffi::*;
use auto::list_widget_item_ffi::*;
use auto::margins_ffi::*;
use auto::paint_device_ffi::*;
use auto::paint_engine_ffi::*;
use auto::paint_engine_state_ffi::*;
use auto::paint_event_ffi::*;
use auto::painter_ffi::*;
use auto::pixel_format_ffi::*;
use auto::point_ffi::*;
use auto::push_button_ffi::*;
use auto::rect_ffi::*;
use auto::screen_ffi::*;
use auto::size_ffi::*;
use auto::widget_ffi::*;
use rute_ffi_base::*;
use std::os::raw::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RuteFFI {
    pub create_application: extern "C" fn(priv_data: *const RUBase) -> RUApplication,
    pub get_application: extern "C" fn(priv_data: *const RUBase) -> RUApplication,
    pub create_backing_store: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUBackingStore,
    pub create_brush: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUBrush,
    pub create_color: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUColor,
    pub get_color: extern "C" fn(priv_data: *const RUBase) -> RUColor,
    pub create_font: extern "C" fn(priv_data: *const RUBase) -> RUFont,
    pub create_gradient: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUGradient,
    pub create_image: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUImage,
    pub get_image: extern "C" fn(priv_data: *const RUBase) -> RUImage,
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
    pub create_margins: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUMargins,
    pub create_paint_device: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUPaintDevice,
    pub get_paint_device: extern "C" fn(priv_data: *const RUBase) -> RUPaintDevice,
    pub create_paint_engine: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUPaintEngine,
    pub create_paint_engine_state: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUPaintEngineState,
    pub create_painter: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUPainter,
    pub get_painter: extern "C" fn(priv_data: *const RUBase) -> RUPainter,
    pub create_pixel_format: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUPixelFormat,
    pub get_pixel_format: extern "C" fn(priv_data: *const RUBase) -> RUPixelFormat,
    pub create_point: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUPoint,
    pub get_point: extern "C" fn(priv_data: *const RUBase) -> RUPoint,
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
    pub get_widget: extern "C" fn(priv_data: *const RUBase) -> RUWidget,
}

extern "C" {
    pub fn rute_static_ffi_get() -> *const RuteFFI;
}
