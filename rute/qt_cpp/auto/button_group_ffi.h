// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "abstract_button_ffi.h"

struct RUButtonGroupFuncs;
struct RUButtonGroup;

typedef struct RUButtonGroupFuncs {
    void (*set_exclusive)(struct RUBase* self_c, bool arg0);
    bool (*exclusive)(struct RUBase* self_c);
    void (*add_button)(struct RUBase* self_c, struct RUBase* arg0, int id);
    void (*remove_button)(struct RUBase* self_c, struct RUBase* arg0);
    struct RUAbstractButton (*checked_button)(struct RUBase* self_c);
    struct RUAbstractButton (*button)(struct RUBase* self_c, int id);
    void (*set_id)(struct RUBase* self_c, struct RUBase* button, int id);
    int (*id)(struct RUBase* self_c, struct RUBase* button);
    int (*checked_id)(struct RUBase* self_c);
    void (*set_button_clicked_event)(void* object, void* user_data,
                                     void* wrapped_func,
                                     void (*event)(void*, void* self_c,
                                                   struct RUBase* arg0));
    void (*set_button_clicked_2_event)(void* object, void* user_data,
                                       void* wrapped_func,
                                       void (*event)(void*, void* self_c,
                                                     int arg0));
    void (*set_button_pressed_event)(void* object, void* user_data,
                                     void* wrapped_func,
                                     void (*event)(void*, void* self_c,
                                                   struct RUBase* arg0));
    void (*set_button_pressed_2_event)(void* object, void* user_data,
                                       void* wrapped_func,
                                       void (*event)(void*, void* self_c,
                                                     int arg0));
    void (*set_button_released_event)(void* object, void* user_data,
                                      void* wrapped_func,
                                      void (*event)(void*, void* self_c,
                                                    struct RUBase* arg0));
    void (*set_button_released_2_event)(void* object, void* user_data,
                                        void* wrapped_func,
                                        void (*event)(void*, void* self_c,
                                                      int arg0));
    void (*set_button_toggled_event)(
        void* object, void* user_data, void* wrapped_func,
        void (*event)(void*, void* self_c, struct RUBase* arg0, bool arg1));
    void (*set_button_toggled_2_event)(void* object, void* user_data,
                                       void* wrapped_func,
                                       void (*event)(void*, void* self_c,
                                                     int arg0, bool arg1));
} RUButtonGroupFuncs;

typedef struct RUButtonGroupAllFuncs {
    struct RUObjectFuncs* object_funcs;
    struct RUButtonGroupFuncs* button_group_funcs;
} RUButtonGroupAllFuncs;

typedef struct RUButtonGroup {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUButtonGroupAllFuncs* all_funcs;
} RUButtonGroup;

extern RUButtonGroupFuncs s_button_group_funcs;
extern RUButtonGroupAllFuncs s_button_group_all_funcs;

#ifdef __cplusplus
}
#endif
