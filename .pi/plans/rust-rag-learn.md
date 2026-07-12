# Progress plan · rust-rag-learn

> Socrates plan v1 — edit sections below. Saves to .pi/plans/rust-rag-learn.md and syncs PROGRESS.md + setup.

## Phase
0 — Warm-up (almost done)

## Context
See rust-rag-learn/docs/STEPS.md for current step

## Last session
(what passed / what you did)

## Next (5-min)
work on the inmemoryvectorstore from_chunks

## Still fuzzy
(optional — concepts still unclear)

## Checklist
- [x] Exercises 1–8 passing
- [x] Exercise 9 — `borrow_first_window`
- [x] `cargo test rust_warmup` — all green
- [x] Can explain why `Chunk` uses `String` not `&str` (one sentence)
- [x] Step 0 complete (ingest run, can explain why not one vector per book)
- [x] Step 1 — `chunk_text` implemented
- [x] `cargo test chunk` green
- [x] `cargo run -- ingest` shows multiple chunks for long docs
- [ ] Demo checkpoint captured (screenshot/log)
- [ ] Blog post 1 drafted or published
- [ ] Step 2 — `embed.rs`
- [ ] Step 3 — `store.rs`
- [ ] `cargo test embed` and `cargo test store` green
- [ ] Demo checkpoint (similarity ordering makes sense)
- [ ] Blog post 2 drafted or published
- [ ] Step 4 — `retrieve.rs` + `search` command
- [ ] `cargo test retrieve` green
- [ ] `cargo run -- search "..."` returns ranked hits
- [ ] Demo checkpoint captured
- [ ] Blog post 3 drafted or published
- [ ] Step 5 — `rag.rs` + `ask` command
- [ ] Ollama wired (embeddings + LLM)
- [ ] `cargo run -- ask "..."` returns grounded answer
- [ ] Demo checkpoint captured (note one success or failure mode)
- [ ] Blog post 4 drafted or published
- [ ] Step 6 — traits extracted
- [ ] Qdrant `VectorStore` implementation
- [ ] Search works against Qdrant
- [ ] Demo checkpoint (compare vs in-memory)
- [ ] Blog post 5 drafted or published
- [ ] README tells the full story + links to blog
- [ ] Architecture diagram matches code
- [ ] Limitations / responsible-use section
- [ ] Clean-clone verify commands documented and tested
- [ ] Blog capstone (post 6) drafted or published
- [ ] Optional: terminal demo recording

---
**Verify:** `cargo test -p rust-rag-learn`
