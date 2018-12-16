// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "core_application_ffi.h"

struct RUCoreApplicationFuncs;
struct RUCoreApplication;

typedef struct RUCoreApplicationFuncs {
    void (*set_organization_domain)(struct RUBase* self_c,
                                    const char* org_domain);
    const char* (*organization_domain)(struct RUBase* self_c);
    void (*set_organization_name)(struct RUBase* self_c, const char* org_name);
    const char* (*organization_name)(struct RUBase* self_c);
    void (*set_application_name)(struct RUBase* self_c,
                                 const char* application);
    const char* (*application_name)(struct RUBase* self_c);
    void (*set_application_version)(struct RUBase* self_c, const char* version);
    const char* (*application_version)(struct RUBase* self_c);
    void (*set_setuid_allowed)(struct RUBase* self_c, bool allow);
    bool (*is_setuid_allowed)(struct RUBase* self_c);
    struct RUCoreApplication (*instance)(struct RUBase* self_c);
    int (*exec)(struct RUBase* self_c);
    void (*exit)(struct RUBase* self_c, int retcode);
    bool (*send_event)(struct RUBase* self_c, struct RUBase* receiver,
                       struct RUBase* event);
    void (*post_event)(struct RUBase* self_c, struct RUBase* receiver,
                       struct RUBase* event, int priority);
    void (*send_posted_events)(struct RUBase* self_c, struct RUBase* receiver,
                               int event_type);
    void (*remove_posted_events)(struct RUBase* self_c, struct RUBase* receiver,
                                 int event_type);
    bool (*has_pending_events)(struct RUBase* self_c);
    bool (*starting_up)(struct RUBase* self_c);
    bool (*closing_down)(struct RUBase* self_c);
    const char* (*application_dir_path)(struct RUBase* self_c);
    const char* (*application_file_path)(struct RUBase* self_c);
    int64_t (*application_pid)(struct RUBase* self_c);
    void (*add_library_path)(struct RUBase* self_c, const char* arg0);
    void (*remove_library_path)(struct RUBase* self_c, const char* arg0);
    void (*flush)(struct RUBase* self_c);
    bool (*is_quit_lock_enabled)(struct RUBase* self_c);
    void (*set_quit_lock_enabled)(struct RUBase* self_c, bool enabled);
    void (*quit)(struct RUBase* self_c);
} RUCoreApplicationFuncs;

typedef struct RUCoreApplicationAllFuncs {
    struct RUObjectFuncs* object_funcs;
    struct RUCoreApplicationFuncs* core_application_funcs;
} RUCoreApplicationAllFuncs;

typedef struct RUCoreApplication {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUCoreApplicationAllFuncs* all_funcs;
} RUCoreApplication;

extern RUCoreApplicationFuncs s_core_application_funcs;
extern RUCoreApplicationAllFuncs s_core_application_all_funcs;

#ifdef __cplusplus
}
#endif
