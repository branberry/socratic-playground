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

### Pointer & heap track (parallel to emulator)

Do these anytime after **ex01** — especially if pointers / `malloc` feel rusty.

| Ex | Topic | Skills |
|----|--------|--------|
| 09 | Pointer arithmetic | `&`, `*`, bounds, scan a range |
| 10 | Heap allocation | `calloc`, `malloc`, `memcpy`, `free` |
| 11 | Ownership | growable `ByteBuf`, who frees what |

```bash
make -C c test EX=09   # pointer drills only
make -C c test EX=01   # one exercise
make -C c test         # all (fails until you implement each)
```

Stretch after ex08: JMP, disassembler, debugger, swap TinyVM to heap-backed memory (ex11 helps).
