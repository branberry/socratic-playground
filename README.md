# Learning playground (monorepo)

Independent learning tracks — shared **Socratic** tutor contract, no shared code.

| Track | Directory | Start here |
|-------|-----------|------------|
| **Rust — RAG** | [rust-rag-learn/](rust-rag-learn/) | `cargo test -p rust-rag-learn` |
| **Rust — WebGPU** | [rust-webgpu/](rust-webgpu/) | `cargo run -p rust-webgpu` (scaffold) |
| **C — TinyVM** | [c/](c/) | `make -C c test EX=01` |

## Shared docs

| Doc | Purpose |
|-----|---------|
| [docs/SOCRATIC_METHOD.md](docs/SOCRATIC_METHOD.md) | Core tutor contract — questions before answers |
| [docs/AI_LEARNING_WORKFLOW.md](docs/AI_LEARNING_WORKFLOW.md) | Session rituals, stuck ladder |

Each track has its own `AGENTS.md`, `docs/`, and verify commands.

## Layout

```
.
├── Cargo.toml              # workspace (rust-rag-learn, rust-webgpu)
├── docs/                   # shared pedagogy
├── rust-rag-learn/         # RAG CLI tutorial
├── rust-webgpu/            # WebGPU tutorial (scaffold)
└── c/                      # C emulator side quest
```

## Workspace commands

```bash
cargo test --workspace          # all Rust crates
cargo test -p rust-rag-learn    # one crate
make -C c test EX=01            # C exercises
```

## Tutor mode

Root **[AGENTS.md](AGENTS.md)** maps the monorepo. Per-project tutors live in each directory. All tracks follow [docs/SOCRATIC_METHOD.md](docs/SOCRATIC_METHOD.md).
