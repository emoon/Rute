// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "object_ffi.h"

struct RUObjectFuncs;
struct RUObject;

typedef struct RUObjectFuncs {
    void (*destroy)(struct RUBase* self);
    const char* (*object_name)(struct RUBase* self_c);
    void (*set_object_name)(struct RUBase* self_c, const char* name);
    bool (*is_widget_type)(struct RUBase* self_c);
    bool (*is_window_type)(struct RUBase* self_c);
    bool (*signals_blocked)(struct RUBase* self_c);
    bool (*block_signals)(struct RUBase* self_c, bool b);
    int (*start_timer)(struct RUBase* self_c, int interval, int timer_type);
    int (*start_timer_2)(struct RUBase* self_c, uint32_t time, int timer_type);
    void (*kill_timer)(struct RUBase* self_c, int id);
    void (*set_parent)(struct RUBase* self_c, struct RUBase* parent);
    void (*install_event_filter)(struct RUBase* self_c,
                                 struct RUBase* filter_obj);
    void (*dump_object_tree)(struct RUBase* self_c);
    void (*dump_object_info)(struct RUBase* self_c);
    void (*dump_object_tree_2)(struct RUBase* self_c);
    void (*dump_object_info_2)(struct RUBase* self_c);
    struct RUObject (*parent)(struct RUBase* self_c);
    void (*delete_later)(struct RUBase* self_c);
    void (*set_custom_event)(void* object, void* user_data, void* wrapped_func,
                             void (*event)(void*, void* self_c,
                                           struct RUBase* event));
    void (*remove_custom_event)(void* object);
} RUObjectFuncs;

typedef struct RUObjectAllFuncs {
    struct RUObjectFuncs* object_funcs;
} RUObjectAllFuncs;

typedef struct RUObject {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUObjectAllFuncs* all_funcs;
} RUObject;

extern RUObjectFuncs s_object_funcs;
extern RUObjectAllFuncs s_object_all_funcs;

#ifdef __cplusplus
}
#endif
