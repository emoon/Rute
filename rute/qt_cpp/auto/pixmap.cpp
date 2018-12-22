////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QPixmap>
#include "pixmap_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pixmap_swap(struct RUBase* self_c, struct RUBase* other) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    qt_value->swap(*((QPixmap*)other));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool pixmap_is_null(struct RUBase* self_c) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->isNull();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int pixmap_width(struct RUBase* self_c) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->width();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int pixmap_height(struct RUBase* self_c) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->height();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize pixmap_size(struct RUBase* self_c) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->size();
    WRSize* new_val = new WRSize();
    *new_val = ret_value;
    struct RUSize ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_size_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RURect pixmap_rect(struct RUBase* self_c) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->rect();
    WRRect* new_val = new WRRect();
    *new_val = ret_value;
    struct RURect ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_rect_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int pixmap_depth(struct RUBase* self_c) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->depth();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int pixmap_default_depth(struct RUBase* self_c) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->defaultDepth();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pixmap_fill(struct RUBase* self_c, struct RUBase* fill_color) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    qt_value->fill(*((QColor*)fill_color));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pixmap_fill_2(struct RUBase* self_c, struct RUBase* device, struct RUBase* ofs) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    qt_value->fill((QPaintDevice*)device, *((QPoint*)ofs));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pixmap_fill_3(struct RUBase* self_c, struct RUBase* device, int xofs, int yofs) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    qt_value->fill((QPaintDevice*)device, xofs, yofs);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBitmap pixmap_mask(struct RUBase* self_c) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->mask();
    QBitmap* new_val = new QBitmap();
    *new_val = ret_value;
    struct RUBitmap ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_bitmap_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pixmap_set_mask(struct RUBase* self_c, struct RUBase* arg0) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    qt_value->setMask(*((QBitmap*)arg0));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float pixmap_device_pixel_ratio(struct RUBase* self_c) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->devicePixelRatio();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pixmap_set_device_pixel_ratio(struct RUBase* self_c, float scale_factor) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    qt_value->setDevicePixelRatio(scale_factor);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool pixmap_has_alpha(struct RUBase* self_c) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->hasAlpha();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool pixmap_has_alpha_channel(struct RUBase* self_c) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->hasAlphaChannel();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBitmap pixmap_create_heuristic_mask(struct RUBase* self_c, bool clip_tight) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->createHeuristicMask(clip_tight);
    QBitmap* new_val = new QBitmap();
    *new_val = ret_value;
    struct RUBitmap ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_bitmap_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBitmap pixmap_create_mask_from_color(struct RUBase* self_c, struct RUBase* mask_color, uint32_t mode) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->createMaskFromColor(*((QColor*)mask_color), (Qt::MaskMode)mode);
    QBitmap* new_val = new QBitmap();
    *new_val = ret_value;
    struct RUBitmap ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_bitmap_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPixmap pixmap_grab_window(struct RUBase* self_c, uint64_t arg0, int x, int y, int w, int h) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->grabWindow(arg0, x, y, w, h);
    WRPixmap* new_val = new WRPixmap();
    *new_val = ret_value;
    struct RUPixmap ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_pixmap_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPixmap pixmap_grab_widget(struct RUBase* self_c, struct RUBase* widget, struct RUBase* rect) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->grabWidget((QObject*)widget, *((QRect*)rect));
    WRPixmap* new_val = new WRPixmap();
    *new_val = ret_value;
    struct RUPixmap ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_pixmap_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPixmap pixmap_grab_widget_2(struct RUBase* self_c, struct RUBase* widget, int x, int y, int w, int h) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->grabWidget((QObject*)widget, x, y, w, h);
    WRPixmap* new_val = new WRPixmap();
    *new_val = ret_value;
    struct RUPixmap ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_pixmap_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPixmap pixmap_scaled(struct RUBase* self_c, int w, int h, uint32_t aspect_mode, uint32_t mode) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->scaled(w, h, (Qt::AspectRatioMode)aspect_mode, (Qt::TransformationMode)mode);
    WRPixmap* new_val = new WRPixmap();
    *new_val = ret_value;
    struct RUPixmap ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_pixmap_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPixmap pixmap_scaled_2(struct RUBase* self_c, struct RUBase* s, uint32_t aspect_mode, uint32_t mode) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->scaled(*((QSize*)s), (Qt::AspectRatioMode)aspect_mode, (Qt::TransformationMode)mode);
    WRPixmap* new_val = new WRPixmap();
    *new_val = ret_value;
    struct RUPixmap ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_pixmap_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPixmap pixmap_scaled_to_width(struct RUBase* self_c, int w, uint32_t mode) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->scaledToWidth(w, (Qt::TransformationMode)mode);
    WRPixmap* new_val = new WRPixmap();
    *new_val = ret_value;
    struct RUPixmap ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_pixmap_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPixmap pixmap_scaled_to_height(struct RUBase* self_c, int h, uint32_t mode) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->scaledToHeight(h, (Qt::TransformationMode)mode);
    WRPixmap* new_val = new WRPixmap();
    *new_val = ret_value;
    struct RUPixmap ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_pixmap_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUImage pixmap_to_image(struct RUBase* self_c) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->toImage();
    WRImage* new_val = new WRImage();
    *new_val = ret_value;
    struct RUImage ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_image_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPixmap pixmap_from_image(struct RUBase* self_c, struct RUBase* image, uint32_t flags) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->fromImage(*((QImage*)image), (Qt::ImageConversionFlags)flags);
    WRPixmap* new_val = new WRPixmap();
    *new_val = ret_value;
    struct RUPixmap ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_pixmap_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPixmap pixmap_from_image_2(struct RUBase* self_c, struct RUBase* image, uint32_t flags) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->fromImage(*((QImage*)image), (Qt::ImageConversionFlags)flags);
    WRPixmap* new_val = new WRPixmap();
    *new_val = ret_value;
    struct RUPixmap ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_pixmap_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool pixmap_convert_from_image(struct RUBase* self_c, struct RUBase* img, uint32_t flags) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->convertFromImage(*((QImage*)img), (Qt::ImageConversionFlags)flags);
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPixmap pixmap_copy(struct RUBase* self_c, int x, int y, int width, int height) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->copy(x, y, width, height);
    WRPixmap* new_val = new WRPixmap();
    *new_val = ret_value;
    struct RUPixmap ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_pixmap_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPixmap pixmap_copy_2(struct RUBase* self_c, struct RUBase* rect) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->copy(*((QRect*)rect));
    WRPixmap* new_val = new WRPixmap();
    *new_val = ret_value;
    struct RUPixmap ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_pixmap_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pixmap_scroll(struct RUBase* self_c, int dx, int dy, int x, int y, int width, int height, struct RUBase* exposed) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    qt_value->scroll(dx, dy, x, y, width, height, (QRegion*)exposed);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pixmap_scroll_2(struct RUBase* self_c, int dx, int dy, struct RUBase* rect, struct RUBase* exposed) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    qt_value->scroll(dx, dy, *((QRect*)rect), (QRegion*)exposed);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int64_t pixmap_cache_key(struct RUBase* self_c) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->cacheKey();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool pixmap_is_detached(struct RUBase* self_c) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->isDetached();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pixmap_detach(struct RUBase* self_c) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    qt_value->detach();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool pixmap_is_q_bitmap(struct RUBase* self_c) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->isQBitmap();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPaintEngine pixmap_paint_engine(struct RUBase* self_c) {
    WRPixmap* qt_value = (WRPixmap*)self_c;
    auto ret_value = qt_value->paintEngine();
    struct RUPaintEngine ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_paint_engine_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPixmap create_pixmap(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = generic_create_func_with_delete<struct RUPixmap, WRPixmap>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_pixmap_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_pixmap(struct RUBase* priv_data) {
    destroy_generic<WRPixmap>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPixmap get_pixmap(struct RUBase* priv_data) {
    (void)priv_data;
    RUPixmap ctl;
    ctl.qt_data = nullptr;
    ctl.host_data = nullptr;
    ctl.all_funcs = &s_pixmap_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUPixmapFuncs s_pixmap_funcs = {
    destroy_pixmap,
    pixmap_swap,
    pixmap_is_null,
    pixmap_width,
    pixmap_height,
    pixmap_size,
    pixmap_rect,
    pixmap_depth,
    pixmap_default_depth,
    pixmap_fill,
    pixmap_fill_2,
    pixmap_fill_3,
    pixmap_mask,
    pixmap_set_mask,
    pixmap_device_pixel_ratio,
    pixmap_set_device_pixel_ratio,
    pixmap_has_alpha,
    pixmap_has_alpha_channel,
    pixmap_create_heuristic_mask,
    pixmap_create_mask_from_color,
    pixmap_grab_window,
    pixmap_grab_widget,
    pixmap_grab_widget_2,
    pixmap_scaled,
    pixmap_scaled_2,
    pixmap_scaled_to_width,
    pixmap_scaled_to_height,
    pixmap_to_image,
    pixmap_from_image,
    pixmap_from_image_2,
    pixmap_convert_from_image,
    pixmap_copy,
    pixmap_copy_2,
    pixmap_scroll,
    pixmap_scroll_2,
    pixmap_cache_key,
    pixmap_is_detached,
    pixmap_detach,
    pixmap_is_q_bitmap,
    pixmap_paint_engine,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUPixmapAllFuncs s_pixmap_all_funcs = {
    &s_paint_device_funcs,
    &s_pixmap_funcs,
};

