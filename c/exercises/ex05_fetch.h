#ifndef EX05_FETCH_H
#define EX05_FETCH_H

#include "ex04_cpu.h"

#include <stdint.h>

/*
 * Read the 16-bit big-endian instruction word at cpu->pc,
 * advance pc by 2, return the word.
 * If pc would read past memory, return 0 and do not advance.
 */
uint16_t fetch_u16(Cpu *cpu);

#endif
