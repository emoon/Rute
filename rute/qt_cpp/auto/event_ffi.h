// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif

struct RUEventFuncs;
struct RUEvent;

typedef struct RUEventFuncs {
    bool (*spontaneous)(struct RUBase* self_c);
    void (*set_accepted)(struct RUBase* self_c, bool accepted);
    bool (*is_accepted)(struct RUBase* self_c);
    void (*accept)(struct RUBase* self_c);
    void (*ignore)(struct RUBase* self_c);
} RUEventFuncs;

typedef struct RUEventAllFuncs {
    struct RUEventFuncs* event_funcs;
} RUEventAllFuncs;

typedef struct RUEvent {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUEventAllFuncs* all_funcs;
} RUEvent;

extern RUEventFuncs s_event_funcs;
extern RUEventAllFuncs s_event_all_funcs;

#ifdef __cplusplus
}
#endif
