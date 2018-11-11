////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QTransform>
#include "transform_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool transform_is_affine(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->isAffine();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool transform_is_identity(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->isIdentity();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool transform_is_invertible(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->isInvertible();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool transform_is_scaling(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->isScaling();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool transform_is_rotating(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->isRotating();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool transform_is_translating(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->isTranslating();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int transform_type(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->type();
    return s_transformation_type_lookup[(int)ret_value];
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float transform_determinant(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->determinant();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float transform_det(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->det();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float transform_m11(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->m11();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float transform_m12(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->m12();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float transform_m13(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->m13();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float transform_m21(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->m21();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float transform_m22(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->m22();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float transform_m23(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->m23();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float transform_m31(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->m31();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float transform_m32(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->m32();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float transform_m33(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->m33();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float transform_dx(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->dx();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float transform_dy(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->dy();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUTransform transform_inverted(struct RUBase* self_c, struct RUBase* invertible) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->inverted(invertible);
    WRTransform* new_val = new WRTransform();
    *new_val = ret_value;
    struct RUTransform ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_transform_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUTransform transform_adjoint(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->adjoint();
    WRTransform* new_val = new WRTransform();
    *new_val = ret_value;
    struct RUTransform ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_transform_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUTransform transform_scale(struct RUBase* self_c, float sx, float sy) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->scale(sx, sy);
    WRTransform* new_val = new WRTransform();
    *new_val = ret_value;
    struct RUTransform ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_transform_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUTransform transform_shear(struct RUBase* self_c, float sh, float sv) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->shear(sh, sv);
    WRTransform* new_val = new WRTransform();
    *new_val = ret_value;
    struct RUTransform ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_transform_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUTransform transform_rotate(struct RUBase* self_c, float a, int axis) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->rotate(a, (Qt::Axis)s_axis_lookup[axis]);
    WRTransform* new_val = new WRTransform();
    *new_val = ret_value;
    struct RUTransform ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_transform_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUTransform transform_rotate_radians(struct RUBase* self_c, float a, int axis) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->rotateRadians(a, (Qt::Axis)s_axis_lookup[axis]);
    WRTransform* new_val = new WRTransform();
    *new_val = ret_value;
    struct RUTransform ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_transform_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool transform_square_to_quad(struct RUBase* self_c, struct RUBase* square, struct RUBase* result) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->squareToQuad(*((QPolygonF*)square), *((QTransform*)result));
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool transform_quad_to_square(struct RUBase* self_c, struct RUBase* quad, struct RUBase* result) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->quadToSquare(*((QPolygonF*)quad), *((QTransform*)result));
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool transform_quad_to_quad(struct RUBase* self_c, struct RUBase* one, struct RUBase* two, struct RUBase* result) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->quadToQuad(*((QPolygonF*)one), *((QPolygonF*)two), *((QTransform*)result));
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void transform_reset(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    qt_value->reset();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPoint transform_map(struct RUBase* self_c, struct RUBase* p) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->map(*((QPoint*)p));
    WRPoint* new_val = new WRPoint();
    *new_val = ret_value;
    struct RUPoint ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_point_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPointF transform_map(struct RUBase* self_c, struct RUBase* p) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->map(*((QPointF*)p));
    WRPointF* new_val = new WRPointF();
    *new_val = ret_value;
    struct RUPointF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_point_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RULine transform_map(struct RUBase* self_c, struct RUBase* l) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->map(*((QLine*)l));
    * new_val = new ();
    *new_val = ret_value;
    struct RULine ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_line_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RULineF transform_map(struct RUBase* self_c, struct RUBase* l) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->map(*((QLineF*)l));
    * new_val = new ();
    *new_val = ret_value;
    struct RULineF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_line_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPolygonF transform_map(struct RUBase* self_c, struct RUBase* a) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->map(*((QPolygonF*)a));
    * new_val = new ();
    *new_val = ret_value;
    struct RUPolygonF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_polygon_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPolygon transform_map(struct RUBase* self_c, struct RUBase* a) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->map(*((QPolygon*)a));
    * new_val = new ();
    *new_val = ret_value;
    struct RUPolygon ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_polygon_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RURegion transform_map(struct RUBase* self_c, struct RUBase* r) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->map(*((QRegion*)r));
    WRRegion* new_val = new WRRegion();
    *new_val = ret_value;
    struct RURegion ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_region_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPainterPath transform_map(struct RUBase* self_c, struct RUBase* p) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->map(*((QPainterPath*)p));
    * new_val = new ();
    *new_val = ret_value;
    struct RUPainterPath ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_painter_path_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPolygon transform_map_to_polygon(struct RUBase* self_c, struct RUBase* r) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->mapToPolygon(*((QRect*)r));
    * new_val = new ();
    *new_val = ret_value;
    struct RUPolygon ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_polygon_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RURect transform_map_rect(struct RUBase* self_c, struct RUBase* arg0) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->mapRect(*((QRect*)arg0));
    WRRect* new_val = new WRRect();
    *new_val = ret_value;
    struct RURect ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_rect_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RURectF transform_map_rect(struct RUBase* self_c, struct RUBase* arg0) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->mapRect(*((QRectF*)arg0));
    WRRectF* new_val = new WRRectF();
    *new_val = ret_value;
    struct RURectF ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_rect_f_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void transform_map(struct RUBase* self_c, int x, int y, struct RUBase* tx, struct RUBase* ty) {
    WRTransform* qt_value = (WRTransform*)self_c;
    qt_value->map(x, y, tx, ty);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void transform_map(struct RUBase* self_c, float x, float y, struct RUBase* tx, struct RUBase* ty) {
    WRTransform* qt_value = (WRTransform*)self_c;
    qt_value->map(x, y, tx, ty);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUMatrix transform_to_affine(struct RUBase* self_c) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->toAffine();
    * new_val = new ();
    *new_val = ret_value;
    struct RUMatrix ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_matrix_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUTransform transform_from_translate(struct RUBase* self_c, float dx, float dy) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->fromTranslate(dx, dy);
    WRTransform* new_val = new WRTransform();
    *new_val = ret_value;
    struct RUTransform ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_transform_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUTransform transform_from_scale(struct RUBase* self_c, float dx, float dy) {
    WRTransform* qt_value = (WRTransform*)self_c;
    auto ret_value = qt_value->fromScale(dx, dy);
    WRTransform* new_val = new WRTransform();
    *new_val = ret_value;
    struct RUTransform ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_transform_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUTransform create_transform(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = generic_create_func_with_delete<struct RUTransform, WRTransform>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_transform_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_transform(struct RUBase* priv_data) {
    destroy_generic<WRTransform>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUTransform get_transform(struct RUBase* priv_data) {
    (void)priv_data;
    RUTransform ctl;
    ctl.qt_data = nullptr;
    ctl.host_data = nullptr;
    ctl.all_funcs = &s_transform_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUTransformFuncs s_transform_funcs = {
    destroy_transform,
    transform_is_affine,
    transform_is_identity,
    transform_is_invertible,
    transform_is_scaling,
    transform_is_rotating,
    transform_is_translating,
    transform_type,
    transform_determinant,
    transform_det,
    transform_m11,
    transform_m12,
    transform_m13,
    transform_m21,
    transform_m22,
    transform_m23,
    transform_m31,
    transform_m32,
    transform_m33,
    transform_dx,
    transform_dy,
    transform_inverted,
    transform_adjoint,
    transform_scale,
    transform_shear,
    transform_rotate,
    transform_rotate_radians,
    transform_square_to_quad,
    transform_quad_to_square,
    transform_quad_to_quad,
    transform_reset,
    transform_map,
    transform_map,
    transform_map,
    transform_map,
    transform_map,
    transform_map,
    transform_map,
    transform_map,
    transform_map_to_polygon,
    transform_map_rect,
    transform_map_rect,
    transform_map,
    transform_map,
    transform_to_affine,
    transform_from_translate,
    transform_from_scale,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUTransformAllFuncs s_transform_all_funcs = {
    &s_transform_funcs,
};
