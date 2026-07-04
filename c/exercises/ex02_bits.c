#include "ex02_bits.h"

/*
 * A byte has 8 bits. A nibble is half of that — 4 bits, one hex digit.
 *
 * Example: 0xAB
 *   high nibble = 0xA  (bits 7–4)
 *   low  nibble = 0xB  (bits 3–0)
 */

uint8_t hi_nibble(uint8_t byte) {
    (void)byte;
    /* Return the upper 4 bits of byte as a value 0x0–0xF. */
    return 0;
}

uint8_t lo_nibble(uint8_t byte) {
    (void)byte;
    /* Return the lower 4 bits of byte as a value 0x0–0xF. */
    return 0;
}

/*
 * TinyVM instructions are 16-bit words stored as two bytes in memory:
 * high byte first, then low byte ("big-endian").
 *
 * pack_be(0x12, 0x34) → 0x1234
 */

uint16_t pack_be(uint8_t hi, uint8_t lo) {
    (void)hi;
    (void)lo;
    /* Combine hi and lo into one 16-bit word; hi is the high byte. */
    return 0;
}

void unpack_be(uint16_t word, uint8_t *hi, uint8_t *lo) {
    (void)word;
    (void)hi;
    (void)lo;
    /*
     * Split word into its two bytes and write them through hi and lo.
     * unpack_be(0xDEAD, &hi, &lo) should set hi = 0xDE and lo = 0xAD.
     */
}
