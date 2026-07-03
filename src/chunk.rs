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

use std::fs;
use std::path::Path;
use thiserror::Error;

// ---------------------------------------------------------------------------
// 1a — Chunk
// ---------------------------------------------------------------------------
//
// A passage of text ready for embedding. All fields are owned `String` so chunks
// can live in a `Vec` after the original file buffer is gone.
//
// TODO(step-1a): Define `Chunk` with pub fields: id, source, text
// TODO(step-1a): Add #[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    pub id: String,
    pub source: String,
    pub text: String,
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

#[derive(Debug, Error)]
pub enum ChunkError {
    #[error("The file could not be be read")]
    Io {
        path: String,
        source: std::io::Error,
    },
    #[error("The document is empty")]
    EmptyDocument { name: String },
}

// ---------------------------------------------------------------------------
// 1c — TextChunker
// ---------------------------------------------------------------------------
//
// TODO(step-1c): struct with chunk_size: usize, chunk_overlap: usize
// TODO(step-1c): impl Default → chunk_size: 500, chunk_overlap: 50
// TODO(step-1c): impl fn new(chunk_size, chunk_overlap) -> Self

pub struct TextChunker {
    chunk_size: usize,
    chunk_overlap: usize,
}

impl TextChunker {
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> Self {
        Self {
            chunk_size: chunk_size,
            chunk_overlap: chunk_overlap,
        }
    }
}

impl Default for TextChunker {
    fn default() -> Self {
        Self {
            chunk_size: 500,
            chunk_overlap: 50,
        }
    }
}
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
impl TextChunker {
    pub fn chunk_file(&self, path: &Path) -> Result<Vec<Chunk>, ChunkError> {
        let source = path
            .file_name()
            .expect("File or directory does not exist")
            .to_str()
            .expect("should not be empty");

        match fs::read_to_string(path) {
            Ok(text) => {
                return self.chunk_text(source, &text);
            }
            Err(e) => {
                return Err(ChunkError::Io {
                    path: source.to_string(),
                    source: e,
                })
            }
        }
    }
    pub fn chunk_text(&self, source: &str, text: &str) -> Result<Vec<Chunk>, ChunkError> {
        let mut chunks: Vec<Chunk> = vec![];
        let text_len = text.len();
        if text.trim().is_empty() {
            return Err(ChunkError::EmptyDocument {
                name: source.to_string(),
            });
        }

        let mut id_index = 0;
        for i in (0..text_len).step_by(self.chunk_overlap) {
            if i + self.chunk_size >= text_len {
                let chunk = Chunk {
                    id: format!("{source}#{id_index}"),
                    source: source.to_string(),
                    text: text[i..].to_string(),
                };

                chunks.push(chunk);
            } else {
                let chunk = Chunk {
                    id: format!("{source}#{id_index}"),
                    source: source.to_string(),
                    text: text[i..i + self.chunk_size].to_string(),
                };

                chunks.push(chunk);
            }

            id_index += 1;
        }

        Ok(chunks)
    }
}

#[cfg(test)]
mod tests {
    // Uncomment each test as you complete the matching substep.

    use super::*;

    #[test]
    fn chunk_struct_can_be_constructed() {
        let chunk = Chunk {
            id: "doc.txt#0".into(),
            source: "doc.txt".into(),
            text: "hello".into(),
        };
        assert_eq!(chunk.id, "doc.txt#0");
        assert!(format!("{:?}", chunk).contains("hello"));
    }

    #[test]
    fn chunk_text_rejects_empty_input() {
        let chunker = TextChunker::default();
        let err = chunker.chunk_text("doc.txt", "   ").unwrap_err();
        assert!(matches!(err, ChunkError::EmptyDocument { .. }));
    }

    #[test]
    fn chunk_text_rejects_empty_string() {
        let chunker = TextChunker::default();
        let err = chunker.chunk_text("doc.txt", "").unwrap_err();
        assert!(matches!(err, ChunkError::EmptyDocument { name } if name == "doc.txt"));
    }

    #[test]
    fn chunk_text_splits_long_documents() {
        let chunker = TextChunker::new(10, 2);
        let text = "abcdefghijklmnopqrstuvwxyz";
        let chunks = chunker.chunk_text("doc.txt", text).unwrap();
        assert!(chunks.len() > 1);
        assert!(chunks.iter().all(|c| c.text.len() <= 10));
    }

    #[test]
    fn chunk_text_exact_windows_and_overlap() {
        let chunker = TextChunker::new(10, 2);
        let chunks = chunker
            .chunk_text("doc.txt", "abcdefghijklmnopqrstuvwxyz")
            .unwrap();

        assert_eq!(chunks.len(), 4);
        assert_eq!(chunks[0].text, "abcdefghij");
        assert_eq!(chunks[1].text, "ijklmnopqr");
        assert_eq!(chunks[2].text, "qrstuvwxyz");
        assert_eq!(chunks[3].text, "yz");
    }

    #[test]
    fn chunk_text_single_chunk_when_text_fits_window() {
        let chunker = TextChunker::new(500, 50);
        let chunks = chunker.chunk_text("short.txt", "hello world").unwrap();

        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].text, "hello world");
        assert_eq!(chunks[0].id, "short.txt#0");
    }

    #[test]
    fn chunk_text_assigns_sequential_ids() {
        let chunker = TextChunker::new(4, 1);
        let chunks = chunker.chunk_text("src.md", "abcdefgh").unwrap();

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].id, "src.md#0");
        assert_eq!(chunks[1].id, "src.md#1");
        assert_eq!(chunks[2].id, "src.md#2");
    }

    #[test]
    fn chunk_text_propagates_source_to_every_chunk() {
        let chunker = TextChunker::new(3, 0);
        let chunks = chunker.chunk_text("notes.txt", "abcdef").unwrap();

        assert!(chunks.len() >= 2);
        assert!(chunks.iter().all(|c| c.source == "notes.txt"));
    }

    #[test]
    fn chunk_text_last_chunk_can_be_shorter_than_window() {
        let chunker = TextChunker::new(10, 2);
        let chunks = chunker
            .chunk_text("doc.txt", "abcdefghijklmnopqrstuvwxyz")
            .unwrap();

        let last = chunks.last().expect("should have chunks");
        assert!(last.text.len() < 10);
        assert_eq!(last.text, "yz");
    }

    #[test]
    fn chunk_text_step_is_chunk_size_minus_overlap() {
        // window=5, overlap=1 → step=4 → two chunks with 1-char overlap
        let chunker = TextChunker::new(5, 1);
        let chunks = chunker.chunk_text("doc.txt", "abcdefgh").unwrap();

        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].text, "abcde");
        assert_eq!(chunks[1].text, "efgh");
    }

    #[test]
    fn chunk_text_zero_overlap_produces_adjacent_non_overlapping_windows() {
        let chunker = TextChunker::new(4, 0);
        let chunks = chunker.chunk_text("doc.txt", "abcdefgh").unwrap();

        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].text, "abcd");
        assert_eq!(chunks[1].text, "efgh");
    }
}
