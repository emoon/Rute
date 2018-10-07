////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QApplication>

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int application_color_spec(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;

    auto ret_value = qt_value->colorSpec();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_set_color_spec(struct RUBase* self_c, int arg0) {
    QApplication* qt_value = (QApplication*)self_c;

    qt_value->setColorSpec(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUFont application_get_font(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;

    auto ret_value = qt_value->font();
    QFont* new_val = new QFont();
    *new_val = ret_value;
    struct RUFont ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_font_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUWidget application_active_popup_widget(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;

    auto ret_value = qt_value->activePopupWidget();
    struct RUWidget ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_widget_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUWidget application_active_modal_widget(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;

    auto ret_value = qt_value->activeModalWidget();
    struct RUWidget ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_widget_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUWidget application_focus_widget(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;

    auto ret_value = qt_value->focusWidget();
    struct RUWidget ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_widget_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUWidget application_active_window(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;

    auto ret_value = qt_value->activeWindow();
    struct RUWidget ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_widget_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_set_active_window(struct RUBase* self_c, struct RUBase* actor) {
    QApplication* qt_value = (QApplication*)self_c;

    qt_value->setActiveWindow((QWidget*)actor);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUWidget application_widget_at(struct RUBase* self_c, int x, int y) {
    QApplication* qt_value = (QApplication*)self_c;

    auto ret_value = qt_value->widgetAt(x, y);
    struct RUWidget ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_widget_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUWidget application_top_level_at(struct RUBase* self_c, int x, int y) {
    QApplication* qt_value = (QApplication*)self_c;

    auto ret_value = qt_value->topLevelAt(x, y);
    struct RUWidget ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_widget_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_beep(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;

    qt_value->beep();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_set_cursor_flash_time(struct RUBase* self_c, int arg0) {
    QApplication* qt_value = (QApplication*)self_c;

    qt_value->setCursorFlashTime(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int application_cursor_flash_time(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;

    auto ret_value = qt_value->cursorFlashTime();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_set_double_click_interval(struct RUBase* self_c, int arg0) {
    QApplication* qt_value = (QApplication*)self_c;

    qt_value->setDoubleClickInterval(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int application_double_click_interval(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;

    auto ret_value = qt_value->doubleClickInterval();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_set_keyboard_input_interval(struct RUBase* self_c, int arg0) {
    QApplication* qt_value = (QApplication*)self_c;

    qt_value->setKeyboardInputInterval(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int application_keyboard_input_interval(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;

    auto ret_value = qt_value->keyboardInputInterval();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_set_wheel_scroll_lines(struct RUBase* self_c, int arg0) {
    QApplication* qt_value = (QApplication*)self_c;

    qt_value->setWheelScrollLines(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int application_wheel_scroll_lines(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;

    auto ret_value = qt_value->wheelScrollLines();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_set_start_drag_time(struct RUBase* self_c, int ms) {
    QApplication* qt_value = (QApplication*)self_c;

    qt_value->setStartDragTime(ms);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int application_start_drag_time(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;

    auto ret_value = qt_value->startDragTime();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_set_start_drag_distance(struct RUBase* self_c, int l) {
    QApplication* qt_value = (QApplication*)self_c;

    qt_value->setStartDragDistance(l);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int application_start_drag_distance(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;

    auto ret_value = qt_value->startDragDistance();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int application_exec(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;

    auto ret_value = qt_value->exec();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_application_about_to_quit_event(void* object, void* user_data, void* trampoline_func, void (*event)(void* self_c)) {
    QSlotWrapperSignal_self_void* wrap = new QSlotWrapperSignal_self_void(user_data, (Signal_self_void)trampoline_func, (void*)event);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(aboutToQuit()), wrap, SLOT(method()));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_application_screen_added_event(void* object, void* user_data, void* trampoline_func, void (*event)(void* self_c, struct RUBase* screen)) {
    QSlotWrapperSignal_self_ScreenType_void* wrap = new QSlotWrapperSignal_self_ScreenType_void(user_data, (Signal_self_ScreenType_void)trampoline_func, (void*)event);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(screenAdded(QScreen*)), wrap, SLOT(method(QScreen*)));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_set_style_sheet(struct RUBase* self_c, const char* sheet) {
    QApplication* qt_value = (QApplication*)self_c;

    qt_value->setStyleSheet(QString::fromUtf8(sheet));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_set_auto_sip_enabled(struct RUBase* self_c, bool enabled) {
    QApplication* qt_value = (QApplication*)self_c;

    qt_value->setAutoSipEnabled(enabled);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool application_auto_sip_enabled(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;

    auto ret_value = qt_value->autoSipEnabled();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_close_all_windows(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;

    qt_value->closeAllWindows();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_about_qt(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;

    qt_value->aboutQt();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUApplication get_application(struct RUBase* priv_data) {
    (void)priv_data;
    RUApplication ctl;
    ctl.qt_data = nullptr;
    ctl.host_data = nullptr;
    ctl.all_funcs = &s_application_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUApplicationFuncs s_application_funcs = {
    destroy_application,
    application_color_spec,
    application_set_color_spec,
    application_get_font,
    application_active_popup_widget,
    application_active_modal_widget,
    application_focus_widget,
    application_active_window,
    application_set_active_window,
    application_widget_at,
    application_top_level_at,
    application_beep,
    application_set_cursor_flash_time,
    application_cursor_flash_time,
    application_set_double_click_interval,
    application_double_click_interval,
    application_set_keyboard_input_interval,
    application_keyboard_input_interval,
    application_set_wheel_scroll_lines,
    application_wheel_scroll_lines,
    application_set_start_drag_time,
    application_start_drag_time,
    application_set_start_drag_distance,
    application_start_drag_distance,
    application_exec,
    set_application_about_to_quit_event,
    set_application_screen_added_event,
    application_set_style_sheet,
    application_set_auto_sip_enabled,
    application_auto_sip_enabled,
    application_close_all_windows,
    application_about_qt,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUApplicationAllFuncs s_application_all_funcs = {
    &s_application_funcs,
};

