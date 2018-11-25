// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "brush_ffi.h"
#include "color_ffi.h"
#include "palette_ffi.h"

struct RUPaletteFuncs;
struct RUPalette;

typedef struct RUPaletteFuncs {
    void (*destroy)(struct RUBase* self);
    void (*swap)(struct RUBase* self_c, struct RUBase* other);
    int (*current_color_group)(struct RUBase* self_c);
    void (*set_current_color_group)(struct RUBase* self_c, int cg);
    struct RUColor (*color)(struct RUBase* self_c, int cg, int cr);
    struct RUBrush (*brush)(struct RUBase* self_c, int cg, int cr);
    void (*set_color)(struct RUBase* self_c, int cg, int cr,
                      struct RUBase* color);
    void (*set_color_2)(struct RUBase* self_c, int cr, struct RUBase* color);
    void (*set_brush)(struct RUBase* self_c, int cr, struct RUBase* brush);
    bool (*is_brush_set)(struct RUBase* self_c, int cg, int cr);
    void (*set_brush_2)(struct RUBase* self_c, int cg, int cr,
                        struct RUBase* brush);
    void (*set_color_group)(struct RUBase* self_c, int cr,
                            struct RUBase* window_text, struct RUBase* button,
                            struct RUBase* light, struct RUBase* dark,
                            struct RUBase* mid, struct RUBase* text,
                            struct RUBase* bright_text, struct RUBase* base,
                            struct RUBase* window);
    bool (*is_equal)(struct RUBase* self_c, int cr1, int cr2);
    struct RUColor (*color_2)(struct RUBase* self_c, int cr);
    struct RUBrush (*brush_2)(struct RUBase* self_c, int cr);
    struct RUBrush (*foreground)(struct RUBase* self_c);
    struct RUBrush (*window_text)(struct RUBase* self_c);
    struct RUBrush (*button)(struct RUBase* self_c);
    struct RUBrush (*light)(struct RUBase* self_c);
    struct RUBrush (*dark)(struct RUBase* self_c);
    struct RUBrush (*mid)(struct RUBase* self_c);
    struct RUBrush (*text)(struct RUBase* self_c);
    struct RUBrush (*base)(struct RUBase* self_c);
    struct RUBrush (*alternate_base)(struct RUBase* self_c);
    struct RUBrush (*tool_tip_base)(struct RUBase* self_c);
    struct RUBrush (*tool_tip_text)(struct RUBase* self_c);
    struct RUBrush (*background)(struct RUBase* self_c);
    struct RUBrush (*window)(struct RUBase* self_c);
    struct RUBrush (*midlight)(struct RUBase* self_c);
    struct RUBrush (*bright_text)(struct RUBase* self_c);
    struct RUBrush (*button_text)(struct RUBase* self_c);
    struct RUBrush (*shadow)(struct RUBase* self_c);
    struct RUBrush (*highlight)(struct RUBase* self_c);
    struct RUBrush (*highlighted_text)(struct RUBase* self_c);
    struct RUBrush (*link)(struct RUBase* self_c);
    struct RUBrush (*link_visited)(struct RUBase* self_c);
    bool (*is_copy_of)(struct RUBase* self_c, struct RUBase* p);
    int64_t (*cache_key)(struct RUBase* self_c);
    struct RUPalette (*resolve)(struct RUBase* self_c, struct RUBase* arg0);
    uint32_t (*resolve_2)(struct RUBase* self_c);
    void (*resolve_3)(struct RUBase* self_c, uint32_t mask);
} RUPaletteFuncs;

typedef struct RUPaletteAllFuncs {
    struct RUPaletteFuncs* palette_funcs;
} RUPaletteAllFuncs;

typedef struct RUPalette {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUPaletteAllFuncs* all_funcs;
} RUPalette;

extern RUPaletteFuncs s_palette_funcs;
extern RUPaletteAllFuncs s_palette_all_funcs;

#ifdef __cplusplus
}
#endif