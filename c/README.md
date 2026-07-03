# C side quest

Separate from the Rust RAG tutorial at the repo root. Goal: get comfortable with
pointers, `malloc`/`free`, structs, and small CLI tools in C.

## Quick start

```bash
make -C c build    # compile bin/c-learn
make -C c test     # run smoke tests
make -C c clean
./c/build/c-learn
```

Run commands from the repo root, or `cd c` and drop the `-C c` prefix.

## Layout

```
c/
├── src/main.c          # entry point (expand as you add topics)
├── tests/run_tests.c   # minimal test runner — no external deps
├── docs/
│   ├── TOPICS.md       # learning arc
│   └── PROGRESS.md     # session log
└── Makefile
```

## Current state

Scaffold only: hello-world binary and one passing smoke test.

## Docs

- [docs/TOPICS.md](docs/TOPICS.md) — planned exercise arc
- [docs/PROGRESS.md](docs/PROGRESS.md) — track sessions here
