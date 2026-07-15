#include "harness.h"

#include "../exercises/ex11_dynbuf.h"

void test_ex11(void) {
    ByteBuf bb;
    bytebuf_init(&bb);
    ASSERT(bytebuf_len(&bb) == 0);

    ASSERT(bytebuf_push(&bb, 10) == 0);
    ASSERT(bytebuf_push(&bb, 20) == 0);
    ASSERT(bytebuf_len(&bb) == 2);
    ASSERT(bytebuf_get(&bb, 0) == 10);
    ASSERT(bytebuf_get(&bb, 1) == 20);
    ASSERT(bytebuf_get(&bb, 99) == 0);

    for (int i = 0; i < 10; i++) {
        ASSERT(bytebuf_push(&bb, (uint8_t)(100 + i)) == 0);
    }
    ASSERT(bytebuf_len(&bb) == 12);
    ASSERT(bytebuf_get(&bb, 11) == 109);

    bytebuf_free(&bb);
    ASSERT(bb.data == NULL);
    ASSERT(bb.len == 0 && bb.cap == 0);
}
