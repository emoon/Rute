////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QKeySequence>
#include "key_sequence_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int key_sequence_count(struct RUBase* self_c) {
    WRKeySequence* qt_value = (WRKeySequence*)self_c;
    auto ret_value = qt_value->count();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool key_sequence_is_empty(struct RUBase* self_c) {
    WRKeySequence* qt_value = (WRKeySequence*)self_c;
    auto ret_value = qt_value->isEmpty();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t key_sequence_matches(struct RUBase* self_c, struct RUBase* seq) {
    WRKeySequence* qt_value = (WRKeySequence*)self_c;
    auto ret_value = qt_value->matches(*((QKeySequence*)seq));
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUKeySequence key_sequence_mnemonic(struct RUBase* self_c, const char* text) {
    WRKeySequence* qt_value = (WRKeySequence*)self_c;
    auto ret_value = qt_value->mnemonic(QString::fromUtf8(text));
    WRKeySequence* new_val = new WRKeySequence();
    *new_val = ret_value;
    struct RUKeySequence ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_key_sequence_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void key_sequence_swap(struct RUBase* self_c, struct RUBase* other) {
    WRKeySequence* qt_value = (WRKeySequence*)self_c;
    qt_value->swap(*((QKeySequence*)other));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool key_sequence_is_detached(struct RUBase* self_c) {
    WRKeySequence* qt_value = (WRKeySequence*)self_c;
    auto ret_value = qt_value->isDetached();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUKeySequence create_key_sequence(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = generic_create_func_with_delete<struct RUKeySequence, WRKeySequence>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_key_sequence_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_key_sequence(struct RUBase* priv_data) {
    destroy_generic<WRKeySequence>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUKeySequence get_key_sequence(struct RUBase* priv_data) {
    (void)priv_data;
    RUKeySequence ctl;
    ctl.qt_data = nullptr;
    ctl.host_data = nullptr;
    ctl.all_funcs = &s_key_sequence_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUKeySequenceFuncs s_key_sequence_funcs = {
    destroy_key_sequence,
    key_sequence_count,
    key_sequence_is_empty,
    key_sequence_matches,
    key_sequence_mnemonic,
    key_sequence_swap,
    key_sequence_is_detached,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUKeySequenceAllFuncs s_key_sequence_all_funcs = {
    &s_key_sequence_funcs,
};

