#ifndef EX10_HEAP_H
#define EX10_HEAP_H

#include <stddef.h>
#include <stdint.h>

/*
 * Heap allocation drills.
 *
 * make_zeroed  — calloc(count, 1); caller must bytes_free the result
 * clone_bytes  — malloc + memcpy copy; caller must bytes_free
 * bytes_free   — free non-NULL pointers (safe to pass NULL)
 *
 * Return NULL from allocators when count/len is 0 or allocation fails.
 */

uint8_t *make_zeroed(size_t count);

uint8_t *clone_bytes(const uint8_t *src, size_t len);

void bytes_free(uint8_t *buf);

#endif
