#pragma once

#include <map>

//class QWidget;

//extern std::map<QWidget*, void*> s_widget_lookup;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PrivData {
    QWidget* parent;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

template<typename T, typename QT> T* create_widget_func(void* priv_data, void* user_data) {
    PrivData* data = (PrivData*)priv_data;
    QT* qt_obj = nullptr;
    if (data) {
        qt_obj = new QT(data->parent);
    } else {
        qt_obj = new QT(nullptr);
    }

    // track a widget with accciated user data. This allows us to callback into Rust or other wrappers
    // when a widget gets destroyed, or used in such way where we want this
    s_widget_lookup[qt_obj] = user_data;

    T* ctl = new T;
    ctl->priv_data = (struct RUBase*)qt_obj;
    qt_obj->m_ru_ctl = ctl;

    return ctl;
}

