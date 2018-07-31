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
};

struct RUWidget {
    struct RUBase* priv_data;
    struct RUWidgetFuncs* widget_funcs;
};

struct RUApplication {
    struct RUBase* priv_data;
    struct RUApplicationFuncs* application_funcs;
};

typedef struct Rute {
    struct RUApplication (*create_application)(struct RUBase* priv_data, void* user_data);
    struct RUWidget (*create_widget)(struct RUBase* priv_data, void* user_data);
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

class WRWidget : public QWidget {
    Q_OBJECT
public:
    WRWidget(QWidget* widget) : QWidget(widget) { }
    virtual ~WRWidget() {}
};

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

struct RUWidgetFuncs s_widget_funcs = {
    widget_show,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUWidget create_widget(struct RUBase* priv_data, void* user_data) {
    auto ctl = create_widget_func<struct RUWidget, WRWidget>(priv_data, user_data);
    ctl.widget_funcs = &s_widget_funcs;
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
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


#ifdef _WIN32
extern "C" __declspec(dllexport) struct Rute* rute_get() {
#else
extern "C" struct Rute* rute_get() {
#endif
    return (Rute*)&s_rute;
}

