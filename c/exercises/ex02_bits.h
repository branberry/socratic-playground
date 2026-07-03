#ifndef EX02_BITS_H
#define EX02_BITS_H

#include <stdint.h>

uint8_t hi_nibble(uint8_t byte);
uint8_t lo_nibble(uint8_t byte);

/* Big-endian: hi byte is the high 8 bits of the 16-bit word. */
uint16_t pack_be(uint8_t hi, uint8_t lo);
void unpack_be(uint16_t word, uint8_t *hi, uint8_t *lo);

#endif
