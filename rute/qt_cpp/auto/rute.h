// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdint.h>
#include <stdbool.h>

#include "../rute_base.h"

#include "application_ffi.h"
#include "event_ffi.h"
#include "font_ffi.h"
#include "list_widget_ffi.h"
#include "list_widget_item_ffi.h"
#include "paint_event_ffi.h"
#include "push_button_ffi.h"
#include "rect_ffi.h"
#include "screen_ffi.h"
#include "size_ffi.h"
#include "widget_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct RuteFFI {
    struct RUApplication (*create_application)(struct RUBase* priv_data);
    struct RUApplication (*get_application)(struct RUBase* priv_data);
    struct RUFont (*create_font)(struct RUBase* priv_data);
    struct RUListWidget (*create_list_widget)(
        struct RUBase* priv_data,
        RUDeleteCallback delete_callback, void* host_data);
    struct RUListWidgetItem (*create_list_widget_item)(
        struct RUBase* priv_data,
        RUDeleteCallback delete_callback, void* host_data);
    struct RUPushButton (*create_push_button)(
        struct RUBase* priv_data,
        RUDeleteCallback delete_callback, void* host_data);
    struct RURect (*create_rect)(
        struct RUBase* priv_data,
        RUDeleteCallback delete_callback, void* host_data);
    struct RUSize (*create_size)(
        struct RUBase* priv_data,
        RUDeleteCallback delete_callback, void* host_data);
    struct RUWidget (*create_widget)(
        struct RUBase* priv_data,
        RUDeleteCallback delete_callback, void* host_data);
} RuteFFI;

extern RuteFFI* rute_static_ffi_get();


#ifdef __cplusplus
}
#endif

