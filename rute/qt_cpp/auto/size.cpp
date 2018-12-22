////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QSize>
#include "size_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool size_is_null(struct RUBase* self_c) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->isNull();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool size_is_empty(struct RUBase* self_c) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->isEmpty();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool size_is_valid(struct RUBase* self_c) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->isValid();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int size_width(struct RUBase* self_c) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->width();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int size_height(struct RUBase* self_c) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->height();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void size_set_width(struct RUBase* self_c, int w) {
    WRSize* qt_value = (WRSize*)self_c;
    qt_value->setWidth(w);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void size_set_height(struct RUBase* self_c, int h) {
    WRSize* qt_value = (WRSize*)self_c;
    qt_value->setHeight(h);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void size_scale(struct RUBase* self_c, int w, int h, uint32_t mode) {
    WRSize* qt_value = (WRSize*)self_c;
    qt_value->scale(w, h, (Qt::AspectRatioMode)mode);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void size_scale_2(struct RUBase* self_c, struct RUBase* s, uint32_t mode) {
    WRSize* qt_value = (WRSize*)self_c;
    qt_value->scale(*((QSize*)s), (Qt::AspectRatioMode)mode);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize size_scaled(struct RUBase* self_c, int w, int h, uint32_t mode) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->scaled(w, h, (Qt::AspectRatioMode)mode);
    WRSize* new_val = new WRSize();
    *new_val = ret_value;
    struct RUSize ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_size_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize size_scaled_2(struct RUBase* self_c, struct RUBase* s, uint32_t mode) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->scaled(*((QSize*)s), (Qt::AspectRatioMode)mode);
    WRSize* new_val = new WRSize();
    *new_val = ret_value;
    struct RUSize ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_size_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize size_expanded_to(struct RUBase* self_c, struct RUBase* arg0) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->expandedTo(*((QSize*)arg0));
    WRSize* new_val = new WRSize();
    *new_val = ret_value;
    struct RUSize ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_size_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize size_bounded_to(struct RUBase* self_c, struct RUBase* arg0) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->boundedTo(*((QSize*)arg0));
    WRSize* new_val = new WRSize();
    *new_val = ret_value;
    struct RUSize ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_size_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int size_rwidth(struct RUBase* self_c) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->rwidth();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int size_rheight(struct RUBase* self_c) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->rheight();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize create_size(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = generic_create_func_with_delete<struct RUSize, WRSize>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_size_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_size(struct RUBase* priv_data) {
    destroy_generic<WRSize>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUSizeFuncs s_size_funcs = {
    destroy_size,
    size_is_null,
    size_is_empty,
    size_is_valid,
    size_width,
    size_height,
    size_set_width,
    size_set_height,
    size_scale,
    size_scale_2,
    size_scaled,
    size_scaled_2,
    size_expanded_to,
    size_bounded_to,
    size_rwidth,
    size_rheight,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUSizeAllFuncs s_size_all_funcs = {
    &s_size_funcs,
};

