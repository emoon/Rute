////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QApplication>
#include "application_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUStyle application_style(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;
    auto ret_value = qt_value->style();
    struct RUStyle ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_style_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_set_style(struct RUBase* self_c, struct RUBase* arg0) {
    QApplication* qt_value = (QApplication*)self_c;
    qt_value->setStyle((QStyle*)arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUStyle application_set_style_2(struct RUBase* self_c, const char* arg0) {
    QApplication* qt_value = (QApplication*)self_c;
    auto ret_value = qt_value->setStyle(QString::fromUtf8(arg0));
    struct RUStyle ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_style_all_funcs;
    return ctl;
}

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

static struct RUPalette application_palette(struct RUBase* self_c, struct RUBase* arg0) {
    QApplication* qt_value = (QApplication*)self_c;
    auto ret_value = qt_value->palette((QWidget*)arg0);
    WRPalette* new_val = new WRPalette();
    *new_val = ret_value;
    struct RUPalette ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_palette_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUFont application_font(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;
    auto ret_value = qt_value->font();
    WRFont* new_val = new WRFont();
    *new_val = ret_value;
    struct RUFont ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_font_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUFont application_font_2(struct RUBase* self_c, struct RUBase* arg0) {
    QApplication* qt_value = (QApplication*)self_c;
    auto ret_value = qt_value->font((QWidget*)arg0);
    WRFont* new_val = new WRFont();
    *new_val = ret_value;
    struct RUFont ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_font_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_set_window_icon(struct RUBase* self_c, struct RUBase* icon) {
    QApplication* qt_value = (QApplication*)self_c;
    qt_value->setWindowIcon(*((QIcon*)icon));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUIcon application_window_icon(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;
    auto ret_value = qt_value->windowIcon();
    WRIcon* new_val = new WRIcon();
    *new_val = ret_value;
    struct RUIcon ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_icon_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUDesktopWidget application_desktop(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;
    auto ret_value = qt_value->desktop();
    struct RUDesktopWidget ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_desktop_widget_all_funcs;
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

static void application_set_active_window(struct RUBase* self_c, struct RUBase* act) {
    QApplication* qt_value = (QApplication*)self_c;
    qt_value->setActiveWindow((QWidget*)act);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUWidget application_widget_at(struct RUBase* self_c, struct RUBase* p) {
    QApplication* qt_value = (QApplication*)self_c;
    auto ret_value = qt_value->widgetAt(*((QPoint*)p));
    struct RUWidget ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_widget_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUWidget application_widget_at_2(struct RUBase* self_c, int x, int y) {
    QApplication* qt_value = (QApplication*)self_c;
    auto ret_value = qt_value->widgetAt(x, y);
    struct RUWidget ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_widget_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUWidget application_top_level_at(struct RUBase* self_c, struct RUBase* p) {
    QApplication* qt_value = (QApplication*)self_c;
    auto ret_value = qt_value->topLevelAt(*((QPoint*)p));
    struct RUWidget ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_widget_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUWidget application_top_level_at_2(struct RUBase* self_c, int x, int y) {
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

static void application_alert(struct RUBase* self_c, struct RUBase* widget, int duration) {
    QApplication* qt_value = (QApplication*)self_c;
    qt_value->alert((QWidget*)widget, duration);
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

static bool application_is_effect_enabled(struct RUBase* self_c, int arg0) {
    QApplication* qt_value = (QApplication*)self_c;
    auto ret_value = qt_value->isEffectEnabled((Qt::UIEffect)s_ui_effect_lookup[arg0]);
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_set_effect_enabled(struct RUBase* self_c, int arg0, bool enable) {
    QApplication* qt_value = (QApplication*)self_c;
    qt_value->setEffectEnabled((Qt::UIEffect)s_ui_effect_lookup[arg0], enable);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int application_exec(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;
    auto ret_value = qt_value->exec();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* application_style_sheet(struct RUBase* self_c) {
    QApplication* qt_value = (QApplication*)self_c;
    auto ret_value = qt_value->styleSheet();
    return q_string_to_const_char(ret_value);
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
    application_style,
    application_set_style,
    application_set_style_2,
    application_color_spec,
    application_set_color_spec,
    application_palette,
    application_font,
    application_font_2,
    application_set_window_icon,
    application_window_icon,
    application_desktop,
    application_active_popup_widget,
    application_active_modal_widget,
    application_focus_widget,
    application_active_window,
    application_set_active_window,
    application_widget_at,
    application_widget_at_2,
    application_top_level_at,
    application_top_level_at_2,
    application_beep,
    application_alert,
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
    application_is_effect_enabled,
    application_set_effect_enabled,
    application_exec,
    application_style_sheet,
    application_set_style_sheet,
    application_set_auto_sip_enabled,
    application_auto_sip_enabled,
    application_close_all_windows,
    application_about_qt,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUApplicationAllFuncs s_application_all_funcs = {
    &s_object_funcs,
    &s_core_application_funcs,
    &s_gui_application_funcs,
    &s_application_funcs,
};

