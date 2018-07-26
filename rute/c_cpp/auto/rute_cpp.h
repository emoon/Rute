#include <QApplication>
#include <QAction>
#include <QWidget>
#include <QListWidgetItem>
#include <QListWidget>
#include <QUrl>
#include <QMimeData>
#include <QTimer>
#include <QIcon>
#include <QMenu>
#include <QMenuBar>
#include <QLabel>
#include <QLineEdit>
#include <QPlainTextEdit>
#include <QSlider>
#include <QMainWindow>
#include <QLayout>
#include <QVBoxLayout>
#include <QHBoxLayout>
extern struct RUApplicationFuncs s_application_funcs;
extern struct RUActionFuncs s_action_funcs;
extern struct RUWidgetFuncs s_widget_funcs;
extern struct RUListWidgetItemFuncs s_list_widget_item_funcs;
extern struct RUListWidgetFuncs s_list_widget_funcs;
extern struct RUUrlFuncs s_url_funcs;
extern struct RUMimeDataFuncs s_mime_data_funcs;
extern struct RUTimerFuncs s_timer_funcs;
extern struct RUIconFuncs s_icon_funcs;
extern struct RUMenuFuncs s_menu_funcs;
extern struct RUMenuBarFuncs s_menu_bar_funcs;
extern struct RULabelFuncs s_label_funcs;
extern struct RULineEditFuncs s_line_edit_funcs;
extern struct RUPlainTextEditFuncs s_plain_text_edit_funcs;
extern struct RUSliderFuncs s_slider_funcs;
extern struct RUMainWindowFuncs s_main_window_funcs;
extern struct RULayoutFuncs s_layout_funcs;
extern struct RUVBoxLayoutFuncs s_v_box_layout_funcs;
extern struct RUHBoxLayoutFuncs s_h_box_layout_funcs;

typedef void (*Signal_self_Action_void)(void* self_c, void* wrapped_func, struct RUAction action);

class QSlotWrapperSignal_self_Action_void : public QObject {
    Q_OBJECT
public:
    QSlotWrapperSignal_self_Action_void(void* data, Signal_self_Action_void func, void* wrapped_func) {
        m_func = func;
        m_data = data;
        m_wrapped_func = wrapped_func;
    }

    Q_SLOT void method(QAction* action) {
        auto temp_arg_1 = RUAction { &s_action_funcs, (struct RUBase*)action };
        m_func(m_data, m_wrapped_func, (struct RUBase*)&temp_arg_1);
    }
private:
    Signal_self_Action_void m_func;
    void* m_data;
    void* m_wrapped_func;
};

typedef void (*Signal_self_ListWidgetItem_void)(void* self_c, void* wrapped_func, struct RUListWidgetItem item);

class QSlotWrapperSignal_self_ListWidgetItem_void : public QObject {
    Q_OBJECT
public:
    QSlotWrapperSignal_self_ListWidgetItem_void(void* data, Signal_self_ListWidgetItem_void func, void* wrapped_func) {
        m_func = func;
        m_data = data;
        m_wrapped_func = wrapped_func;
    }

    Q_SLOT void method(QListWidgetItem* item) {
        auto temp_arg_1 = RUListWidgetItem { &s_list_widget_item_funcs, (struct RUBase*)item };
        m_func(m_data, m_wrapped_func, (struct RUBase*)&temp_arg_1);
    }
private:
    Signal_self_ListWidgetItem_void m_func;
    void* m_data;
    void* m_wrapped_func;
};

typedef void (*Signal_self_int_void)(void* self_c, void* wrapped_func, int row);

class QSlotWrapperSignal_self_int_void : public QObject {
    Q_OBJECT
public:
    QSlotWrapperSignal_self_int_void(void* data, Signal_self_int_void func, void* wrapped_func) {
        m_func = func;
        m_data = data;
        m_wrapped_func = wrapped_func;
    }

    Q_SLOT void method(int row) {
        m_func(m_data, m_wrapped_func, row);
    }
private:
    Signal_self_int_void m_func;
    void* m_data;
    void* m_wrapped_func;
};

typedef void (*Signal_self_void)(void* self_c, void* wrapped_func);

class QSlotWrapperSignal_self_void : public QObject {
    Q_OBJECT
public:
    QSlotWrapperSignal_self_void(void* data, Signal_self_void func, void* wrapped_func) {
        m_func = func;
        m_data = data;
        m_wrapped_func = wrapped_func;
    }

    Q_SLOT void method() {
        m_func(m_data, m_wrapped_func);
    }
private:
    Signal_self_void m_func;
    void* m_data;
    void* m_wrapped_func;
};

