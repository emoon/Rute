#include "c_api.h"
#include "qt_api_gen.h"
#include <assert.h>
#include <QWidget>
#include <QPushButton>
#include <QPainter>
#include <QListWidgetItem>
#include <QListWidget>
#include <QSlider>
#include <QMainWindow>
#include <QApplication>

struct PrivData {
    QWidget* parent;
    void* user_data;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern struct PUWidgetFuncs s_widget_funcs;
extern struct PUPushButtonFuncs s_push_button_funcs;
extern struct PUPainterFuncs s_painter_funcs;
extern struct PUListWidgetItemFuncs s_list_widget_item_funcs;
extern struct PUListWidgetFuncs s_list_widget_funcs;
extern struct PUSliderFuncs s_slider_funcs;
extern struct PUMainWindowFuncs s_main_window_funcs;
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

static void widget_show(void* self_c) { 
    WRWidget* qt_data = (WRWidget*)self_c;
    qt_data->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void widget_resize(void* self_c, int width, int height) { 
    WRWidget* qt_data = (WRWidget*)self_c;
    qt_data->resize(width, height);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void push_button_show(void* self_c) { 
    WRPushButton* qt_data = (WRPushButton*)self_c;
    qt_data->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void push_button_resize(void* self_c, int width, int height) { 
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

static void push_button_set_text(void* self_c, const char* text) { 
    WRPushButton* qt_data = (WRPushButton*)self_c;
    qt_data->setText(QString::fromLatin1(text));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void push_button_set_flat(void* self_c, bool flat) { 
    WRPushButton* qt_data = (WRPushButton*)self_c;
    qt_data->setFlat(flat);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void painter_draw_line(void* self_c, int x1, int y1, int x2, int y2) { 
    QPainter* qt_data = (QPainter*)self_c;
    qt_data->drawLine(x1, y1, x2, y2);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_text(void* self_c, const char* text) { 
    QListWidgetItem* qt_data = (QListWidgetItem*)self_c;
    qt_data->setText(QString::fromLatin1(text));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_show(void* self_c) { 
    WRListWidget* qt_data = (WRListWidget*)self_c;
    qt_data->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_resize(void* self_c, int width, int height) { 
    WRListWidget* qt_data = (WRListWidget*)self_c;
    qt_data->resize(width, height);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_add_item(void* self_c, const char* text) { 
    WRListWidget* qt_data = (WRListWidget*)self_c;
    qt_data->addItem(QString::fromLatin1(text));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUListWidgetItem list_widget_item(void* self_c, int index) { 
    WRListWidget* qt_data = (WRListWidget*)self_c;
    auto ret_value = qt_data->item(index);
    PUListWidgetItem ctl;
    ctl.funcs = &s_list_widget_item_funcs;
    ctl.priv_data = ret_value;
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

static void slider_show(void* self_c) { 
    WRSlider* qt_data = (WRSlider*)self_c;
    qt_data->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void slider_resize(void* self_c, int width, int height) { 
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

static void main_window_show(void* self_c) { 
    WRMainWindow* qt_data = (WRMainWindow*)self_c;
    qt_data->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void main_window_resize(void* self_c, int width, int height) { 
    WRMainWindow* qt_data = (WRMainWindow*)self_c;
    qt_data->resize(width, height);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool main_window_is_animated(void* self_c) { 
    WRMainWindow* qt_data = (WRMainWindow*)self_c;
    auto ret_value = qt_data->isAnimated();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void main_window_set_central_widget(void* self_c, struct PUWidgetType* widget) { 
    WRMainWindow* qt_data = (WRMainWindow*)self_c;
    qt_data->setCentralWidget((QWidget*)widget);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_set_style(void* self_c, const char* style) { 
    QApplication* qt_data = (QApplication*)self_c;
    qt_data->setStyle(QString::fromLatin1(style));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_exec(void* self_c) { 
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
    ctl.priv_data = qt_obj;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

template<typename T, typename F, typename QT> T create_generic_func(F* funcs, void* priv_data) {
    QT* qt_obj = new QT();
    T ctl;
    ctl.funcs = funcs;
    ctl.priv_data = qt_obj;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

template<typename QT> void destroy_generic(void* qt_data) {
    QT* qt_obj = (QT*)qt_data;
    delete qt_obj;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUWidget create_widget(void* priv_data) {
    return create_widget_func<struct PUWidget, struct PUWidgetFuncs, WRWidget>(&s_widget_funcs, priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_widget(void* priv_data) {
    destroy_generic<WRWidget>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUPushButton create_push_button(void* priv_data) {
    return create_widget_func<struct PUPushButton, struct PUPushButtonFuncs, WRPushButton>(&s_push_button_funcs, priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_push_button(void* priv_data) {
    destroy_generic<WRPushButton>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUPainter create_painter(void* priv_data) {
    return create_generic_func<struct PUPainter, struct PUPainterFuncs, QPainter>(&s_painter_funcs, priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_painter(void* priv_data) {
    destroy_generic<QPainter>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUListWidgetItem create_list_widget_item(void* priv_data) {
    return create_generic_func<struct PUListWidgetItem, struct PUListWidgetItemFuncs, QListWidgetItem>(&s_list_widget_item_funcs, priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_list_widget_item(void* priv_data) {
    destroy_generic<QListWidgetItem>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUListWidget create_list_widget(void* priv_data) {
    return create_widget_func<struct PUListWidget, struct PUListWidgetFuncs, WRListWidget>(&s_list_widget_funcs, priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_list_widget(void* priv_data) {
    destroy_generic<WRListWidget>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUSlider create_slider(void* priv_data) {
    return create_widget_func<struct PUSlider, struct PUSliderFuncs, WRSlider>(&s_slider_funcs, priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_slider(void* priv_data) {
    destroy_generic<WRSlider>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUMainWindow create_main_window(void* priv_data) {
    return create_widget_func<struct PUMainWindow, struct PUMainWindowFuncs, WRMainWindow>(&s_main_window_funcs, priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_main_window(void* priv_data) {
    destroy_generic<WRMainWindow>(priv_data);
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUApplication create_application(void* priv_data) {
    static int argc = 0;
    QApplication* qt_obj = new QApplication(argc, 0);
    struct PUApplication ctl;
    ctl.funcs = &s_application_funcs;
    ctl.priv_data = qt_obj;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_application(void* priv_data) {
    destroy_generic<QApplication>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_add_widget_item(void* self_c, struct PUListWidgetItem* item) {
    WRListWidget* qt_data = (WRListWidget*)self_c;
    qt_data->addItem((QListWidgetItem*)item);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PUWidgetFuncs s_widget_funcs = {
    destroy_widget,
    widget_show,
    widget_resize,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PUPushButtonFuncs s_push_button_funcs = {
    destroy_push_button,
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
    slider_show,
    slider_resize,
    set_slider_value_changed_event,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PUMainWindowFuncs s_main_window_funcs = {
    destroy_main_window,
    main_window_show,
    main_window_resize,
    main_window_is_animated,
    main_window_set_central_widget,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PUApplicationFuncs s_application_funcs = {
    destroy_application,
    application_set_style,
    application_exec,
};

static struct PU s_pu = {
    create_widget,
    create_push_button,
    create_painter,
    create_list_widget_item,
    create_list_widget,
    create_slider,
    create_main_window,
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
