////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QLayoutItem>
#include "layout_item_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize layout_item_size_hint(struct RUBase* self_c) {
    QLayoutItem* qt_value = (QLayoutItem*)self_c;
    auto ret_value = qt_value->sizeHint();
    WRSize* new_val = new WRSize();
    *new_val = ret_value;
    struct RUSize ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_size_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize layout_item_minimum_size(struct RUBase* self_c) {
    QLayoutItem* qt_value = (QLayoutItem*)self_c;
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

static struct RUSize layout_item_maximum_size(struct RUBase* self_c) {
    QLayoutItem* qt_value = (QLayoutItem*)self_c;
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

static uint32_t layout_item_expanding_directions(struct RUBase* self_c) {
    QLayoutItem* qt_value = (QLayoutItem*)self_c;
    auto ret_value = qt_value->expandingDirections();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool layout_item_is_empty(struct RUBase* self_c) {
    QLayoutItem* qt_value = (QLayoutItem*)self_c;
    auto ret_value = qt_value->isEmpty();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool layout_item_has_height_for_width(struct RUBase* self_c) {
    QLayoutItem* qt_value = (QLayoutItem*)self_c;
    auto ret_value = qt_value->hasHeightForWidth();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int layout_item_height_for_width(struct RUBase* self_c, int arg0) {
    QLayoutItem* qt_value = (QLayoutItem*)self_c;
    auto ret_value = qt_value->heightForWidth(arg0);
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int layout_item_minimum_height_for_width(struct RUBase* self_c, int arg0) {
    QLayoutItem* qt_value = (QLayoutItem*)self_c;
    auto ret_value = qt_value->minimumHeightForWidth(arg0);
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void layout_item_invalidate(struct RUBase* self_c) {
    QLayoutItem* qt_value = (QLayoutItem*)self_c;
    qt_value->invalidate();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUWidget layout_item_widget(struct RUBase* self_c) {
    QLayoutItem* qt_value = (QLayoutItem*)self_c;
    auto ret_value = qt_value->widget();
    struct RUWidget ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_widget_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RULayout layout_item_layout(struct RUBase* self_c) {
    QLayoutItem* qt_value = (QLayoutItem*)self_c;
    auto ret_value = qt_value->layout();
    struct RULayout ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_layout_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSpacerItem layout_item_spacer_item(struct RUBase* self_c) {
    QLayoutItem* qt_value = (QLayoutItem*)self_c;
    auto ret_value = qt_value->spacerItem();
    struct RUSpacerItem ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_spacer_item_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t layout_item_alignment(struct RUBase* self_c) {
    QLayoutItem* qt_value = (QLayoutItem*)self_c;
    auto ret_value = qt_value->alignment();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void layout_item_set_alignment(struct RUBase* self_c, uint32_t a) {
    QLayoutItem* qt_value = (QLayoutItem*)self_c;
    qt_value->setAlignment((Qt::Alignment)a);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RULayoutItemFuncs s_layout_item_funcs = {
    layout_item_size_hint,
    layout_item_minimum_size,
    layout_item_maximum_size,
    layout_item_expanding_directions,
    layout_item_is_empty,
    layout_item_has_height_for_width,
    layout_item_height_for_width,
    layout_item_minimum_height_for_width,
    layout_item_invalidate,
    layout_item_widget,
    layout_item_layout,
    layout_item_spacer_item,
    layout_item_alignment,
    layout_item_set_alignment,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RULayoutItemAllFuncs s_layout_item_all_funcs = {
    &s_layout_item_funcs,
};

