#ifndef HARNESS_H
#define HARNESS_H

#include <stdio.h>
#include <stdlib.h>

extern int g_passes;
extern int g_failures;

#define ASSERT(cond)                                                        \
    do {                                                                    \
        if (cond) {                                                         \
            g_passes++;                                                     \
        } else {                                                            \
            fprintf(stderr, "  FAIL: %s (%s:%d)\n", #cond, __FILE__,        \
                    __LINE__);                                              \
            g_failures++;                                                   \
        }                                                                   \
    } while (0)

#define RUN(suite)                                                          \
    do {                                                                    \
        printf("  %s\n", #suite);                                           \
        suite();                                                            \
    } while (0)

#endif
