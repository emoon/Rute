#pragma once

#include <QWidget>
#include <QListWidget>
#include <QListWidgetItem>
#include <QIcon>

typedef void (*DeleteCallback)(void* data);

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRWidget : public QWidget {
    Q_OBJECT
public:
    WRWidget(QWidget* widget) : QWidget(widget) { }
    virtual ~WRWidget() {
        if (m_delete_callback) {
            m_delete_callback(m_private_data);
        }
    }

    DeleteCallback m_delete_callback = nullptr;
    void* m_private_data = nullptr;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRListWidget : public QListWidget {
    Q_OBJECT
public:
    WRListWidget (QWidget* widget) : QListWidget(widget) { }
    virtual ~WRListWidget() {
        if (m_delete_callback) {
            m_delete_callback(m_private_data);
        }
    }

    DeleteCallback m_delete_callback = nullptr;
    void* m_private_data = nullptr;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRListWidgetItem : public QListWidgetItem {
public:
    WRListWidgetItem() : QListWidgetItem() { }
    virtual ~WRListWidgetItem() {
        if (m_delete_callback) {
            m_delete_callback(m_private_data);
        }
    }

    DeleteCallback m_delete_callback = nullptr;
    void* m_private_data = nullptr;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRIcon : public QIcon {
public:
    WRIcon() : QIcon() { }
    WRIcon(const QIcon& icon) : QIcon(icon) { }
    virtual ~WRIcon() {
        if (m_delete_callback) {
            m_delete_callback(m_private_data);
        }
    }

    DeleteCallback m_delete_callback = nullptr;
    void* m_private_data = nullptr;
};


