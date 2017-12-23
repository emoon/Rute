#include "c_api.h"
#include "qt_api_gen.h"
#include <QWidget>
#include <QPushButton>
#include <QSlider>
#include <QApplication>
#include <QPaintEvent>
#include <QPainter>

struct PrivData {
    QWidget* parent;
    void* user_data;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRWidget : public QWidget {
public:
    virtual ~WRWidget() {}

    virtual void paintEvent(QPaintEvent* event) {
        if (m_paint_event) {
            PUPaintEvent e;
            memcpy(&e, s_paint_event_funcs, sizeof(e));
            e.priv_data = event;
            m_paint_event((PUPaintEvent*)&e, m_paint_event_user_data);
        } else {
            QWidget::paintEvent(event);
        }
    }

    PUPaintEventFunc m_paint_event = nullptr;
    void* m_paint_event_user_data= nullptr;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRPushButton : public QPushButton {
public:
    virtual ~WRPushButton() {}

    virtual void paintEvent(QPaintEvent* event) {
        if (m_paint_event) {
            PUPaintEvent e;
            memcpy(&e, s_paint_event_funcs, sizeof(e));
            e.priv_data = event;
            m_paint_event((PUPaintEvent*)&e, m_paint_event_user_data);
        } else {
            QPushButton::paintEvent(event);
        }
    }

    PUPaintEventFunc m_paint_event = nullptr;
    void* m_paint_event_user_data= nullptr;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRSlider : public QSlider {
public:
    virtual ~WRSlider() {}

    virtual void paintEvent(QPaintEvent* event) {
        if (m_paint_event) {
            PUPaintEvent e;
            memcpy(&e, s_paint_event_funcs, sizeof(e));
            e.priv_data = event;
            m_paint_event((PUPaintEvent*)&e, m_paint_event_user_data);
        } else {
            QSlider::paintEvent(event);
        }
    }

    PUPaintEventFunc m_paint_event = nullptr;
    void* m_paint_event_user_data= nullptr;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRApplication : public QApplication {
public:
    virtual ~WRApplication() {}

};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRPaintEvent : public QPaintEvent {
public:
    virtual ~WRPaintEvent() {}

};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRPainter : public QPainter {
public:
    virtual ~WRPainter() {}

};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void widget_show(void* self_c) { 
    QWidget* qt_data = (QWidget*)self_c;
    qt_data->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void widget_resize(void* self_c, int width, int height) { 
    QWidget* qt_data = (QWidget*)self_c;
    qt_data->resize(width, height);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void push_button_show(void* self_c) { 
    QPushButton* qt_data = (QPushButton*)self_c;
    qt_data->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void push_button_resize(void* self_c, int width, int height) { 
    QPushButton* qt_data = (QPushButton*)self_c;
    qt_data->resize(width, height);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void connect_push_button_released(void* object, void* user_data, void (*callback)(void* self_c)) {
    QSlotWrapperSignal_self_void* wrap = new QSlotWrapperSignal_self_void(user_data, (Signal_self_void)callback);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(released()), wrap, SLOT(method()));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void push_button_set_text(void* self_c, const char* text) { 
    QPushButton* qt_data = (QPushButton*)self_c;
    qt_data->setText(QString::fromLatin1(text));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void push_button_set_flat(void* self_c, bool flat) { 
    QPushButton* qt_data = (QPushButton*)self_c;
    qt_data->setFlat(flat);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void slider_show(void* self_c) { 
    QSlider* qt_data = (QSlider*)self_c;
    qt_data->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void slider_resize(void* self_c, int width, int height) { 
    QSlider* qt_data = (QSlider*)self_c;
    qt_data->resize(width, height);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void connect_slider_value_changed(void* object, void* user_data, void (*callback)(void* self_c, int value)) {
    QSlotWrapperSignal_self_i32_void* wrap = new QSlotWrapperSignal_self_i32_void(user_data, (Signal_self_i32_void)callback);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(valueChanged(int)), wrap, SLOT(method(int)));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_set_style(void* self_c, const char* style) { 
    QApplication* qt_data = (QApplication*)self_c;
    qt_data->setStyle(QString::fromLatin1(style));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void application_run(void* self_c) { 
    QApplication* qt_data = (QApplication*)self_c;
    qt_data->run();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PURect paint_event_rect(void* self_c) { 
    QPaintEvent* qt_data = (QPaintEvent*)self_c;
    qt_data->rect();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void painter_draw_line(void* self_c, int x1, int y1, int x2, int y2) { 
    QPainter* qt_data = (QPainter*)self_c;
    qt_data->drawLine(x1, y1, x2, y2);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUWidget s_widget = {
    widget_show,
    widget_resize,
    0,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUPushButton s_push_button = {
    push_button_show,
    push_button_resize,
    connect_push_button_released,
    push_button_set_text,
    push_button_set_flat,
    0,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUSlider s_slider = {
    slider_show,
    slider_resize,
    connect_slider_value_changed,
    0,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUApplication s_application = {
    application_set_style,
    application_run,
    0,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUPaintEvent s_paint_event = {
    paint_event_rect,
    0,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUPainter s_painter = {
    painter_draw_line,
    0,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


template<typename T, typename QT> T* create_func(T* struct_data, void* priv_data) {
    PrivData* data = (PrivData*)priv_data;
    QT* qt_obj = new QT(data->parent);
    T* ctl = new T;
    memcpy(ctl, struct_data, sizeof(T));
    ctl->priv_data = qt_obj;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUWidget* create_widget(void* priv_data) {
    return create_func<struct PUWidget, WRWidget>(&s_widget, priv_data);
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUPushButton* create_push_button(void* priv_data) {
    return create_func<struct PUPushButton, WRPushButton>(&s_push_button, priv_data);
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUSlider* create_slider(void* priv_data) {
    return create_func<struct PUSlider, WRSlider>(&s_slider, priv_data);
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUApplication* create_application(void* priv_data) {
    return create_func<struct PUApplication, WRApplication>(&s_application, priv_data);
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUPaintEvent* create_paint_event(void* priv_data) {
    return create_func<struct PUPaintEvent, WRPaintEvent>(&s_paint_event, priv_data);
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUPainter* create_painter(void* priv_data) {
    return create_func<struct PUPainter, WRPainter>(&s_painter, priv_data);
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PU s_pu = {
    create_widget,
    create_push_button,
    create_slider,
    create_application,
    create_paint_event,
    create_painter,
};


struct PU* PU_create_instance(void* user_data, QWidget* parent) {
    struct PU* instance = new PU;
    memcpy(instance, &s_pu, sizeof(PU));
    PrivData* priv_data = new PrivData;
    priv_data->parent = parent;
    priv_data->user_data = user_data;
    return instance;
}

