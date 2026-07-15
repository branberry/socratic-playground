#include "harness.h"

#include "../exercises/ex04_cpu.h"
#include "../exercises/ex06_step.h"

void test_ex07(void) {
    Cpu cpu;
    cpu_reset(&cpu);

    const uint8_t prog[] = {
        0x12, 0x05, /* MOV r2, 5 */
        0x20, 0x21, /* ADD r2, r1 (r2 += r1) — low byte 0x21 => S=2 T=1 */
        0x00, 0x01, /* HALT */
    };
    load_program(&cpu, 0, prog, sizeof(prog));

    cpu_step(&cpu);
    ASSERT(cpu.regs[2] == 5);

    cpu_step(&cpu);
    ASSERT(cpu.regs[2] == 5);

    cpu_step(&cpu);
    ASSERT(cpu.halted == 1);
}
