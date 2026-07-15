#include "ex09_ptr.h"

uint8_t *byte_at(uint8_t *buf, size_t len, size_t index) {
    (void)buf;
    (void)len;
    (void)index;
    /* TODO: return &buf[index] if index < len, else NULL */
    return NULL;
}

void swap_bytes(uint8_t *a, uint8_t *b) {
    (void)a;
    (void)b;
    /* TODO: exchange values through the pointers */
}

const uint8_t *find_byte(const uint8_t *start, const uint8_t *end, uint8_t value) {
    (void)start;
    (void)end;
    (void)value;
    /* TODO: walk [start, end); return pointer to first match or NULL */
    return NULL;
}

size_t ptr_distance(const uint8_t *start, const uint8_t *end) {
    (void)start;
    (void)end;
    /* TODO: return end - start as a byte count (assume end >= start) */
    return 0;
}
