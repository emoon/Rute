
#include <QStyleFactory>
#include <DarkStyle.h>
#include <QFileDialog>
#include <QSvgRenderer>

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUApplication create_application(struct PUBase* priv_data) {
    static int argc = 0;
    QApplication* qt_obj = new QApplication(argc, 0);

    //QGuiApplication::setOrganizationName(QStringLiteral("TBL"));
    //QCoreApplication::setOrganizationDomain(QStringLiteral("tbl.org"));

    qt_obj->setStyle(new DarkStyle);

    /*
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

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void painter_fill_rect_color(struct PUBase* self_c, struct PURect rect, struct PUColor color) {
    QPainter* qt_data = (QPainter*)self_c;
    qt_data->fillRect(QRect(rect.x, rect.y, rect.width, rect.height), QColor(color.r, color.g, color.b, color.a));
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_string_data(struct PUBase* self_c, const char* text) {
    QListWidgetItem* qt_data = (QListWidgetItem*)self_c;
    qt_data->setData(Qt::UserRole, QString::fromLatin1(text));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* list_widget_item_get_string_data(struct PUBase* self_c) {
    QListWidgetItem* qt_data = (QListWidgetItem*)self_c;
    auto ret_value = qt_data->data(Qt::UserRole).toString();
    QByteArray ba = ret_value.toUtf8();
    const char* c_str = ba.data();
    assert((ba.size() + 1) < sizeof(s_temp_string_buffer));
    memcpy(s_temp_string_buffer, c_str, ba.size() + 1);
    return s_temp_string_buffer;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_add_text_item(struct PUBase* self_c, const char* text) {
    WRListWidget* qt_data = (WRListWidget*)self_c;
    qt_data->addItem(QString::fromLatin1(text));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct PUArray application_get_files(struct PUBase* self_c) {
    QApplication* qt_data = (QApplication*)self_c;
    (void)qt_data;
    auto ret_value = QFileDialog::getOpenFileUrls();
    int count = ret_value.size();
    PUArray array = { 0 };
    if (count > 0) {
        PUUrl* elements = new PUUrl[count];
        for (int i = 0; i < count; ++i) {
            elements[i].funcs = &s_url_funcs;
            QUrl* temp = new QUrl(ret_value.at(i));
            elements[i].priv_data = (struct PUBase*)temp;
       }
       array.elements = (void*)elements;
       array.count = int(count);
   }
   return array;
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/*
static void icon_add_file(struct PUBase* self_c, const char* filename) { 
    QIcon* qt_data = (QIcon*)self_c;
    QSvgRenderer svgRenderer(QString::fromLatin1(filename));
    printf("filling with svg renderer\n");
    // create pixmap target (could be a QImage)
    QPixmap pix(svgRenderer.defaultSize());
    pix.fill(Qt::transparent);
    // create painter to act over pixmap
    QPainter pixPainter(&pix);
    // use renderer to render over painter which paints on pixmap
    svgRenderer.render(&pixPainter);
    //QIcon myicon(pix);
    qt_data->addPixmap(pix);
}
*/


