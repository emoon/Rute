#pragma once

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef void (*RUDeleteCallback)(void* data);

struct RUBase;
struct RUArray {
    // Callback used to free the elements allocated for this
    RUDeleteCallback delete_callback;
    void* priv_data;
    void* elements;
    // used when passing back arrays with Qt/Rc objects that needs the functions
    void* all_funcs;
    unsigned char* owners;
    uint32_t count;
};

#ifdef __cplusplus
}

#endif

