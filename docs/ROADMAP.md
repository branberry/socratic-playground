# Project roadmap

Your long-term plan for **rust-rag-learn** — paced for 2–4 sessions/week (~30–60 min each), with no hard deadline.

**Living checklist:** update [PROGRESS.md](PROGRESS.md) after each session.

---

## Long-term goal

> **Build a from-scratch Rust RAG CLI (through Qdrant), documented enough that I can share my knowledge and experience with others** — via a blog series and a polished GitHub repository.

### What “done” looks like

| Deliverable | Success criteria |
|-------------|------------------|
| **Working system** | Steps 0–6 complete: ingest → chunk → embed → retrieve → `ask` → Qdrant-backed search with traits |
| **Blog series** | One post per major phase (see below) — problem-first, not tutorial dump |
| **Polished repo** | README tells the story; architecture diagram; verify commands work; honest limitations section |
| **Peer value** | Another engineer could learn *why* you made each choice, not just copy code |

### Why this matters (career fuel)

When motivation dips, reconnect to **who this helps**:

| Audience | Problem you’re solving |
|----------|------------------------|
| **Engineers learning RAG** | Most RAG content is Python notebooks — you’re showing the same pipeline in Rust, with tests |
| **Platform / infra peers** | Chunking, embedding trade-offs, linear scan vs Qdrant — real decisions teams face at scale |
| **Future you** | Citation-grounded retrieval, vector search, and LLM wiring — portable skills for health/clinical AI work |

You’re not “doing exercises.” You’re building a **reference implementation** you can stand behind in conversation.

---

## Pace & timeline

| Assumption | Value |
|------------|-------|
| Sessions per week | 2–4 |
| Session length | 30–60 min (15 min counts on bad days) |
| Hard deadline | None — consistency over speed |

**Rough calendar:** 3–5 months steady, 4–6 months with normal gaps.

| Phase | Focus | Sessions (est.) |
|-------|-------|-----------------|
| 0 | Finish warm-up | 1 |
| 1 | Chunking + blog post 1 | 2–4 |
| 2 | Embeddings + store + blog post 2 | 5–8 |
| 3 | Retrieval CLI + blog post 3 | 3–5 |
| 4 | RAG + Ollama + blog post 4 | 4–6 |
| 5 | Qdrant + traits + blog post 5 | 4–8 |
| 6 | Repo polish + capstone post | 3–6 |

---

## Phases

### Phase 0 — Warm-up complete

**Code:** Exercise 9 (`borrow_first_window`) → all `cargo test rust_warmup` green.

**Demo checkpoint:** Run full warm-up test suite; note one sentence on why `Chunk` uses `String` not `&str`.

**Career hook:** Lifetimes and ownership aren’t academic — they’re why your stored chunks outlive the file buffer.

**Blog:** No post yet. Optional: one social/note — “Why I’m building RAG in Rust.”

---

### Phase 1 — Chunking that actually splits documents

**Code:** [STEPS.md](STEPS.md) Step 1 — `TextChunker::chunk_text` in `chunk.rs`.

**Verify:**
```bash
cargo test chunk
cargo run -- ingest   # long docs → multiple chunks
```

**Demo checkpoint:** Screenshot or terminal log showing `rust_basics.txt` or a long doc producing 3+ chunks with stable ids.

**Career hook:** Every RAG system fails silently with bad chunking. You’re solving the first real production problem.

**Blog post 1 (draft title):** *“Why RAG doesn’t embed whole books — and how I chunk text in Rust”*

Cover: context limits, window vs overlap, byte vs char boundaries, what breaks at scale.

---

### Phase 2 — Embeddings & in-memory search

**Code:** Steps 2–3 — `embed.rs`, `store.rs`.

**Verify:**
```bash
cargo test embed
cargo test store
```

**Demo checkpoint:** Unit test or small script showing similar texts score higher than unrelated texts; top-k returns sensible order.

**Career hook:** This is the “vector DB” conversation in miniature — normalization, similarity, metadata.

