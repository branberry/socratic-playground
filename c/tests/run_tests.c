#include <stdio.h>
#include <stdlib.h>

static int passes;
static int failures;

#define ASSERT(cond)                                                        \
    do {                                                                    \
        if (cond) {                                                         \
            passes++;                                                       \
        } else {                                                            \
            fprintf(stderr, "FAIL: %s (%s:%d)\n", #cond, __FILE__, __LINE__); \
            failures++;                                                     \
        }                                                                   \
    } while (0)

static void test_smoke(void) {
    ASSERT(1 + 1 == 2);
}

int main(void) {
    test_smoke();

    printf("Tests: %d passed, %d failed\n", passes, failures);
    return failures == 0 ? EXIT_SUCCESS : EXIT_FAILURE;
}
