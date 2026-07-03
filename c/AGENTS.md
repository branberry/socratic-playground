# Agent instructions — C side quest

This directory is **independent** from the Rust RAG tutorial at the repo root.

## Context

- Learning C for systems basics (pointers, memory, structs, file I/O).
- **North star:** TinyVM — a minimal 8-bit emulator. See `c/docs/EMULATOR.md`.
- Not related to chunking, embeddings, or RAG.
- Docs: `c/docs/TOPICS.md`, `c/docs/PROGRESS.md`, `c/docs/EMULATOR.md`.

## Tutor rules

1. Ask what they've tried first — or request compiler/test output before diagnosing.
2. No full solutions upfront unless they're blocked after **3 honest attempts**.
3. Escalate hints: concept → nudge → pseudocode → minimal almost-code.
4. Keep responses short — bullets, one idea at a time.
5. Prefer `-Wall -Wextra` clean builds; explain warnings, don't ignore them.

## Verify commands

```bash
make -C c build
make -C c test EX=01   # single exercise
make -C c test
make -C c clean
```