**Blog post 2:** *“From text to vectors: a mock embedder before you reach for OpenAI”*

Cover: cosine similarity, L2 normalize, why mock embedders are useful for tests, dimension trade-offs.

---

### Phase 3 — Retrieval CLI

**Code:** Step 4 — `retrieve.rs`, `search` subcommand in `main.rs`.

**Verify:**
```bash
cargo test retrieve
cargo run -- search "ownership and borrowing"
```

**Demo checkpoint:** Query returns ranked hits with scores, source ids, and snippet text.

**Career hook:** Retrieval quality *is* product quality — irrelevant chunks poison downstream LLM answers.

**Blog post 3:** *“Search without generation: why I built retrieval before touching an LLM”*

Cover: embed query → top-k → format context; failure modes (empty hits, wrong chunks).

---

### Phase 4 — Full RAG with Ollama

**Code:** Step 5 — `rag.rs`, `ask` subcommand, Ollama embeddings + LLM.

**Verify:**
```bash
ollama pull nomic-embed-text
ollama pull llama3.2
cargo run -- ask "What is a vector database?"
```

**Demo checkpoint:** Record terminal: question → retrieved context → grounded answer (note one hallucination or success).

**Career hook:** This is the demo clients and peers ask for — grounded Q&A, not chatbot theater.

**Blog post 4:** *“Grounding LLM answers: prompt design, context injection, and when the model ignores you”*

Cover: `build_prompt`, temperature, “answer only from context,” insufficient-evidence fallback (stretch).

---

### Phase 5 — Production shape: traits + Qdrant

**Code:** Step 6 — `Embedder`, `VectorStore`, `Retriever` traits; Qdrant implementation.

**Verify:**
```bash
# Qdrant running (docker or local)
cargo test   # trait impls + integration
cargo run -- search "..."   # against Qdrant backend
```

**Demo checkpoint:** Same query against in-memory store vs Qdrant — note latency/ops story.

**Career hook:** “When does linear scan stop being enough?” — a question you can answer from experience.

**Blog post 5:** *“Outgrowing linear scan: swapping in Qdrant without rewriting the pipeline”*

Cover: HNSW vs exact search, trait boundaries, ops (Docker, persistence, filtering preview).

---

### Phase 6 — Peer polish (part of the goal, not optional)

**Repo polish checklist:**

- [ ] README: problem statement, architecture, quick start, link to blog series
- [ ] `docs/STEPS.md` still accurate; add “I built this” narrative in README
- [ ] Architecture diagram matches final code
- [ ] Limitations / responsible-use section (mock embedder, no PHI, hallucination risk)
- [ ] All verify commands pass on a clean clone
- [ ] Optional: 2-min terminal demo recording (asciinema or screen recording)

**Blog capstone:** *“Building RAG in Rust from scratch: what I’d do differently”*

Retrospective: timeline, mistakes, what surprised you, who should (and shouldn’t) copy this approach.

**Order:** Blog series can publish **as you finish each phase** — don’t wait until Phase 6. Repo polish is the final pass once posts exist.

---

## Session template (30–60 min)

Copy into your notes or Cursor chat at session start:

```
Session start
- Phase: [N]
- Goal (one thing): [e.g. "pass cargo test chunk"]
- Energy: low / medium / high
- Career reminder: [one line — who does this help?]
```

At session end, update [PROGRESS.md](PROGRESS.md):

```
Session end
- Done: [test name / function / paragraph drafted]
- Stuck on: [optional]
- Next 5-min task: [exact re-entry point]
```

**Minimum session (15 min):** One test, one function, or three sentences of blog draft. Counts.

---

## Re-entry ritual (after a gap)

Motivation fades when the project feels abstract. **Don’t restart with guilt — restart with curiosity.**

### Step 1 — Orient (2 min)

1. Open [PROGRESS.md](PROGRESS.md)
2. Read “Last session” and “Next 5-min task”
3. Run one verify command for your current phase

