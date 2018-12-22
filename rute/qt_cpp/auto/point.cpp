////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QPoint>
#include "point_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool point_is_null(struct RUBase* self_c) {
    WRPoint* qt_value = (WRPoint*)self_c;
    auto ret_value = qt_value->isNull();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int point_x(struct RUBase* self_c) {
    WRPoint* qt_value = (WRPoint*)self_c;
    auto ret_value = qt_value->x();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int point_y(struct RUBase* self_c) {
    WRPoint* qt_value = (WRPoint*)self_c;
    auto ret_value = qt_value->y();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void point_set_x(struct RUBase* self_c, int x) {
    WRPoint* qt_value = (WRPoint*)self_c;
    qt_value->setX(x);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void point_set_y(struct RUBase* self_c, int y) {
    WRPoint* qt_value = (WRPoint*)self_c;
    qt_value->setY(y);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int point_manhattan_length(struct RUBase* self_c) {
    WRPoint* qt_value = (WRPoint*)self_c;
    auto ret_value = qt_value->manhattanLength();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int point_rx(struct RUBase* self_c) {
    WRPoint* qt_value = (WRPoint*)self_c;
    auto ret_value = qt_value->rx();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int point_ry(struct RUBase* self_c) {
    WRPoint* qt_value = (WRPoint*)self_c;
    auto ret_value = qt_value->ry();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int point_dot_product(struct RUBase* self_c, struct RUBase* p1, struct RUBase* p2) {
    WRPoint* qt_value = (WRPoint*)self_c;
    auto ret_value = qt_value->dotProduct(*((WRPoint*)p1), *((WRPoint*)p2));
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPoint create_point(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = generic_create_func_with_delete<struct RUPoint, WRPoint>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_point_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_point(struct RUBase* priv_data) {
    destroy_generic<WRPoint>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPoint get_point(struct RUBase* priv_data) {
    (void)priv_data;
    RUPoint ctl;
    ctl.qt_data = nullptr;
    ctl.host_data = nullptr;
    ctl.all_funcs = &s_point_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUPointFuncs s_point_funcs = {
    destroy_point,
    point_is_null,
    point_x,
    point_y,
    point_set_x,
    point_set_y,
    point_manhattan_length,
    point_rx,
    point_ry,
    point_dot_product,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUPointAllFuncs s_point_all_funcs = {
    &s_point_funcs,
};

