////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QRectF>
#include "rect_f_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool rect_f_is_null(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->isNull();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool rect_f_is_empty(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->isEmpty();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool rect_f_is_valid(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->isValid();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RURectF rect_f_normalized(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->normalized();
    WRRectF* new_val = new WRRectF();
    *new_val = ret_value;
    struct RURectF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_rect_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float rect_f_left(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->left();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float rect_f_top(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->top();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float rect_f_right(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->right();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float rect_f_bottom(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->bottom();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float rect_f_x(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->x();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float rect_f_y(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->y();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_set_left(struct RUBase* self_c, float pos) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->setLeft(pos);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_set_top(struct RUBase* self_c, float pos) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->setTop(pos);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_set_right(struct RUBase* self_c, float pos) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->setRight(pos);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_set_bottom(struct RUBase* self_c, float pos) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->setBottom(pos);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_set_x(struct RUBase* self_c, float pos) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->setX(pos);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_set_y(struct RUBase* self_c, float pos) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->setY(pos);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPointF rect_f_top_left(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->topLeft();
    WRPointF* new_val = new WRPointF();
    *new_val = ret_value;
    struct RUPointF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_point_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPointF rect_f_bottom_right(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->bottomRight();
    WRPointF* new_val = new WRPointF();
    *new_val = ret_value;
    struct RUPointF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_point_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPointF rect_f_top_right(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->topRight();
    WRPointF* new_val = new WRPointF();
    *new_val = ret_value;
    struct RUPointF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_point_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPointF rect_f_bottom_left(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->bottomLeft();
    WRPointF* new_val = new WRPointF();
    *new_val = ret_value;
    struct RUPointF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_point_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPointF rect_f_center(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
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

static void rect_f_set_top_left(struct RUBase* self_c, struct RUBase* p) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->setTopLeft(*((QPointF*)p));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_set_bottom_right(struct RUBase* self_c, struct RUBase* p) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->setBottomRight(*((QPointF*)p));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_set_top_right(struct RUBase* self_c, struct RUBase* p) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->setTopRight(*((QPointF*)p));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_set_bottom_left(struct RUBase* self_c, struct RUBase* p) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->setBottomLeft(*((QPointF*)p));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_move_left(struct RUBase* self_c, float pos) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->moveLeft(pos);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_move_top(struct RUBase* self_c, float pos) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->moveTop(pos);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_move_right(struct RUBase* self_c, float pos) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->moveRight(pos);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_move_bottom(struct RUBase* self_c, float pos) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->moveBottom(pos);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_move_top_left(struct RUBase* self_c, struct RUBase* p) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->moveTopLeft(*((QPointF*)p));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_move_bottom_right(struct RUBase* self_c, struct RUBase* p) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->moveBottomRight(*((QPointF*)p));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_move_top_right(struct RUBase* self_c, struct RUBase* p) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->moveTopRight(*((QPointF*)p));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_move_bottom_left(struct RUBase* self_c, struct RUBase* p) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->moveBottomLeft(*((QPointF*)p));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_move_center(struct RUBase* self_c, struct RUBase* p) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->moveCenter(*((QPointF*)p));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_move_to(struct RUBase* self_c, float x, float y) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->moveTo(x, y);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_move_to(struct RUBase* self_c, struct RUBase* p) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->moveTo(*((QPointF*)p));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_set_rect(struct RUBase* self_c, float x, float y, float w, float h) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->setRect(x, y, w, h);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_get_rect(struct RUBase* self_c, struct RUBase* x, struct RUBase* y, struct RUBase* w, struct RUBase* h) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->getRect(x, y, w, h);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_set_coords(struct RUBase* self_c, float x1, float y1, float x2, float y2) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->setCoords(x1, y1, x2, y2);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_get_coords(struct RUBase* self_c, struct RUBase* x1, struct RUBase* y1, struct RUBase* x2, struct RUBase* y2) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->getCoords(x1, y1, x2, y2);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_adjust(struct RUBase* self_c, float x1, float y1, float x2, float y2) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->adjust(x1, y1, x2, y2);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RURectF rect_f_adjusted(struct RUBase* self_c, float x1, float y1, float x2, float y2) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->adjusted(x1, y1, x2, y2);
    WRRectF* new_val = new WRRectF();
    *new_val = ret_value;
    struct RURectF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_rect_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSizeF rect_f_size(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->size();
    WRSizeF* new_val = new WRSizeF();
    *new_val = ret_value;
    struct RUSizeF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_size_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float rect_f_width(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->width();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float rect_f_height(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->height();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_set_width(struct RUBase* self_c, float w) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->setWidth(w);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_set_height(struct RUBase* self_c, float h) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->setHeight(h);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void rect_f_set_size(struct RUBase* self_c, struct RUBase* s) {
    WRRectF* qt_value = (WRRectF*)self_c;
    qt_value->setSize(*((QSizeF*)s));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool rect_f_contains(struct RUBase* self_c, struct RUBase* r) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->contains(*((QRectF*)r));
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool rect_f_contains(struct RUBase* self_c, struct RUBase* p) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->contains(*((QPointF*)p));
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool rect_f_contains(struct RUBase* self_c, float x, float y) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->contains(x, y);
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RURectF rect_f_united(struct RUBase* self_c, struct RUBase* other) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->united(*((QRectF*)other));
    WRRectF* new_val = new WRRectF();
    *new_val = ret_value;
    struct RURectF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_rect_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RURectF rect_f_intersected(struct RUBase* self_c, struct RUBase* other) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->intersected(*((QRectF*)other));
    WRRectF* new_val = new WRRectF();
    *new_val = ret_value;
    struct RURectF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_rect_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool rect_f_intersects(struct RUBase* self_c, struct RUBase* r) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->intersects(*((QRectF*)r));
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RURectF rect_f_margins_added(struct RUBase* self_c, struct RUBase* margins) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->marginsAdded(*((QMarginsF*)margins));
    WRRectF* new_val = new WRRectF();
    *new_val = ret_value;
    struct RURectF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_rect_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RURectF rect_f_margins_removed(struct RUBase* self_c, struct RUBase* margins) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->marginsRemoved(*((QMarginsF*)margins));
    WRRectF* new_val = new WRRectF();
    *new_val = ret_value;
    struct RURectF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_rect_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RURect rect_f_to_rect(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->toRect();
    WRRect* new_val = new WRRect();
    *new_val = ret_value;
    struct RURect ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_rect_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RURect rect_f_to_aligned_rect(struct RUBase* self_c) {
    WRRectF* qt_value = (WRRectF*)self_c;
    auto ret_value = qt_value->toAlignedRect();
    WRRect* new_val = new WRRect();
    *new_val = ret_value;
    struct RURect ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_rect_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RURectF create_rect_f(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = generic_create_func_with_delete<struct RURectF, WRRectF>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_rect_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_rect_f(struct RUBase* priv_data) {
    destroy_generic<WRRectF>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RURectFFuncs s_rect_f_funcs = {
    destroy_rect_f,
    rect_f_is_null,
    rect_f_is_empty,
    rect_f_is_valid,
    rect_f_normalized,
    rect_f_left,
    rect_f_top,
    rect_f_right,
    rect_f_bottom,
    rect_f_x,
    rect_f_y,
    rect_f_set_left,
    rect_f_set_top,
    rect_f_set_right,
    rect_f_set_bottom,
    rect_f_set_x,
    rect_f_set_y,
    rect_f_top_left,
    rect_f_bottom_right,
    rect_f_top_right,
    rect_f_bottom_left,
    rect_f_center,
    rect_f_set_top_left,
    rect_f_set_bottom_right,
    rect_f_set_top_right,
    rect_f_set_bottom_left,
    rect_f_move_left,
    rect_f_move_top,
    rect_f_move_right,
    rect_f_move_bottom,
    rect_f_move_top_left,
    rect_f_move_bottom_right,
    rect_f_move_top_right,
    rect_f_move_bottom_left,
    rect_f_move_center,
    rect_f_move_to,
    rect_f_move_to,
    rect_f_set_rect,
    rect_f_get_rect,
    rect_f_set_coords,
    rect_f_get_coords,
    rect_f_adjust,
    rect_f_adjusted,
    rect_f_size,
    rect_f_width,
    rect_f_height,
    rect_f_set_width,
    rect_f_set_height,
    rect_f_set_size,
    rect_f_contains,
    rect_f_contains,
    rect_f_contains,
    rect_f_united,
    rect_f_intersected,
    rect_f_intersects,
    rect_f_margins_added,
    rect_f_margins_removed,
    rect_f_to_rect,
    rect_f_to_aligned_rect,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RURectFAllFuncs s_rect_f_all_funcs = {
    &s_rect_f_funcs,
};
