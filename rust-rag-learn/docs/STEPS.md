# Step-by-step implementation guide

Work through these steps in order. **You create each new module yourself** — the repo starts with only `chunk.rs`.

After each step, run the verify commands listed below.

---

## Step 0 — Run the scaffold

**Concept:** Before writing code, understand the problem. RAG retrieves *passages*, not whole documents. Chunking is the first step.

**Your task:**
1. Read the sample docs in `data/sample_docs/`
2. Run ingest and inspect the output
3. In your own words: *why doesn't RAG embed an entire book as one vector?*

**Verify:**
```bash
cargo test
cargo run -- ingest
```

You should see file names and byte counts — chunking comes in Step 1.

---

## Step 1 — Chunking (`src/chunk.rs`)

**Concept:** Embedding models and LLMs have context limits. Chunking splits large documents into searchable passages small enough to embed meaningfully and fit in a prompt.

**Your task:** Build the entire `chunk.rs` module yourself — types first, then logic. The file has guided TODOs and commented-out tests; uncomment tests as you go.

### Substep 1a — `Chunk` struct

Define a struct with owned `String` fields: `id`, `source`, `text`.

Add derives — start with:
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
```

**Discussion:** Why `String` not `&str`? (You answered this in warm-up Ex 9.)

**Verify:** Uncomment `chunk_struct_can_be_constructed` in `chunk.rs`, run `cargo test chunk`.

### Substep 1b — `ChunkError`

Enum with `Io { path, source }` and `EmptyDocument { name }`. Use `thiserror` — copy the pattern from `ParseError` in `rust_warmup.rs`.

**Verify:** compiles; you'll test it in 1d.

### Substep 1c — `TextChunker`

Fields: `chunk_size`, `chunk_overlap`. Implement `Default` (500 / 50) and `new`.

**Verify:** compiles.

### Substep 1d — `chunk_file` + `chunk_text`

- `chunk_file`: read a path, derive source name, call `chunk_text`
- `chunk_text`: trim, reject empty, sliding window (reuse your Ex 7 loop)
- ids: `{source}#0`, `{source}#1`, ...

Wire up `lib.rs` re-exports and `main.rs` ingest (see TODOs there).

**Verify:**
```bash
cargo test chunk
cargo run -- ingest   # long docs should produce multiple chunks
```

**Stretch:** Split on paragraph boundaries first, then window within oversized paragraphs.

---

## Step 2 — Embeddings & similarity (create `src/embed.rs`)

**Concept:** Embeddings map text to fixed-size float vectors. Similar texts should land close together in vector space. **Cosine similarity** measures how aligned two vectors are (1.0 = same direction).

**Your task:** Create a new file `src/embed.rs` and wire it in `lib.rs`.

Build:
1. `normalize(vector: &mut [f32])` — L2-normalize in place
2. `cosine_similarity(a: &[f32], b: &[f32]) -> f32` — dot product (vectors must be same length)
3. `MockEmbedder { dimension: usize }` with an `embed(&self, text: &str) -> Vec<f32>` method
4. Hash-based mock: texts sharing words should score higher than unrelated texts

**Hint (only if stuck):** Tokenize on whitespace, hash each token into a bucket index, accumulate, then normalize.

**Discussion:** Why normalize before taking the dot product? What does dimension `128` vs `1536` mean?

**Verify:**
```bash
cargo test embed
```

Add unit tests: unit-length after normalize, identical texts → similarity ≈ 1.0.

---

## Step 3 — Vector store (create `src/store.rs`)

**Concept:** A vector database stores `(id, vector, metadata)` and answers: *give me the top-k vectors closest to this query vector.*

**Your task:** Create `src/store.rs` and wire it in `lib.rs`.

Build:
1. `Document { chunk: Chunk, vector: Vec<f32> }`
2. `ScoredDocument { document: Document, score: f32 }`
3. `InMemoryVectorStore` — holds a `Vec<Document>`, supports:
   - `from_chunks(embedder, chunks)` — embed all chunks, store them
   - `search(query_vector, top_k)` — score every doc with cosine similarity, return top-k

**Hint (only if stuck):** Linear scan is fine. Score all, sort descending, truncate to `top_k`.

**Discussion:** What's the time complexity of linear scan? When does in-memory stop being enough?

**Verify:**
```bash
cargo test store
```

---

## Step 4 — Retrieval (create `src/retrieve.rs`)

**Concept:** Retrieval = embed the query → search the store → format results as context for an LLM.

**Your task:** Create `src/retrieve.rs`, wire it in `lib.rs`, and add a `search` command to `main.rs`.

Build:
1. `RetrievalResult { query, hits: Vec<ScoredDocument> }`
2. A function that embeds a query string, searches the store, returns results
3. `format_context(results) -> String` — turn hits into a prompt-friendly block

**Hint (only if stuck):** Reuse your embedder + store from Steps 2–3. Format each hit with score, source, and text.

**Discussion:** What happens if you retrieve irrelevant chunks? How does that affect the final answer?

**Verify:**
```bash
cargo test retrieve
cargo run -- search "ownership and borrowing"
```

You'll need to extend `main.rs` to load docs, build the store, and run a query. Re-read how the old `search` subcommand worked if you saved it — or design your own.

**Stretch — Hybrid search:** Dense (embedding) search alone misses exact terms (`pgvector`, error codes, proper nouns). Production RAG often combines:

1. **Dense** — cosine / IP over embeddings (what you already have)
2. **Sparse / keyword** — BM25 (or a simple TF–IDF / term-overlap scorer) over chunk text
3. **Fusion** — merge ranked lists with **Reciprocal Rank Fusion (RRF)** (or a weighted sum of normalized scores)

