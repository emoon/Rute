#pragma once

#include <QMainWindow>
#include <ToolWindowManager.h>

class TopWindow : public QMainWindow {
public:
    TopWindow(QWidget* widget) : TopWindow(widget)
    {
        m_window_manager = new ToolWindowManager(this);
    }
    virtual ~TopWindow()
    {
        delete m_window_manager;
    }

    ToolWindowManager* m_window_manager = nullptr;
};


