#include "harness.h"

#include "../exercises/ex04_cpu.h"
#include "../exercises/ex05_fetch.h"

void test_ex05(void) {
    Cpu cpu;
    cpu_reset(&cpu);
    cpu.memory[0] = 0x12;
    cpu.memory[1] = 0x34;
    cpu.memory[2] = 0x00;
    cpu.memory[3] = 0x01;

    ASSERT(fetch_u16(&cpu) == 0x1234);
    ASSERT(cpu.pc == 2);

    ASSERT(fetch_u16(&cpu) == 0x0001);
    ASSERT(cpu.pc == 4);
}
