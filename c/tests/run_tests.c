#include "harness.h"

#include <stdio.h>
#include <string.h>

int g_passes;
int g_failures;

void test_ex01(void);
void test_ex02(void);
void test_ex03(void);
void test_ex04(void);
void test_ex05(void);
void test_ex06(void);
void test_ex07(void);
void test_ex08(void);

static int report(void) {
    printf("Tests: %d passed, %d failed\n", g_passes, g_failures);
    return g_failures == 0 ? 0 : 1;
}

static void run_one(const char *id) {
    if (strcmp(id, "01") == 0) {
        RUN(test_ex01);
    } else if (strcmp(id, "02") == 0) {
        RUN(test_ex02);
    } else if (strcmp(id, "03") == 0) {
        RUN(test_ex03);
    } else if (strcmp(id, "04") == 0) {
        RUN(test_ex04);
    } else if (strcmp(id, "05") == 0) {
        RUN(test_ex05);
    } else if (strcmp(id, "06") == 0) {
        RUN(test_ex06);
    } else if (strcmp(id, "07") == 0) {
        RUN(test_ex07);
    } else if (strcmp(id, "08") == 0) {
        RUN(test_ex08);
    } else {
        fprintf(stderr, "Unknown exercise: %s (use 01-08)\n", id);
        g_failures++;
    }
}

int main(int argc, char **argv) {
    if (argc == 2) {
        run_one(argv[1]);
        return report();
    }

    RUN(test_ex01);
    RUN(test_ex02);
    RUN(test_ex03);
    RUN(test_ex04);
    RUN(test_ex05);
    RUN(test_ex06);
    RUN(test_ex07);
    RUN(test_ex08);
    return report();
}
