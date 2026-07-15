#include "harness.h"

#include "../exercises/ex02_bits.h"

void test_ex02(void) {
    ASSERT(hi_nibble(0xAB) == 0x0A);
    ASSERT(lo_nibble(0xAB) == 0x0B);

    ASSERT(pack_be(0x12, 0x34) == 0x1234);

    uint8_t hi = 0;
    uint8_t lo = 0;
    unpack_be(0xDEAD, &hi, &lo);
    ASSERT(hi == 0xDE && lo == 0xAD);
}
