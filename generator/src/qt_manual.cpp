
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

static void menu_add_action_text(void* self_c, const char* text) {
    WRMenu* qt_data = (WRMenu*)self_c;
    qt_data->addAction(QString::fromLatin1(text));
}



