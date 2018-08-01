#include "rute_test_bed.h"
#include <QApplication>
#include <QWidget>

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PrivData {
    QWidget* parent;
};

struct RUBase;

struct RUApplicationFuncs {
    void (*exec)(struct RUBase* self_c);
};

struct RUWidgetFuncs {
    void (*show)(struct RUBase* self_c);
    void (*set_parent)(struct RUBase* self_c, struct RUBase* parent);
};

struct RUListWidgetFuncs {
    void (*show)(struct RUBase* self_c);
};

struct RUWidget {
    struct RUBase* priv_data;
    struct RUWidgetFuncs* widget_funcs;
};

struct RUListWidget {
    struct RUBase* priv_data;
    struct RUListWidgetFuncs* widget_funcs;
};

struct RUApplication {
    struct RUBase* priv_data;
    struct RUApplicationFuncs* application_funcs;
};

typedef struct Rute {
    struct RUApplication (*create_application)(struct RUBase* priv_data, void* user_data);
    struct RUWidget (*create_widget)(struct RUBase* priv_data, void* user_data);
    struct RUListWidget (*create_list_widget)(struct RUBase* priv_data, void* user_data);
} Rute;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

template<typename T, typename QT> T create_widget_func(void* priv_data, void* user_data) {
    PrivData* data = (PrivData*)priv_data;
    (void)user_data;
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

template<typename T, typename QT> T create_generic_func() {
    QT* qt_obj = new QT();
    T ctl;
    ctl.priv_data = (struct PUBase*)qt_obj;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void widget_show(struct RUBase* self_c) {
    WRWidget* qt_data = (WRWidget*)self_c;
    qt_data->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void widget_set_parent(struct RUBase* self_c, struct RUBase* parent) {
    WRWidget* qt_data = (WRWidget*)self_c;
    WRWidget* p = (WRWidget*)p;
    qt_data->setParent(p);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUWidgetFuncs s_widget_funcs = {
    widget_show,
    widget_set_parent,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUListWidgetFuncs s_list_widget_funcs = {
    widget_show,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUWidget create_widget(struct RUBase* priv_data, void* user_data) {
    auto ctl = create_widget_func<struct RUWidget, WRWidget>(priv_data, user_data);
    ctl.widget_funcs = &s_widget_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUWidget create_list_widget(struct RUBase* priv_data, void* user_data) {
    auto ctl = create_widget_func<struct RUListWidget, WRListWidget>(priv_data, user_data);
    ctl.list_widget_funcs = &s_list_widget_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_exec(struct RUBase* self_c) {
    QApplication* qt_data = (QApplication*)self_c;
    qt_data->exec();
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUApplicationFuncs s_application_funcs = {
    application_exec,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUApplication create_application(struct RUBase* priv_data, void* user_data) {
    static int argc = 0;
    QApplication* qt_obj = new QApplication(argc, 0);

    (void)priv_data;
    (void)user_data;
    RUApplication ctl;
    ctl.application_funcs = &s_application_funcs;
    ctl.priv_data = (struct RUBase*)qt_obj;

    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct Rute s_rute = {
    create_application,
    create_widget,
    create_list_widget,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern "C" struct Rute* rute_get() {
    return (Rute*)&s_rute;
}

