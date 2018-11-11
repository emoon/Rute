////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QPaintDevice>
#include "paint_device_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_paint_device_dev_type(void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c)) {
    WRPaintDevice* qt_object = (WRPaintDevice*)object;
    qt_object->m_dev_type = trampoline_func;
    qt_object->m_dev_type_user_data = user_data;
    qt_object->m_dev_type_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_paint_device_dev_type(void* object) {
    WRPaintDevice* qt_object = (WRPaintDevice*)object;
    qt_object->m_dev_type = nullptr;
    qt_object->m_dev_type_user_data = nullptr;
    qt_object->m_dev_type_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool paint_device_painting_active(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->paintingActive();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_paint_device_paint_engine(void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c)) {
    WRPaintDevice* qt_object = (WRPaintDevice*)object;
    qt_object->m_paint_engine = trampoline_func;
    qt_object->m_paint_engine_user_data = user_data;
    qt_object->m_paint_engine_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_paint_device_paint_engine(void* object) {
    WRPaintDevice* qt_object = (WRPaintDevice*)object;
    qt_object->m_paint_engine = nullptr;
    qt_object->m_paint_engine_user_data = nullptr;
    qt_object->m_paint_engine_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_width(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->width();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_height(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->height();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_width_mm(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->widthMm();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_height_mm(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->heightMm();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_logical_dpi_x(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->logicalDpiX();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_logical_dpi_y(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->logicalDpiY();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_physical_dpi_x(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->physicalDpiX();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_physical_dpi_y(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->physicalDpiY();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_device_pixel_ratio(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->devicePixelRatio();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float paint_device_device_pixel_ratio_f(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->devicePixelRatioF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_color_count(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->colorCount();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_depth(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->depth();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float paint_device_device_pixel_ratio_f_scale(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->devicePixelRatioFScale();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_paint_device_init_painter(void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* painter)) {
    WRPaintDevice* qt_object = (WRPaintDevice*)object;
    qt_object->m_init_painter = trampoline_func;
    qt_object->m_init_painter_user_data = user_data;
    qt_object->m_init_painter_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_paint_device_init_painter(void* object) {
    WRPaintDevice* qt_object = (WRPaintDevice*)object;
    qt_object->m_init_painter = nullptr;
    qt_object->m_init_painter_user_data = nullptr;
    qt_object->m_init_painter_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_paint_device_redirected(void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* offset)) {
    WRPaintDevice* qt_object = (WRPaintDevice*)object;
    qt_object->m_redirected = trampoline_func;
    qt_object->m_redirected_user_data = user_data;
    qt_object->m_redirected_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_paint_device_redirected(void* object) {
    WRPaintDevice* qt_object = (WRPaintDevice*)object;
    qt_object->m_redirected = nullptr;
    qt_object->m_redirected_user_data = nullptr;
    qt_object->m_redirected_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_paint_device_shared_painter(void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c)) {
    WRPaintDevice* qt_object = (WRPaintDevice*)object;
    qt_object->m_shared_painter = trampoline_func;
    qt_object->m_shared_painter_user_data = user_data;
    qt_object->m_shared_painter_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_paint_device_shared_painter(void* object) {
    WRPaintDevice* qt_object = (WRPaintDevice*)object;
    qt_object->m_shared_painter = nullptr;
    qt_object->m_shared_painter_user_data = nullptr;
    qt_object->m_shared_painter_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPaintDevice create_paint_device(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = generic_create_func_with_delete<struct RUPaintDevice, WRPaintDevice>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_paint_device_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_paint_device(struct RUBase* priv_data) {
    destroy_generic<WRPaintDevice>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPaintDevice get_paint_device(struct RUBase* priv_data) {
    (void)priv_data;
    RUPaintDevice ctl;
    ctl.qt_data = nullptr;
    ctl.host_data = nullptr;
    ctl.all_funcs = &s_paint_device_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUPaintDeviceFuncs s_paint_device_funcs = {
    destroy_paint_device,
    set_paint_device_dev_type,
    remove_paint_device_dev_type,
    paint_device_painting_active,
    set_paint_device_paint_engine,
    remove_paint_device_paint_engine,
    paint_device_width,
    paint_device_height,
    paint_device_width_mm,
    paint_device_height_mm,
    paint_device_logical_dpi_x,
    paint_device_logical_dpi_y,
    paint_device_physical_dpi_x,
    paint_device_physical_dpi_y,
    paint_device_device_pixel_ratio,
    paint_device_device_pixel_ratio_f,
    paint_device_color_count,
    paint_device_depth,
    paint_device_device_pixel_ratio_f_scale,
    set_paint_device_init_painter,
    remove_paint_device_init_painter,
    set_paint_device_redirected,
    remove_paint_device_redirected,
    set_paint_device_shared_painter,
    remove_paint_device_shared_painter,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUPaintDeviceAllFuncs s_paint_device_all_funcs = {
    &s_paint_device_funcs,
};
