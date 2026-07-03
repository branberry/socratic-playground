# C side quest — progress

Update at the end of each C session (separate from root `docs/PROGRESS.md`).

**North star:** [EMULATOR.md](EMULATOR.md) — build TinyVM, a minimal 8-bit emulator.

## Current

| Field | Value |
|-------|--------|
| **Phase** | 1 — exercise stubs |
| **Last session** | _not started_ |
| **Next 5-min task** | Implement `reverse_bytes` in `exercises/ex01_bytes.c`, run `make -C c test EX=01` |

## Exercise checklist

- [ ] **ex01** — `reverse_bytes`, `sum_bytes`
- [ ] **ex02** — nibbles, `pack_be` / `unpack_be`
- [ ] **ex03** — `read_file`
- [ ] **ex04** — `Cpu` struct, memory read/write, `load_program`
- [ ] **ex05** — `fetch_u16`
- [ ] **ex06** — `cpu_step`: NOP + HALT
- [ ] **ex07** — extend `cpu_step`: MOV + ADD
- [ ] **ex08** — `cpu_run`, `cpu_load_rom`

## Log

| Date | Done | Next |
|------|------|------|
| _—_ | Exercise scaffold + TinyVM spec | ex01 |
