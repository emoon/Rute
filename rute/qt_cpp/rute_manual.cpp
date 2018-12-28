#include "rute_manual.h"
#include "auto/application.h"
#include "auto/application_ffi.h"
#include <QApplication>

extern RUApplicationAllFuncs s_application_all_funcs;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void free_primitive_alloc(void* data) {
    free(data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

RUArray alloc_rc_array(int count) {
    RUArray array = { };
    // alloc size of pointer + 1 extra byte for info bit
    uint8_t* mem = (uint8_t*)malloc((sizeof(void*) + 1) * count);
    array.priv_data = nullptr;
    array.owners = nullptr;
    array.delete_callback = free_primitive_alloc;
    array.elements = mem;
    array.owners = mem + count;
    return array;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

RUArray alloc_primitive_array(int size) {
    RUArray array = { };
    array.priv_data = nullptr;
    array.owners = nullptr;
    array.delete_callback = free_primitive_alloc;
    array.elements = malloc(size);
    return array;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUApplication create_application(struct RUBase* priv_data) {
    static int argc = 0;
    QApplication* qt_obj = new QApplication(argc, 0);

    (void)priv_data;

    RUApplication ctl;
    ctl.qt_data = (struct RUBase*)qt_obj;
    ctl.host_data = nullptr;
    ctl.all_funcs = &s_application_all_funcs;

    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void destroy_application(struct RUBase* priv_data) {
    QApplication* t = (QApplication*)priv_data;

    delete t;
}

