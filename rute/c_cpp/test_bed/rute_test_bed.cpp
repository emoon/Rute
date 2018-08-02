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
    void (*destroy)(struct RUBase* self_c);
};

struct RUWidget {
    struct RUBase* priv_data;
    struct RUWidgetFuncs* widget_funcs;
};

struct RUListWidget {
    struct RUBase* priv_data;
    struct RUWidgetFuncs* widget_funcs;
    struct RUListWidgetFuncs* list_widget_funcs;
};

//////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUListWidgetItemFuncs {
    void (*set_text)(struct RUBase* self_c, const char* text);
}

struct RUListWidgetItem {
    struct RUBase* priv_data;
    struct RUListWidgetItemFuncs* list_widget_item_funcs;
};

struct RUListWidgetItemFuncs s_list_widget_item_funcs = {
    set_text,
};

//////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUApplication {
    struct RUBase* priv_data;
    struct RUApplicationFuncs* application_funcs;
};

typedef struct Rute {
    struct RUApplication (*create_application)(struct RUBase* priv_data);
    struct RUWidget (*create_widget)(struct RUBase* priv_data);
    struct RUListWidget (*create_list_widget)(
        struct RUBase* priv_data,
        DeleteCallback delete_callback, void* delete_user_data);
    struct RUListWidget (*create_list_widget_item)(
        struct RUBase* priv_data,
        DeleteCallback delete_callback, void* delete_user_data);
} Rute;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

template<typename T, typename QT> T create_widget_func(
    void* priv_data, 
    DeleteCallback delete_callback, void* delete_user_data) {
    PrivData* data = (PrivData*)priv_data;
    QT* qt_obj = nullptr;
    if (data) {
        qt_obj = new QT(data->parent);
    } else {
        qt_obj = new QT(nullptr);
    }

    qt_obj->m_delete_callback = delete_callback;
    qt_obj->m_delete_callback_data = delete_user_data;

    T ctl;
    ctl.priv_data = (struct RUBase*)qt_obj;

    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

template<typename T, typename QT> T create_generic_func(
    void* priv_data, 
    DeleteCallback delete_callback, void* delete_user_data) {
    (void)priv_data;

    QT* qt_obj = QT();
    qt_obj->m_delete_callback = delete_callback;
    qt_obj->m_delete_callback_data = delete_user_data;

    T ctl;
    ctl.priv_data = (struct RUBase*)qt_obj;

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
    QWidget* p = (QWidget*)parent;
    qt_data->setParent(p);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUWidgetFuncs s_widget_funcs = {
    widget_show,
    widget_set_parent,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_destroy(struct RUBase* self_c) {
    WRListWidget* qt_data = (WRListWidget*)self_c;
    delete qt_data;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUListWidgetFuncs s_list_widget_funcs = {
    list_widget_destroy,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUWidget create_widget(struct RUBase* priv_data) {
    auto ctl = create_widget_func<struct RUWidget, WRWidget>(priv_data, 0, 0);
    ctl.widget_funcs = &s_widget_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUListWidget create_list_widget(
    struct RUBase* priv_data,
    DeleteCallback delete_callback, void* delete_user_data) {

    auto ctl = create_widget_func<struct RUListWidget, WRListWidget>(
        priv_data, delete_callback, delete_user_data);

    ctl.widget_funcs = &s_widget_funcs;
    ctl.list_widget_funcs = &s_list_widget_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_text(struct RUBase* base, const char* text) {
    WRListWidgetItem* item = (WRListWidgetItem*)base;
    text->setText(text);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUListWidgetFuncs s_list_widget_item_funcs = {
    list_widget_item_set_text,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUListWidget list_widget_item_create (
    struct RUBase* priv_data,
    DeleteCallback delete_callback, void* delete_user_data) {

    auto ctl = create_generic_func<struct RUListWidgetItem, WRListWidgetItem>(
        priv_data, delete_callback, delete_user_data);

    ctl.list_widget_item_funcs = &s_list_widget_item_funcs;
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

static struct RUApplication create_application(struct RUBase* priv_data) {
    static int argc = 0;
    QApplication* qt_obj = new QApplication(argc, 0);

    (void)priv_data;
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
    create_list_widget_item,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern "C" struct Rute* rute_get() {
    return (Rute*)&s_rute;
}

