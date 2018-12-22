////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QMouseEvent>
#include "mouse_event_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPoint mouse_event_pos(struct RUBase* self_c) {
    QMouseEvent* qt_value = (QMouseEvent*)self_c;
    auto ret_value = qt_value->pos();
    WRPoint* new_val = new WRPoint();
    *new_val = ret_value;
    struct RUPoint ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_point_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPoint mouse_event_global_pos(struct RUBase* self_c) {
    QMouseEvent* qt_value = (QMouseEvent*)self_c;
    auto ret_value = qt_value->globalPos();
    WRPoint* new_val = new WRPoint();
    *new_val = ret_value;
    struct RUPoint ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_point_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int mouse_event_x(struct RUBase* self_c) {
    QMouseEvent* qt_value = (QMouseEvent*)self_c;
    auto ret_value = qt_value->x();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int mouse_event_y(struct RUBase* self_c) {
    QMouseEvent* qt_value = (QMouseEvent*)self_c;
    auto ret_value = qt_value->y();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int mouse_event_global_x(struct RUBase* self_c) {
    QMouseEvent* qt_value = (QMouseEvent*)self_c;
    auto ret_value = qt_value->globalX();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int mouse_event_global_y(struct RUBase* self_c) {
    QMouseEvent* qt_value = (QMouseEvent*)self_c;
    auto ret_value = qt_value->globalY();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPointF mouse_event_local_pos(struct RUBase* self_c) {
    QMouseEvent* qt_value = (QMouseEvent*)self_c;
    auto ret_value = qt_value->localPos();
    WRPointF* new_val = new WRPointF();
    *new_val = ret_value;
    struct RUPointF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_point_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPointF mouse_event_window_pos(struct RUBase* self_c) {
    QMouseEvent* qt_value = (QMouseEvent*)self_c;
    auto ret_value = qt_value->windowPos();
    WRPointF* new_val = new WRPointF();
    *new_val = ret_value;
    struct RUPointF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_point_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPointF mouse_event_screen_pos(struct RUBase* self_c) {
    QMouseEvent* qt_value = (QMouseEvent*)self_c;
    auto ret_value = qt_value->screenPos();
    WRPointF* new_val = new WRPointF();
    *new_val = ret_value;
    struct RUPointF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_point_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t mouse_event_button(struct RUBase* self_c) {
    QMouseEvent* qt_value = (QMouseEvent*)self_c;
    auto ret_value = qt_value->button();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t mouse_event_buttons(struct RUBase* self_c) {
    QMouseEvent* qt_value = (QMouseEvent*)self_c;
    auto ret_value = qt_value->buttons();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void mouse_event_set_local_pos(struct RUBase* self_c, struct RUBase* local_position) {
    QMouseEvent* qt_value = (QMouseEvent*)self_c;
    qt_value->setLocalPos(*((WRPointF*)local_position));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t mouse_event_source(struct RUBase* self_c) {
    QMouseEvent* qt_value = (QMouseEvent*)self_c;
    auto ret_value = qt_value->source();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t mouse_event_flags(struct RUBase* self_c) {
    QMouseEvent* qt_value = (QMouseEvent*)self_c;
    auto ret_value = qt_value->flags();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUMouseEventFuncs s_mouse_event_funcs = {
    mouse_event_pos,
    mouse_event_global_pos,
    mouse_event_x,
    mouse_event_y,
    mouse_event_global_x,
    mouse_event_global_y,
    mouse_event_local_pos,
    mouse_event_window_pos,
    mouse_event_screen_pos,
    mouse_event_button,
    mouse_event_buttons,
    mouse_event_set_local_pos,
    mouse_event_source,
    mouse_event_flags,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUMouseEventAllFuncs s_mouse_event_all_funcs = {
    &s_event_funcs,
    &s_input_event_funcs,
    &s_mouse_event_funcs,
};

