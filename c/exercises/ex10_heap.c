#include "ex10_heap.h"

#include <stdlib.h>
#include <string.h>

uint8_t *make_zeroed(size_t count) {
    (void)count;
    /* TODO: calloc; return NULL when count == 0 or on failure */
    return NULL;
}

uint8_t *clone_bytes(const uint8_t *src, size_t len) {
    (void)src;
    (void)len;
    /* TODO: malloc len bytes, memcpy, return copy; NULL if len==0 or src==NULL */
    return NULL;
}

void bytes_free(uint8_t *buf) {
    (void)buf;
    /* TODO: free(buf) when buf is non-NULL */
}
