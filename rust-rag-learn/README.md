# Vector Databases & RAG in Rust

Hands-on tutorial: build RAG from scratch, one module at a time.

**Monorepo home:** [../README.md](../README.md) · shared tutor docs in [../docs/](../docs/)

## Quick start

From the repo root:

```bash
cargo test -p rust-rag-learn rust_warmup
cargo test -p rust-rag-learn
cargo run -p rust-rag-learn -- ingest
```

Or `cd rust-rag-learn` and use `cargo test` / `cargo run -- ingest` as usual.

## Current state

| What | Where |
|------|-------|
| Document chunking | `src/chunk.rs` |
| Embeddings (in progress) | `src/embed.rs` |
| Rust warm-up exercises | `src/rust_warmup.rs` — [docs/RUST_WARMUP.md](docs/RUST_WARMUP.md) |
| Ingest CLI | `src/main.rs` |
| Sample corpus | `data/sample_docs/` |
| Walkthrough | [docs/STEPS.md](docs/STEPS.md) |
| Reading list | [docs/RESOURCES.md](docs/RESOURCES.md) |
| Roadmap & progress | [docs/ROADMAP.md](docs/ROADMAP.md) · [docs/PROGRESS.md](docs/PROGRESS.md) |

## What you'll build

| Step | You create | Concept |
|------|------------|---------|
| 1 | `chunk.rs` | Sliding-window chunking |
| 2 | `embed.rs` | Vectors, cosine similarity, mock embedder |
| 3 | `store.rs` | In-memory vector search |
| 4 | `retrieve.rs` + `search` CLI | Query → context (+ hybrid search stretch) |
| 5 | `rag.rs` + `ask` CLI | Prompt + LLM generation |
| 6 | Traits + optional Qdrant/LanceDB | Swappable backends |
| 7 | `eval.rs` + golden set (stretch) | Hit@k / MRR (+ optional faithfulness) |

Read the full guide in **[docs/STEPS.md](docs/STEPS.md)**.

## Layout

```
rust-rag-learn/
├── data/sample_docs/
├── docs/
├── src/
│   ├── chunk.rs
│   ├── embed.rs
│   ├── rust_warmup.rs
│   ├── lib.rs
│   └── main.rs
└── (you add store.rs, retrieve.rs, rag.rs)
```

## Tutor mode

Project tutor prompt: **[AGENTS.md](AGENTS.md)**. Shared Socratic contract: [../docs/SOCRATIC_METHOD.md](../docs/SOCRATIC_METHOD.md).
