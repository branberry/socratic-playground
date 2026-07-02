//! # Step 1 — build this module yourself
//!
//! Nothing here is implemented for you. Work through substeps **1a → 1d** in order.
//! Uncomment the matching test when you're ready to verify each piece.
//!
//! ## `#[derive(...)]` — what and why
//!
//! Rust can auto-implement common traits on your types:
//!
//! | Trait | Enables |
//! |-------|---------|
//! | `Debug` | `println!("{:?}", chunk)` — essential while learning |
//! | `Clone` | `.clone()` — duplicate a value |
//! | `PartialEq` | `==` comparison (needed for tests) |
//! | `Eq` | stronger equality (safe for types with no floats) |
//!
//! Example: `#[derive(Debug, Clone, PartialEq, Eq)]` above a struct.
//!
//! Errors use `thiserror`: `#[derive(Debug, Error)]` on an enum plus `#[error("...")]`
//! on each variant — see `ParseError` in `rust_warmup.rs` for a working example.
//!
//! ## Substeps
//!
//! **1a** — `Chunk` struct (owned `String` fields: `id`, `source`, `text`)
//! **1b** — `ChunkError` enum (`Io`, `EmptyDocument`)
//! **1c** — `TextChunker` struct + `Default` + `new`
//! **1d** — `chunk_file` + `chunk_text` (reuse your `split_windows` loop from warm-up Ex 7)
//!
//! When 1d is done: `cargo test chunk` and `cargo run -- ingest`

// ---------------------------------------------------------------------------
// 1a — Chunk
// ---------------------------------------------------------------------------
//
// A passage of text ready for embedding. All fields are owned `String` so chunks
// can live in a `Vec` after the original file buffer is gone.
//
// TODO(step-1a): Define `Chunk` with pub fields: id, source, text
// TODO(step-1a): Add #[derive(Debug, Clone, PartialEq, Eq)]
struct Chunk {
    id: String,
    source: String,
    text: String,
}

// ---------------------------------------------------------------------------
// 1b — ChunkError
// ---------------------------------------------------------------------------
//
// TODO(step-1b): enum ChunkError with:
//   - Io { path: String, source: std::io::Error }
//   - EmptyDocument { name: String }
// TODO(step-1b): #[derive(Debug, Error)] from thiserror, #[error(...)] on variants
//
// use thiserror::Error;
// use std::path::Path;

// ---------------------------------------------------------------------------
// 1c — TextChunker
// ---------------------------------------------------------------------------
//
// TODO(step-1c): struct with chunk_size: usize, chunk_overlap: usize
// TODO(step-1c): impl Default → chunk_size: 500, chunk_overlap: 50
// TODO(step-1c): impl fn new(chunk_size, chunk_overlap) -> Self

// ---------------------------------------------------------------------------
// 1d — chunking logic
// ---------------------------------------------------------------------------
//
// TODO(step-1d): chunk_file(&self, path: &Path) -> Result<Vec<Chunk>, ChunkError>
//   - read file, derive source from file_name, delegate to chunk_text
//   - map io errors to ChunkError::Io
//
// TODO(step-1d): chunk_text(&self, source: &str, text: &str) -> Result<Vec<Chunk>, ChunkError>
//   - trim, reject empty → EmptyDocument
//   - sliding window: window = chunk_size, step = chunk_size - chunk_overlap
//   - ids: "{source}#0", "{source}#1", ...
//
// Wire main.rs when done (see TODO in src/main.rs).

#[cfg(test)]
mod tests {
    // Uncomment each test as you complete the matching substep.

    // use super::*;

    // #[test]
    // fn chunk_struct_can_be_constructed() {
    //     let chunk = Chunk {
    //         id: "doc.txt#0".into(),
    //         source: "doc.txt".into(),
    //         text: "hello".into(),
    //     };
    //     assert_eq!(chunk.id, "doc.txt#0");
    //     assert!(format!("{:?}", chunk).contains("hello"));
    // }

    // #[test]
    // fn chunk_text_rejects_empty_input() {
    //     let chunker = TextChunker::default();
    //     let err = chunker.chunk_text("doc.txt", "   ").unwrap_err();
    //     assert!(matches!(err, ChunkError::EmptyDocument { .. }));
    // }

    // #[test]
    // fn chunk_text_splits_long_documents() {
    //     let chunker = TextChunker::new(10, 2);
    //     let text = "abcdefghijklmnopqrstuvwxyz";
    //     let chunks = chunker.chunk_text("doc.txt", text).unwrap();
    //     assert!(chunks.len() > 1);
    //     assert!(chunks.iter().all(|c| c.text.len() <= 10));
    // }
}
