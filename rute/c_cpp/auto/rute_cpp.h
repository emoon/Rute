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
extern struct RUStaticFuncsFuncs s_static_funcs_funcs;

typedef void (*Signal_self_Action_void)(void* self_c, struct RUAction action);

class QSlotWrapperSignal_self_Action_void : public QObject {
    Q_OBJECT
public:
    QSlotWrapperSignal_self_Action_void(void* data, Signal_self_Action_void func) {
        m_func = func;
        m_data = data;
    }

Q_SLOT void method(QAction* action) 
        auto temp_arg_1 = RUAction { &s_action_funcs, (struct RUBase*)action };
        m_func(m_data, (struct RUBase*)&temp_arg_1);
    }
private:
    Signal_self_Action_void m_func;
    void* m_data;
};

typedef void (*Signal_self_ListWidgetItem_void)(void* self_c, struct RUListWidgetItem item);

class QSlotWrapperSignal_self_ListWidgetItem_void : public QObject {
    Q_OBJECT
public:
    QSlotWrapperSignal_self_ListWidgetItem_void(void* data, Signal_self_ListWidgetItem_void func) {
        m_func = func;
        m_data = data;
    }

Q_SLOT void method(QListWidgetItem* item) 
        auto temp_arg_1 = RUListWidgetItem { &s_list_widget_item_funcs, (struct RUBase*)item };
        m_func(m_data, (struct RUBase*)&temp_arg_1);
    }
private:
    Signal_self_ListWidgetItem_void m_func;
    void* m_data;
};

typedef void (*Signal_self_int_void)(void* self_c, int row);

class QSlotWrapperSignal_self_int_void : public QObject {
    Q_OBJECT
public:
    QSlotWrapperSignal_self_int_void(void* data, Signal_self_int_void func) {
        m_func = func;
        m_data = data;
    }

Q_SLOT void method(int row) 
        m_func(m_data, row);
    }
private:
    Signal_self_int_void m_func;
    void* m_data;
};

typedef void (*Signal_self_void)(void* self_c);

class QSlotWrapperSignal_self_void : public QObject {
    Q_OBJECT
public:
    QSlotWrapperSignal_self_void(void* data, Signal_self_void func) {
        m_func = func;
        m_data = data;
    }

Q_SLOT void method() 
        m_func(m_data);
    }
private:
    Signal_self_void m_func;
    void* m_data;
};

