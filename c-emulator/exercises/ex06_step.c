#include "ex06_step.h"

#include "ex05_fetch.h"

void cpu_step(Cpu *cpu) {
    (void)cpu;
    /*
     * TODO: fetch_u16, then dispatch:
     *   0x0000 — NOP
     *   0x0001 — HALT (cpu->halted = 1)
     *
     * Ex07 extends this with MOV and ADD — see docs/EMULATOR.md.
     */
}