### Step 2 — Tangential spark (5–10 min)

Pick **one** random task from the bank below (or ask Cursor: *“Give me a tangential 5-min quiz for Phase [N]”*).

Do it **before** main work — it’s the ramp, not procrastination.

### Step 3 — Main work (15–40 min)

One goal only. Stop when the timer ends.

---

## Tangential task & quiz bank

Use when returning after a break. Related enough to reconnect; novel enough to feel fresh.

### Any phase

- Explain to an imaginary junior dev: *“Why doesn’t RAG embed a whole PDF as one vector?”* (out loud, 2 min)
- Add a weird `.txt` file to `data/sample_docs/` and re-run ingest — what breaks?
- Draw the pipeline on paper from memory (Docs → Chunker → … → LLM)
- Find one production RAG failure story online; write two sentences on how your design avoids it

### Phase 0–1 (Rust + chunking)

- What happens if you slice `"héllo"` at byte index 1? Why does the tutorial use ASCII?
- Without looking: write the `window_starts(26, 10, 8)` answer on paper; then test
- Quiz: `&str` vs `String` — when must `Chunk` own its text?

### Phase 2 (embeddings)

- Normalize `[3, 4]` by hand — what’s the length after?
- Two sentences: why dot product after normalize equals cosine similarity
- What embedding dimension would you pick for a clinical note search MVP and why?

### Phase 3 (retrieval)

- Write a query that should fail (no good hits) — what should the CLI show?
- Rank three fake snippets by hand; compare to your mock embedder
- One paragraph: how would citation-required answers change your `format_context`?

### Phase 4 (RAG)

- Tighten the system prompt in one sentence; run the same question twice — diff the answers
- What should happen when retrieval returns zero chunks?
- Role-play: a peer asks *“How do you know the LLM didn’t hallucinate?”* — draft your answer

### Phase 5 (Qdrant)

- Sketch: when does O(n) linear scan hurt? At 10k docs? 1M?
- Read one Qdrant doc page; note one feature you didn’t know
- Compare: embedded LanceDB vs server Qdrant for a solo dev tool

### Phase 6 (polish)

- Read your README as a stranger — what’s missing in the first 30 seconds?
- One paragraph: who is this repo **not** for?
- Outline the capstone blog post in five bullet headers only

---

## Blog series outline

Publish when each phase’s demo checkpoint passes — repo can stay “in progress” until Phase 6.

| # | Working title | Publish when |
|---|---------------|--------------|
| 1 | Why RAG doesn’t embed whole books — chunking in Rust | Phase 1 demo |
| 2 | From text to vectors: mock embedder before OpenAI | Phase 2 demo |
| 3 | Search without generation: retrieval before LLMs | Phase 3 demo |
| 4 | Grounding LLM answers: prompts and failure modes | Phase 4 demo |
| 5 | Outgrowing linear scan: Qdrant + traits | Phase 5 demo |
| 6 | Building RAG in Rust: retrospective | Phase 6 polish |

**Draft location suggestion:** `docs/blog/` (create when you publish post 1) or your external blog — link from README.

---

## Related docs

| Doc | Use |
|-----|-----|
| [PROGRESS.md](PROGRESS.md) | Checkboxes + last session notes |
| [STEPS.md](STEPS.md) | Implementation tasks per step |
| [RUST_WARMUP.md](RUST_WARMUP.md) | Warm-up concepts |
| [AI_LEARNING_WORKFLOW.md](AI_LEARNING_WORKFLOW.md) | AI tutor sessions & stuck ladder |

---

## When you finish

Celebrate explicitly — this is a multi-month build.

- [ ] All phase checkboxes in PROGRESS.md green
- [ ] Blog series linked from README
- [ ] One peer conversation: walk someone through a demo or post
- [ ] Note in PROGRESS.md: *“What I’d build next”* (e.g. citations, eval set, fastembed-rs)

Then decide: archive as reference, extend (citations, eval harness), or fork for a client-facing prototype.
