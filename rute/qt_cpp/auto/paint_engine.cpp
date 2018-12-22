////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QPaintEngine>
#include "paint_engine_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool paint_engine_is_active(struct RUBase* self_c) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    auto ret_value = qt_value->isActive();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void paint_engine_set_active(struct RUBase* self_c, bool new_state) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    qt_value->setActive(new_state);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool paint_engine_begin(struct RUBase* self_c, struct RUBase* pdev) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    auto ret_value = qt_value->begin((QPaintDevice*)pdev);
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool paint_engine_end(struct RUBase* self_c) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    auto ret_value = qt_value->end();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void paint_engine_update_state(struct RUBase* self_c, struct RUBase* state) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    qt_value->updateState(*((QPaintEngineState*)state));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void paint_engine_draw_rects(struct RUBase* self_c, struct RUBase* rects, int rect_count) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    qt_value->drawRects((QRect*)rects, rect_count);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void paint_engine_draw_rects_2(struct RUBase* self_c, struct RUBase* rects, int rect_count) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    qt_value->drawRects((QRectF*)rects, rect_count);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void paint_engine_draw_lines(struct RUBase* self_c, struct RUBase* lines, int line_count) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    qt_value->drawLines((QLine*)lines, line_count);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void paint_engine_draw_lines_2(struct RUBase* self_c, struct RUBase* lines, int line_count) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    qt_value->drawLines((QLineF*)lines, line_count);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void paint_engine_draw_ellipse(struct RUBase* self_c, struct RUBase* r) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    qt_value->drawEllipse(*((QRectF*)r));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void paint_engine_draw_ellipse_2(struct RUBase* self_c, struct RUBase* r) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    qt_value->drawEllipse(*((QRect*)r));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void paint_engine_draw_points(struct RUBase* self_c, struct RUBase* points, int point_count) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    qt_value->drawPoints((QPointF*)points, point_count);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void paint_engine_draw_points_2(struct RUBase* self_c, struct RUBase* points, int point_count) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    qt_value->drawPoints((QPoint*)points, point_count);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void paint_engine_draw_polygon(struct RUBase* self_c, struct RUBase* points, int point_count, uint32_t mode) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    qt_value->drawPolygon((QPointF*)points, point_count, (QPaintEngine::PolygonDrawMode)mode);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void paint_engine_draw_polygon_2(struct RUBase* self_c, struct RUBase* points, int point_count, uint32_t mode) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    qt_value->drawPolygon((QPoint*)points, point_count, (QPaintEngine::PolygonDrawMode)mode);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void paint_engine_draw_pixmap(struct RUBase* self_c, struct RUBase* r, struct RUBase* pm, struct RUBase* sr) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    qt_value->drawPixmap(*((QRectF*)r), *((QPixmap*)pm), *((QRectF*)sr));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void paint_engine_draw_tiled_pixmap(struct RUBase* self_c, struct RUBase* r, struct RUBase* pixmap, struct RUBase* s) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    qt_value->drawTiledPixmap(*((QRectF*)r), *((QPixmap*)pixmap), *((QPointF*)s));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void paint_engine_draw_image(struct RUBase* self_c, struct RUBase* r, struct RUBase* pm, struct RUBase* sr, uint32_t flags) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    qt_value->drawImage(*((QRectF*)r), *((QImage*)pm), *((QRectF*)sr), (Qt::ImageConversionFlags)flags);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void paint_engine_set_paint_device(struct RUBase* self_c, struct RUBase* device) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    qt_value->setPaintDevice((QPaintDevice*)device);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPaintDevice paint_engine_paint_device(struct RUBase* self_c) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    auto ret_value = qt_value->paintDevice();
    struct RUPaintDevice ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_paint_device_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void paint_engine_set_system_clip(struct RUBase* self_c, struct RUBase* base_clip) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    qt_value->setSystemClip(*((QRegion*)base_clip));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RURegion paint_engine_system_clip(struct RUBase* self_c) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    auto ret_value = qt_value->systemClip();
    WRRegion* new_val = new WRRegion();
    *new_val = ret_value;
    struct RURegion ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_region_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void paint_engine_set_system_rect(struct RUBase* self_c, struct RUBase* rect) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    qt_value->setSystemRect(*((QRect*)rect));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RURect paint_engine_system_rect(struct RUBase* self_c) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    auto ret_value = qt_value->systemRect();
    WRRect* new_val = new WRRect();
    *new_val = ret_value;
    struct RURect ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_rect_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPoint paint_engine_coordinate_offset(struct RUBase* self_c) {
    QPaintEngine* qt_value = (QPaintEngine*)self_c;
    auto ret_value = qt_value->coordinateOffset();
    WRPoint* new_val = new WRPoint();
    *new_val = ret_value;
    struct RUPoint ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_point_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUPaintEngineFuncs s_paint_engine_funcs = {
    paint_engine_is_active,
    paint_engine_set_active,
    paint_engine_begin,
    paint_engine_end,
    paint_engine_update_state,
    paint_engine_draw_rects,
    paint_engine_draw_rects_2,
    paint_engine_draw_lines,
    paint_engine_draw_lines_2,
    paint_engine_draw_ellipse,
    paint_engine_draw_ellipse_2,
    paint_engine_draw_points,
    paint_engine_draw_points_2,
    paint_engine_draw_polygon,
    paint_engine_draw_polygon_2,
    paint_engine_draw_pixmap,
    paint_engine_draw_tiled_pixmap,
    paint_engine_draw_image,
    paint_engine_set_paint_device,
    paint_engine_paint_device,
    paint_engine_set_system_clip,
    paint_engine_system_clip,
    paint_engine_set_system_rect,
    paint_engine_system_rect,
    paint_engine_coordinate_offset,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUPaintEngineAllFuncs s_paint_engine_all_funcs = {
    &s_paint_engine_funcs,
};

