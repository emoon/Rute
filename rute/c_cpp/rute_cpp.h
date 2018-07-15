#include <QObject>
#include <QSlider>

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef void (*Signal_self_i32_void)(void* self_c, void* wrapped_func, int row);

class QSlotWrapperSignal_self_i32_void : public QObject {
    Q_OBJECT
public:
    QSlotWrapperSignal_self_i32_void(void* data, Signal_self_i32_void func, void* wrapped_func) {
        m_func = func;
        m_data = data;
        m_wrapped_func = wrapped_func;
    }

    Q_SLOT void method(int row) {
        m_func(m_data, m_wrapped_func, row);
    }
private:
    Signal_self_i32_void m_func;
    void* m_data;
    void* m_wrapped_func;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRWidget : public QWidget {
    Q_OBJECT
public:
    WRWidget(QWidget* widget) : QWidget(widget) {}
    virtual ~WRWidget() {
        if (m_delete_callback) {
           m_delete_callback(m_delete_callback_data);
        }
    }

    void (*m_delete_callback)(void* self_c) = nullptr;
    void* m_delete_callback_data = nullptr;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRSlider : public QSlider {
    Q_OBJECT
public:

    WRSlider(QWidget* widget) : QSlider(widget) {}
    virtual ~WRSlider() {
        if (m_delete_callback) {
           m_delete_callback((void*)this);
        }
    }

    void (*m_delete_callback)(void* self_c) = nullptr;
};

