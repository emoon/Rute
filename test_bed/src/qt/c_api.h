
#pragma once

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

struct PURect;
struct PUWidget;
struct PUWidgetFuncs;
struct PUPushButton;
struct PUPushButtonFuncs;
struct PUPainter;
struct PUPainterFuncs;
struct PUListWidgetItem;
struct PUListWidgetItemFuncs;
struct PUListWidget;
struct PUListWidgetFuncs;
struct PUSlider;
struct PUSliderFuncs;
struct PUMainWindow;
struct PUMainWindowFuncs;
struct PUApplication;
struct PUApplicationFuncs;
struct PUPaintDevice;
struct PUWidgetType;

struct PURect {
    int x;
    int y;
    int width;
    int height;
};

struct PUWidgetFuncs {
    void (*destroy)(void* self_c);
    void (*show)(void* self_c);
    void (*resize)(void* self_c, int width, int height);
};

struct PUWidget {
    struct PUWidgetFuncs* funcs;
    void* priv_data;
};

struct PUPushButtonFuncs {
    void (*destroy)(void* self_c);
    void (*show)(void* self_c);
    void (*resize)(void* self_c, int width, int height);
    void (*set_released_event)(void* object, void* user_data, void (*event)(void* self_c));
    void (*set_text)(void* self_c, const char* text);
    void (*set_flat)(void* self_c, bool flat);
};

struct PUPushButton {
    struct PUPushButtonFuncs* funcs;
    void* priv_data;
};

struct PUPainterFuncs {
    void (*destroy)(void* self_c);
    void (*draw_line)(void* self_c, int x1, int y1, int x2, int y2);
};

struct PUPainter {
    struct PUPainterFuncs* funcs;
    void* priv_data;
};

struct PUListWidgetItemFuncs {
    void (*destroy)(void* self_c);
    void (*set_text)(void* self_c, const char* text);
};

struct PUListWidgetItem {
    struct PUListWidgetItemFuncs* funcs;
    void* priv_data;
};

struct PUListWidgetFuncs {
    void (*destroy)(void* self_c);
    void (*show)(void* self_c);
    void (*resize)(void* self_c, int width, int height);
    void (*add_item)(void* self_c, const char* text);
    struct PUListWidgetItem (*item)(void* self_c, int index);
    void (*add_widget_item)(void* self_c, struct PUListWidgetItem* item);
    void (*set_current_row_changed_event)(void* object, void* user_data, void (*event)(void* self_c, int row));
};

struct PUListWidget {
    struct PUListWidgetFuncs* funcs;
    void* priv_data;
};

struct PUSliderFuncs {
    void (*destroy)(void* self_c);
    void (*show)(void* self_c);
    void (*resize)(void* self_c, int width, int height);
    void (*set_value_changed_event)(void* object, void* user_data, void (*event)(void* self_c, int value));
};

struct PUSlider {
    struct PUSliderFuncs* funcs;
    void* priv_data;
};

struct PUMainWindowFuncs {
    void (*destroy)(void* self_c);
    void (*show)(void* self_c);
    void (*resize)(void* self_c, int width, int height);
    bool (*is_animated)(void* self_c);
    void (*set_central_widget)(void* self_c, struct PUWidgetType* widget);
};

struct PUMainWindow {
    struct PUMainWindowFuncs* funcs;
    void* priv_data;
};

struct PUApplicationFuncs {
    void (*destroy)(void* self_c);
    void (*set_style)(void* self_c, const char* style);
    void (*exec)(void* self_c);
};

struct PUApplication {
    struct PUApplicationFuncs* funcs;
    void* priv_data;
};

typedef struct PU { 
    struct PUWidget (*create_widget)(void* self);
    struct PUPushButton (*create_push_button)(void* self);
    struct PUPainter (*create_painter)(void* self);
    struct PUListWidgetItem (*create_list_widget_item)(void* self);
    struct PUListWidget (*create_list_widget)(void* self);
    struct PUSlider (*create_slider)(void* self);
    struct PUMainWindow (*create_main_window)(void* self);
    struct PUApplication (*create_application)(void* self);
    void* priv_data;
} PU;

#ifdef __cplusplus
}
#endif
