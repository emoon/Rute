////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QPen>
#include "pen_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pen_swap(struct RUBase* self_c, struct RUBase* other) {
    WRPen* qt_value = (WRPen*)self_c;
    qt_value->swap(*((QPen*)other));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int pen_style(struct RUBase* self_c) {
    WRPen* qt_value = (WRPen*)self_c;
    auto ret_value = qt_value->style();
    return s_pen_style_lookup[(int)ret_value];
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pen_set_style(struct RUBase* self_c, int arg0) {
    WRPen* qt_value = (WRPen*)self_c;
    qt_value->setStyle((Qt::PenStyle)s_pen_style_lookup[arg0]);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUArray pen_dash_pattern(struct RUBase* self_c) {
    WRPen* qt_value = (WRPen*)self_c;
    auto ret_value = qt_value->dashPattern();
    RUArray ru_array = alloc_primitive_array(sizeof(float) * ret_value.size());
    ru_array.count = (uint32_t)ret_value.size();
    float* ru_dest = (float*)ru_array.elements;
    for (int i = 0, len = (int)ret_value.size(); i < len; ++i) {
        *ru_dest++ = (float)ret_value.at(i);
    }
    return ru_array;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float pen_dash_offset(struct RUBase* self_c) {
    WRPen* qt_value = (WRPen*)self_c;
    auto ret_value = qt_value->dashOffset();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pen_set_dash_offset(struct RUBase* self_c, float doffset) {
    WRPen* qt_value = (WRPen*)self_c;
    qt_value->setDashOffset(doffset);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float pen_miter_limit(struct RUBase* self_c) {
    WRPen* qt_value = (WRPen*)self_c;
    auto ret_value = qt_value->miterLimit();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pen_set_miter_limit(struct RUBase* self_c, float limit) {
    WRPen* qt_value = (WRPen*)self_c;
    qt_value->setMiterLimit(limit);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float pen_width_f(struct RUBase* self_c) {
    WRPen* qt_value = (WRPen*)self_c;
    auto ret_value = qt_value->widthF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pen_set_width_f(struct RUBase* self_c, float width) {
    WRPen* qt_value = (WRPen*)self_c;
    qt_value->setWidthF(width);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int pen_width(struct RUBase* self_c) {
    WRPen* qt_value = (WRPen*)self_c;
    auto ret_value = qt_value->width();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pen_set_width(struct RUBase* self_c, int width) {
    WRPen* qt_value = (WRPen*)self_c;
    qt_value->setWidth(width);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor pen_color(struct RUBase* self_c) {
    WRPen* qt_value = (WRPen*)self_c;
    auto ret_value = qt_value->color();
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pen_set_color(struct RUBase* self_c, struct RUBase* color) {
    WRPen* qt_value = (WRPen*)self_c;
    qt_value->setColor(*((QColor*)color));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush pen_brush(struct RUBase* self_c) {
    WRPen* qt_value = (WRPen*)self_c;
    auto ret_value = qt_value->brush();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pen_set_brush(struct RUBase* self_c, struct RUBase* brush) {
    WRPen* qt_value = (WRPen*)self_c;
    qt_value->setBrush(*((QBrush*)brush));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool pen_is_solid(struct RUBase* self_c) {
    WRPen* qt_value = (WRPen*)self_c;
    auto ret_value = qt_value->isSolid();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int pen_cap_style(struct RUBase* self_c) {
    WRPen* qt_value = (WRPen*)self_c;
    auto ret_value = qt_value->capStyle();
    return s_pen_cap_style_lookup[(int)ret_value];
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pen_set_cap_style(struct RUBase* self_c, int pcs) {
    WRPen* qt_value = (WRPen*)self_c;
    qt_value->setCapStyle((Qt::PenCapStyle)s_pen_cap_style_lookup[pcs]);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int pen_join_style(struct RUBase* self_c) {
    WRPen* qt_value = (WRPen*)self_c;
    auto ret_value = qt_value->joinStyle();
    return s_pen_join_style_lookup[(int)ret_value];
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pen_set_join_style(struct RUBase* self_c, int pcs) {
    WRPen* qt_value = (WRPen*)self_c;
    qt_value->setJoinStyle((Qt::PenJoinStyle)s_pen_join_style_lookup[pcs]);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool pen_is_cosmetic(struct RUBase* self_c) {
    WRPen* qt_value = (WRPen*)self_c;
    auto ret_value = qt_value->isCosmetic();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void pen_set_cosmetic(struct RUBase* self_c, bool cosmetic) {
    WRPen* qt_value = (WRPen*)self_c;
    qt_value->setCosmetic(cosmetic);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool pen_is_detached(struct RUBase* self_c) {
    WRPen* qt_value = (WRPen*)self_c;
    auto ret_value = qt_value->isDetached();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPen create_pen(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = generic_create_func_with_delete<struct RUPen, WRPen>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_pen_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_pen(struct RUBase* priv_data) {
    destroy_generic<WRPen>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUPenFuncs s_pen_funcs = {
    destroy_pen,
    pen_swap,
    pen_style,
    pen_set_style,
    pen_dash_pattern,
    pen_dash_offset,
    pen_set_dash_offset,
    pen_miter_limit,
    pen_set_miter_limit,
    pen_width_f,
    pen_set_width_f,
    pen_width,
    pen_set_width,
    pen_color,
    pen_set_color,
    pen_brush,
    pen_set_brush,
    pen_is_solid,
    pen_cap_style,
    pen_set_cap_style,
    pen_join_style,
    pen_set_join_style,
    pen_is_cosmetic,
    pen_set_cosmetic,
    pen_is_detached,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUPenAllFuncs s_pen_all_funcs = {
    &s_pen_funcs,
};

