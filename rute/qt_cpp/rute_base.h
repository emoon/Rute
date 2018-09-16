#pragma once

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

struct RUBase;
struct RUArray {
    void* priv_data;
    void* elements;
    uint32_t count;
};

typedef void (*RUDeleteCallback)(void* data);

#ifdef __cplusplus
}

#endif

