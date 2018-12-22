// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif

struct RUToolButtonFuncs;
struct RUToolButton;

typedef struct RUToolButtonFuncs {
    void (*destroy)(struct RUBase* self);
    uint32_t (*tool_button_style)(struct RUBase* self_c);
    uint32_t (*arrow_type)(struct RUBase* self_c);
    void (*set_arrow_type)(struct RUBase* self_c, uint32_t arrow_type);
    void (*set_popup_mode)(struct RUBase* self_c, uint32_t mode);
    uint32_t (*popup_mode)(struct RUBase* self_c);
    bool (*auto_raise)(struct RUBase* self_c);
    void (*show_menu)(struct RUBase* self_c);
    void (*set_tool_button_style)(struct RUBase* self_c, uint32_t style);
    void (*set_mouse_press_event)(void* object, void* user_data,
                                  void* wrapped_func,
                                  void (*event)(void*, void* self_c,
                                                struct RUBase* arg0));
    void (*remove_mouse_press_event)(void* object);
    void (*set_mouse_release_event)(void* object, void* user_data,
                                    void* wrapped_func,
                                    void (*event)(void*, void* self_c,
                                                  struct RUBase* arg0));
    void (*remove_mouse_release_event)(void* object);
    void (*set_paint_event)(void* object, void* user_data, void* wrapped_func,
                            void (*event)(void*, void* self_c,
                                          struct RUBase* arg0));
    void (*remove_paint_event)(void* object);
    void (*set_enter_event)(void* object, void* user_data, void* wrapped_func,
                            void (*event)(void*, void* self_c,
                                          struct RUBase* arg0));
    void (*remove_enter_event)(void* object);
    void (*set_leave_event)(void* object, void* user_data, void* wrapped_func,
                            void (*event)(void*, void* self_c,
                                          struct RUBase* arg0));
    void (*remove_leave_event)(void* object);
    void (*set_change_event)(void* object, void* user_data, void* wrapped_func,
                             void (*event)(void*, void* self_c,
                                           struct RUBase* arg0));
    void (*remove_change_event)(void* object);
} RUToolButtonFuncs;

typedef struct RUToolButtonAllFuncs {
    struct RUObjectFuncs* object_funcs;
    struct RUPaintDeviceFuncs* paint_device_funcs;
    struct RUWidgetFuncs* widget_funcs;
    struct RUAbstractButtonFuncs* abstract_button_funcs;
    struct RUToolButtonFuncs* tool_button_funcs;
} RUToolButtonAllFuncs;

typedef struct RUToolButton {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUToolButtonAllFuncs* all_funcs;
} RUToolButton;

extern RUToolButtonFuncs s_tool_button_funcs;
extern RUToolButtonAllFuncs s_tool_button_all_funcs;

#ifdef __cplusplus
}
#endif