Build (optional module or methods on the store/retriever):
- `keyword_search(query: &str, top_k) -> Vec<ScoredDocument>`
- `hybrid_search(query, query_vector, top_k)` — run both, fuse, return top-k
- Unit test: a query with a rare exact token ranks higher under hybrid than under dense-only

**Discussion:** When would BM25 beat embeddings? When would embeddings beat BM25?

**Verify (stretch):**
```bash
cargo test hybrid
cargo run -- search "ownership and borrowing"   # compare dense vs hybrid if you add a flag
```

---

## Step 5 — RAG (create `src/rag.rs`)

**Concept:** RAG = **R**etrieval + **A**ugmented prompt + **G**eneration. You retrieve context at query time and inject it into an LLM prompt.

**Your task:** Create `src/rag.rs`, wire it in `lib.rs`, and add an `ask` command to `main.rs`.

Build:
1. `build_prompt(question, context) -> String` — instruct the LLM to answer only from context
2. `RagPipeline` — retrieve → format context → generate answer
3. Start with a stub answer, then replace with a real HTTP call

**Real embeddings + LLM (recommended: Ollama):**

Add to `Cargo.toml`:
```toml
reqwest = { version = "0.12", features = ["json", "blocking"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

```bash
ollama pull nomic-embed-text
ollama pull llama3.2
ollama serve   # if not already running
```

Ollama embeddings: `POST http://localhost:11434/api/embeddings`

**Discussion:** What if the LLM ignores the context and hallucinates? How would you tighten the prompt?

**Verify:**
```bash
cargo run -- ask "What is a vector database?"
```

---

## Step 6 — Traits & production vector DB (stretch)

**Concept:** Traits let you swap implementations without changing callers. This is when abstractions pay off.

**Your task:**
1. Extract `Embedder`, `VectorStore`, and `Retriever` traits from your concrete code
2. Pick a production backend and implement `VectorStore` for it

| Database | Rust crate | Good for |
|----------|------------|----------|
| Qdrant | `qdrant-client` | Self-hosted, HNSW, filtering |
| LanceDB | `lancedb` | Embedded, columnar, local-first |
| pgvector | `sqlx` + Postgres | When you already use Postgres |

**Discussion:** What's the trade-off between HNSW (approximate) and linear scan (exact)?

---

## Step 7 — Evaluation harness (stretch)

**Concept:** You cannot improve what you do not measure. Production RAG teams keep a **golden set** of questions with expected sources (and optionally expected answers), then score retrieval and generation on every pipeline change.

**Your task:** Create `src/eval.rs` (or `tests/eval_*.rs`) and a small dataset under `data/eval/`.

Build:
1. **Golden set** — JSON/JSONL with at least ~10–20 items, e.g. `{ "question", "expected_source_ids": [...], "notes"? }`
2. **Retrieval metrics** — for each question, run search and compute:
   - **Hit@k** — did any expected source appear in top-k?
   - **MRR** (mean reciprocal rank) — how high was the first relevant hit?
3. **Generation metrics** (after Step 5 works) — pick one path:
   - **Deterministic:** assert answer contains a required substring / citation id
   - **LLM-as-judge (optional):** faithfulness / relevance rubric (calibrate against your own labels)
4. **CLI or test target** — `cargo test eval` or `cargo run -- eval` prints a summary table

**Hint (only if stuck):** Start with Hit@5 only. Add faithfulness later. Keep the golden set in git so regressions are visible in CI.

**Discussion:** Why gate on regression vs main rather than a hard “90%” threshold? What failure modes does Hit@k miss?

**Verify:**
```bash
cargo test eval
# or
cargo run -- eval
```

**Stretch further:** Wire failing prod/manual queries back into `data/eval/` (eval feedback loop). Optional: try [Ragas](https://github.com/explodinggradients/ragas)-style faithfulness/context-precision ideas in Rust or via a small Python sidecar.

---

## Tutor mode

When you ask for help, expect:
- Questions about *your* plan before hints
- Concepts explained, not full solutions pasted
- Progressive hints only if you're stuck
- Critical review of your code when you share it

You implement; the tutor guides. Disagreements welcome.

---

## Troubleshooting

| Problem | Check |
|---------|-------|
| `cargo run -- ingest` finds no files | Is `data/sample_docs/` present? Are files `.txt`? |
| Chunk count still 1 per file | Step 1 not done yet — check `chunk_text` |
| All search scores similar | Mock embedder may need word-overlap logic (Step 2) |
| LLM ignores context | Tighten prompt; retrieve more chunks; lower temperature |
| Exact term never ranks | Try hybrid search stretch (Step 4) — dense-only can miss rare tokens |
| “It feels better” but no numbers | Add Step 7 golden set + Hit@k before tuning more |

---

## Glossary

- **Embedding** — Fixed-size float vector representing meaning
- **Top-k** — The k highest-scoring results
- **Cosine similarity** — Angle between vectors (−1 to 1; 1 = identical direction)
- **HNSW** — Hierarchical graph index for fast approximate search
- **RAG** — Retrieve docs at query time, inject into LLM prompt
- **BM25** — Classic keyword ranking; strong on exact terms
- **Hybrid search** — Combine dense (vector) and sparse (keyword) retrieval
- **RRF** — Reciprocal Rank Fusion; merges ranked lists without score calibration
- **Hit@k** — Fraction of queries with a relevant doc in the top-k
- **MRR** — Mean reciprocal rank of the first relevant hit
- **Faithfulness** — Whether the answer is supported by retrieved context
