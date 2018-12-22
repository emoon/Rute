////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QTabletEvent>
#include "tablet_event_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPoint tablet_event_pos(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
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

static struct RUPoint tablet_event_global_pos(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
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

static struct RUPointF tablet_event_pos_f(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
    auto ret_value = qt_value->posF();
    WRPointF* new_val = new WRPointF();
    *new_val = ret_value;
    struct RUPointF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_point_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPointF tablet_event_global_pos_f(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
    auto ret_value = qt_value->globalPosF();
    WRPointF* new_val = new WRPointF();
    *new_val = ret_value;
    struct RUPointF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_point_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int tablet_event_x(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
    auto ret_value = qt_value->x();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int tablet_event_y(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
    auto ret_value = qt_value->y();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int tablet_event_global_x(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
    auto ret_value = qt_value->globalX();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int tablet_event_global_y(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
    auto ret_value = qt_value->globalY();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float tablet_event_hi_res_global_x(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
    auto ret_value = qt_value->hiResGlobalX();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float tablet_event_hi_res_global_y(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
    auto ret_value = qt_value->hiResGlobalY();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t tablet_event_device(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
    auto ret_value = qt_value->device();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t tablet_event_pointer_type(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
    auto ret_value = qt_value->pointerType();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int64_t tablet_event_unique_id(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
    auto ret_value = qt_value->uniqueId();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float tablet_event_pressure(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
    auto ret_value = qt_value->pressure();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int tablet_event_z(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
    auto ret_value = qt_value->z();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float tablet_event_tangential_pressure(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
    auto ret_value = qt_value->tangentialPressure();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float tablet_event_rotation(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
    auto ret_value = qt_value->rotation();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int tablet_event_x_tilt(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
    auto ret_value = qt_value->xTilt();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int tablet_event_y_tilt(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
    auto ret_value = qt_value->yTilt();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t tablet_event_button(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
    auto ret_value = qt_value->button();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t tablet_event_buttons(struct RUBase* self_c) {
    QTabletEvent* qt_value = (QTabletEvent*)self_c;
    auto ret_value = qt_value->buttons();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUTabletEventFuncs s_tablet_event_funcs = {
    tablet_event_pos,
    tablet_event_global_pos,
    tablet_event_pos_f,
    tablet_event_global_pos_f,
    tablet_event_x,
    tablet_event_y,
    tablet_event_global_x,
    tablet_event_global_y,
    tablet_event_hi_res_global_x,
    tablet_event_hi_res_global_y,
    tablet_event_device,
    tablet_event_pointer_type,
    tablet_event_unique_id,
    tablet_event_pressure,
    tablet_event_z,
    tablet_event_tangential_pressure,
    tablet_event_rotation,
    tablet_event_x_tilt,
    tablet_event_y_tilt,
    tablet_event_button,
    tablet_event_buttons,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUTabletEventAllFuncs s_tablet_event_all_funcs = {
    &s_event_funcs,
    &s_input_event_funcs,
    &s_tablet_event_funcs,
};

