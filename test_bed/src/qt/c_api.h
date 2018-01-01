
#pragma once

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

struct PURect;
struct PUWidget;
struct PUPushButton;
struct PUPainter;
struct PUListWidgetItem;
struct PUListWidget;
struct PUSlider;
struct PUMainWindow;
struct PUApplication;
struct PUPaintEvent;
struct PUPaintDevice;
struct PUWidgetType;

struct PURect {
    int x;
    int y;
    int width;
    int height;
};

struct PUWidget {
    void (*destroy)(void* self_c);
    void (*show)(void* self_c);
    void (*resize)(void* self_c, int width, int height);
    void (*set_paint_event_event)(void* object, void* user_data, void (*event)(void* self_c, struct PUPaintEvent* event));
    void* priv_data;
};

struct PUPushButton {
    void (*destroy)(void* self_c);
    void (*show)(void* self_c);
    void (*resize)(void* self_c, int width, int height);
    void (*set_released_event)(void* object, void* user_data, void (*event)(void* self_c));
    void (*set_text)(void* self_c, const char* text);
    void (*set_flat)(void* self_c, bool flat);
    void* priv_data;
};

struct PUPainter {
    void (*destroy)(void* self_c);
    void (*draw_line)(void* self_c, int x1, int y1, int x2, int y2);
    void* priv_data;
};

struct PUListWidgetItem {
    void (*destroy)(void* self_c);
    void (*set_text)(void* self_c, const char* text);
    void* priv_data;
};

struct PUListWidget {
    void (*destroy)(void* self_c);
    void (*show)(void* self_c);
    void (*resize)(void* self_c, int width, int height);
    void (*add_item)(void* self_c, const char* text);
    void (*add_widget_item)(void* self_c, struct PUListWidgetItem* item);
    void (*set_current_row_changed_event)(void* object, void* user_data, void (*event)(void* self_c, int row));
    void* priv_data;
};

struct PUSlider {
    void (*destroy)(void* self_c);
    void (*show)(void* self_c);
    void (*resize)(void* self_c, int width, int height);
    void (*set_value_changed_event)(void* object, void* user_data, void (*event)(void* self_c, int value));
    void* priv_data;
};

struct PUMainWindow {
    void (*destroy)(void* self_c);
    void (*show)(void* self_c);
    void (*resize)(void* self_c, int width, int height);
    bool (*is_animated)(void* self_c);
    void (*set_central_widget)(void* self_c, struct PUWidgetType* widget);
    void* priv_data;
};

struct PUApplication {
    void (*destroy)(void* self_c);
    void (*set_style)(void* self_c, const char* style);
    void (*exec)(void* self_c);
    void* priv_data;
};

struct PUPaintEvent {
    struct PURect (*rect)(void* self_c);
    void* priv_data;
};

typedef struct PU { 
    struct PUWidget* (*create_widget)(void* self);
    struct PUPushButton* (*create_push_button)(void* self);
    struct PUPainter* (*create_painter)(void* self);
    struct PUListWidgetItem* (*create_list_widget_item)(void* self);
    struct PUListWidget* (*create_list_widget)(void* self);
    struct PUSlider* (*create_slider)(void* self);
    struct PUMainWindow* (*create_main_window)(void* self);
    struct PUApplication* (*create_application)(void* self);
    void* priv_data;
} PU;

#ifdef __cplusplus
}
#endif
