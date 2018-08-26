
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "Rute.h"
#include "rute_cpp.h"
#include "../rute_manual.h"
#include <QApplication>
#include <QWidget>
#include <QListWidgetItem>
#include <QListWidget>
#include <QFont>

static char s_temp_string_buffer[1024*1024];

#include <map>
std::map<QWidget*, void*> s_widget_lookup;

extern struct RUApplicationFuncs s_application_funcs;
extern struct RUWidgetFuncs s_widget_funcs;
extern struct RUListWidgetItemFuncs s_list_widget_item_funcs;
extern struct RUListWidgetFuncs s_list_widget_funcs;
extern struct RUFontFuncs s_font_funcs;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


struct KeyVal { int val, key; };

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void create_enum_mappings() {
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_set_style(struct RUBase* self_c, const char* style) { 
    QApplication* qt_data = (QApplication*)self_c;
    qt_data->setStyle(QString::fromUtf8(style));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int application_exec(struct RUBase* self_c) { 
    QApplication* qt_data = (QApplication*)self_c;
    auto ret_value = qt_data->exec();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUFont application_font(struct RUBase* self_c) { 
    QApplication* qt_data = (QApplication*)self_c;
    auto ret_value = qt_data->font();
    RUFont ctl;
    ctl.font_funcs = &s_font_funcs;
    ctl.priv_data = (struct RUBase*)new QFont(ret_value);
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_application_about_to_quit_event(void* object, void* user_data, void* trampoline_func, void (*event)(void* self_c)) {
    QSlotWrapperself_void* wrap = new QSlotWrapperself_void(user_data, (self_void)trampoline_func, (void*)event);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(aboutToQuit()), wrap, SLOT(method()));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_beep(struct RUBase* self_c) { 
    QApplication* qt_data = (QApplication*)self_c;
    qt_data->beep();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_about_qt(struct RUBase* self_c) { 
    QApplication* qt_data = (QApplication*)self_c;
    qt_data->aboutQt();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void widget_show(struct RUBase* self_c) { 
    WRWidget* qt_data = (WRWidget*)self_c;
    qt_data->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void widget_set_fixed_height(struct RUBase* self_c, int width) { 
    WRWidget* qt_data = (WRWidget*)self_c;
    qt_data->setFixedHeight(width);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void widget_set_fixed_width(struct RUBase* self_c, int width) { 
    WRWidget* qt_data = (WRWidget*)self_c;
    qt_data->setFixedWidth(width);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void widget_resize(struct RUBase* self_c, int width, int height) { 
    WRWidget* qt_data = (WRWidget*)self_c;
    qt_data->resize(width, height);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void widget_update(struct RUBase* self_c) { 
    WRWidget* qt_data = (WRWidget*)self_c;
    qt_data->update();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_text(struct RUBase* self_c, const char* text) { 
    QListWidgetItem* qt_data = (QListWidgetItem*)self_c;
    qt_data->setText(QString::fromUtf8(text));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* list_widget_item_text(struct RUBase* self_c) { 
    QListWidgetItem* qt_data = (QListWidgetItem*)self_c;
    auto ret_value = qt_data->text();
    QByteArray ba = ret_value.toUtf8();
    const char* c_str = ba.data();
    assert((ba.size() + 1) < sizeof(s_temp_string_buffer));
    memcpy(s_temp_string_buffer, c_str, ba.size() + 1);
    return s_temp_string_buffer;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_clear(struct RUBase* self_c) { 
    WRListWidget* qt_data = (WRListWidget*)self_c;
    qt_data->clear();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUListWidgetItem list_widget_current_item(struct RUBase* self_c) { 
    WRListWidget* qt_data = (WRListWidget*)self_c;
    auto ret_value = qt_data->currentItem();
    RUListWidgetItem ctl;
    ctl.list_widget_item_funcs = &s_list_widget_item_funcs;
    ctl.priv_data = (struct RUBase*)ret_value;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int list_widget_current_row(struct RUBase* self_c) { 
    WRListWidget* qt_data = (WRListWidget*)self_c;
    auto ret_value = qt_data->currentRow();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_set_current_row(struct RUBase* self_c, int index) { 
    WRListWidget* qt_data = (WRListWidget*)self_c;
    qt_data->setCurrentRow(index);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int list_widget_count(struct RUBase* self_c) { 
    WRListWidget* qt_data = (WRListWidget*)self_c;
    auto ret_value = qt_data->count();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_set_drag_enabled(struct RUBase* self_c, bool state) { 
    WRListWidget* qt_data = (WRListWidget*)self_c;
    qt_data->setDragEnabled(state);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_set_drop_indicator_shown(struct RUBase* self_c, bool state) { 
    WRListWidget* qt_data = (WRListWidget*)self_c;
    qt_data->setDropIndicatorShown(state);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_list_widget_current_row_changed_event(void* object, void* user_data, void* trampoline_func, void (*event)(void* self_c, int row)) {
    QSlotWrapperself_int_void* wrap = new QSlotWrapperself_int_void(user_data, (self_int_void)trampoline_func, (void*)event);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(currentRowChanged(int)), wrap, SLOT(method(int)));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_pixel_size(struct RUBase* self_c, int size) { 
    QFont* qt_data = (QFont*)self_c;
    qt_data->setPixelSize(size);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int font_pixel_size(struct RUBase* self_c) { 
    QFont* qt_data = (QFont*)self_c;
    auto ret_value = qt_data->pixelSize();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUWidget create_widget(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = create_widget_func<struct RUWidget, WRWidget>(priv_data, delete_callback, private_user_data);
    ctl.widget_funcs = &s_widget_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_widget(struct RUBase* priv_data) {
    destroy_generic<WRWidget>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUListWidgetItem create_list_widget_item(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = generic_create_func_with_delete<struct RUListWidgetItem, WRListWidgetItem>(priv_data, delete_callback, private_user_data);
    ctl.list_widget_item_funcs = &s_list_widget_item_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_list_widget_item(struct RUBase* priv_data) {
    destroy_generic<WRListWidgetItem>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUListWidget create_list_widget(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = create_widget_func<struct RUListWidget, WRListWidget>(priv_data, delete_callback, private_user_data);
    ctl.widget_funcs = &s_widget_funcs;
    ctl.list_widget_funcs = &s_list_widget_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_list_widget(struct RUBase* priv_data) {
    destroy_generic<WRListWidget>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUFont create_font(struct RUBase* priv_data) {
    auto ctl = generic_create_func<struct RUFont, QFont>(priv_data);
    ctl.font_funcs = &s_font_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_font(struct RUBase* priv_data) {
    destroy_generic<QFont>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUApplication get_application(struct RUBase* priv_data) {
    (void)priv_data;
    RUApplication ctl;
    ctl.priv_data = nullptr;
    ctl.application_funcs = &s_application_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUApplicationFuncs s_application_funcs = {
    destroy_application,
    application_set_style,
    application_exec,
    application_font,
    set_application_about_to_quit_event,
    application_beep,
    application_about_qt,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUWidgetFuncs s_widget_funcs = {
    destroy_widget,
    widget_show,
    widget_set_fixed_height,
    widget_set_fixed_width,
    widget_resize,
    widget_update,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUListWidgetItemFuncs s_list_widget_item_funcs = {
    destroy_list_widget_item,
    list_widget_item_set_text,
    list_widget_item_text,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUListWidgetFuncs s_list_widget_funcs = {
    destroy_list_widget,
    list_widget_clear,
    list_widget_current_item,
    list_widget_current_row,
    list_widget_set_current_row,
    list_widget_count,
    list_widget_set_drag_enabled,
    list_widget_set_drop_indicator_shown,
    set_list_widget_current_row_changed_event,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUFontFuncs s_font_funcs = {
    destroy_font,
    font_set_pixel_size,
    font_pixel_size,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct Rute s_rute = { 
    nullptr,
    create_application,
    get_application,
    create_widget,
    create_list_widget_item,
    create_list_widget,
    create_font,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#ifdef _WIN32
extern "C" __declspec(dllexport) struct Rute* rute_get() {
#else
extern "C" struct Rute* rute_get() {
#endif
    return (Rute*)&s_rute;
}
