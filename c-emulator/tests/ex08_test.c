#include "harness.h"

#include "../exercises/ex04_cpu.h"
#include "../exercises/ex08_run.h"

void test_ex08(void) {
    Cpu cpu;
    ASSERT(cpu_load_rom(&cpu, "fixtures/ex08_hello.rom") == 0);

    size_t steps = cpu_run(&cpu, 100);
    ASSERT(steps > 0);
    ASSERT(cpu.halted == 1);
    ASSERT(cpu.regs[2] == 5);
}
