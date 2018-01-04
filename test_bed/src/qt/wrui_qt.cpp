#include "c_api.h"
#include "qt_api_gen.h"
#include <assert.h>
#include <QObject>
#include <QWidget>
#include <QPushButton>
#include <QPainter>
#include <QListWidgetItem>
#include <QListWidget>
#include <QSlider>
#include <QMainWindow>
#include <QAction>
#include <QMenu>
#include <QMenuBar>
#include <QApplication>

struct PrivData {
    QWidget* parent;
    void* user_data;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern struct PUObjectFuncs s_object_funcs;
extern struct PUWidgetFuncs s_widget_funcs;
extern struct PUPushButtonFuncs s_push_button_funcs;
extern struct PUPainterFuncs s_painter_funcs;
extern struct PUListWidgetItemFuncs s_list_widget_item_funcs;
extern struct PUListWidgetFuncs s_list_widget_funcs;
extern struct PUSliderFuncs s_slider_funcs;
extern struct PUMainWindowFuncs s_main_window_funcs;
extern struct PUActionFuncs s_action_funcs;
extern struct PUMenuFuncs s_menu_funcs;
extern struct PUMenuBarFuncs s_menu_bar_funcs;
extern struct PUApplicationFuncs s_application_funcs;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRWidget : public QWidget {
public:
    WRWidget(QWidget* widget) : QWidget(widget) {}
    virtual ~WRWidget() {}

};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRPushButton : public QPushButton {
public:
    WRPushButton(QWidget* widget) : QPushButton(widget) {}
    virtual ~WRPushButton() {}

};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRListWidget : public QListWidget {
public:
    WRListWidget(QWidget* widget) : QListWidget(widget) {}
    virtual ~WRListWidget() {}

};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRSlider : public QSlider {
public:
    WRSlider(QWidget* widget) : QSlider(widget) {}
    virtual ~WRSlider() {}

};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRMainWindow : public QMainWindow {
public:
    WRMainWindow(QWidget* widget) : QMainWindow(widget) {}
    virtual ~WRMainWindow() {}

};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRAction : public QAction {
public:
    WRAction(QWidget* widget) : QAction(widget) {}
    virtual ~WRAction() {}

};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRMenu : public QMenu {
public:
    WRMenu(QWidget* widget) : QMenu(widget) {}
    virtual ~WRMenu() {}

};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRMenuBar : public QMenuBar {
public:
    WRMenuBar(QWidget* widget) : QMenuBar(widget) {}
    virtual ~WRMenuBar() {}

};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool object_is_widget_type(struct PUBase* self_c) { 
    QObject* qt_data = (QObject*)self_c;
    auto ret_value = qt_data->isWidgetType();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool widget_is_widget_type(struct PUBase* self_c) { 
    QWidget* qt_data = (QWidget*)self_c;
    auto ret_value = qt_data->isWidgetType();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void widget_show(struct PUBase* self_c) { 
    WRWidget* qt_data = (WRWidget*)self_c;
    qt_data->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void widget_resize(struct PUBase* self_c, int width, int height) { 
    WRWidget* qt_data = (WRWidget*)self_c;
    qt_data->resize(width, height);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool push_button_is_widget_type(struct PUBase* self_c) { 
    QPushButton* qt_data = (QPushButton*)self_c;
    auto ret_value = qt_data->isWidgetType();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void push_button_show(struct PUBase* self_c) { 
    WRPushButton* qt_data = (WRPushButton*)self_c;
    qt_data->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void push_button_resize(struct PUBase* self_c, int width, int height) { 
    WRPushButton* qt_data = (WRPushButton*)self_c;
    qt_data->resize(width, height);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_push_button_released_event(void* object, void* user_data, void (*event)(void* self_c)) {
    QSlotWrapperSignal_self_void* wrap = new QSlotWrapperSignal_self_void(user_data, (Signal_self_void)event);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(released()), wrap, SLOT(method()));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void push_button_set_text(struct PUBase* self_c, const char* text) { 
    WRPushButton* qt_data = (WRPushButton*)self_c;
    qt_data->setText(QString::fromLatin1(text));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void push_button_set_flat(struct PUBase* self_c, bool flat) { 
    WRPushButton* qt_data = (WRPushButton*)self_c;
    qt_data->setFlat(flat);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void painter_draw_line(struct PUBase* self_c, int x1, int y1, int x2, int y2) { 
    QPainter* qt_data = (QPainter*)self_c;
    qt_data->drawLine(x1, y1, x2, y2);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_text(struct PUBase* self_c, const char* text) { 
    QListWidgetItem* qt_data = (QListWidgetItem*)self_c;
    qt_data->setText(QString::fromLatin1(text));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool list_widget_is_widget_type(struct PUBase* self_c) { 
    QListWidget* qt_data = (QListWidget*)self_c;
    auto ret_value = qt_data->isWidgetType();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_show(struct PUBase* self_c) { 
    WRListWidget* qt_data = (WRListWidget*)self_c;
    qt_data->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_resize(struct PUBase* self_c, int width, int height) { 
    WRListWidget* qt_data = (WRListWidget*)self_c;
    qt_data->resize(width, height);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_add_item(struct PUBase* self_c, const char* text) { 
    WRListWidget* qt_data = (WRListWidget*)self_c;
    qt_data->addItem(QString::fromLatin1(text));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUListWidgetItem list_widget_item(struct PUBase* self_c, int index) { 
    WRListWidget* qt_data = (WRListWidget*)self_c;
    auto ret_value = qt_data->item(index);
    PUListWidgetItem ctl;
    ctl.funcs = &s_list_widget_item_funcs;
    ctl.priv_data = (struct PUBase*)ret_value;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_list_widget_current_row_changed_event(void* object, void* user_data, void (*event)(void* self_c, int row)) {
    QSlotWrapperSignal_self_i32_void* wrap = new QSlotWrapperSignal_self_i32_void(user_data, (Signal_self_i32_void)event);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(currentRowChanged(int)), wrap, SLOT(method(int)));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool slider_is_widget_type(struct PUBase* self_c) { 
    QSlider* qt_data = (QSlider*)self_c;
    auto ret_value = qt_data->isWidgetType();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void slider_show(struct PUBase* self_c) { 
    WRSlider* qt_data = (WRSlider*)self_c;
    qt_data->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void slider_resize(struct PUBase* self_c, int width, int height) { 
    WRSlider* qt_data = (WRSlider*)self_c;
    qt_data->resize(width, height);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_slider_value_changed_event(void* object, void* user_data, void (*event)(void* self_c, int value)) {
    QSlotWrapperSignal_self_i32_void* wrap = new QSlotWrapperSignal_self_i32_void(user_data, (Signal_self_i32_void)event);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(valueChanged(int)), wrap, SLOT(method(int)));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool main_window_is_widget_type(struct PUBase* self_c) { 
    QMainWindow* qt_data = (QMainWindow*)self_c;
    auto ret_value = qt_data->isWidgetType();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void main_window_show(struct PUBase* self_c) { 
    WRMainWindow* qt_data = (WRMainWindow*)self_c;
    qt_data->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void main_window_resize(struct PUBase* self_c, int width, int height) { 
    WRMainWindow* qt_data = (WRMainWindow*)self_c;
    qt_data->resize(width, height);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool main_window_is_animated(struct PUBase* self_c) { 
    WRMainWindow* qt_data = (WRMainWindow*)self_c;
    auto ret_value = qt_data->isAnimated();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUMenuBar main_window_menu_bar(struct PUBase* self_c) { 
    WRMainWindow* qt_data = (WRMainWindow*)self_c;
    auto ret_value = qt_data->menuBar();
    PUMenuBar ctl;
    ctl.funcs = &s_menu_bar_funcs;
    ctl.priv_data = (struct PUBase*)ret_value;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void main_window_set_central_widget(struct PUBase* self_c, struct PUBase* widget) { 
    WRMainWindow* qt_data = (WRMainWindow*)self_c;
    qt_data->setCentralWidget((QWidget*)widget);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool action_is_widget_type(struct PUBase* self_c) { 
    QAction* qt_data = (QAction*)self_c;
    auto ret_value = qt_data->isWidgetType();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool action_is_enabled(struct PUBase* self_c) { 
    WRAction* qt_data = (WRAction*)self_c;
    auto ret_value = qt_data->isEnabled();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void action_set_text(struct PUBase* self_c, const char* text) { 
    WRAction* qt_data = (WRAction*)self_c;
    qt_data->setText(QString::fromLatin1(text));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool menu_is_widget_type(struct PUBase* self_c) { 
    QMenu* qt_data = (QMenu*)self_c;
    auto ret_value = qt_data->isWidgetType();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void menu_show(struct PUBase* self_c) { 
    WRMenu* qt_data = (WRMenu*)self_c;
    qt_data->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void menu_resize(struct PUBase* self_c, int width, int height) { 
    WRMenu* qt_data = (WRMenu*)self_c;
    qt_data->resize(width, height);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void menu_add_action(struct PUBase* self_c, struct PUBase* action) { 
    WRMenu* qt_data = (WRMenu*)self_c;
    qt_data->addAction((QAction*)action);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void menu_set_title(struct PUBase* self_c, const char* title) { 
    WRMenu* qt_data = (WRMenu*)self_c;
    qt_data->setTitle(QString::fromLatin1(title));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool menu_bar_is_widget_type(struct PUBase* self_c) { 
    QMenuBar* qt_data = (QMenuBar*)self_c;
    auto ret_value = qt_data->isWidgetType();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void menu_bar_show(struct PUBase* self_c) { 
    WRMenuBar* qt_data = (WRMenuBar*)self_c;
    qt_data->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void menu_bar_resize(struct PUBase* self_c, int width, int height) { 
    WRMenuBar* qt_data = (WRMenuBar*)self_c;
    qt_data->resize(width, height);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void menu_bar_add_menu(struct PUBase* self_c, struct PUBase* menu) { 
    WRMenuBar* qt_data = (WRMenuBar*)self_c;
    qt_data->addMenu((QMenu*)menu);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_set_style(struct PUBase* self_c, const char* style) { 
    QApplication* qt_data = (QApplication*)self_c;
    qt_data->setStyle(QString::fromLatin1(style));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_exec(struct PUBase* self_c) { 
    QApplication* qt_data = (QApplication*)self_c;
    qt_data->exec();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

template<typename T, typename F, typename QT> T create_widget_func(F* funcs, void* priv_data) {
    PrivData* data = (PrivData*)priv_data;
    QT* qt_obj = nullptr;
    if (data) {
        qt_obj = new QT(data->parent);
    } else {
        qt_obj = new QT(nullptr);
    }
    T ctl;
    ctl.funcs = funcs;
    ctl.priv_data = (struct PUBase*)qt_obj;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

template<typename T, typename F, typename QT> T create_generic_func(F* funcs, void* priv_data) {
    QT* qt_obj = new QT();
    T ctl;
    ctl.funcs = funcs;
    ctl.priv_data = (struct PUBase*)qt_obj;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

template<typename QT> void destroy_generic(struct PUBase* qt_data) {
    QT* qt_obj = (QT*)qt_data;
    delete qt_obj;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUObject create_object(struct PUBase* priv_data) {
    return create_generic_func<struct PUObject, struct PUObjectFuncs, QObject>(&s_object_funcs, priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_object(struct PUBase* priv_data) {
    destroy_generic<QObject>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUWidget create_widget(struct PUBase* priv_data) {
    return create_widget_func<struct PUWidget, struct PUWidgetFuncs, WRWidget>(&s_widget_funcs, priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_widget(struct PUBase* priv_data) {
    destroy_generic<WRWidget>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUPushButton create_push_button(struct PUBase* priv_data) {
    return create_widget_func<struct PUPushButton, struct PUPushButtonFuncs, WRPushButton>(&s_push_button_funcs, priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_push_button(struct PUBase* priv_data) {
    destroy_generic<WRPushButton>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUPainter create_painter(struct PUBase* priv_data) {
    return create_generic_func<struct PUPainter, struct PUPainterFuncs, QPainter>(&s_painter_funcs, priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_painter(struct PUBase* priv_data) {
    destroy_generic<QPainter>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUListWidgetItem create_list_widget_item(struct PUBase* priv_data) {
    return create_generic_func<struct PUListWidgetItem, struct PUListWidgetItemFuncs, QListWidgetItem>(&s_list_widget_item_funcs, priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_list_widget_item(struct PUBase* priv_data) {
    destroy_generic<QListWidgetItem>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUListWidget create_list_widget(struct PUBase* priv_data) {
    return create_widget_func<struct PUListWidget, struct PUListWidgetFuncs, WRListWidget>(&s_list_widget_funcs, priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_list_widget(struct PUBase* priv_data) {
    destroy_generic<WRListWidget>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUSlider create_slider(struct PUBase* priv_data) {
    return create_widget_func<struct PUSlider, struct PUSliderFuncs, WRSlider>(&s_slider_funcs, priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_slider(struct PUBase* priv_data) {
    destroy_generic<WRSlider>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUMainWindow create_main_window(struct PUBase* priv_data) {
    return create_widget_func<struct PUMainWindow, struct PUMainWindowFuncs, WRMainWindow>(&s_main_window_funcs, priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_main_window(struct PUBase* priv_data) {
    destroy_generic<WRMainWindow>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUAction create_action(struct PUBase* priv_data) {
    return create_widget_func<struct PUAction, struct PUActionFuncs, WRAction>(&s_action_funcs, priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_action(struct PUBase* priv_data) {
    destroy_generic<WRAction>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUMenu create_menu(struct PUBase* priv_data) {
    return create_widget_func<struct PUMenu, struct PUMenuFuncs, WRMenu>(&s_menu_funcs, priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_menu(struct PUBase* priv_data) {
    destroy_generic<WRMenu>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUMenuBar create_menu_bar(struct PUBase* priv_data) {
    return create_widget_func<struct PUMenuBar, struct PUMenuBarFuncs, WRMenuBar>(&s_menu_bar_funcs, priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_menu_bar(struct PUBase* priv_data) {
    destroy_generic<WRMenuBar>(priv_data);
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUApplication create_application(struct PUBase* priv_data) {
    static int argc = 0;
    QApplication* qt_obj = new QApplication(argc, 0);
    struct PUApplication ctl;
    ctl.funcs = &s_application_funcs;
    ctl.priv_data = (struct PUBase*)qt_obj;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_application(struct PUBase* priv_data) {
    destroy_generic<QApplication>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_add_widget_item(struct PUBase* self_c, struct PUBase* item) {
    WRListWidget* qt_data = (WRListWidget*)self_c;
    qt_data->addItem((QListWidgetItem*)item);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void menu_add_action_text(struct PUBase* self_c, const char* text) {
    WRMenu* qt_data = (WRMenu*)self_c;
    qt_data->addAction(QString::fromLatin1(text));
}



///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PUObjectFuncs s_object_funcs = {
    destroy_object,
    object_is_widget_type,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PUWidgetFuncs s_widget_funcs = {
    destroy_widget,
    widget_is_widget_type,
    widget_show,
    widget_resize,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PUPushButtonFuncs s_push_button_funcs = {
    destroy_push_button,
    push_button_is_widget_type,
    push_button_show,
    push_button_resize,
    set_push_button_released_event,
    push_button_set_text,
    push_button_set_flat,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PUPainterFuncs s_painter_funcs = {
    destroy_painter,
    painter_draw_line,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PUListWidgetItemFuncs s_list_widget_item_funcs = {
    destroy_list_widget_item,
    list_widget_item_set_text,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PUListWidgetFuncs s_list_widget_funcs = {
    destroy_list_widget,
    list_widget_is_widget_type,
    list_widget_show,
    list_widget_resize,
    list_widget_add_item,
    list_widget_item,
    list_widget_add_widget_item,
    set_list_widget_current_row_changed_event,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PUSliderFuncs s_slider_funcs = {
    destroy_slider,
    slider_is_widget_type,
    slider_show,
    slider_resize,
    set_slider_value_changed_event,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PUMainWindowFuncs s_main_window_funcs = {
    destroy_main_window,
    main_window_is_widget_type,
    main_window_show,
    main_window_resize,
    main_window_is_animated,
    main_window_menu_bar,
    main_window_set_central_widget,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PUActionFuncs s_action_funcs = {
    destroy_action,
    action_is_widget_type,
    action_is_enabled,
    action_set_text,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PUMenuFuncs s_menu_funcs = {
    destroy_menu,
    menu_is_widget_type,
    menu_show,
    menu_resize,
    menu_add_action_text,
    menu_add_action,
    menu_set_title,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PUMenuBarFuncs s_menu_bar_funcs = {
    destroy_menu_bar,
    menu_bar_is_widget_type,
    menu_bar_show,
    menu_bar_resize,
    menu_bar_add_menu,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PUApplicationFuncs s_application_funcs = {
    destroy_application,
    application_set_style,
    application_exec,
};

static struct PU s_pu = {
    create_object,
    create_widget,
    create_push_button,
    create_painter,
    create_list_widget_item,
    create_list_widget,
    create_slider,
    create_main_window,
    create_action,
    create_menu,
    create_menu_bar,
    create_application,
    0,

};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


struct PU* PU_create_instance(void* user_data, QWidget* parent) {
    struct PU* instance = new PU;
    memcpy(instance, &s_pu, sizeof(PU));
    PrivData* priv_data = new PrivData;
    priv_data->parent = parent;
    priv_data->user_data = user_data;
    return instance;
}



///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern "C" struct PU* wrui_get() {
    return (PU*)&s_pu;
}
