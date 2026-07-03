#include "harness.h"

#include "../exercises/ex04_cpu.h"
#include "../exercises/ex06_step.h"

void test_ex06(void) {
    Cpu cpu;
    cpu_reset(&cpu);
    cpu.memory[0] = 0x00;
    cpu.memory[1] = 0x00;
    cpu.memory[2] = 0x00;
    cpu.memory[3] = 0x01;

    cpu_step(&cpu);
    ASSERT(cpu.pc == 2);
    ASSERT(cpu.halted == 0);

    cpu_step(&cpu);
    ASSERT(cpu.halted == 1);
}
