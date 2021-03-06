////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QLineF>
#include "line_f_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RULineF line_f_from_polar(struct RUBase* self_c, float length, float angle) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->fromPolar(length, angle);
    WRLineF* new_val = new WRLineF();
    *new_val = ret_value;
    struct RULineF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_line_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool line_f_is_null(struct RUBase* self_c) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->isNull();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPointF line_f_p1(struct RUBase* self_c) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->p1();
    WRPointF* new_val = new WRPointF();
    *new_val = ret_value;
    struct RUPointF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_point_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPointF line_f_p2(struct RUBase* self_c) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->p2();
    WRPointF* new_val = new WRPointF();
    *new_val = ret_value;
    struct RUPointF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_point_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float line_f_x1(struct RUBase* self_c) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->x1();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float line_f_y1(struct RUBase* self_c) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->y1();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float line_f_x2(struct RUBase* self_c) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->x2();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float line_f_y2(struct RUBase* self_c) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->y2();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float line_f_dx(struct RUBase* self_c) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->dx();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float line_f_dy(struct RUBase* self_c) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->dy();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float line_f_length(struct RUBase* self_c) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->length();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_f_set_length(struct RUBase* self_c, float len) {
    WRLineF* qt_value = (WRLineF*)self_c;
    qt_value->setLength(len);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float line_f_angle(struct RUBase* self_c) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->angle();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_f_set_angle(struct RUBase* self_c, float angle) {
    WRLineF* qt_value = (WRLineF*)self_c;
    qt_value->setAngle(angle);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float line_f_angle_to(struct RUBase* self_c, struct RUBase* l) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->angleTo(*((WRLineF*)l));
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RULineF line_f_unit_vector(struct RUBase* self_c) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->unitVector();
    WRLineF* new_val = new WRLineF();
    *new_val = ret_value;
    struct RULineF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_line_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RULineF line_f_normal_vector(struct RUBase* self_c) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->normalVector();
    WRLineF* new_val = new WRLineF();
    *new_val = ret_value;
    struct RULineF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_line_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t line_f_intersect(struct RUBase* self_c, struct RUBase* l, struct RUBase* intersection_point) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->intersect(*((WRLineF*)l), (WRPointF*)intersection_point);
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float line_f_angle_2(struct RUBase* self_c, struct RUBase* l) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->angle(*((WRLineF*)l));
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPointF line_f_point_at(struct RUBase* self_c, float t) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->pointAt(t);
    WRPointF* new_val = new WRPointF();
    *new_val = ret_value;
    struct RUPointF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_point_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPointF line_f_center(struct RUBase* self_c) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->center();
    WRPointF* new_val = new WRPointF();
    *new_val = ret_value;
    struct RUPointF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_point_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_f_set_p1(struct RUBase* self_c, struct RUBase* p1) {
    WRLineF* qt_value = (WRLineF*)self_c;
    qt_value->setP1(*((WRPointF*)p1));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_f_set_p2(struct RUBase* self_c, struct RUBase* p2) {
    WRLineF* qt_value = (WRLineF*)self_c;
    qt_value->setP2(*((WRPointF*)p2));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_f_set_points(struct RUBase* self_c, struct RUBase* p1, struct RUBase* p2) {
    WRLineF* qt_value = (WRLineF*)self_c;
    qt_value->setPoints(*((WRPointF*)p1), *((WRPointF*)p2));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_f_set_line(struct RUBase* self_c, float x1, float y1, float x2, float y2) {
    WRLineF* qt_value = (WRLineF*)self_c;
    qt_value->setLine(x1, y1, x2, y2);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RULine line_f_to_line(struct RUBase* self_c) {
    WRLineF* qt_value = (WRLineF*)self_c;
    auto ret_value = qt_value->toLine();
    WRLine* new_val = new WRLine();
    *new_val = ret_value;
    struct RULine ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_line_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RULineF create_line_f(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = generic_create_func_with_delete<struct RULineF, WRLineF>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_line_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_line_f(struct RUBase* priv_data) {
    destroy_generic<WRLineF>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RULineF get_line_f(struct RUBase* priv_data) {
    (void)priv_data;
    RULineF ctl;
    ctl.qt_data = nullptr;
    ctl.host_data = nullptr;
    ctl.all_funcs = &s_line_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RULineFFuncs s_line_f_funcs = {
    destroy_line_f,
    line_f_from_polar,
    line_f_is_null,
    line_f_p1,
    line_f_p2,
    line_f_x1,
    line_f_y1,
    line_f_x2,
    line_f_y2,
    line_f_dx,
    line_f_dy,
    line_f_length,
    line_f_set_length,
    line_f_angle,
    line_f_set_angle,
    line_f_angle_to,
    line_f_unit_vector,
    line_f_normal_vector,
    line_f_intersect,
    line_f_angle_2,
    line_f_point_at,
    line_f_center,
    line_f_set_p1,
    line_f_set_p2,
    line_f_set_points,
    line_f_set_line,
    line_f_to_line,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RULineFAllFuncs s_line_f_all_funcs = {
    &s_line_f_funcs,
};

