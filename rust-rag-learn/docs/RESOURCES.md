# Reading list — rust-rag-learn track

Curated papers, docs, and articles aligned with [STEPS.md](STEPS.md). Read **one or two per step** — you do not need everything before coding.

**Legend:** 📄 paper · 📘 docs · 📝 article/blog

---

## Step 0 — Why RAG?

| Resource | Type | Why read it |
|----------|------|-------------|
| [Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks](https://arxiv.org/abs/2005.11401) (Lewis et al., NeurIPS 2020) | 📄 | The paper that named and formalized RAG: retrieve passages, then generate. |
| [What is RAG?](https://docs.llamaindex.ai/en/stable/getting_started/concepts/) (LlamaIndex) | 📘 | Short conceptual map: indexing vs query time. |
| [Retrieval Augmented Generation (RAG) Explained](https://www.pinecone.io/learn/retrieval-augmented-generation/) (Pinecone) | 📝 | Accessible pipeline overview with diagrams. |

---

## Step 1 — Chunking

| Resource | Type | Why read it |
|----------|------|-------------|
| [Text splitters](https://python.langchain.com/docs/concepts/text_splitters/) (LangChain) | 📘 | Recursive vs fixed-size splitting; overlap rationale. |
| [Best Chunking Strategies for RAG (2026)](https://www.firecrawl.dev/blog/best-chunking-strategies-rag) | 📝 | Compares fixed, semantic, and page-level strategies with trade-offs. |
| [Lost in the Middle: How Language Models Use Long Contexts](https://arxiv.org/abs/2307.03172) (Liu et al., 2023) | 📄 | Why *which* passage you retrieve matters — models favor start/end of context. |
| [Chunking strategies](https://docs.llamaindex.ai/en/stable/optimizing/basic_strategies/basic_strategies/) (LlamaIndex) | 📘 | Practical defaults (size, overlap, structure-aware splits). |

---

## Step 2 — Embeddings & similarity

| Resource | Type | Why read it |
|----------|------|-------------|
| [Sentence-BERT: Sentence Embeddings using Siamese BERT-Networks](https://arxiv.org/abs/1908.10084) (Reimers & Gurevych, 2019) | 📄 | Classic dense text embeddings; cosine similarity as the metric. |
| [Dense Passage Retrieval for Open-Domain Question Answering](https://arxiv.org/abs/2004.04906) (Karpukhin et al., EMNLP 2020) | 📄 | Dual-encoder retrieval; dot product / MIPS over passage vectors. |
| [Embeddings guide](https://platform.openai.com/docs/guides/embeddings) (OpenAI) | 📘 | What dimensions mean; same model for docs and queries. |
| [Cosine similarity](https://en.wikipedia.org/wiki/Cosine_similarity) (Wikipedia) | 📘 | Math behind normalize → dot product. |

---

## Step 3 — Vector store

| Resource | Type | Why read it |
|----------|------|-------------|
| [Billion-scale similarity search with GPUs](https://arxiv.org/abs/1702.08734) (Johnson et al., 2017) — FAISS | 📄 | How production systems index and search billions of vectors. |
| [Efficient and robust approximate nearest neighbor search using HNSW graphs](https://arxiv.org/abs/1603.09320) (Malkov & Yashunin, 2018) | 📄 | The ANN algorithm behind most vector DBs (including Qdrant). |
| [Vector database](https://www.pinecone.io/learn/vector-database/) (Pinecone) | 📝 | `(id, vector, metadata)` mental model; exact vs approximate search. |
| [pgvector](https://github.com/pgvector/pgvector) (GitHub README) | 📘 | Postgres extension when you already have a SQL stack. |
| [FLOAT_ORD.md](FLOAT_ORD.md) (this repo) | 📘 | Why `f32` needs `total_cmp` / `partial_cmp` when sorting scores. |

---

## Step 4 — Retrieval & hybrid search

| Resource | Type | Why read it |
|----------|------|-------------|
| [Dense Passage Retrieval](https://arxiv.org/abs/2004.04906) (Karpukhin et al.) | 📄 | Dense retrieval baseline your mock embedder approximates in spirit. |
| [The Probabilistic Relevance Framework: BM25 and Beyond](https://www.staff.city.ac.uk/~sbrp622/papers/foundations_bm25_review.pdf) (Robertson & Zaragoza, 2009) | 📄 | Classic sparse / keyword ranking — still strong on exact terms. |
| [Reciprocal Rank Fusion outperforms Condorcet and individual Rank Learning Methods](https://plg.uwaterloo.ca/~gvcormac/cormacksigir09-rrf.pdf) (Cormack et al., SIGIR 2009) | 📄 | How to merge dense + sparse ranked lists (RRF stretch). |
| [Hybrid search explained](https://www.elastic.co/search-labs/blog/hybrid-search-elasticsearch) (Elastic Search Labs) | 📝 | Dense + BM25 in production; when each wins. |

---

## Step 5 — RAG (retrieve + generate)

| Resource | Type | Why read it |
|----------|------|-------------|
| [Retrieval-Augmented Generation](https://arxiv.org/abs/2005.11401) (Lewis et al.) | 📄 | End-to-end: retriever + generator; BM25 vs dense ablations. |
| [Lost in the Middle](https://arxiv.org/abs/2307.03172) (Liu et al.) | 📄 | Prompt layout: put the best chunks where the model actually reads them. |
| [Prompt engineering](https://platform.openai.com/docs/guides/prompt-engineering) (OpenAI) | 📘 | Grounding instructions (“answer only from context”). |
| [Ollama API — embeddings](https://github.com/ollama/ollama/blob/main/docs/api.md#generate-embeddings) | 📘 | Local embeddings for Step 5 wiring. |

---

## Step 6 — Traits & production backends

| Resource | Type | Why read it |
|----------|------|-------------|
| [Qdrant documentation](https://qdrant.tech/documentation/) | 📘 | HNSW, filtering, self-hosted vector search. |
| [LanceDB documentation](https://lancedb.github.io/lancedb/) | 📘 | Embedded, local-first columnar vector store. |
| [HNSW paper](https://arxiv.org/abs/1603.09320) (Malkov & Yashunin) | 📄 | Approximate vs exact search trade-off. |
| [The Rust Book — Traits](https://doc.rust-lang.org/book/ch10-02-traits.html) | 📘 | Why `Embedder` / `VectorStore` traits pay off here. |

---

## Step 7 — Evaluation

| Resource | Type | Why read it |
|----------|------|-------------|
| [RAGAS: Automated Evaluation of Retrieval Augmented Generation](https://arxiv.org/abs/2309.15217) (Es et al., 2023) | 📄 | Faithfulness, answer relevance, context precision/recall metrics. |
| [BEIR: A Heterogenous Benchmark for Zero-shot Evaluation of IR Models](https://arxiv.org/abs/2104.08663) (Thakur et al., 2021) | 📄 | Retrieval benchmarks; BM25 vs dense vs hybrid baselines. |
| [MTEB: Massive Text Embedding Benchmark](https://arxiv.org/abs/2210.07316) (Muennighoff et al., 2022) | 📄 | Embedding model quality across many tasks. |
| [Ragas documentation](https://docs.ragas.io/) | 📘 | Practical eval loop ideas (golden set, CI regression). |

---

## Cross-cutting (anytime)

| Resource | Type | Why read it |
|----------|------|-------------|
| [Building Production RAG Pipelines](https://backendbytes.com/articles/production-rag-pipelines-go/) | 📝 | Chunking → embed → store → retrieve at scale. |
| [RAG Pipeline Production Architecture 2026](https://superml.dev/rag-pipeline-production-architecture-2026) | 📝 | Hybrid search, reranking, eval from a systems lens. |
| [The Rust Book](https://doc.rust-lang.org/book/) | 📘 | Ownership, borrowing, traits — maps directly to your store/search bugs. |

---

## Suggested reading order (minimal)

If you want the smallest high-signal path:

1. Step 0: Lewis RAG paper (intro + figures)
2. Step 1: LangChain text splitters + Firecrawl chunking post
3. Step 2: Sentence-BERT + OpenAI embeddings guide
4. Step 3: Pinecone vector DB post + HNSW paper (skim)
5. Step 4: DPR paper + Elastic hybrid search post
6. Step 5: Lewis RAG (generation section) + Lost in the Middle
7. Step 6: Qdrant or LanceDB docs (pick your backend)
8. Step 7: RAGAS paper + BEIR abstract

Update [PROGRESS.md](PROGRESS.md) when you finish a resource — even a skim counts.
