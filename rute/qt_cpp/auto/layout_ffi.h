// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "layout_ffi.h"
#include "layout_item_ffi.h"
#include "margins_ffi.h"
#include "rect_ffi.h"
#include "size_ffi.h"
#include "widget_ffi.h"

struct RULayoutFuncs;
struct RULayout;

typedef struct RULayoutFuncs {
    int (*margin)(struct RUBase* self_c);
    int (*spacing)(struct RUBase* self_c);
    void (*set_margin)(struct RUBase* self_c, int arg0);
    void (*set_spacing)(struct RUBase* self_c, int arg0);
    void (*set_contents_margins)(struct RUBase* self_c, int left, int top,
                                 int right, int bottom);
    void (*set_contents_margins_2)(struct RUBase* self_c,
                                   struct RUBase* margins);
    struct RUMargins (*contents_margins)(struct RUBase* self_c);
    struct RURect (*contents_rect)(struct RUBase* self_c);
    bool (*set_alignment)(struct RUBase* self_c, struct RUBase* w,
                          uint32_t alignment);
    bool (*set_alignment_2)(struct RUBase* self_c, struct RUBase* l,
                            uint32_t alignment);
    void (*set_menu_bar)(struct RUBase* self_c, struct RUBase* w);
    struct RUWidget (*menu_bar)(struct RUBase* self_c);
    struct RUWidget (*parent_widget)(struct RUBase* self_c);
    void (*invalidate)(struct RUBase* self_c);
    bool (*activate)(struct RUBase* self_c);
    void (*update)(struct RUBase* self_c);
    void (*add_widget)(struct RUBase* self_c, struct RUBase* w);
    void (*add_item)(struct RUBase* self_c, struct RUBase* arg0);
    void (*remove_widget)(struct RUBase* self_c, struct RUBase* w);
    void (*remove_item)(struct RUBase* self_c, struct RUBase* arg0);
    uint32_t (*expanding_directions)(struct RUBase* self_c);
    struct RUSize (*minimum_size)(struct RUBase* self_c);
    struct RUSize (*maximum_size)(struct RUBase* self_c);
    struct RULayoutItem (*item_at)(struct RUBase* self_c, int index);
    struct RULayoutItem (*take_at)(struct RUBase* self_c, int index);
    int (*index_of)(struct RUBase* self_c, struct RUBase* arg0);
    int (*count)(struct RUBase* self_c);
    bool (*is_empty)(struct RUBase* self_c);
    struct RULayoutItem (*replace_widget)(struct RUBase* self_c,
                                          struct RUBase* from,
                                          struct RUBase* to, uint32_t options);
    int (*total_height_for_width)(struct RUBase* self_c, int w);
    struct RUSize (*total_minimum_size)(struct RUBase* self_c);
    struct RUSize (*total_maximum_size)(struct RUBase* self_c);
    struct RUSize (*total_size_hint)(struct RUBase* self_c);
    struct RULayout (*layout)(struct RUBase* self_c);
    void (*set_size_constraint)(struct RUBase* self_c, uint32_t constraint);
    uint32_t (*size_constraint)(struct RUBase* self_c);
    void (*set_enabled)(struct RUBase* self_c, bool arg0);
    bool (*is_enabled)(struct RUBase* self_c);
} RULayoutFuncs;

typedef struct RULayoutAllFuncs {
    struct RUObjectFuncs* object_funcs;
    struct RULayoutItemFuncs* layout_item_funcs;
    struct RULayoutFuncs* layout_funcs;
} RULayoutAllFuncs;

typedef struct RULayout {
    RUBase* qt_data;
    RUBase* host_data;
    struct RULayoutAllFuncs* all_funcs;
} RULayout;

extern RULayoutFuncs s_layout_funcs;
extern RULayoutAllFuncs s_layout_all_funcs;

#ifdef __cplusplus
}
#endif
