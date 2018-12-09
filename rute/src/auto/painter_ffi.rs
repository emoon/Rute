// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::brush_ffi::RUBrush;
#[allow(unused_imports)]
use auto::font_ffi::RUFont;
#[allow(unused_imports)]
use auto::paint_device_ffi::RUPaintDevice;
#[allow(unused_imports)]
use auto::paint_engine_ffi::RUPaintEngine;
#[allow(unused_imports)]
use auto::pen_ffi::RUPen;
#[allow(unused_imports)]
use auto::point_ffi::RUPoint;
#[allow(unused_imports)]
use auto::rect_f_ffi::RURectF;
#[allow(unused_imports)]
use auto::rect_ffi::RURect;
#[allow(unused_imports)]
use auto::region_ffi::RURegion;
#[allow(unused_imports)]
use auto::transform_ffi::RUTransform;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUPainterFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub device: extern "C" fn(self_c: *const RUBase) -> RUPaintDevice,
    pub begin: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase) -> bool,
    pub end: extern "C" fn(self_c: *const RUBase) -> bool,
    pub is_active: extern "C" fn(self_c: *const RUBase) -> bool,
    pub init_from: extern "C" fn(self_c: *const RUBase, device: *const RUBase),
    pub set_composition_mode: extern "C" fn(self_c: *const RUBase, mode: i32),
    pub composition_mode: extern "C" fn(self_c: *const RUBase) -> i32,
    pub font: extern "C" fn(self_c: *const RUBase) -> RUFont,
    pub set_font: extern "C" fn(self_c: *const RUBase, f: *const RUBase),
    pub set_pen: extern "C" fn(self_c: *const RUBase, color: *const RUBase),
    pub set_pen_2: extern "C" fn(self_c: *const RUBase, pen: *const RUBase),
    pub set_pen_3: extern "C" fn(self_c: *const RUBase, style: i32),
    pub pen: extern "C" fn(self_c: *const RUBase) -> RUPen,
    pub set_brush: extern "C" fn(self_c: *const RUBase, brush: *const RUBase),
    pub set_brush_2: extern "C" fn(self_c: *const RUBase, style: i32),
    pub brush: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub set_background_mode: extern "C" fn(self_c: *const RUBase, mode: i32),
    pub background_mode: extern "C" fn(self_c: *const RUBase) -> i32,
    pub brush_origin: extern "C" fn(self_c: *const RUBase) -> RUPoint,
    pub set_brush_origin: extern "C" fn(self_c: *const RUBase, x: i32, y: i32),
    pub set_brush_origin_2: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase),
    pub set_brush_origin_3: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase),
    pub set_background: extern "C" fn(self_c: *const RUBase, bg: *const RUBase),
    pub background: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub opacity: extern "C" fn(self_c: *const RUBase) -> f32,
    pub set_opacity: extern "C" fn(self_c: *const RUBase, opacity: f32),
    pub clip_region: extern "C" fn(self_c: *const RUBase) -> RURegion,
    pub set_clip_rect: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase, op: i32),
    pub set_clip_rect_2: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase, op: i32),
    pub set_clip_rect_3:
        extern "C" fn(self_c: *const RUBase, x: i32, y: i32, w: i32, h: i32, op: i32),
    pub set_clip_region: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase, op: i32),
    pub set_clipping: extern "C" fn(self_c: *const RUBase, enable: bool),
    pub has_clipping: extern "C" fn(self_c: *const RUBase) -> bool,
    pub clip_bounding_rect: extern "C" fn(self_c: *const RUBase) -> RURectF,
    pub save: extern "C" fn(self_c: *const RUBase),
    pub restore: extern "C" fn(self_c: *const RUBase),
    pub set_transform:
        extern "C" fn(self_c: *const RUBase, transform: *const RUBase, combine: bool),
    pub device_transform: extern "C" fn(self_c: *const RUBase) -> RUTransform,
    pub reset_transform: extern "C" fn(self_c: *const RUBase),
    pub set_world_transform:
        extern "C" fn(self_c: *const RUBase, matrix: *const RUBase, combine: bool),
    pub world_transform: extern "C" fn(self_c: *const RUBase) -> RUTransform,
    pub combined_transform: extern "C" fn(self_c: *const RUBase) -> RUTransform,
    pub scale: extern "C" fn(self_c: *const RUBase, sx: f32, sy: f32),
    pub shear: extern "C" fn(self_c: *const RUBase, sh: f32, sv: f32),
    pub rotate: extern "C" fn(self_c: *const RUBase, a: f32),
    pub window: extern "C" fn(self_c: *const RUBase) -> RURect,
    pub set_window: extern "C" fn(self_c: *const RUBase, window: *const RUBase),
    pub set_window_2: extern "C" fn(self_c: *const RUBase, x: i32, y: i32, w: i32, h: i32),
    pub viewport: extern "C" fn(self_c: *const RUBase) -> RURect,
    pub set_viewport: extern "C" fn(self_c: *const RUBase, viewport: *const RUBase),
    pub set_viewport_2: extern "C" fn(self_c: *const RUBase, x: i32, y: i32, w: i32, h: i32),
    pub set_view_transform_enabled: extern "C" fn(self_c: *const RUBase, enable: bool),
    pub view_transform_enabled: extern "C" fn(self_c: *const RUBase) -> bool,
    pub draw_point: extern "C" fn(self_c: *const RUBase, pt: *const RUBase),
    pub draw_point_2: extern "C" fn(self_c: *const RUBase, p: *const RUBase),
    pub draw_point_3: extern "C" fn(self_c: *const RUBase, x: i32, y: i32),
    pub draw_points: extern "C" fn(self_c: *const RUBase, points: *const RUBase, point_count: i32),
    pub draw_points_2: extern "C" fn(self_c: *const RUBase, points: *const RUBase),
    pub draw_points_3:
        extern "C" fn(self_c: *const RUBase, points: *const RUBase, point_count: i32),
    pub draw_points_4: extern "C" fn(self_c: *const RUBase, points: *const RUBase),
    pub draw_line: extern "C" fn(self_c: *const RUBase, line: *const RUBase),
    pub draw_line_2: extern "C" fn(self_c: *const RUBase, line: *const RUBase),
    pub draw_line_3: extern "C" fn(self_c: *const RUBase, x1: i32, y1: i32, x2: i32, y2: i32),
    pub draw_line_4: extern "C" fn(self_c: *const RUBase, p1: *const RUBase, p2: *const RUBase),
    pub draw_line_5: extern "C" fn(self_c: *const RUBase, p1: *const RUBase, p2: *const RUBase),
    pub draw_lines: extern "C" fn(self_c: *const RUBase, lines: *const RUBase, line_count: i32),
    pub draw_lines_3:
        extern "C" fn(self_c: *const RUBase, point_pairs: *const RUBase, line_count: i32),
    pub draw_lines_7:
        extern "C" fn(self_c: *const RUBase, point_pairs: *const RUBase, line_count: i32),
    pub draw_rect: extern "C" fn(self_c: *const RUBase, rect: *const RUBase),
    pub draw_rect_2: extern "C" fn(self_c: *const RUBase, x1: i32, y1: i32, w: i32, h: i32),
    pub draw_rect_3: extern "C" fn(self_c: *const RUBase, rect: *const RUBase),
    pub draw_rects: extern "C" fn(self_c: *const RUBase, rects: *const RUBase, rect_count: i32),
    pub draw_rects_3: extern "C" fn(self_c: *const RUBase, rects: *const RUBase, rect_count: i32),
    pub draw_ellipse: extern "C" fn(self_c: *const RUBase, r: *const RUBase),
    pub draw_ellipse_2: extern "C" fn(self_c: *const RUBase, r: *const RUBase),
    pub draw_ellipse_3: extern "C" fn(self_c: *const RUBase, x: i32, y: i32, w: i32, h: i32),
    pub draw_ellipse_4:
        extern "C" fn(self_c: *const RUBase, center: *const RUBase, rx: f32, ry: f32),
    pub draw_ellipse_5:
        extern "C" fn(self_c: *const RUBase, center: *const RUBase, rx: i32, ry: i32),
    pub draw_polyline:
        extern "C" fn(self_c: *const RUBase, points: *const RUBase, point_count: i32),
    pub draw_polyline_2: extern "C" fn(self_c: *const RUBase, polyline: *const RUBase),
    pub draw_polyline_3:
        extern "C" fn(self_c: *const RUBase, points: *const RUBase, point_count: i32),
    pub draw_polyline_4: extern "C" fn(self_c: *const RUBase, polygon: *const RUBase),
    pub draw_polygon: extern "C" fn(
        self_c: *const RUBase,
        points: *const RUBase,
        point_count: i32,
        fill_rule: i32,
    ),
    pub draw_polygon_2:
        extern "C" fn(self_c: *const RUBase, polygon: *const RUBase, fill_rule: i32),
    pub draw_polygon_3: extern "C" fn(
        self_c: *const RUBase,
        points: *const RUBase,
        point_count: i32,
        fill_rule: i32,
    ),
    pub draw_polygon_4:
        extern "C" fn(self_c: *const RUBase, polygon: *const RUBase, fill_rule: i32),
    pub draw_convex_polygon:
        extern "C" fn(self_c: *const RUBase, points: *const RUBase, point_count: i32),
    pub draw_convex_polygon_2: extern "C" fn(self_c: *const RUBase, polygon: *const RUBase),
    pub draw_convex_polygon_3:
        extern "C" fn(self_c: *const RUBase, points: *const RUBase, point_count: i32),
    pub draw_convex_polygon_4: extern "C" fn(self_c: *const RUBase, polygon: *const RUBase),
    pub draw_arc: extern "C" fn(self_c: *const RUBase, rect: *const RUBase, a: i32, alen: i32),
    pub draw_arc_2: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase, a: i32, alen: i32),
    pub draw_arc_3:
        extern "C" fn(self_c: *const RUBase, x: i32, y: i32, w: i32, h: i32, a: i32, alen: i32),
    pub draw_pie: extern "C" fn(self_c: *const RUBase, rect: *const RUBase, a: i32, alen: i32),
    pub draw_pie_2:
        extern "C" fn(self_c: *const RUBase, x: i32, y: i32, w: i32, h: i32, a: i32, alen: i32),
    pub draw_pie_3: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase, a: i32, alen: i32),
    pub draw_chord: extern "C" fn(self_c: *const RUBase, rect: *const RUBase, a: i32, alen: i32),
    pub draw_chord_2:
        extern "C" fn(self_c: *const RUBase, x: i32, y: i32, w: i32, h: i32, a: i32, alen: i32),
    pub draw_chord_3: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase, a: i32, alen: i32),
    pub draw_rounded_rect: extern "C" fn(
        self_c: *const RUBase,
        rect: *const RUBase,
        x_radius: f32,
        y_radius: f32,
        mode: i32,
    ),
    pub draw_rounded_rect_2: extern "C" fn(
        self_c: *const RUBase,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        x_radius: f32,
        y_radius: f32,
        mode: i32,
    ),
    pub draw_rounded_rect_3: extern "C" fn(
        self_c: *const RUBase,
        rect: *const RUBase,
        x_radius: f32,
        y_radius: f32,
        mode: i32,
    ),
    pub draw_round_rect:
        extern "C" fn(self_c: *const RUBase, r: *const RUBase, xround: i32, yround: i32),
    pub draw_round_rect_2:
        extern "C" fn(self_c: *const RUBase, x: i32, y: i32, w: i32, h: i32, arg0: i32, arg1: i32),
    pub draw_round_rect_3:
        extern "C" fn(self_c: *const RUBase, r: *const RUBase, xround: i32, yround: i32),
    pub draw_tiled_pixmap: extern "C" fn(
        self_c: *const RUBase,
        rect: *const RUBase,
        pm: *const RUBase,
        offset: *const RUBase,
    ),
    pub draw_tiled_pixmap_2: extern "C" fn(
        self_c: *const RUBase,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        arg0: *const RUBase,
        sx: i32,
        sy: i32,
    ),
    pub draw_tiled_pixmap_3: extern "C" fn(
        self_c: *const RUBase,
        arg0: *const RUBase,
        arg1: *const RUBase,
        arg2: *const RUBase,
    ),
    pub draw_pixmap: extern "C" fn(
        self_c: *const RUBase,
        target_rect: *const RUBase,
        pixmap: *const RUBase,
        source_rect: *const RUBase,
    ),
    pub draw_pixmap_2: extern "C" fn(
        self_c: *const RUBase,
        target_rect: *const RUBase,
        pixmap: *const RUBase,
        source_rect: *const RUBase,
    ),
    pub draw_pixmap_3: extern "C" fn(
        self_c: *const RUBase,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        pm: *const RUBase,
        sx: i32,
        sy: i32,
        sw: i32,
        sh: i32,
    ),
    pub draw_pixmap_4: extern "C" fn(
        self_c: *const RUBase,
        x: i32,
        y: i32,
        pm: *const RUBase,
        sx: i32,
        sy: i32,
        sw: i32,
        sh: i32,
    ),
    pub draw_pixmap_5: extern "C" fn(
        self_c: *const RUBase,
        p: *const RUBase,
        pm: *const RUBase,
        sr: *const RUBase,
    ),
    pub draw_pixmap_6: extern "C" fn(
        self_c: *const RUBase,
        p: *const RUBase,
        pm: *const RUBase,
        sr: *const RUBase,
    ),
    pub draw_pixmap_7: extern "C" fn(self_c: *const RUBase, p: *const RUBase, pm: *const RUBase),
    pub draw_pixmap_8: extern "C" fn(self_c: *const RUBase, p: *const RUBase, pm: *const RUBase),
    pub draw_pixmap_9: extern "C" fn(self_c: *const RUBase, x: i32, y: i32, pm: *const RUBase),
    pub draw_pixmap_10: extern "C" fn(self_c: *const RUBase, r: *const RUBase, pm: *const RUBase),
    pub draw_pixmap_11:
        extern "C" fn(self_c: *const RUBase, x: i32, y: i32, w: i32, h: i32, pm: *const RUBase),
    pub draw_image: extern "C" fn(
        self_c: *const RUBase,
        target_rect: *const RUBase,
        image: *const RUBase,
        source_rect: *const RUBase,
        flags: i32,
    ),
    pub draw_image_2: extern "C" fn(
        self_c: *const RUBase,
        target_rect: *const RUBase,
        image: *const RUBase,
        source_rect: *const RUBase,
        flags: i32,
    ),
    pub draw_image_3: extern "C" fn(
        self_c: *const RUBase,
        p: *const RUBase,
        image: *const RUBase,
        sr: *const RUBase,
        flags: i32,
    ),
    pub draw_image_4: extern "C" fn(
        self_c: *const RUBase,
        p: *const RUBase,
        image: *const RUBase,
        sr: *const RUBase,
        flags: i32,
    ),
    pub draw_image_5: extern "C" fn(self_c: *const RUBase, r: *const RUBase, image: *const RUBase),
    pub draw_image_6: extern "C" fn(self_c: *const RUBase, r: *const RUBase, image: *const RUBase),
    pub draw_image_7: extern "C" fn(self_c: *const RUBase, p: *const RUBase, image: *const RUBase),
    pub draw_image_8: extern "C" fn(self_c: *const RUBase, p: *const RUBase, image: *const RUBase),
    pub draw_image_9: extern "C" fn(
        self_c: *const RUBase,
        x: i32,
        y: i32,
        image: *const RUBase,
        sx: i32,
        sy: i32,
        sw: i32,
        sh: i32,
        flags: i32,
    ),
    pub set_layout_direction: extern "C" fn(self_c: *const RUBase, direction: i32),
    pub layout_direction: extern "C" fn(self_c: *const RUBase) -> i32,
    pub draw_text:
        extern "C" fn(self_c: *const RUBase, p: *const RUBase, s: *const ::std::os::raw::c_char),
    pub draw_text_2:
        extern "C" fn(self_c: *const RUBase, p: *const RUBase, s: *const ::std::os::raw::c_char),
    pub draw_text_3:
        extern "C" fn(self_c: *const RUBase, x: i32, y: i32, s: *const ::std::os::raw::c_char),
    pub draw_text_4: extern "C" fn(
        self_c: *const RUBase,
        p: *const RUBase,
        str: *const ::std::os::raw::c_char,
        tf: i32,
        justification_padding: i32,
    ),
    pub draw_text_5: extern "C" fn(
        self_c: *const RUBase,
        r: *const RUBase,
        flags: i32,
        text: *const ::std::os::raw::c_char,
        br: *const RUBase,
    ),
    pub draw_text_6: extern "C" fn(
        self_c: *const RUBase,
        r: *const RUBase,
        flags: i32,
        text: *const ::std::os::raw::c_char,
        br: *const RUBase,
    ),
    pub draw_text_7: extern "C" fn(
        self_c: *const RUBase,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        flags: i32,
        text: *const ::std::os::raw::c_char,
        br: *const RUBase,
    ),
    pub bounding_rect: extern "C" fn(
        self_c: *const RUBase,
        rect: *const RUBase,
        flags: i32,
        text: *const ::std::os::raw::c_char,
    ) -> RURectF,
    pub bounding_rect_2: extern "C" fn(
        self_c: *const RUBase,
        rect: *const RUBase,
        flags: i32,
        text: *const ::std::os::raw::c_char,
    ) -> RURect,
    pub bounding_rect_3: extern "C" fn(
        self_c: *const RUBase,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        flags: i32,
        text: *const ::std::os::raw::c_char,
    ) -> RURect,
    pub fill_rect: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase, arg1: *const RUBase),
    pub fill_rect_2:
        extern "C" fn(self_c: *const RUBase, x: i32, y: i32, w: i32, h: i32, arg0: *const RUBase),
    pub fill_rect_3: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase, arg1: *const RUBase),
    pub fill_rect_4:
        extern "C" fn(self_c: *const RUBase, arg0: *const RUBase, color: *const RUBase),
    pub fill_rect_5:
        extern "C" fn(self_c: *const RUBase, x: i32, y: i32, w: i32, h: i32, color: *const RUBase),
    pub fill_rect_6:
        extern "C" fn(self_c: *const RUBase, arg0: *const RUBase, color: *const RUBase),
    pub fill_rect_7: extern "C" fn(self_c: *const RUBase, x: i32, y: i32, w: i32, h: i32, c: i32),
    pub fill_rect_8: extern "C" fn(self_c: *const RUBase, r: *const RUBase, c: i32),
    pub fill_rect_9: extern "C" fn(self_c: *const RUBase, r: *const RUBase, c: i32),
    pub fill_rect_10:
        extern "C" fn(self_c: *const RUBase, x: i32, y: i32, w: i32, h: i32, style: i32),
    pub fill_rect_11: extern "C" fn(self_c: *const RUBase, r: *const RUBase, style: i32),
    pub fill_rect_12: extern "C" fn(self_c: *const RUBase, r: *const RUBase, style: i32),
    pub erase_rect: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase),
    pub erase_rect_2: extern "C" fn(self_c: *const RUBase, x: i32, y: i32, w: i32, h: i32),
    pub erase_rect_3: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase),
    pub set_render_hint: extern "C" fn(self_c: *const RUBase, hint: i32, on: bool),
    pub set_render_hints: extern "C" fn(self_c: *const RUBase, hints: i32, on: bool),
    pub render_hints: extern "C" fn(self_c: *const RUBase) -> i32,
    pub test_render_hint: extern "C" fn(self_c: *const RUBase, hint: i32) -> bool,
    pub paint_engine: extern "C" fn(self_c: *const RUBase) -> RUPaintEngine,
    pub set_redirected: extern "C" fn(
        self_c: *const RUBase,
        device: *const RUBase,
        replacement: *const RUBase,
        offset: *const RUBase,
    ),
    pub redirected: extern "C" fn(
        self_c: *const RUBase,
        device: *const RUBase,
        offset: *const RUBase,
    ) -> RUPaintDevice,
    pub restore_redirected: extern "C" fn(self_c: *const RUBase, device: *const RUBase),
    pub begin_native_painting: extern "C" fn(self_c: *const RUBase),
    pub end_native_painting: extern "C" fn(self_c: *const RUBase),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUPainterAllFuncs {
    pub painter_funcs: *const RUPainterFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUPainter {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUPainterAllFuncs,
}
