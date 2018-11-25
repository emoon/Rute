// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "point_ffi.h"
#include "polygon_ffi.h"
#include "rect_ffi.h"

struct RUPolygonFuncs;
struct RUPolygon;

typedef struct RUPolygonFuncs {
    void (*destroy)(struct RUBase* self);
    void (*swap)(struct RUBase* self_c, struct RUBase* other);
    struct RURect (*bounding_rect)(struct RUBase* self_c);
    struct RUPoint (*point_2)(struct RUBase* self_c, int i);
    void (*set_point)(struct RUBase* self_c, int index, int x, int y);
    void (*set_point_2)(struct RUBase* self_c, int index, struct RUBase* p);
    void (*set_points_2)(struct RUBase* self_c, int n_points, int firstx,
                         int firsty);
    void (*put_points_2)(struct RUBase* self_c, int index, int n_points,
                         int firstx, int firsty);
    void (*put_points_3)(struct RUBase* self_c, int index, int n_points,
                         struct RUBase* from, int from_index);
    bool (*contains_point)(struct RUBase* self_c, struct RUBase* pt,
                           int fill_rule);
    struct RUPolygon (*united)(struct RUBase* self_c, struct RUBase* r);
    struct RUPolygon (*intersected)(struct RUBase* self_c, struct RUBase* r);
    bool (*intersects)(struct RUBase* self_c, struct RUBase* r);
} RUPolygonFuncs;

typedef struct RUPolygonAllFuncs {
    struct RUPointFuncs* point_funcs;
    struct RUPolygonFuncs* polygon_funcs;
} RUPolygonAllFuncs;

typedef struct RUPolygon {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUPolygonAllFuncs* all_funcs;
} RUPolygon;

extern RUPolygonFuncs s_polygon_funcs;
extern RUPolygonAllFuncs s_polygon_all_funcs;

#ifdef __cplusplus
}
#endif