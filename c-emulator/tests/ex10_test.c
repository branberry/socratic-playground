#include "harness.h"

#include "../exercises/ex10_heap.h"

void test_ex10(void) {
    uint8_t *zeros = make_zeroed(4);
    ASSERT(zeros != NULL);
    ASSERT(zeros[0] == 0 && zeros[3] == 0);
    bytes_free(zeros);

    uint8_t src[] = {1, 2, 3};
    uint8_t *copy = clone_bytes(src, 3);
    ASSERT(copy != NULL);
    ASSERT(copy[0] == 1 && copy[2] == 3);
    copy[1] = 99;
    ASSERT(src[1] == 2);
    bytes_free(copy);

    ASSERT(make_zeroed(0) == NULL);
    ASSERT(clone_bytes(src, 0) == NULL);
    ASSERT(clone_bytes(NULL, 3) == NULL);

    bytes_free(NULL);
}
