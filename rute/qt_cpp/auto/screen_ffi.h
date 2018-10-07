// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdint.h>
#include <stdbool.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "size_ffi.h"

struct RUScreenFuncs;
struct RUScreen;

typedef struct RUScreenFuncs {
    const char* (*name)(struct RUBase* self_c);
    const char* (*manufacturer)(struct RUBase* self_c);
    const char* (*model)(struct RUBase* self_c);
    const char* (*serial_number)(struct RUBase* self_c);
    int (*depth)(struct RUBase* self_c);
    float (*physical_dots_per_inch_x)(struct RUBase* self_c);
    float (*physical_dots_per_inch_y)(struct RUBase* self_c);
    float (*physical_dots_per_inch)(struct RUBase* self_c);
    float (*logical_dots_per_inch_x)(struct RUBase* self_c);
    float (*logical_dots_per_inch_y)(struct RUBase* self_c);
    float (*logical_dots_per_inch)(struct RUBase* self_c);
    float (*device_pixel_ratio)(struct RUBase* self_c);
    struct RUSize (*available_size)(struct RUBase* self_c);
    struct RUSize (*virtual_size)(struct RUBase* self_c);
    struct RUSize (*available_virtual_size)(struct RUBase* self_c);
    int (*angle_between)(struct RUBase* self_c, int a, int b);
    bool (*is_landscape)(struct RUBase* self_c, int orientation);
    float (*refresh_rate)(struct RUBase* self_c);
} RUScreenFuncs;

typedef struct RUScreenAllFuncs {
    struct RUScreenFuncs* screen_funcs;
} RUScreenAllFuncs;

typedef struct RUScreen {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUScreenAllFuncs* all_funcs;
} RUScreen;

extern RUScreenFuncs s_screen_funcs;
extern RUScreenAllFuncs s_screen_all_funcs;

#ifdef __cplusplus
}
#endif