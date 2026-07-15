#include "harness.h"

#include "../exercises/ex03_file.h"

void test_ex03(void) {
    uint8_t buf[16];
    size_t len = 0;

    ASSERT(read_file("fixtures/ex03_tiny.bin", buf, sizeof(buf), &len) == 0);
    ASSERT(len == 4);
    ASSERT(buf[0] == 0xDE && buf[1] == 0xAD);
    ASSERT(buf[2] == 0xBE && buf[3] == 0xEF);

    ASSERT(read_file("fixtures/no_such_file.bin", buf, sizeof(buf), &len) == -1);

    uint8_t tiny[2];
    ASSERT(read_file("fixtures/ex03_tiny.bin", tiny, 2, &len) == -1);
}
