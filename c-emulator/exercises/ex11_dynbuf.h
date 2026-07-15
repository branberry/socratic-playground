#ifndef EX11_DYNBUF_H
#define EX11_DYNBUF_H

#include <stddef.h>
#include <stdint.h>

/*
 * Growable byte buffer on the heap (ownership practice).
 *
 * Push doubles capacity when full. After bytebuf_free, data is NULL and
 * len/cap are 0.
 */

typedef struct {
    uint8_t *data;
    size_t len;
    size_t cap;
} ByteBuf;

void bytebuf_init(ByteBuf *bb);

void bytebuf_free(ByteBuf *bb);

/* Append one byte. Return 0 on success, -1 on allocation failure. */
int bytebuf_push(ByteBuf *bb, uint8_t value);

size_t bytebuf_len(const ByteBuf *bb);

/* Return byte at index, or 0 if index >= len. */
uint8_t bytebuf_get(const ByteBuf *bb, size_t index);

#endif
