
#pragma once

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

struct PURect;
struct PUWidget;
struct PUPushButton;
struct PUSlider;
struct PUApplication;
struct PUPaintEvent;
struct PUPainter;

struct PURect {
    float x;
    float y;
    float width;
    float height;
};

struct PUWidget {
    void (*show)(void* self_c);
    void (*resize)(void* self_c, int width, int height);
    void (*set_paint_event_event)(void* object, void* user_data, void (*event)(void* self_c, struct PUPaintEvent* event));
    void* priv_data;
};

struct PUPushButton {
    void (*show)(void* self_c);
    void (*resize)(void* self_c, int width, int height);
    void (*set_released_event)(void* object, void* user_data, void (*event)(void* self_c));
    void (*set_text)(void* self_c, const char* text);
    void (*set_flat)(void* self_c, bool flat);
    void* priv_data;
};

struct PUSlider {
    void (*show)(void* self_c);
    void (*resize)(void* self_c, int width, int height);
    void (*set_value_changed_event)(void* object, void* user_data, void (*event)(void* self_c, int value));
    void* priv_data;
};

struct PUApplication {
    void (*set_style)(void* self_c, const char* style);
    void (*exec)(void* self_c);
    void* priv_data;
};

struct PUPaintEvent {
    struct PURect (*rect)(void* self_c);
    void* priv_data;
};

struct PUPainter {
    void (*draw_line)(void* self_c, int x1, int y1, int x2, int y2);
    void* priv_data;
};

typedef struct PU { 
    struct PUWidget* (*create_widget)(void* self);
    struct PUPushButton* (*create_push_button)(void* self);
    struct PUSlider* (*create_slider)(void* self);
    struct PUApplication* (*create_application)(void* self);
    struct PUPainter* (*create_painter)(void* self);
    void* priv_data;
} PU;

#ifdef __cplusplus
}
#endif
