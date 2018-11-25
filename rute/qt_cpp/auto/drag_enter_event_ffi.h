// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif

struct RUDragEnterEventFuncs;
struct RUDragEnterEvent;

typedef struct RUDragEnterEventFuncs {
} RUDragEnterEventFuncs;

typedef struct RUDragEnterEventAllFuncs {
    struct RUEventFuncs* event_funcs;
    struct RUDropEventFuncs* drop_event_funcs;
    struct RUDragMoveEventFuncs* drag_move_event_funcs;
    struct RUDragEnterEventFuncs* drag_enter_event_funcs;
} RUDragEnterEventAllFuncs;

typedef struct RUDragEnterEvent {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUDragEnterEventAllFuncs* all_funcs;
} RUDragEnterEvent;

extern RUDragEnterEventFuncs s_drag_enter_event_funcs;
extern RUDragEnterEventAllFuncs s_drag_enter_event_all_funcs;

#ifdef __cplusplus
}
#endif