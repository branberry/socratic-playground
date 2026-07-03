# C learning topics

Arc toward a **simple emulator** (TinyVM). Full spec: [EMULATOR.md](EMULATOR.md).

| Ex | Topic | Emulator skill |
|----|--------|----------------|
| 01 | Pointers, byte arrays | Memory buffers |
| 02 | Bit ops, endianness | Opcode decoding |
| 03 | File I/O | Load ROM |
| 04 | Structs, bounds checks | CPU state |
| 05 | Fetch cycle | Read instruction word |
| 06 | `switch` dispatch | NOP + HALT |
| 07 | More opcodes | MOV + ADD |
| 08 | Run loop | Execute programs from disk |

```bash
make -C c test EX=01   # one exercise
make -C c test         # all (fails until you implement each)
```

Stretch after ex08: JMP, disassembler, debugger, heap-backed memory.
