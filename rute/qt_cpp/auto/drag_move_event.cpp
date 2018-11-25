////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QDragMoveEvent>
#include "drag_move_event_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RURect drag_move_event_answer_rect(struct RUBase* self_c) {
    QDragMoveEvent* qt_value = (QDragMoveEvent*)self_c;
    auto ret_value = qt_value->answerRect();
    WRRect* new_val = new WRRect();
    *new_val = ret_value;
    struct RURect ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_rect_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void drag_move_event_accept(struct RUBase* self_c) {
    QDragMoveEvent* qt_value = (QDragMoveEvent*)self_c;
    qt_value->accept();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void drag_move_event_ignore(struct RUBase* self_c) {
    QDragMoveEvent* qt_value = (QDragMoveEvent*)self_c;
    qt_value->ignore();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void drag_move_event_accept_2(struct RUBase* self_c, struct RUBase* r) {
    QDragMoveEvent* qt_value = (QDragMoveEvent*)self_c;
    qt_value->accept(*((QRect*)r));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void drag_move_event_ignore_2(struct RUBase* self_c, struct RUBase* r) {
    QDragMoveEvent* qt_value = (QDragMoveEvent*)self_c;
    qt_value->ignore(*((QRect*)r));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUDragMoveEventFuncs s_drag_move_event_funcs = {
    drag_move_event_answer_rect,
    drag_move_event_accept,
    drag_move_event_ignore,
    drag_move_event_accept_2,
    drag_move_event_ignore_2,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUDragMoveEventAllFuncs s_drag_move_event_all_funcs = {
    &s_event_funcs,
    &s_drop_event_funcs,
    &s_drag_move_event_funcs,
};

