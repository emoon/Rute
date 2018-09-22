#pragma once

#include <map>
#include "rute_base.h"

class QWidget;

extern std::map<void*, void*> s_host_data_lookup;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PrivData {
    QWidget* parent;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

template<typename QT> void destroy_generic(struct RUBase* qt_data) {
    QT* qt_obj = (QT*)qt_data;
    delete qt_obj;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

template<typename T, typename F, typename QT> T create_widget_func(
    struct RUBase* priv_data, const F* all_funcs,
    RUDeleteCallback delete_callback, void* host_data)
{
    PrivData* data = (PrivData*)priv_data;
    QT* qt_obj = nullptr;
    if (data) {
        qt_obj = new QT(data->parent);
    } else {
        qt_obj = new QT(nullptr);
    }

    s_host_data_lookup[qt_obj] = host_data;
    qt_obj->m_delete_callback = delete_callback;

    T ctl = ctl_template;
    ctl.qt_data = (struct RUBase*)qt_obj;
    ctl.host_data = nullptr;
    ctl.all_funcs = all_funcs;

    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

template<typename T, typename F, typename QT> T generic_create_func(F* funcs)
{
    (void)priv_data;
    QT* qt_obj = new QT();

    T ctl;
    ctl.qt_data = (struct RUBase*)qt_obj;
    ctl.host_data = nullptr;
    ctl.all_funcs = all_funcs;

    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

template<typename T, typename F, typename QT> T generic_create_func_with_delete(
    struct RUBase* priv_data, F* all_funcs,
    RUDeleteCallback delete_callback, void* private_user_data) {

    (void)priv_data;

    QT* qt_obj = new QT();
    qt_obj->m_delete_callback = delete_callback;
    qt_obj->m_private_data = private_user_data;

    T ctl;
    ctl.qt_data = (struct RUBase*)qt_obj;
    ctl.host_data = nullptr;
    ctl.all_funs = all_funcs;

    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUApplication create_application(struct RUBase* priv_data);
void destroy_application(struct RUBase* priv_data);


