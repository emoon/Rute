#pragma once

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PrivData {
    QWidget* parent;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

template<typename T, typename QT> T create_widget_func(void* priv_data) {
    PrivData* data = (PrivData*)priv_data;
    QT* qt_obj = nullptr;
    if (data) {
        qt_obj = new QT(data->parent);
    } else {
        qt_obj = new QT(nullptr);
    }

    T ctl;
    ctl.priv_data = (struct RUBase*)qt_obj;

    return ctl;
}

