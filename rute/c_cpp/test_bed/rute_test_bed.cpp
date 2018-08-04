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

struct RUArray {
    void** data;
    int count;
};

struct RUWidgetFuncs {
    void (*show)(struct RUBase* self_c);
    void (*set_parent)(struct RUBase* self_c, struct RUBase* parent);
    void (*set_size)(struct RUBase* self_c, int width, int height);
};

struct RUListWidgetFuncs {
    void (*destroy)(struct RUBase* self_c);
    void (*add_item)(struct RUBase* self_c, struct RUListWidgetItem* item);
    void (*clear)(struct RUBase* self_c);
    RUArray (*selected_items)(struct RUBase* self_c);
    void (*set_current_row)(struct RUBase* self_c, int item);
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
};

struct RUListWidgetItem {
    struct RUBase* priv_data;
    struct RUListWidgetItemFuncs* list_widget_item_funcs;
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
        DeleteCallback delete_callback, void* private_user_data);
    struct RUListWidgetItem (*create_list_widget_item)(
        struct RUBase* priv_data,
        DeleteCallback delete_callback, void* private_user_data);
} Rute;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

template<typename T, typename QT> T create_widget_func(
    void* priv_data,
    DeleteCallback delete_callback, void* private_user_data) {
    PrivData* data = (PrivData*)priv_data;
    QT* qt_obj = nullptr;
    if (data) {
        qt_obj = new QT(data->parent);
    } else {
        qt_obj = new QT(nullptr);
    }

    qt_obj->m_delete_callback = delete_callback;
    qt_obj->m_private_data = private_user_data;

    T ctl;
    ctl.priv_data = (struct RUBase*)qt_obj;

    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

template<typename T, typename QT> T create_generic_func(
    void* priv_data,
    DeleteCallback delete_callback, void* private_user_data) {
    (void)priv_data;

    QT* qt_obj = new QT();
    qt_obj->m_delete_callback = delete_callback;
    qt_obj->m_private_data = private_user_data;

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

static void widget_set_size(struct RUBase* self_c, int width, int height) {
    QWidget* qt_data = (QWidget*)self_c;
    qt_data->resize(width, height);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUWidgetFuncs s_widget_funcs = {
    widget_show,
    widget_set_parent,
    widget_set_size,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_destroy(struct RUBase* self_c) {
    WRListWidget* qt_data = (WRListWidget*)self_c;
    delete qt_data;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_clear(struct RUBase* self_c) {
    WRListWidget* qt_data = (WRListWidget*)self_c;
    qt_data->clear();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static RUArray list_widget_selected_items(struct RUBase* self_c) {
    WRListWidget* qt_data = (WRListWidget*)self_c;
    auto list = qt_data->selectedItems();
    int count = list.count();
    int real_count = 0;

    void** data = new void*[count];

    for (auto widget : list) {
        auto item = (WRListWidgetItem*)widget;
        // TODO: set user data on widget to make sure it's the correct type

        /*
        if (item == nullptr) {
            printf("item is of non-WRListWidgetItem type when collecing list. Skipping");
            continue;
        }
        */

        data[real_count] = item->m_private_data;
        real_count++;
    }

    return RUArray { data, real_count };
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_add_item(struct RUBase* self_c, struct RUListWidgetItem* data) {
    WRListWidgetItem* qt_in = (WRListWidgetItem*)data->priv_data;
    WRListWidget* qt_data = (WRListWidget*)self_c;
    qt_data->addItem(qt_in);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_set_current_row(struct RUBase* self_c, int item) {
    WRListWidget* qt_data = (WRListWidget*)self_c;
    qt_data->setCurrentRow(item);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUListWidgetFuncs s_list_widget_funcs = {
    list_widget_destroy,
    list_widget_add_item,
    list_widget_clear,
    list_widget_selected_items,
    list_widget_set_current_row,
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
    DeleteCallback delete_callback, void* private_user_data) {

    auto ctl = create_widget_func<struct RUListWidget, WRListWidget>(
        priv_data, delete_callback, private_user_data);

    ctl.widget_funcs = &s_widget_funcs;
    ctl.list_widget_funcs = &s_list_widget_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_text(struct RUBase* base, const char* text) {
    WRListWidgetItem* item = (WRListWidgetItem*)base;
    item->setText(text);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUListWidgetItemFuncs s_list_widget_item_funcs = {
    list_widget_item_set_text,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUListWidgetItem create_list_widget_item(
    struct RUBase* priv_data,
    DeleteCallback delete_callback, void* private_user_data) {

    auto ctl = create_generic_func<struct RUListWidgetItem, WRListWidgetItem>(
        priv_data, delete_callback, private_user_data);

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

