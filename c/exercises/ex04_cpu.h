#ifndef EX04_CPU_H
#define EX04_CPU_H

#include <stddef.h>
#include <stdint.h>

#define TVM_MEM_SIZE 256
#define TVM_NUM_REGS 4

typedef struct {
    uint8_t regs[TVM_NUM_REGS];
    uint16_t pc;
    uint8_t memory[TVM_MEM_SIZE];
    int halted;
} Cpu;

void cpu_reset(Cpu *cpu);
uint8_t mem_read(const Cpu *cpu, uint16_t addr);
void mem_write(Cpu *cpu, uint16_t addr, uint8_t value);

/* Copy len bytes into memory starting at offset; return 0 or -1 if overflow. */
int load_program(Cpu *cpu, uint16_t offset, const uint8_t *bytes, size_t len);

#endif
