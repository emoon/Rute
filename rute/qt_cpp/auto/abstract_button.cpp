////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QAbstractButton>
#include "abstract_button_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void abstract_button_set_text(struct RUBase* self_c, const char* text) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    qt_value->setText(QString::fromUtf8(text));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* abstract_button_text(struct RUBase* self_c) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    auto ret_value = qt_value->text();
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void abstract_button_set_icon(struct RUBase* self_c, struct RUBase* icon) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    qt_value->setIcon(*((WRIcon*)icon));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUIcon abstract_button_icon(struct RUBase* self_c) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    auto ret_value = qt_value->icon();
    WRIcon* new_val = new WRIcon();
    *new_val = ret_value;
    struct RUIcon ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_icon_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize abstract_button_icon_size(struct RUBase* self_c) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    auto ret_value = qt_value->iconSize();
    WRSize* new_val = new WRSize();
    *new_val = ret_value;
    struct RUSize ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_size_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void abstract_button_set_shortcut(struct RUBase* self_c, struct RUBase* key) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    qt_value->setShortcut(*((WRKeySequence*)key));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUKeySequence abstract_button_shortcut(struct RUBase* self_c) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    auto ret_value = qt_value->shortcut();
    WRKeySequence* new_val = new WRKeySequence();
    *new_val = ret_value;
    struct RUKeySequence ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_key_sequence_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void abstract_button_set_checkable(struct RUBase* self_c, bool arg0) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    qt_value->setCheckable(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool abstract_button_is_checkable(struct RUBase* self_c) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    auto ret_value = qt_value->isCheckable();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool abstract_button_is_checked(struct RUBase* self_c) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    auto ret_value = qt_value->isChecked();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void abstract_button_set_down(struct RUBase* self_c, bool arg0) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    qt_value->setDown(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool abstract_button_is_down(struct RUBase* self_c) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    auto ret_value = qt_value->isDown();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void abstract_button_set_auto_repeat(struct RUBase* self_c, bool arg0) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    qt_value->setAutoRepeat(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool abstract_button_auto_repeat(struct RUBase* self_c) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    auto ret_value = qt_value->autoRepeat();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void abstract_button_set_auto_repeat_delay(struct RUBase* self_c, int arg0) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    qt_value->setAutoRepeatDelay(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int abstract_button_auto_repeat_delay(struct RUBase* self_c) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    auto ret_value = qt_value->autoRepeatDelay();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void abstract_button_set_auto_repeat_interval(struct RUBase* self_c, int arg0) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    qt_value->setAutoRepeatInterval(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int abstract_button_auto_repeat_interval(struct RUBase* self_c) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    auto ret_value = qt_value->autoRepeatInterval();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void abstract_button_set_auto_exclusive(struct RUBase* self_c, bool arg0) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    qt_value->setAutoExclusive(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool abstract_button_auto_exclusive(struct RUBase* self_c) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    auto ret_value = qt_value->autoExclusive();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUButtonGroup abstract_button_group(struct RUBase* self_c) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    auto ret_value = qt_value->group();
    struct RUButtonGroup ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_button_group_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void abstract_button_set_icon_size(struct RUBase* self_c, struct RUBase* size) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    qt_value->setIconSize(*((WRSize*)size));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void abstract_button_animate_click(struct RUBase* self_c, int msec) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    qt_value->animateClick(msec);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void abstract_button_click(struct RUBase* self_c) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    qt_value->click();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void abstract_button_toggle(struct RUBase* self_c) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    qt_value->toggle();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void abstract_button_set_checked(struct RUBase* self_c, bool arg0) {
    WRAbstractButton* qt_value = (WRAbstractButton*)self_c;
    qt_value->setChecked(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_abstract_button_pressed_event(void* object, void* user_data, void* wrapped_func, void (*event)(void*, void* self_c)) {
    QSlotWrapperSignal_self_void* wrap = new QSlotWrapperSignal_self_void(user_data, (Signal_self_void)event, (void*)wrapped_func);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(pressed()), wrap, SLOT(method()));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_abstract_button_released_event(void* object, void* user_data, void* wrapped_func, void (*event)(void*, void* self_c)) {
    QSlotWrapperSignal_self_void* wrap = new QSlotWrapperSignal_self_void(user_data, (Signal_self_void)event, (void*)wrapped_func);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(released()), wrap, SLOT(method()));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_abstract_button_clicked_event(void* object, void* user_data, void* wrapped_func, void (*event)(void*, void* self_c, bool checked)) {
    QSlotWrapperSignal_self_bool_void* wrap = new QSlotWrapperSignal_self_bool_void(user_data, (Signal_self_bool_void)event, (void*)wrapped_func);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(clicked(bool)), wrap, SLOT(method(bool)));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_abstract_button_toggled_event(void* object, void* user_data, void* wrapped_func, void (*event)(void*, void* self_c, bool checked)) {
    QSlotWrapperSignal_self_bool_void* wrap = new QSlotWrapperSignal_self_bool_void(user_data, (Signal_self_bool_void)event, (void*)wrapped_func);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(toggled(bool)), wrap, SLOT(method(bool)));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_abstract_button_paint_event(void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* e)) {
    WRAbstractButton* qt_object = (WRAbstractButton*)object;
    qt_object->m_paint_event = trampoline_func;
    qt_object->m_paint_event_user_data = user_data;
    qt_object->m_paint_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_abstract_button_paint_event(void* object) {
    WRAbstractButton* qt_object = (WRAbstractButton*)object;
    qt_object->m_paint_event = nullptr;
    qt_object->m_paint_event_user_data = nullptr;
    qt_object->m_paint_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_abstract_button_key_press_event(void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* e)) {
    WRAbstractButton* qt_object = (WRAbstractButton*)object;
    qt_object->m_key_press_event = trampoline_func;
    qt_object->m_key_press_event_user_data = user_data;
    qt_object->m_key_press_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_abstract_button_key_press_event(void* object) {
    WRAbstractButton* qt_object = (WRAbstractButton*)object;
    qt_object->m_key_press_event = nullptr;
    qt_object->m_key_press_event_user_data = nullptr;
    qt_object->m_key_press_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_abstract_button_key_release_event(void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* e)) {
    WRAbstractButton* qt_object = (WRAbstractButton*)object;
    qt_object->m_key_release_event = trampoline_func;
    qt_object->m_key_release_event_user_data = user_data;
    qt_object->m_key_release_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_abstract_button_key_release_event(void* object) {
    WRAbstractButton* qt_object = (WRAbstractButton*)object;
    qt_object->m_key_release_event = nullptr;
    qt_object->m_key_release_event_user_data = nullptr;
    qt_object->m_key_release_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_abstract_button_mouse_press_event(void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* e)) {
    WRAbstractButton* qt_object = (WRAbstractButton*)object;
    qt_object->m_mouse_press_event = trampoline_func;
    qt_object->m_mouse_press_event_user_data = user_data;
    qt_object->m_mouse_press_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_abstract_button_mouse_press_event(void* object) {
    WRAbstractButton* qt_object = (WRAbstractButton*)object;
    qt_object->m_mouse_press_event = nullptr;
    qt_object->m_mouse_press_event_user_data = nullptr;
    qt_object->m_mouse_press_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_abstract_button_mouse_release_event(void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* e)) {
    WRAbstractButton* qt_object = (WRAbstractButton*)object;
    qt_object->m_mouse_release_event = trampoline_func;
    qt_object->m_mouse_release_event_user_data = user_data;
    qt_object->m_mouse_release_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_abstract_button_mouse_release_event(void* object) {
    WRAbstractButton* qt_object = (WRAbstractButton*)object;
    qt_object->m_mouse_release_event = nullptr;
    qt_object->m_mouse_release_event_user_data = nullptr;
    qt_object->m_mouse_release_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_abstract_button_mouse_move_event(void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* e)) {
    WRAbstractButton* qt_object = (WRAbstractButton*)object;
    qt_object->m_mouse_move_event = trampoline_func;
    qt_object->m_mouse_move_event_user_data = user_data;
    qt_object->m_mouse_move_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_abstract_button_mouse_move_event(void* object) {
    WRAbstractButton* qt_object = (WRAbstractButton*)object;
    qt_object->m_mouse_move_event = nullptr;
    qt_object->m_mouse_move_event_user_data = nullptr;
    qt_object->m_mouse_move_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_abstract_button_focus_in_event(void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* e)) {
    WRAbstractButton* qt_object = (WRAbstractButton*)object;
    qt_object->m_focus_in_event = trampoline_func;
    qt_object->m_focus_in_event_user_data = user_data;
    qt_object->m_focus_in_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_abstract_button_focus_in_event(void* object) {
    WRAbstractButton* qt_object = (WRAbstractButton*)object;
    qt_object->m_focus_in_event = nullptr;
    qt_object->m_focus_in_event_user_data = nullptr;
    qt_object->m_focus_in_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_abstract_button_focus_out_event(void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* e)) {
    WRAbstractButton* qt_object = (WRAbstractButton*)object;
    qt_object->m_focus_out_event = trampoline_func;
    qt_object->m_focus_out_event_user_data = user_data;
    qt_object->m_focus_out_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_abstract_button_focus_out_event(void* object) {
    WRAbstractButton* qt_object = (WRAbstractButton*)object;
    qt_object->m_focus_out_event = nullptr;
    qt_object->m_focus_out_event_user_data = nullptr;
    qt_object->m_focus_out_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_abstract_button_change_event(void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* e)) {
    WRAbstractButton* qt_object = (WRAbstractButton*)object;
    qt_object->m_change_event = trampoline_func;
    qt_object->m_change_event_user_data = user_data;
    qt_object->m_change_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_abstract_button_change_event(void* object) {
    WRAbstractButton* qt_object = (WRAbstractButton*)object;
    qt_object->m_change_event = nullptr;
    qt_object->m_change_event_user_data = nullptr;
    qt_object->m_change_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUAbstractButtonFuncs s_abstract_button_funcs = {
    abstract_button_set_text,
    abstract_button_text,
    abstract_button_set_icon,
    abstract_button_icon,
    abstract_button_icon_size,
    abstract_button_set_shortcut,
    abstract_button_shortcut,
    abstract_button_set_checkable,
    abstract_button_is_checkable,
    abstract_button_is_checked,
    abstract_button_set_down,
    abstract_button_is_down,
    abstract_button_set_auto_repeat,
    abstract_button_auto_repeat,
    abstract_button_set_auto_repeat_delay,
    abstract_button_auto_repeat_delay,
    abstract_button_set_auto_repeat_interval,
    abstract_button_auto_repeat_interval,
    abstract_button_set_auto_exclusive,
    abstract_button_auto_exclusive,
    abstract_button_group,
    abstract_button_set_icon_size,
    abstract_button_animate_click,
    abstract_button_click,
    abstract_button_toggle,
    abstract_button_set_checked,
    set_abstract_button_pressed_event,
    set_abstract_button_released_event,
    set_abstract_button_clicked_event,
    set_abstract_button_toggled_event,
    set_abstract_button_paint_event,
    remove_abstract_button_paint_event,
    set_abstract_button_key_press_event,
    remove_abstract_button_key_press_event,
    set_abstract_button_key_release_event,
    remove_abstract_button_key_release_event,
    set_abstract_button_mouse_press_event,
    remove_abstract_button_mouse_press_event,
    set_abstract_button_mouse_release_event,
    remove_abstract_button_mouse_release_event,
    set_abstract_button_mouse_move_event,
    remove_abstract_button_mouse_move_event,
    set_abstract_button_focus_in_event,
    remove_abstract_button_focus_in_event,
    set_abstract_button_focus_out_event,
    remove_abstract_button_focus_out_event,
    set_abstract_button_change_event,
    remove_abstract_button_change_event,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUAbstractButtonAllFuncs s_abstract_button_all_funcs = {
    &s_object_funcs,
    &s_paint_device_funcs,
    &s_widget_funcs,
    &s_abstract_button_funcs,
};

