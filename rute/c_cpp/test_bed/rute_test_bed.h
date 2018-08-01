#pragma once

#include <QWidget>
#include <QListWidget>

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRWidget : public QWidget {
    Q_OBJECT
public:
    WRWidget(QWidget* widget) : QWidget(widget) { }
    virtual ~WRWidget() {}
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRListWidget : public QListWidget {
    Q_OBJECT
public:
    WRListWidget (QWidget* widget) : QListWidget(widget) { }
    virtual ~WRListWidget() {}
};

