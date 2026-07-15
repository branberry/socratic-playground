#ifndef EX08_RUN_H
#define EX08_RUN_H

#include "ex04_cpu.h"

#include <stddef.h>
#include <stdint.h>

/* Run until halted or max_steps reached. Returns number of steps executed. */
size_t cpu_run(Cpu *cpu, size_t max_steps);

/* Load a .rom file from disk into memory at offset 0; reset cpu. Returns 0 or -1. */
int cpu_load_rom(Cpu *cpu, const char *path);

#endif
