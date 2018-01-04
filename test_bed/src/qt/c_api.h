
#pragma once

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif


struct PUBase;
struct PURect;
struct PUObject;
struct PUObjectFuncs;
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
struct PUAction;
struct PUActionFuncs;
struct PUMenu;
struct PUMenuFuncs;
struct PUMenuBar;
struct PUMenuBarFuncs;
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

struct PUObjectFuncs {
    void (*destroy)(struct PUBase* self_c);
    bool (*is_widget_type)(struct PUBase* self_c);
};

struct PUObject {
    struct PUObjectFuncs* funcs;
    struct PUBase* priv_data;
};

struct PUWidgetFuncs {
    void (*destroy)(struct PUBase* self_c);
    bool (*is_widget_type)(struct PUBase* self_c);
    void (*show)(struct PUBase* self_c);
    void (*resize)(struct PUBase* self_c, int width, int height);
};

struct PUWidget {
    struct PUWidgetFuncs* funcs;
    struct PUBase* priv_data;
};

struct PUPushButtonFuncs {
    void (*destroy)(struct PUBase* self_c);
    bool (*is_widget_type)(struct PUBase* self_c);
    void (*show)(struct PUBase* self_c);
    void (*resize)(struct PUBase* self_c, int width, int height);
    void (*set_released_event)(void* object, void* user_data, void (*event)(void* self_c));
    void (*set_text)(struct PUBase* self_c, const char* text);
    void (*set_flat)(struct PUBase* self_c, bool flat);
};

struct PUPushButton {
    struct PUPushButtonFuncs* funcs;
    struct PUBase* priv_data;
};

struct PUPainterFuncs {
    void (*destroy)(struct PUBase* self_c);
    void (*draw_line)(struct PUBase* self_c, int x1, int y1, int x2, int y2);
};

struct PUPainter {
    struct PUPainterFuncs* funcs;
    struct PUBase* priv_data;
};

struct PUListWidgetItemFuncs {
    void (*destroy)(struct PUBase* self_c);
    void (*set_text)(struct PUBase* self_c, const char* text);
};

struct PUListWidgetItem {
    struct PUListWidgetItemFuncs* funcs;
    struct PUBase* priv_data;
};

struct PUListWidgetFuncs {
    void (*destroy)(struct PUBase* self_c);
    bool (*is_widget_type)(struct PUBase* self_c);
    void (*show)(struct PUBase* self_c);
    void (*resize)(struct PUBase* self_c, int width, int height);
    void (*add_item)(struct PUBase* self_c, const char* text);
    struct PUListWidgetItem (*item)(struct PUBase* self_c, int index);
    void (*add_widget_item)(struct PUBase* self_c, struct PUBase* item);
    void (*set_current_row_changed_event)(void* object, void* user_data, void (*event)(void* self_c, int row));
};

struct PUListWidget {
    struct PUListWidgetFuncs* funcs;
    struct PUBase* priv_data;
};

struct PUSliderFuncs {
    void (*destroy)(struct PUBase* self_c);
    bool (*is_widget_type)(struct PUBase* self_c);
    void (*show)(struct PUBase* self_c);
    void (*resize)(struct PUBase* self_c, int width, int height);
    void (*set_value_changed_event)(void* object, void* user_data, void (*event)(void* self_c, int value));
};

struct PUSlider {
    struct PUSliderFuncs* funcs;
    struct PUBase* priv_data;
};

struct PUMainWindowFuncs {
    void (*destroy)(struct PUBase* self_c);
    bool (*is_widget_type)(struct PUBase* self_c);
    void (*show)(struct PUBase* self_c);
    void (*resize)(struct PUBase* self_c, int width, int height);
    bool (*is_animated)(struct PUBase* self_c);
    struct PUMenuBar (*menu_bar)(struct PUBase* self_c);
    void (*set_central_widget)(struct PUBase* self_c, struct PUBase* widget);
};

struct PUMainWindow {
    struct PUMainWindowFuncs* funcs;
    struct PUBase* priv_data;
};

struct PUActionFuncs {
    void (*destroy)(struct PUBase* self_c);
    bool (*is_widget_type)(struct PUBase* self_c);
    bool (*is_enabled)(struct PUBase* self_c);
    void (*set_text)(struct PUBase* self_c, const char* text);
};

struct PUAction {
    struct PUActionFuncs* funcs;
    struct PUBase* priv_data;
};

struct PUMenuFuncs {
    void (*destroy)(struct PUBase* self_c);
    bool (*is_widget_type)(struct PUBase* self_c);
    void (*show)(struct PUBase* self_c);
    void (*resize)(struct PUBase* self_c, int width, int height);
    void (*add_action_text)(struct PUBase* self_c, const char* text);
    void (*add_action)(struct PUBase* self_c, struct PUBase* action);
    void (*set_title)(struct PUBase* self_c, const char* title);
};

struct PUMenu {
    struct PUMenuFuncs* funcs;
    struct PUBase* priv_data;
};

struct PUMenuBarFuncs {
    void (*destroy)(struct PUBase* self_c);
    bool (*is_widget_type)(struct PUBase* self_c);
    void (*show)(struct PUBase* self_c);
    void (*resize)(struct PUBase* self_c, int width, int height);
    void (*add_menu)(struct PUBase* self_c, struct PUBase* menu);
};

struct PUMenuBar {
    struct PUMenuBarFuncs* funcs;
    struct PUBase* priv_data;
};

struct PUApplicationFuncs {
    void (*destroy)(struct PUBase* self_c);
    void (*set_style)(struct PUBase* self_c, const char* style);
    void (*exec)(struct PUBase* self_c);
};

struct PUApplication {
    struct PUApplicationFuncs* funcs;
    struct PUBase* priv_data;
};

typedef struct PU { 
    struct PUObject (*create_object)(PUBase* self);
    struct PUWidget (*create_widget)(PUBase* self);
    struct PUPushButton (*create_push_button)(PUBase* self);
    struct PUPainter (*create_painter)(PUBase* self);
    struct PUListWidgetItem (*create_list_widget_item)(PUBase* self);
    struct PUListWidget (*create_list_widget)(PUBase* self);
    struct PUSlider (*create_slider)(PUBase* self);
    struct PUMainWindow (*create_main_window)(PUBase* self);
    struct PUAction (*create_action)(PUBase* self);
    struct PUMenu (*create_menu)(PUBase* self);
    struct PUMenuBar (*create_menu_bar)(PUBase* self);
    struct PUApplication (*create_application)(PUBase* self);
    struct PUBase* priv_data;
} PU;

#ifdef __cplusplus
}
#endif
