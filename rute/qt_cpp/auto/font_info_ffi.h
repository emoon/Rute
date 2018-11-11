// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdint.h>
#include <stdbool.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif

struct RUFontInfoFuncs;
struct RUFontInfo;

typedef struct RUFontInfoFuncs {
    void (*destroy)(struct RUBase* self);
    void (*swap)(struct RUBase* self_c, struct RUBase* other);
    const char* (*family)(struct RUBase* self_c);
    const char* (*style_name)(struct RUBase* self_c);
    int (*pixel_size)(struct RUBase* self_c);
    int (*point_size)(struct RUBase* self_c);
    float (*point_size_f)(struct RUBase* self_c);
    bool (*italic)(struct RUBase* self_c);
    int (*style)(struct RUBase* self_c);
    int (*weight)(struct RUBase* self_c);
    bool (*bold)(struct RUBase* self_c);
    bool (*underline)(struct RUBase* self_c);
    bool (*overline)(struct RUBase* self_c);
    bool (*fixed_pitch)(struct RUBase* self_c);
    int (*style_hint)(struct RUBase* self_c);
    bool (*raw_mode)(struct RUBase* self_c);
    bool (*exact_match)(struct RUBase* self_c);
} RUFontInfoFuncs;

typedef struct RUFontInfoAllFuncs {
    struct RUFontInfoFuncs* font_info_funcs;
} RUFontInfoAllFuncs;

typedef struct RUFontInfo {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUFontInfoAllFuncs* all_funcs;
} RUFontInfo;

extern RUFontInfoFuncs s_font_info_funcs;
extern RUFontInfoAllFuncs s_font_info_all_funcs;

#ifdef __cplusplus
}
#endif