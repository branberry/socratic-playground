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

You should see one chunk per file today (placeholder behavior). That's intentional.

---

## Step 1 — Chunking (`src/chunk.rs`)

**Concept:** Embedding models and LLMs have context limits. Chunking splits large documents into searchable passages small enough to embed meaningfully and fit in a prompt.

**Your task:** Implement sliding-window chunking in `TextChunker::chunk_text`.

Requirements:
- Window size = `self.chunk_size` characters
- Step size = `chunk_size - chunk_overlap`
- Stable ids: `{source}#0`, `{source}#1`, ...
- Reject empty/whitespace-only input

**Hint (only if stuck):** Think about iterating start indices with `step_by(chunk_size - chunk_overlap)`, slicing `text[start..end]`, and stopping when you've covered the full string.

**Discussion:** What happens at document boundaries? Should the last chunk be shorter than `chunk_size`?

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

---

## Glossary

- **Embedding** — Fixed-size float vector representing meaning
- **Top-k** — The k highest-scoring results
- **Cosine similarity** — Angle between vectors (−1 to 1; 1 = identical direction)
- **HNSW** — Hierarchical graph index for fast approximate search
- **RAG** — Retrieve docs at query time, inject into LLM prompt
