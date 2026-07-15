#ifndef EX09_PTR_H
#define EX09_PTR_H

#include <stddef.h>
#include <stdint.h>

/*
 * Pointer drills (no heap yet).
 *
 * byte_at     — bounds-checked address of buf[index], or NULL
 * swap_bytes  — exchange *a and *b through pointers
 * find_byte   — first match in [start, end), or NULL
 * ptr_distance — element count from start to end (end must be >= start)
 */

uint8_t *byte_at(uint8_t *buf, size_t len, size_t index);

void swap_bytes(uint8_t *a, uint8_t *b);

const uint8_t *find_byte(const uint8_t *start, const uint8_t *end, uint8_t value);

size_t ptr_distance(const uint8_t *start, const uint8_t *end);

#endif
