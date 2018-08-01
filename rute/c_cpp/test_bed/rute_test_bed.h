#pragma once

#include <QWidget>
#include <QListWidget>

typedef void (*DeleteCallback)(void* data);

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRWidget : public QWidget {
    Q_OBJECT
public:
    WRWidget(QWidget* widget) : QWidget(widget) { }
    virtual ~WRWidget() {
        if (m_delete_callback) {
            m_delete_callback(m_delete_callback_data);
        }
    }

    DeleteCallback m_delete_callback = nullptr;
    void* m_delete_callback_data = nullptr;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRListWidget : public QListWidget {
    Q_OBJECT
public:
    WRListWidget (QWidget* widget) : QListWidget(widget) { }
    virtual ~WRListWidget() {
        if (m_delete_callback) {
            m_delete_callback(m_delete_callback_data);
        }
    }

    DeleteCallback m_delete_callback = nullptr;
    void* m_delete_callback_data = nullptr;
};

