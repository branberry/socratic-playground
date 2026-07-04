# TinyVM — emulator north star

The C side quest builds toward **TinyVM**: a minimal 8-bit virtual machine you
implement yourself. No SDL, no CHIP-8 keypad — just memory, registers, fetch/decode/execute.

## Architecture

```
┌─────────────────────────────────────┐
│  Cpu                                │
│  regs[4]   pc   halted              │
│  memory[256]                        │
└─────────────────────────────────────┘
         ▲
         │ load .rom
    fixtures/*.rom
```

Each exercise adds one layer until `cpu_run` executes programs from disk.

## Instruction format

- **16-bit words**, big-endian (high byte at `pc`, low byte at `pc+1`)
- After fetch, `pc` advances by 2

| Word   | Mnemonic      | Effect |
|--------|---------------|--------|
| 0x0000 | NOP           | No operation |
| 0x0001 | HALT          | Stop the CPU |
| 0x10NN | MOV r0, NN    | `regs[0] = NN` |
| 0x11NN | MOV r1, NN    | `regs[1] = NN` |
| 0x12NN | MOV r2, NN    | `regs[2] = NN` |
| 0x13NN | MOV r3, NN    | `regs[3] = NN` |
| 0x20ST | ADD rS, rT    | `regs[S] += regs[T]` (S,T = low nibbles of low byte) |

Example program (`fixtures/ex08_hello.rom`):

```
12 05   MOV r2, 5
00 01   HALT
```

After ex08, `cpu_load_rom` + `cpu_run` should leave `regs[2] == 5` and `halted == 1`.

## Exercise map

| Ex | Skill | Emulator piece |
|----|--------|----------------|
| 01 | Pointers, byte arrays | Raw memory manipulation |
| 02 | Bit ops, endianness | Decode opcode bytes |
| 03 | File I/O | Load ROM from disk |
| 04 | Structs, bounds checks | CPU state + memory |
| 05 | Fetch cycle | Read instruction word |
| 06 | Dispatch (`switch`) | NOP + HALT |
| 07 | More opcodes | MOV + ADD in `cpu_step` |
| 08 | Run loop | Full `cpu_run` + `cpu_load_rom` |

## After ex08 (stretch)

- **JMP** — change `pc` mid-instruction
- **malloc** — dynamic memory size instead of fixed 256 bytes (see **ex11** `ByteBuf`)
- **Disassembler** — print mnemonics for a ROM
- **Debugger** — step, dump regs, watch memory

## Verify one exercise at a time

```bash
make -C c test EX=01
make -C c test EX=08
make -C c test      # all exercises
```

Implement in `exercises/exNN_*.c`. Ex07 extends `cpu_step` in `ex06_step.c`.
