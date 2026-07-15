#ifndef EX02_BITS_H
#define EX02_BITS_H

#include <stdint.h>

/* Upper 4 bits of byte as a value 0x0–0xF (e.g. 0xAB → 0x0A). */
uint8_t hi_nibble(uint8_t byte);

/* Lower 4 bits of byte as a value 0x0–0xF (e.g. 0xAB → 0x0B). */
uint8_t lo_nibble(uint8_t byte);

/* Combine two bytes into a 16-bit word; hi is the high byte (big-endian). */
uint16_t pack_be(uint8_t hi, uint8_t lo);

/* Split a 16-bit word into its high and low bytes (written through *hi, *lo). */
void unpack_be(uint16_t word, uint8_t *hi, uint8_t *lo);

#endif
