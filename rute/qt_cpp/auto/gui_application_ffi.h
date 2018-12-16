// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "clipboard_ffi.h"
#include "cursor_ffi.h"
#include "font_ffi.h"
#include "icon_ffi.h"
#include "object_ffi.h"
#include "palette_ffi.h"
#include "screen_ffi.h"
#include "window_ffi.h"

struct RUGuiApplicationFuncs;
struct RUGuiApplication;

typedef struct RUGuiApplicationFuncs {
    void (*set_application_display_name)(struct RUBase* self_c,
                                         const char* name);
    const char* (*application_display_name)(struct RUBase* self_c);
    void (*set_desktop_file_name)(struct RUBase* self_c, const char* name);
    const char* (*desktop_file_name)(struct RUBase* self_c);
    struct RUWindow (*top_level_at)(struct RUBase* self_c, struct RUBase* pos);
    void (*set_window_icon)(struct RUBase* self_c, struct RUBase* icon);
    struct RUIcon (*window_icon)(struct RUBase* self_c);
    const char* (*platform_name)(struct RUBase* self_c);
    struct RUWindow (*modal_window)(struct RUBase* self_c);
    struct RUWindow (*focus_window)(struct RUBase* self_c);
    struct RUObject (*focus_object)(struct RUBase* self_c);
    struct RUScreen (*primary_screen)(struct RUBase* self_c);
    struct RUScreen (*screen_at)(struct RUBase* self_c, struct RUBase* point);
    float (*device_pixel_ratio)(struct RUBase* self_c);
    struct RUCursor (*override_cursor)(struct RUBase* self_c);
    void (*set_override_cursor)(struct RUBase* self_c, struct RUBase* arg0);
    void (*change_override_cursor)(struct RUBase* self_c, struct RUBase* arg0);
    void (*restore_override_cursor)(struct RUBase* self_c);
    struct RUFont (*font)(struct RUBase* self_c);
    void (*set_font)(struct RUBase* self_c, struct RUBase* arg0);
    struct RUClipboard (*clipboard)(struct RUBase* self_c);
    struct RUPalette (*palette)(struct RUBase* self_c);
    void (*set_palette)(struct RUBase* self_c, struct RUBase* pal);
    int (*keyboard_modifiers)(struct RUBase* self_c);
    int (*query_keyboard_modifiers)(struct RUBase* self_c);
    int (*mouse_buttons)(struct RUBase* self_c);
    void (*set_layout_direction)(struct RUBase* self_c, int direction);
    int (*layout_direction)(struct RUBase* self_c);
    bool (*is_right_to_left)(struct RUBase* self_c);
    bool (*is_left_to_right)(struct RUBase* self_c);
    void (*set_desktop_settings_aware)(struct RUBase* self_c, bool on);
    bool (*desktop_settings_aware)(struct RUBase* self_c);
    void (*set_quit_on_last_window_closed)(struct RUBase* self_c, bool quit);
    bool (*quit_on_last_window_closed)(struct RUBase* self_c);
    int (*application_state)(struct RUBase* self_c);
    int (*exec)(struct RUBase* self_c);
    bool (*is_session_restored)(struct RUBase* self_c);
    const char* (*session_id)(struct RUBase* self_c);
    const char* (*session_key)(struct RUBase* self_c);
    bool (*is_saving_session)(struct RUBase* self_c);
    bool (*is_fallback_session_management_enabled)(struct RUBase* self_c);
    void (*set_fallback_session_management_enabled)(struct RUBase* self_c,
                                                    bool arg0);
    void (*sync)(struct RUBase* self_c);
} RUGuiApplicationFuncs;

typedef struct RUGuiApplicationAllFuncs {
    struct RUObjectFuncs* object_funcs;
    struct RUCoreApplicationFuncs* core_application_funcs;
    struct RUGuiApplicationFuncs* gui_application_funcs;
} RUGuiApplicationAllFuncs;

typedef struct RUGuiApplication {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUGuiApplicationAllFuncs* all_funcs;
} RUGuiApplication;

extern RUGuiApplicationFuncs s_gui_application_funcs;
extern RUGuiApplicationAllFuncs s_gui_application_all_funcs;

#ifdef __cplusplus
}
#endif
