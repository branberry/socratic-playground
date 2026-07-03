#include "harness.h"

#include "../exercises/ex01_bytes.h"

#include <string.h>

void test_ex01(void) {
    uint8_t buf[] = {1, 2, 3, 4, 5};
    reverse_bytes(buf, 5);
    ASSERT(buf[0] == 5 && buf[4] == 1);

    uint8_t single[] = {42};
    reverse_bytes(single, 1);
    ASSERT(single[0] == 42);

    reverse_bytes(buf, 0);

    uint8_t nums[] = {10, 20, 30};
    ASSERT(sum_bytes(nums, 3) == 60);
    ASSERT(sum_bytes(nums, 0) == 0);
}
