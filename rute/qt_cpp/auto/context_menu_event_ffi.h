// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "point_ffi.h"

struct RUContextMenuEventFuncs;
struct RUContextMenuEvent;

typedef struct RUContextMenuEventFuncs {
    int (*x)(struct RUBase* self_c);
    int (*y)(struct RUBase* self_c);
    int (*global_x)(struct RUBase* self_c);
    int (*global_y)(struct RUBase* self_c);
    struct RUPoint (*pos)(struct RUBase* self_c);
    struct RUPoint (*global_pos)(struct RUBase* self_c);
    uint32_t (*reason)(struct RUBase* self_c);
} RUContextMenuEventFuncs;

typedef struct RUContextMenuEventAllFuncs {
    struct RUEventFuncs* event_funcs;
    struct RUInputEventFuncs* input_event_funcs;
    struct RUContextMenuEventFuncs* context_menu_event_funcs;
} RUContextMenuEventAllFuncs;

typedef struct RUContextMenuEvent {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUContextMenuEventAllFuncs* all_funcs;
} RUContextMenuEvent;

extern RUContextMenuEventFuncs s_context_menu_event_funcs;
extern RUContextMenuEventAllFuncs s_context_menu_event_all_funcs;

#ifdef __cplusplus
}
#endif
