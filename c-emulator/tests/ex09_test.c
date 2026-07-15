#include "harness.h"

#include "../exercises/ex09_ptr.h"

void test_ex09(void) {
    uint8_t a = 1;
    uint8_t b = 2;
    swap_bytes(&a, &b);
    ASSERT(a == 2 && b == 1);

    uint8_t arr[] = {10, 20, 30, 40};
    uint8_t *mid = byte_at(arr, 4, 1);
    ASSERT(mid != NULL && *mid == 20);

    uint8_t *slot = byte_at(arr, 4, 1);
    if (slot) {
        *slot = 99;
    }
    ASSERT(arr[1] == 99);

    ASSERT(byte_at(arr, 4, 4) == NULL);

    const uint8_t *hit = find_byte(arr, arr + 4, 40);
    ASSERT(hit == &arr[3]);
    ASSERT(find_byte(arr, arr + 4, 7) == NULL);

    ASSERT(ptr_distance(arr, arr + 4) == 4);
    ASSERT(ptr_distance(arr + 2, arr + 2) == 0);
}
