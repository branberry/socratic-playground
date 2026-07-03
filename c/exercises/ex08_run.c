#include "ex08_run.h"

#include "ex03_file.h"
#include "ex06_step.h"

size_t cpu_run(Cpu *cpu, size_t max_steps) {
    (void)cpu;
    (void)max_steps;
    /* TODO: loop cpu_step until halted or max_steps */
    return 0;
}

int cpu_load_rom(Cpu *cpu, const char *path) {
    (void)cpu;
    (void)path;
    /* TODO: read_file into buffer, cpu_reset, load_program at 0 */
    return -1;
}
