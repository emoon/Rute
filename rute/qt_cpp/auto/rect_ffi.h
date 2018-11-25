// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "point_ffi.h"
#include "rect_ffi.h"
#include "size_ffi.h"

struct RURectFuncs;
struct RURect;

typedef struct RURectFuncs {
    void (*destroy)(struct RUBase* self);
    bool (*is_null)(struct RUBase* self_c);
    bool (*is_empty)(struct RUBase* self_c);
    bool (*is_valid)(struct RUBase* self_c);
    int (*left)(struct RUBase* self_c);
    int (*top)(struct RUBase* self_c);
    int (*right)(struct RUBase* self_c);
    int (*bottom)(struct RUBase* self_c);
    struct RURect (*normalized)(struct RUBase* self_c);
    int (*x)(struct RUBase* self_c);
    int (*y)(struct RUBase* self_c);
    void (*set_left)(struct RUBase* self_c, int pos);
    void (*set_top)(struct RUBase* self_c, int pos);
    void (*set_right)(struct RUBase* self_c, int pos);
    void (*set_bottom)(struct RUBase* self_c, int pos);
    void (*set_x)(struct RUBase* self_c, int x);
    void (*set_y)(struct RUBase* self_c, int y);
    void (*set_top_left)(struct RUBase* self_c, struct RUBase* p);
    void (*set_bottom_right)(struct RUBase* self_c, struct RUBase* p);
    void (*set_top_right)(struct RUBase* self_c, struct RUBase* p);
    void (*set_bottom_left)(struct RUBase* self_c, struct RUBase* p);
    struct RUPoint (*top_left)(struct RUBase* self_c);
    struct RUPoint (*bottom_right)(struct RUBase* self_c);
    struct RUPoint (*top_right)(struct RUBase* self_c);
    struct RUPoint (*bottom_left)(struct RUBase* self_c);
    struct RUPoint (*center)(struct RUBase* self_c);
    void (*move_left)(struct RUBase* self_c, int pos);
    void (*move_top)(struct RUBase* self_c, int pos);
    void (*move_right)(struct RUBase* self_c, int pos);
    void (*move_bottom)(struct RUBase* self_c, int pos);
    void (*move_top_left)(struct RUBase* self_c, struct RUBase* p);
    void (*move_bottom_right)(struct RUBase* self_c, struct RUBase* p);
    void (*move_top_right)(struct RUBase* self_c, struct RUBase* p);
    void (*move_bottom_left)(struct RUBase* self_c, struct RUBase* p);
    void (*move_center)(struct RUBase* self_c, struct RUBase* p);
    void (*move_to)(struct RUBase* self_c, int x, int t);
    void (*move_to_2)(struct RUBase* self_c, struct RUBase* p);
    void (*set_rect)(struct RUBase* self_c, int x, int y, int w, int h);
    void (*set_coords)(struct RUBase* self_c, int x1, int y1, int x2, int y2);
    void (*adjust)(struct RUBase* self_c, int x1, int y1, int x2, int y2);
    struct RURect (*adjusted)(struct RUBase* self_c, int x1, int y1, int x2,
                              int y2);
    struct RUSize (*size)(struct RUBase* self_c);
    int (*width)(struct RUBase* self_c);
    int (*height)(struct RUBase* self_c);
    void (*set_width)(struct RUBase* self_c, int w);
    void (*set_height)(struct RUBase* self_c, int h);
    void (*set_size)(struct RUBase* self_c, struct RUBase* s);
    bool (*contains)(struct RUBase* self_c, struct RUBase* r, bool proper);
    bool (*contains_2)(struct RUBase* self_c, struct RUBase* p, bool proper);
    bool (*contains_3)(struct RUBase* self_c, int x, int y);
    bool (*contains_4)(struct RUBase* self_c, int x, int y, bool proper);
    struct RURect (*united)(struct RUBase* self_c, struct RUBase* other);
    struct RURect (*intersected)(struct RUBase* self_c, struct RUBase* other);
    bool (*intersects)(struct RUBase* self_c, struct RUBase* r);
    struct RURect (*margins_added)(struct RUBase* self_c,
                                   struct RUBase* margins);
    struct RURect (*margins_removed)(struct RUBase* self_c,
                                     struct RUBase* margins);
} RURectFuncs;

typedef struct RURectAllFuncs {
    struct RURectFuncs* rect_funcs;
} RURectAllFuncs;

typedef struct RURect {
    RUBase* qt_data;
    RUBase* host_data;
    struct RURectAllFuncs* all_funcs;
} RURect;

extern RURectFuncs s_rect_funcs;
extern RURectAllFuncs s_rect_all_funcs;

#ifdef __cplusplus
}
#endif
