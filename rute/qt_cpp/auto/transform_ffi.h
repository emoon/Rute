// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "line_f_ffi.h"
#include "line_ffi.h"
#include "matrix_ffi.h"
#include "point_f_ffi.h"
#include "point_ffi.h"
#include "polygon_f_ffi.h"
#include "polygon_ffi.h"
#include "rect_f_ffi.h"
#include "rect_ffi.h"
#include "region_ffi.h"
#include "transform_ffi.h"

struct RUTransformFuncs;
struct RUTransform;

typedef struct RUTransformFuncs {
    void (*destroy)(struct RUBase* self);
    bool (*is_affine)(struct RUBase* self_c);
    bool (*is_identity)(struct RUBase* self_c);
    bool (*is_invertible)(struct RUBase* self_c);
    bool (*is_scaling)(struct RUBase* self_c);
    bool (*is_rotating)(struct RUBase* self_c);
    bool (*is_translating)(struct RUBase* self_c);
    uint32_t (*get_type)(struct RUBase* self_c);
    float (*determinant)(struct RUBase* self_c);
    float (*det)(struct RUBase* self_c);
    float (*m11)(struct RUBase* self_c);
    float (*m12)(struct RUBase* self_c);
    float (*m13)(struct RUBase* self_c);
    float (*m21)(struct RUBase* self_c);
    float (*m22)(struct RUBase* self_c);
    float (*m23)(struct RUBase* self_c);
    float (*m31)(struct RUBase* self_c);
    float (*m32)(struct RUBase* self_c);
    float (*m33)(struct RUBase* self_c);
    float (*dx)(struct RUBase* self_c);
    float (*dy)(struct RUBase* self_c);
    struct RUTransform (*adjoint)(struct RUBase* self_c);
    struct RUTransform (*scale)(struct RUBase* self_c, float sx, float sy);
    struct RUTransform (*shear)(struct RUBase* self_c, float sh, float sv);
    struct RUTransform (*rotate)(struct RUBase* self_c, float a, uint32_t axis);
    struct RUTransform (*rotate_radians)(struct RUBase* self_c, float a,
                                         uint32_t axis);
    bool (*square_to_quad)(struct RUBase* self_c, struct RUBase* square,
                           struct RUBase* result);
    bool (*quad_to_square)(struct RUBase* self_c, struct RUBase* quad,
                           struct RUBase* result);
    bool (*quad_to_quad)(struct RUBase* self_c, struct RUBase* one,
                         struct RUBase* two, struct RUBase* result);
    void (*reset)(struct RUBase* self_c);
    struct RUPoint (*map)(struct RUBase* self_c, struct RUBase* p);
    struct RUPointF (*map_2)(struct RUBase* self_c, struct RUBase* p);
    struct RULine (*map_3)(struct RUBase* self_c, struct RUBase* l);
    struct RULineF (*map_4)(struct RUBase* self_c, struct RUBase* l);
    struct RUPolygonF (*map_5)(struct RUBase* self_c, struct RUBase* a);
    struct RUPolygon (*map_6)(struct RUBase* self_c, struct RUBase* a);
    struct RURegion (*map_7)(struct RUBase* self_c, struct RUBase* r);
    struct RUPolygon (*map_to_polygon)(struct RUBase* self_c, struct RUBase* r);
    struct RURect (*map_rect)(struct RUBase* self_c, struct RUBase* arg0);
    struct RURectF (*map_rect_2)(struct RUBase* self_c, struct RUBase* arg0);
    struct RUMatrix (*to_affine)(struct RUBase* self_c);
    struct RUTransform (*from_translate)(struct RUBase* self_c, float dx,
                                         float dy);
    struct RUTransform (*from_scale)(struct RUBase* self_c, float dx, float dy);
} RUTransformFuncs;

typedef struct RUTransformAllFuncs {
    struct RUTransformFuncs* transform_funcs;
} RUTransformAllFuncs;

typedef struct RUTransform {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUTransformAllFuncs* all_funcs;
} RUTransform;

extern RUTransformFuncs s_transform_funcs;
extern RUTransformAllFuncs s_transform_all_funcs;

#ifdef __cplusplus
}
#endif
