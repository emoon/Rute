
#include <QStyleFactory>
#include <DarkStyle.h>
#include <QFileDialog>
#include <QTextStream>
//#include <QSvgRenderer>

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUApplication create_application(struct RUBase* priv_data) {
    static int argc = 0;
    QApplication* qt_obj = new QApplication(argc, 0);

    create_enum_mappings();

    struct RUApplication ctl;
    ctl.funcs = &s_application_funcs;
    ctl.priv_data = (struct RUBase*)qt_obj;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_application(struct RUBase* priv_data) {
    destroy_generic<QApplication>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_add_widget_item(struct RUBase* self_c, struct RUBase* item) {
    WRListWidget* qt_data = (WRListWidget*)self_c;
    qt_data->addItem((QListWidgetItem*)item);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void menu_add_action_text(struct RUBase* self_c, const char* text) {
    WRMenu* qt_data = (WRMenu*)self_c;
    qt_data->addAction(QString::fromLatin1(text));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_set_accept_drops(struct RUBase* self_c, bool state) {
    WRListWidget* qt_data = (WRListWidget*)self_c;
	qt_data->viewport()->setAcceptDrops(state);
	qt_data->setDragDropMode(QAbstractItemView::InternalMove);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void painter_fill_rect_color(struct RUBase* self_c, struct RURect rect, struct RUColor color) {
    QPainter* qt_data = (QPainter*)self_c;
    qt_data->fillRect(QRect(rect.x, rect.y, rect.width, rect.height), QColor(color.r, color.g, color.b, color.a));
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_string_data(struct RUBase* self_c, const char* text) {
    QListWidgetItem* qt_data = (QListWidgetItem*)self_c;
    qt_data->setData(Qt::UserRole, QString::fromLatin1(text));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* list_widget_item_get_string_data(struct RUBase* self_c) {
    QListWidgetItem* qt_data = (QListWidgetItem*)self_c;
    auto ret_value = qt_data->data(Qt::UserRole).toString();
    QByteArray ba = ret_value.toUtf8();
    const char* c_str = ba.data();
    assert((ba.size() + 1) < sizeof(s_temp_string_buffer));
    memcpy(s_temp_string_buffer, c_str, ba.size() + 1);
    return s_temp_string_buffer;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_add_text_item(struct RUBase* self_c, const char* text) {
    WRListWidget* qt_data = (WRListWidget*)self_c;
    qt_data->addItem(QString::fromLatin1(text));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUArray application_get_files(struct RUBase* self_c) {
   QApplication* qt_data = (QApplication*)self_c;
   RUArray array = { 0 };
   (void)qt_data;
   QFileDialog dialog(QApplication::activeWindow());
   dialog.setFileMode(QFileDialog::ExistingFiles);
   dialog.show();

   if (!dialog.exec())
		return array;

   auto ret_value = dialog.selectedFiles();

   int count = ret_value.size();

   if (count > 0) {
       RUUrl* elements = new RUUrl[count];
       for (int i = 0; i < count; ++i) {
           elements[i].funcs = &s_url_funcs;
           QUrl* temp = new QUrl(QUrl::fromLocalFile(ret_value.at(i)));
           elements[i].priv_data = (struct RUBase*)temp;
      }

      array.elements = (void*)elements;
      array.count = int(count);
  }

   return array;
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/*
static void icon_add_file(struct RUBase* self_c, const char* filename) {
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
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void action_set_int_data(struct RUBase* self_c, int data) {
    QAction* qt_data = (QAction*)self_c;
    qt_data->setData(QVariant(data));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int action_get_int_data(struct RUBase* self_c) {
    QAction* qt_data = (QAction*)self_c;
    return qt_data->data().toInt();
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void action_set_shortcut(struct RUBase* self_c, RUKeys key) {
    QAction* qt_data = (QAction*)self_c;
    qt_data->setShortcut(s_keys_lookup[(int)key]);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void action_set_shortcut_mod(struct RUBase* self_c, RUKeys key, RUMetaKeys modifier) {
    QAction* qt_data = (QAction*)self_c;
    int tkey = s_keys_lookup[(int)key];
    int tmod = s_meta_keys_lookup[(int)modifier];

    qt_data->setShortcut(tkey + tmod);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void tool_window_manager_add_to_docking(struct RUBase* self_c, struct RUBase* widget) {
    WRToolWindowManager* qt_data = (WRToolWindowManager*)self_c;
    ToolWindowManager::AreaReferenceType type;

    static int hack = 0;

    if (hack == 0) {
        type = ToolWindowManager::EmptySpace;
        hack = 1;
    } else {
        type = ToolWindowManager::LastUsedArea;
    }

    qt_data->addToolWindow((QWidget*)widget, type);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void tool_window_manager_add_to_docking_floating(struct RUBase* self_c, struct RUBase* widget) {
    WRToolWindowManager* qt_data = (WRToolWindowManager*)self_c;
    auto type = ToolWindowManager::NewFloatingArea;
    qt_data->addToolWindow((QWidget*)widget, type);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int application_set_style_sheet(struct RUBase* self_c, const char* filename) {
    QApplication* qt_data = (QApplication*)self_c;
    QFile f(QString::fromUtf8(filename));

    if (!f.exists()) {
        printf("Unable to set stylesheet: %s, file not found\n", filename);
        return 0;
    } else {
        f.open(QFile::ReadOnly | QFile::Text);
        QTextStream ts(&f);
        qt_data->setStyleSheet(ts.readAll());
    }

    return 1;
}



