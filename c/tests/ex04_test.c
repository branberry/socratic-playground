#include "harness.h"

#include "../exercises/ex04_cpu.h"

#include <string.h>

void test_ex04(void) {
    Cpu cpu;
    cpu_reset(&cpu);
    ASSERT(cpu.pc == 0);
    ASSERT(cpu.halted == 0);
    ASSERT(cpu.regs[0] == 0);

    mem_write(&cpu, 10, 0x55);
    ASSERT(mem_read(&cpu, 10) == 0x55);
    ASSERT(mem_read(&cpu, TVM_MEM_SIZE) == 0);

    mem_write(&cpu, TVM_MEM_SIZE, 0xFF);
    ASSERT(mem_read(&cpu, TVM_MEM_SIZE) == 0);

    const uint8_t prog[] = {0x12, 0x05, 0x00, 0x01};
    ASSERT(load_program(&cpu, 0, prog, sizeof(prog)) == 0);
    ASSERT(cpu.memory[0] == 0x12 && cpu.memory[3] == 0x01);

    ASSERT(load_program(&cpu, TVM_MEM_SIZE - 1, prog, 2) == -1);
}
