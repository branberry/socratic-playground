# Progress tracker

Update this file at the **end of every session** — especially the “Next 5-min task” line. Future-you re-enters here.

**Roadmap:** [ROADMAP.md](ROADMAP.md) · **Implementation:** [STEPS.md](STEPS.md)

---

## Long-term goal

Build a from-scratch Rust RAG CLI (through Qdrant), **documented enough that I can share my knowledge and experience with others** — blog series + polished GitHub repo.

---

## Current status

| Field | Value |
|-------|-------|
| **Phase** | 0 — Warm-up (almost done) |
| **Last updated** | _(set date when you edit)_ |
| **Last session** | _(what you did)_ |
| **Next 5-min task** | Implement `borrow_first_window` in `rust_warmup.rs`, run `cargo test exercise_9` |

---

## Phase checklist

### Phase 0 — Warm-up

- [x] Exercises 1–8 passing
- [ ] Exercise 9 — `borrow_first_window`
- [ ] `cargo test rust_warmup` — all green
- [ ] Can explain why `Chunk` uses `String` not `&str` (one sentence)

### Phase 1 — Chunking

- [ ] Step 0 complete (ingest run, can explain why not one vector per book)
- [ ] Step 1 — `chunk_text` implemented
- [ ] `cargo test chunk` green
- [ ] `cargo run -- ingest` shows multiple chunks for long docs
- [ ] Demo checkpoint captured (screenshot/log)
- [ ] Blog post 1 drafted or published

### Phase 2 — Embeddings & store

- [ ] Step 2 — `embed.rs`
- [ ] Step 3 — `store.rs`
- [ ] `cargo test embed` and `cargo test store` green
- [ ] Demo checkpoint (similarity ordering makes sense)
- [ ] Blog post 2 drafted or published

### Phase 3 — Retrieval CLI

- [ ] Step 4 — `retrieve.rs` + `search` command
- [ ] `cargo test retrieve` green
- [ ] `cargo run -- search "..."` returns ranked hits
- [ ] Demo checkpoint captured
- [ ] Blog post 3 drafted or published

### Phase 4 — RAG + Ollama

- [ ] Step 5 — `rag.rs` + `ask` command
- [ ] Ollama wired (embeddings + LLM)
- [ ] `cargo run -- ask "..."` returns grounded answer
- [ ] Demo checkpoint captured (note one success or failure mode)
- [ ] Blog post 4 drafted or published

### Phase 5 — Traits + Qdrant

- [ ] Step 6 — traits extracted
- [ ] Qdrant `VectorStore` implementation
- [ ] Search works against Qdrant
- [ ] Demo checkpoint (compare vs in-memory)
- [ ] Blog post 5 drafted or published

### Phase 6 — Peer polish

- [ ] README tells the full story + links to blog
- [ ] Architecture diagram matches code
- [ ] Limitations / responsible-use section
- [ ] Clean-clone verify commands documented and tested
- [ ] Blog capstone (post 6) drafted or published
- [ ] Optional: terminal demo recording

---

## Session log

_Newest first. One line per session is enough._

| Date | Phase | Done | Next 5-min task |
|------|-------|------|-----------------|
| _example_ | 0 | Exercises 1–8 green | Finish ex 9: `borrow_first_window` |
| | | | |

---

## Re-entry (copy when you’ve been away)

1. Read **Next 5-min task** above
2. Pick one [tangential quiz from ROADMAP.md](ROADMAP.md#tangential-task--quiz-bank) for your phase
3. Do the 5-min task, then decide if you have energy for more

**Career reminder:** _Who benefits when this ships? Engineers learning RAG in Rust; peers evaluating vector search trade-offs; future you on citation-grounded systems._
