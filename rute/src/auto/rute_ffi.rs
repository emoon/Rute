#[allow(unused_imports)]
use auto::abstract_button_ffi::*;
#[allow(unused_imports)]
use auto::application_ffi::*;
#[allow(unused_imports)]
use auto::backing_store_ffi::*;
#[allow(unused_imports)]
use auto::bitmap_ffi::*;
#[allow(unused_imports)]
use auto::box_layout_ffi::*;
#[allow(unused_imports)]
use auto::brush_ffi::*;
#[allow(unused_imports)]
use auto::button_group_ffi::*;
#[allow(unused_imports)]
use auto::clipboard_ffi::*;
#[allow(unused_imports)]
use auto::close_event_ffi::*;
#[allow(unused_imports)]
use auto::color_ffi::*;
#[allow(unused_imports)]
use auto::context_menu_event_ffi::*;
#[allow(unused_imports)]
use auto::core_application_ffi::*;
#[allow(unused_imports)]
use auto::cursor_ffi::*;
#[allow(unused_imports)]
use auto::desktop_widget_ffi::*;
#[allow(unused_imports)]
use auto::drag_enter_event_ffi::*;
#[allow(unused_imports)]
use auto::drag_leave_event_ffi::*;
#[allow(unused_imports)]
use auto::drag_move_event_ffi::*;
#[allow(unused_imports)]
use auto::drop_event_ffi::*;
#[allow(unused_imports)]
use auto::event_ffi::*;
#[allow(unused_imports)]
use auto::expose_event_ffi::*;
#[allow(unused_imports)]
use auto::focus_event_ffi::*;
#[allow(unused_imports)]
use auto::font_ffi::*;
#[allow(unused_imports)]
use auto::font_info_ffi::*;
#[allow(unused_imports)]
use auto::gradient_ffi::*;
#[allow(unused_imports)]
use auto::grid_layout_ffi::*;
#[allow(unused_imports)]
use auto::gui_application_ffi::*;
#[allow(unused_imports)]
use auto::h_box_layout_ffi::*;
#[allow(unused_imports)]
use auto::hide_event_ffi::*;
#[allow(unused_imports)]
use auto::icon_ffi::*;
#[allow(unused_imports)]
use auto::image_ffi::*;
#[allow(unused_imports)]
use auto::input_event_ffi::*;
#[allow(unused_imports)]
use auto::key_event_ffi::*;
#[allow(unused_imports)]
use auto::key_sequence_ffi::*;
#[allow(unused_imports)]
use auto::layout_ffi::*;
#[allow(unused_imports)]
use auto::layout_item_ffi::*;
#[allow(unused_imports)]
use auto::line_edit_ffi::*;
#[allow(unused_imports)]
use auto::line_f_ffi::*;
#[allow(unused_imports)]
use auto::line_ffi::*;
#[allow(unused_imports)]
use auto::list_widget_ffi::*;
#[allow(unused_imports)]
use auto::list_widget_item_ffi::*;
#[allow(unused_imports)]
use auto::margins_ffi::*;
#[allow(unused_imports)]
use auto::matrix_ffi::*;
#[allow(unused_imports)]
use auto::mime_data_ffi::*;
#[allow(unused_imports)]
use auto::mouse_event_ffi::*;
#[allow(unused_imports)]
use auto::move_event_ffi::*;
#[allow(unused_imports)]
use auto::object_ffi::*;
#[allow(unused_imports)]
use auto::paint_device_ffi::*;
#[allow(unused_imports)]
use auto::paint_engine_ffi::*;
#[allow(unused_imports)]
use auto::paint_engine_state_ffi::*;
#[allow(unused_imports)]
use auto::paint_event_ffi::*;
#[allow(unused_imports)]
use auto::painter_ffi::*;
#[allow(unused_imports)]
use auto::palette_ffi::*;
#[allow(unused_imports)]
use auto::pen_ffi::*;
#[allow(unused_imports)]
use auto::pixel_format_ffi::*;
#[allow(unused_imports)]
use auto::pixmap_ffi::*;
#[allow(unused_imports)]
use auto::point_f_ffi::*;
#[allow(unused_imports)]
use auto::point_ffi::*;
#[allow(unused_imports)]
use auto::polygon_f_ffi::*;
#[allow(unused_imports)]
use auto::polygon_ffi::*;
#[allow(unused_imports)]
use auto::push_button_ffi::*;
#[allow(unused_imports)]
use auto::rect_f_ffi::*;
#[allow(unused_imports)]
use auto::rect_ffi::*;
#[allow(unused_imports)]
use auto::region_ffi::*;
#[allow(unused_imports)]
use auto::resize_event_ffi::*;
#[allow(unused_imports)]
use auto::screen_ffi::*;
#[allow(unused_imports)]
use auto::show_event_ffi::*;
#[allow(unused_imports)]
use auto::size_f_ffi::*;
#[allow(unused_imports)]
use auto::size_ffi::*;
#[allow(unused_imports)]
use auto::size_policy_ffi::*;
#[allow(unused_imports)]
use auto::spacer_item_ffi::*;
#[allow(unused_imports)]
use auto::style_ffi::*;
#[allow(unused_imports)]
use auto::surface_ffi::*;
#[allow(unused_imports)]
use auto::surface_format_ffi::*;
#[allow(unused_imports)]
use auto::tablet_event_ffi::*;
#[allow(unused_imports)]
use auto::tool_button_ffi::*;
#[allow(unused_imports)]
use auto::touch_event_ffi::*;
#[allow(unused_imports)]
use auto::transform_ffi::*;
#[allow(unused_imports)]
use auto::v_box_layout_ffi::*;
#[allow(unused_imports)]
use auto::wheel_event_ffi::*;
#[allow(unused_imports)]
use auto::widget_ffi::*;
#[allow(unused_imports)]
use auto::window_ffi::*;
use rute_ffi_base::*;
use std::os::raw::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RuteFFI {
    pub create_application: extern "C" fn(priv_data: *const RUBase) -> RUApplication,
    pub get_application: extern "C" fn(priv_data: *const RUBase) -> RUApplication,
    pub create_bitmap: extern "C" fn(priv_data: *const RUBase) -> RUBitmap,
    pub get_bitmap: extern "C" fn(priv_data: *const RUBase) -> RUBitmap,
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
    pub get_core_application: extern "C" fn(priv_data: *const RUBase) -> RUCoreApplication,
    pub create_cursor: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUCursor,
    pub get_cursor: extern "C" fn(priv_data: *const RUBase) -> RUCursor,
    pub create_font: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUFont,
    pub get_font: extern "C" fn(priv_data: *const RUBase) -> RUFont,
    pub create_gradient: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUGradient,
    pub create_grid_layout: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUGridLayout,
    pub get_gui_application: extern "C" fn(priv_data: *const RUBase) -> RUGuiApplication,
    pub create_h_box_layout: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUHBoxLayout,
    pub create_icon: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUIcon,
    pub get_icon: extern "C" fn(priv_data: *const RUBase) -> RUIcon,
    pub create_image: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUImage,
    pub get_image: extern "C" fn(priv_data: *const RUBase) -> RUImage,
    pub create_key_sequence: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUKeySequence,
    pub get_key_sequence: extern "C" fn(priv_data: *const RUBase) -> RUKeySequence,
    pub create_line: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RULine,
    pub create_line_edit: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RULineEdit,
    pub create_line_f: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RULineF,
    pub get_line_f: extern "C" fn(priv_data: *const RUBase) -> RULineF,
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
    pub create_matrix: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUMatrix,
    pub create_mime_data: extern "C" fn(priv_data: *const RUBase) -> RUMimeData,
    pub create_object: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUObject,
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
    pub create_palette: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUPalette,
    pub create_pen: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUPen,
    pub create_pixel_format: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUPixelFormat,
    pub create_pixmap: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUPixmap,
    pub get_pixmap: extern "C" fn(priv_data: *const RUBase) -> RUPixmap,
    pub create_point: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUPoint,
    pub get_point: extern "C" fn(priv_data: *const RUBase) -> RUPoint,
    pub create_point_f: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUPointF,
    pub get_point_f: extern "C" fn(priv_data: *const RUBase) -> RUPointF,
    pub create_polygon: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUPolygon,
    pub create_polygon_f: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUPolygonF,
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
    pub create_rect_f: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RURectF,
    pub create_region: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RURegion,
    pub create_size: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUSize,
    pub create_size_f: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUSizeF,
    pub create_size_policy: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUSizePolicy,
    pub create_surface_format: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUSurfaceFormat,
    pub get_surface_format: extern "C" fn(priv_data: *const RUBase) -> RUSurfaceFormat,
    pub create_tool_button: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUToolButton,
    pub create_transform: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUTransform,
    pub get_transform: extern "C" fn(priv_data: *const RUBase) -> RUTransform,
    pub create_v_box_layout: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUVBoxLayout,
    pub create_widget: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUWidget,
    pub get_widget: extern "C" fn(priv_data: *const RUBase) -> RUWidget,
    pub create_window: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        host_data: *const c_void,
    ) -> RUWindow,
    pub get_window: extern "C" fn(priv_data: *const RUBase) -> RUWindow,
}

extern "C" {
    pub fn rute_static_ffi_get() -> *const RuteFFI;
}
