////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QLayout>
#include "layout_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int layout_margin(struct RUBase* self_c) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->margin();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int layout_spacing(struct RUBase* self_c) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->spacing();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void layout_set_margin(struct RUBase* self_c, int arg0) {
    WRLayout* qt_value = (WRLayout*)self_c;
    qt_value->setMargin(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void layout_set_spacing(struct RUBase* self_c, int arg0) {
    WRLayout* qt_value = (WRLayout*)self_c;
    qt_value->setSpacing(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void layout_set_contents_margins(struct RUBase* self_c, int left, int top, int right, int bottom) {
    WRLayout* qt_value = (WRLayout*)self_c;
    qt_value->setContentsMargins(left, top, right, bottom);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void layout_set_contents_margins_2(struct RUBase* self_c, struct RUBase* margins) {
    WRLayout* qt_value = (WRLayout*)self_c;
    qt_value->setContentsMargins(*((QMargins*)margins));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUMargins layout_contents_margins(struct RUBase* self_c) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->contentsMargins();
    WRMargins* new_val = new WRMargins();
    *new_val = ret_value;
    struct RUMargins ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_margins_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RURect layout_contents_rect(struct RUBase* self_c) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->contentsRect();
    WRRect* new_val = new WRRect();
    *new_val = ret_value;
    struct RURect ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_rect_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool layout_set_alignment(struct RUBase* self_c, struct RUBase* w, uint32_t alignment) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->setAlignment((QWidget*)w, (Qt::Alignment)alignment);
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool layout_set_alignment_2(struct RUBase* self_c, struct RUBase* l, uint32_t alignment) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->setAlignment((QLayout*)l, (Qt::Alignment)alignment);
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void layout_set_menu_bar(struct RUBase* self_c, struct RUBase* w) {
    WRLayout* qt_value = (WRLayout*)self_c;
    qt_value->setMenuBar((QWidget*)w);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUWidget layout_menu_bar(struct RUBase* self_c) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->menuBar();
    struct RUWidget ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_widget_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUWidget layout_parent_widget(struct RUBase* self_c) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->parentWidget();
    struct RUWidget ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_widget_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void layout_invalidate(struct RUBase* self_c) {
    WRLayout* qt_value = (WRLayout*)self_c;
    qt_value->invalidate();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool layout_activate(struct RUBase* self_c) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->activate();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void layout_update(struct RUBase* self_c) {
    WRLayout* qt_value = (WRLayout*)self_c;
    qt_value->update();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void layout_add_widget(struct RUBase* self_c, struct RUBase* w) {
    WRLayout* qt_value = (WRLayout*)self_c;
    qt_value->addWidget((QWidget*)w);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void layout_add_item(struct RUBase* self_c, struct RUBase* arg0) {
    WRLayout* qt_value = (WRLayout*)self_c;
    qt_value->addItem((QLayoutItem*)arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void layout_remove_widget(struct RUBase* self_c, struct RUBase* w) {
    WRLayout* qt_value = (WRLayout*)self_c;
    qt_value->removeWidget((QWidget*)w);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void layout_remove_item(struct RUBase* self_c, struct RUBase* arg0) {
    WRLayout* qt_value = (WRLayout*)self_c;
    qt_value->removeItem((QLayoutItem*)arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t layout_expanding_directions(struct RUBase* self_c) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->expandingDirections();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize layout_minimum_size(struct RUBase* self_c) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->minimumSize();
    WRSize* new_val = new WRSize();
    *new_val = ret_value;
    struct RUSize ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_size_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize layout_maximum_size(struct RUBase* self_c) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->maximumSize();
    WRSize* new_val = new WRSize();
    *new_val = ret_value;
    struct RUSize ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_size_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RULayoutItem layout_item_at(struct RUBase* self_c, int index) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->itemAt(index);
    struct RULayoutItem ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_layout_item_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RULayoutItem layout_take_at(struct RUBase* self_c, int index) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->takeAt(index);
    struct RULayoutItem ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_layout_item_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int layout_index_of(struct RUBase* self_c, struct RUBase* arg0) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->indexOf((QWidget*)arg0);
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int layout_count(struct RUBase* self_c) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->count();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool layout_is_empty(struct RUBase* self_c) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->isEmpty();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RULayoutItem layout_replace_widget(struct RUBase* self_c, struct RUBase* from, struct RUBase* to, uint32_t options) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->replaceWidget((QWidget*)from, (QWidget*)to, (Qt::FindChildOptions)options);
    struct RULayoutItem ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_layout_item_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int layout_total_height_for_width(struct RUBase* self_c, int w) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->totalHeightForWidth(w);
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize layout_total_minimum_size(struct RUBase* self_c) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->totalMinimumSize();
    WRSize* new_val = new WRSize();
    *new_val = ret_value;
    struct RUSize ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_size_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize layout_total_maximum_size(struct RUBase* self_c) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->totalMaximumSize();
    WRSize* new_val = new WRSize();
    *new_val = ret_value;
    struct RUSize ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_size_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize layout_total_size_hint(struct RUBase* self_c) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->totalSizeHint();
    WRSize* new_val = new WRSize();
    *new_val = ret_value;
    struct RUSize ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_size_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RULayout layout_layout(struct RUBase* self_c) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->layout();
    struct RULayout ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_layout_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void layout_set_enabled(struct RUBase* self_c, bool arg0) {
    WRLayout* qt_value = (WRLayout*)self_c;
    qt_value->setEnabled(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool layout_is_enabled(struct RUBase* self_c) {
    WRLayout* qt_value = (WRLayout*)self_c;
    auto ret_value = qt_value->isEnabled();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RULayoutFuncs s_layout_funcs = {
    layout_margin,
    layout_spacing,
    layout_set_margin,
    layout_set_spacing,
    layout_set_contents_margins,
    layout_set_contents_margins_2,
    layout_contents_margins,
    layout_contents_rect,
    layout_set_alignment,
    layout_set_alignment_2,
    layout_set_menu_bar,
    layout_menu_bar,
    layout_parent_widget,
    layout_invalidate,
    layout_activate,
    layout_update,
    layout_add_widget,
    layout_add_item,
    layout_remove_widget,
    layout_remove_item,
    layout_expanding_directions,
    layout_minimum_size,
    layout_maximum_size,
    layout_item_at,
    layout_take_at,
    layout_index_of,
    layout_count,
    layout_is_empty,
    layout_replace_widget,
    layout_total_height_for_width,
    layout_total_minimum_size,
    layout_total_maximum_size,
    layout_total_size_hint,
    layout_layout,
    layout_set_enabled,
    layout_is_enabled,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RULayoutAllFuncs s_layout_all_funcs = {
    &s_object_funcs,
    &s_layout_item_funcs,
    &s_layout_funcs,
};

