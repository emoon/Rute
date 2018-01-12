
#include <QStyleFactory>

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUApplication create_application(struct PUBase* priv_data) {
    static int argc = 0;
    QApplication* qt_obj = new QApplication(argc, 0);

	/*
    QCoreApplication::setOrganizationName(QStringLiteral("TBL"));
    QCoreApplication::setOrganizationDomain(QStringLiteral("tbl.org"));

    qt_obj->setStyle(QStyleFactory::create(QStringLiteral("Fusion")));

    QPalette darkPalette;
    darkPalette.setColor(QPalette::Window, QColor(53,53,53));
    darkPalette.setColor(QPalette::WindowText, QColor(170,170,170));
    darkPalette.setColor(QPalette::Text, QColor(170,170,170));
    darkPalette.setColor(QPalette::Base, QColor(25,25,25));
    darkPalette.setColor(QPalette::AlternateBase, QColor(53,53,53));
    darkPalette.setColor(QPalette::ToolTipBase, Qt::white);
    darkPalette.setColor(QPalette::ToolTipText, Qt::white);
    darkPalette.setColor(QPalette::Button, QColor(53,53,53));
    darkPalette.setColor(QPalette::ButtonText, Qt::white);
    darkPalette.setColor(QPalette::BrightText, Qt::red);
    darkPalette.setColor(QPalette::Link, QColor(42, 130, 218));

    darkPalette.setColor(QPalette::Highlight, QColor(50, 60, 70));
    darkPalette.setColor(QPalette::HighlightedText, Qt::black);

    qt_obj->setPalette(darkPalette);
    */

    struct PUApplication ctl;
    ctl.funcs = &s_application_funcs;
    ctl.priv_data = (struct PUBase*)qt_obj;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_application(struct PUBase* priv_data) {
    destroy_generic<QApplication>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_add_widget_item(struct PUBase* self_c, struct PUBase* item) {
    WRListWidget* qt_data = (WRListWidget*)self_c;
    qt_data->addItem((QListWidgetItem*)item);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void menu_add_action_text(struct PUBase* self_c, const char* text) {
    WRMenu* qt_data = (WRMenu*)self_c;
    qt_data->addAction(QString::fromLatin1(text));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_set_accept_drops(struct PUBase* self_c, bool state) {
    WRListWidget* qt_data = (WRListWidget*)self_c;
	qt_data->viewport()->setAcceptDrops(state);
	qt_data->setDragDropMode(QAbstractItemView::InternalMove);
}




