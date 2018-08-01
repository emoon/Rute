#pragma once

#include <QWidget>

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
    WRWidget(QWidget* widget) : QWidget(widget) { }
    virtual ~WRWidget() {}
};

