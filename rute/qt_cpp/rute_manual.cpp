#include "rute_manual.h"
#include <QApplication>

extern RUApplicationFuncs s_application_funcs;
extern void create_enum_mappings();

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUApplication create_application(struct RUBase* priv_data) {
    static int argc = 0;
    QApplication* qt_obj = new QApplication(argc, 0);

    (void)priv_data;

    create_enum_mappings();

    RUApplication ctl;
    ctl.application_funcs = &s_application_funcs;
    ctl.priv_data = (struct RUBase*)qt_obj;

    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void destroy_application(struct RUBase* priv_data) {
    QApplication* t = (QApplication*)priv_data;

    delete t;
}

