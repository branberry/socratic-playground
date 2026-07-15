#include "ex04_cpu.h"

void cpu_reset(Cpu *cpu) {
    (void)cpu;
    /* TODO: zero regs, pc = 0, halted = 0, clear memory */
}

uint8_t mem_read(const Cpu *cpu, uint16_t addr) {
    (void)cpu;
    (void)addr;
    /* TODO: bounds check — return 0 if addr >= TVM_MEM_SIZE */
    return 0;
}

void mem_write(Cpu *cpu, uint16_t addr, uint8_t value) {
    (void)cpu;
    (void)addr;
    (void)value;
    /* TODO: bounds check — ignore out-of-range writes */
}

int load_program(Cpu *cpu, uint16_t offset, const uint8_t *bytes, size_t len) {
    (void)cpu;
    (void)offset;
    (void)bytes;
    (void)len;
    /* TODO: copy bytes into memory; return -1 if offset + len overflows */
    return -1;
}
