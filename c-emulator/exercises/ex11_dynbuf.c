#include "ex11_dynbuf.h"

#include <stdlib.h>

void bytebuf_init(ByteBuf *bb) {
    (void)bb;
    /* TODO: zero data/len/cap */
}

void bytebuf_free(ByteBuf *bb) {
    (void)bb;
    /* TODO: free data; reset fields to empty state */
}

int bytebuf_push(ByteBuf *bb, uint8_t value) {
    (void)bb;
    (void)value;
    /* TODO: grow when len == cap (start cap 4); append value; return -1 on OOM */
    return -1;
}

size_t bytebuf_len(const ByteBuf *bb) {
    (void)bb;
    /* TODO */
    return 0;
}

uint8_t bytebuf_get(const ByteBuf *bb, size_t index) {
    (void)bb;
    (void)index;
    /* TODO: bounds check — return 0 when index >= len */
    return 0;
}
