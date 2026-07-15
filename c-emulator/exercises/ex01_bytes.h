#ifndef EX01_BYTES_H
#define EX01_BYTES_H

#include <stddef.h>
#include <stdint.h>

/* Reverse buf in place (first byte swaps with last, etc.). */
void reverse_bytes(uint8_t *buf, size_t len);

/* Sum all bytes; return the low 16 bits. */
uint16_t sum_bytes(const uint8_t *buf, size_t len);

#endif
