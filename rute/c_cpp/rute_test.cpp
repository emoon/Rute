#include <QApplication>
#include <QPushButton>
#include <QHBoxLayout>
#include <QWidget>
#include "rute_cpp.h"

struct RUBase;

extern "C" void dummy() {
    int c = 0;
    QApplication app (c, (char**)nullptr);

    QWidget *window = new QWidget;
    QPushButton *button1 = new QPushButton("One");
    QPushButton *button2 = new QPushButton("Two");
    QPushButton *button3 = new QPushButton("Three");
    QPushButton *button4 = new QPushButton("Four");
    QPushButton *button5 = new QPushButton("Five");

    QHBoxLayout *layout = new QHBoxLayout;
    layout->addWidget(button1);
    layout->addWidget(button2);
    layout->addWidget(button3);
    layout->addWidget(button4);
    layout->addWidget(button5);

    window->setLayout(layout);

    delete layout;

    //button5->setText("changed");

    window->show();

    app.exec();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUWidget {
    struct RUBase* priv_data;
};

struct PrivData {
    QWidget* parent;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern "C" void widget_show(struct RUBase* self_c) {
    WRWidget* qt_data = (WRWidget*)self_c;
    qt_data->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern "C" void slider_show(struct RUBase* self_c) {
    WRWidget* qt_data = (WRWidget*)self_c;
    qt_data->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern "C" void slider_connect_value_changed(struct RUBase* self_c, void* user_data,
    void (*tampoline_callback)(void* user_data, void* func_call, int value), void* wrapped_func) {
    WRSlider* slider = (WRSlider*)self_c;

    QSlotWrapperSignal_self_i32_void* wrap = new QSlotWrapperSignal_self_i32_void(
        user_data,
        (Signal_self_i32_void)tampoline_callback,
        wrapped_func);

    QObject::connect(slider, SIGNAL(valueChanged(int)), wrap, SLOT(method(int)));
}

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

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern "C" RUWidget create_widget() {
    return create_widget_func<struct RUWidget, WRWidget>(nullptr);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern "C" RUWidget create_slider() {
    return create_widget_func<struct RUWidget, WRSlider>(nullptr);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern "C" void widget_set_delete_callback(RUBase* base, void* user_data, void (*callback)(void*)) {
    WRWidget* w = (WRWidget*)base;
    w->m_delete_callback_data = user_data;
    w->m_delete_callback = callback;
}

static QApplication* s_application = nullptr;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern "C" void create_application() {
    static int c = 0;
    s_application = new QApplication (c, (char**)nullptr);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern "C" void run_application() {
    s_application->exec();
}

