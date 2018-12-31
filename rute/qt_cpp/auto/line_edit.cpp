////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QLineEdit>
#include "line_edit_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* line_edit_text(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->text();
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* line_edit_display_text(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->displayText();
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* line_edit_placeholder_text(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->placeholderText();
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_set_placeholder_text(struct RUBase* self_c, const char* arg0) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->setPlaceholderText(QString::fromUtf8(arg0));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int line_edit_max_length(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->maxLength();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_set_max_length(struct RUBase* self_c, int arg0) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->setMaxLength(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_set_frame(struct RUBase* self_c, bool arg0) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->setFrame(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool line_edit_has_frame(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->hasFrame();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_set_clear_button_enabled(struct RUBase* self_c, bool enable) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->setClearButtonEnabled(enable);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool line_edit_is_clear_button_enabled(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->isClearButtonEnabled();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t line_edit_echo_mode(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->echoMode();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_set_echo_mode(struct RUBase* self_c, uint32_t arg0) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->setEchoMode((QLineEdit::EchoMode)arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool line_edit_is_read_only(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->isReadOnly();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_set_read_only(struct RUBase* self_c, bool arg0) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->setReadOnly(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int line_edit_cursor_position(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->cursorPosition();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_set_cursor_position(struct RUBase* self_c, int arg0) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->setCursorPosition(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int line_edit_cursor_position_at(struct RUBase* self_c, struct RUBase* pos) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->cursorPositionAt(*((WRPoint*)pos));
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_set_alignment(struct RUBase* self_c, uint32_t flag) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->setAlignment((Qt::Alignment)flag);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t line_edit_alignment(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->alignment();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_cursor_forward(struct RUBase* self_c, bool mark, int steps) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->cursorForward(mark, steps);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_cursor_backward(struct RUBase* self_c, bool mark, int steps) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->cursorBackward(mark, steps);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_cursor_word_forward(struct RUBase* self_c, bool mark) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->cursorWordForward(mark);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_cursor_word_backward(struct RUBase* self_c, bool mark) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->cursorWordBackward(mark);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_backspace(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->backspace();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_del(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->del();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_home(struct RUBase* self_c, bool mark) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->home(mark);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_end(struct RUBase* self_c, bool mark) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->end(mark);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool line_edit_is_modified(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->isModified();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_set_modified(struct RUBase* self_c, bool arg0) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->setModified(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_set_selection(struct RUBase* self_c, int arg0, int arg1) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->setSelection(arg0, arg1);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool line_edit_has_selected_text(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->hasSelectedText();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* line_edit_selected_text(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->selectedText();
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int line_edit_selection_start(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->selectionStart();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int line_edit_selection_end(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->selectionEnd();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int line_edit_selection_length(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->selectionLength();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool line_edit_is_undo_available(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->isUndoAvailable();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool line_edit_is_redo_available(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->isRedoAvailable();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_set_drag_enabled(struct RUBase* self_c, bool b) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->setDragEnabled(b);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool line_edit_drag_enabled(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->dragEnabled();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_set_cursor_move_style(struct RUBase* self_c, uint32_t style) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->setCursorMoveStyle((Qt::CursorMoveStyle)style);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t line_edit_cursor_move_style(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->cursorMoveStyle();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* line_edit_input_mask(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->inputMask();
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_set_input_mask(struct RUBase* self_c, const char* input_mask) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->setInputMask(QString::fromUtf8(input_mask));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool line_edit_has_acceptable_input(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->hasAcceptableInput();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_set_text_margins(struct RUBase* self_c, int left, int top, int right, int bottom) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->setTextMargins(left, top, right, bottom);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_set_text_margins_2(struct RUBase* self_c, struct RUBase* margins) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->setTextMargins(*((WRMargins*)margins));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUMargins line_edit_text_margins(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    auto ret_value = qt_value->textMargins();
    WRMargins* new_val = new WRMargins();
    *new_val = ret_value;
    struct RUMargins ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_margins_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_set_text(struct RUBase* self_c, const char* arg0) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->setText(QString::fromUtf8(arg0));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_clear(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->clear();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_select_all(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->selectAll();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_undo(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->undo();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_redo(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->redo();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_cut(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->cut();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_copy(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->copy();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_paste(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->paste();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_deselect(struct RUBase* self_c) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->deselect();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void line_edit_insert(struct RUBase* self_c, const char* arg0) {
    WRLineEdit* qt_value = (WRLineEdit*)self_c;
    qt_value->insert(QString::fromUtf8(arg0));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_text_changed_event(void* object, void* user_data, void* wrapped_func, void (*event)(void*, void* self_c, const char* arg0)) {
    QSlotWrapperSignal_self_string_void* wrap = new QSlotWrapperSignal_self_string_void(user_data, (Signal_self_string_void)event, (void*)wrapped_func);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(textChanged(QString)), wrap, SLOT(method(QString)));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_text_edited_event(void* object, void* user_data, void* wrapped_func, void (*event)(void*, void* self_c, const char* arg0)) {
    QSlotWrapperSignal_self_string_void* wrap = new QSlotWrapperSignal_self_string_void(user_data, (Signal_self_string_void)event, (void*)wrapped_func);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(textEdited(QString)), wrap, SLOT(method(QString)));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_cursor_position_changed_event(void* object, void* user_data, void* wrapped_func, void (*event)(void*, void* self_c, int arg0, int arg1)) {
    QSlotWrapperSignal_self_int_int_void* wrap = new QSlotWrapperSignal_self_int_int_void(user_data, (Signal_self_int_int_void)event, (void*)wrapped_func);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(cursorPositionChanged(int, int)), wrap, SLOT(method(int, int)));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_return_pressed_event(void* object, void* user_data, void* wrapped_func, void (*event)(void*, void* self_c)) {
    QSlotWrapperSignal_self_void* wrap = new QSlotWrapperSignal_self_void(user_data, (Signal_self_void)event, (void*)wrapped_func);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(returnPressed()), wrap, SLOT(method()));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_editing_finished_event(void* object, void* user_data, void* wrapped_func, void (*event)(void*, void* self_c)) {
    QSlotWrapperSignal_self_void* wrap = new QSlotWrapperSignal_self_void(user_data, (Signal_self_void)event, (void*)wrapped_func);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(editingFinished()), wrap, SLOT(method()));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_selection_changed_event(void* object, void* user_data, void* wrapped_func, void (*event)(void*, void* self_c)) {
    QSlotWrapperSignal_self_void* wrap = new QSlotWrapperSignal_self_void(user_data, (Signal_self_void)event, (void*)wrapped_func);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(selectionChanged()), wrap, SLOT(method()));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_mouse_press_event(
    void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* arg0)) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_mouse_press_event = trampoline_func;
    qt_object->m_mouse_press_event_user_data = user_data;
    qt_object->m_mouse_press_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_line_edit_mouse_press_event(void* object) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_mouse_press_event = nullptr;
    qt_object->m_mouse_press_event_user_data = nullptr;
    qt_object->m_mouse_press_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_mouse_move_event(
    void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* arg0)) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_mouse_move_event = trampoline_func;
    qt_object->m_mouse_move_event_user_data = user_data;
    qt_object->m_mouse_move_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_line_edit_mouse_move_event(void* object) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_mouse_move_event = nullptr;
    qt_object->m_mouse_move_event_user_data = nullptr;
    qt_object->m_mouse_move_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_mouse_release_event(
    void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* arg0)) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_mouse_release_event = trampoline_func;
    qt_object->m_mouse_release_event_user_data = user_data;
    qt_object->m_mouse_release_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_line_edit_mouse_release_event(void* object) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_mouse_release_event = nullptr;
    qt_object->m_mouse_release_event_user_data = nullptr;
    qt_object->m_mouse_release_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_mouse_double_click_event(
    void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* arg0)) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_mouse_double_click_event = trampoline_func;
    qt_object->m_mouse_double_click_event_user_data = user_data;
    qt_object->m_mouse_double_click_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_line_edit_mouse_double_click_event(void* object) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_mouse_double_click_event = nullptr;
    qt_object->m_mouse_double_click_event_user_data = nullptr;
    qt_object->m_mouse_double_click_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_key_press_event(
    void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* arg0)) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_key_press_event = trampoline_func;
    qt_object->m_key_press_event_user_data = user_data;
    qt_object->m_key_press_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_line_edit_key_press_event(void* object) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_key_press_event = nullptr;
    qt_object->m_key_press_event_user_data = nullptr;
    qt_object->m_key_press_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_focus_in_event(
    void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* arg0)) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_focus_in_event = trampoline_func;
    qt_object->m_focus_in_event_user_data = user_data;
    qt_object->m_focus_in_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_line_edit_focus_in_event(void* object) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_focus_in_event = nullptr;
    qt_object->m_focus_in_event_user_data = nullptr;
    qt_object->m_focus_in_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_focus_out_event(
    void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* arg0)) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_focus_out_event = trampoline_func;
    qt_object->m_focus_out_event_user_data = user_data;
    qt_object->m_focus_out_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_line_edit_focus_out_event(void* object) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_focus_out_event = nullptr;
    qt_object->m_focus_out_event_user_data = nullptr;
    qt_object->m_focus_out_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_paint_event(
    void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* arg0)) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_paint_event = trampoline_func;
    qt_object->m_paint_event_user_data = user_data;
    qt_object->m_paint_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_line_edit_paint_event(void* object) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_paint_event = nullptr;
    qt_object->m_paint_event_user_data = nullptr;
    qt_object->m_paint_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_drag_enter_event(
    void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* arg0)) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_drag_enter_event = trampoline_func;
    qt_object->m_drag_enter_event_user_data = user_data;
    qt_object->m_drag_enter_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_line_edit_drag_enter_event(void* object) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_drag_enter_event = nullptr;
    qt_object->m_drag_enter_event_user_data = nullptr;
    qt_object->m_drag_enter_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_drag_move_event(
    void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* e)) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_drag_move_event = trampoline_func;
    qt_object->m_drag_move_event_user_data = user_data;
    qt_object->m_drag_move_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_line_edit_drag_move_event(void* object) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_drag_move_event = nullptr;
    qt_object->m_drag_move_event_user_data = nullptr;
    qt_object->m_drag_move_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_drag_leave_event(
    void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* e)) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_drag_leave_event = trampoline_func;
    qt_object->m_drag_leave_event_user_data = user_data;
    qt_object->m_drag_leave_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_line_edit_drag_leave_event(void* object) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_drag_leave_event = nullptr;
    qt_object->m_drag_leave_event_user_data = nullptr;
    qt_object->m_drag_leave_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_drop_event(
    void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* arg0)) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_drop_event = trampoline_func;
    qt_object->m_drop_event_user_data = user_data;
    qt_object->m_drop_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_line_edit_drop_event(void* object) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_drop_event = nullptr;
    qt_object->m_drop_event_user_data = nullptr;
    qt_object->m_drop_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_change_event(
    void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* arg0)) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_change_event = trampoline_func;
    qt_object->m_change_event_user_data = user_data;
    qt_object->m_change_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_line_edit_change_event(void* object) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_change_event = nullptr;
    qt_object->m_change_event_user_data = nullptr;
    qt_object->m_change_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_line_edit_context_menu_event(
    void* object, void* user_data, void* wrapped_func, void (*trampoline_func)(void*, void* self_c, struct RUBase* arg0)) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_context_menu_event = trampoline_func;
    qt_object->m_context_menu_event_user_data = user_data;
    qt_object->m_context_menu_event_wrapped_func = wrapped_func;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void remove_line_edit_context_menu_event(void* object) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_context_menu_event = nullptr;
    qt_object->m_context_menu_event_user_data = nullptr;
    qt_object->m_context_menu_event_wrapped_func = nullptr;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RULineEdit create_line_edit(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = create_widget_func<struct RULineEdit, WRLineEdit>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_line_edit_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_line_edit(struct RUBase* priv_data) {
    destroy_generic<WRLineEdit>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RULineEditFuncs s_line_edit_funcs = {
    destroy_line_edit,
    line_edit_text,
    line_edit_display_text,
    line_edit_placeholder_text,
    line_edit_set_placeholder_text,
    line_edit_max_length,
    line_edit_set_max_length,
    line_edit_set_frame,
    line_edit_has_frame,
    line_edit_set_clear_button_enabled,
    line_edit_is_clear_button_enabled,
    line_edit_echo_mode,
    line_edit_set_echo_mode,
    line_edit_is_read_only,
    line_edit_set_read_only,
    line_edit_cursor_position,
    line_edit_set_cursor_position,
    line_edit_cursor_position_at,
    line_edit_set_alignment,
    line_edit_alignment,
    line_edit_cursor_forward,
    line_edit_cursor_backward,
    line_edit_cursor_word_forward,
    line_edit_cursor_word_backward,
    line_edit_backspace,
    line_edit_del,
    line_edit_home,
    line_edit_end,
    line_edit_is_modified,
    line_edit_set_modified,
    line_edit_set_selection,
    line_edit_has_selected_text,
    line_edit_selected_text,
    line_edit_selection_start,
    line_edit_selection_end,
    line_edit_selection_length,
    line_edit_is_undo_available,
    line_edit_is_redo_available,
    line_edit_set_drag_enabled,
    line_edit_drag_enabled,
    line_edit_set_cursor_move_style,
    line_edit_cursor_move_style,
    line_edit_input_mask,
    line_edit_set_input_mask,
    line_edit_has_acceptable_input,
    line_edit_set_text_margins,
    line_edit_set_text_margins_2,
    line_edit_text_margins,
    line_edit_set_text,
    line_edit_clear,
    line_edit_select_all,
    line_edit_undo,
    line_edit_redo,
    line_edit_cut,
    line_edit_copy,
    line_edit_paste,
    line_edit_deselect,
    line_edit_insert,
    set_line_edit_text_changed_event,
    set_line_edit_text_edited_event,
    set_line_edit_cursor_position_changed_event,
    set_line_edit_return_pressed_event,
    set_line_edit_editing_finished_event,
    set_line_edit_selection_changed_event,
    set_line_edit_mouse_press_event,
    remove_line_edit_mouse_press_event,
    set_line_edit_mouse_move_event,
    remove_line_edit_mouse_move_event,
    set_line_edit_mouse_release_event,
    remove_line_edit_mouse_release_event,
    set_line_edit_mouse_double_click_event,
    remove_line_edit_mouse_double_click_event,
    set_line_edit_key_press_event,
    remove_line_edit_key_press_event,
    set_line_edit_focus_in_event,
    remove_line_edit_focus_in_event,
    set_line_edit_focus_out_event,
    remove_line_edit_focus_out_event,
    set_line_edit_paint_event,
    remove_line_edit_paint_event,
    set_line_edit_drag_enter_event,
    remove_line_edit_drag_enter_event,
    set_line_edit_drag_move_event,
    remove_line_edit_drag_move_event,
    set_line_edit_drag_leave_event,
    remove_line_edit_drag_leave_event,
    set_line_edit_drop_event,
    remove_line_edit_drop_event,
    set_line_edit_change_event,
    remove_line_edit_change_event,
    set_line_edit_context_menu_event,
    remove_line_edit_context_menu_event,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RULineEditAllFuncs s_line_edit_all_funcs = {
    &s_object_funcs,
    &s_paint_device_funcs,
    &s_widget_funcs,
    &s_line_edit_funcs,
};

