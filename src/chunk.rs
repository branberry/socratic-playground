use std::path::Path;

use thiserror::Error;

/// A slice of source text ready for embedding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    pub id: String,
    pub source: String,
    pub text: String,
}

#[derive(Debug, Error)]
pub enum ChunkError {
    #[error("failed to read file `{path}`: {source}")]
    Io { path: String, source: std::io::Error },
    #[error("empty document: {name}")]
    EmptyDocument { name: String },
}

pub struct TextChunker {
    pub chunk_size: usize,
    pub chunk_overlap: usize,
}

impl Default for TextChunker {
    fn default() -> Self {
        Self {
            chunk_size: 500,
            chunk_overlap: 50,
        }
    }
}

impl TextChunker {
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> Self {
        Self {
            chunk_size,
            chunk_overlap,
        }
    }

    pub fn chunk_file(&self, path: &Path) -> Result<Vec<Chunk>, ChunkError> {
        let source = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        let text = std::fs::read_to_string(path).map_err(|err| ChunkError::Io {
            path: path.display().to_string(),
            source: err,
        })?;

        self.chunk_text(&source, &text)
    }

    pub fn chunk_text(&self, source: &str, text: &str) -> Result<Vec<Chunk>, ChunkError> {
        // TODO(step-1): Implement sliding-window chunking.
        //
        // Goals:
        // - Split `text` into windows of `self.chunk_size` characters.
        // - Advance by `chunk_size - chunk_overlap` each step.
        // - Assign ids like `{source}#0`, `{source}#1`, ...
        // - Return `ChunkError::EmptyDocument` when text is empty/whitespace.
        //
        // Hint: iterate with `(start..end).step_by(chunk_size - overlap)`.
        // For now, return the whole document as a single chunk so the project compiles.

        let trimmed = text.trim();
        if trimmed.is_empty() {
            return Err(ChunkError::EmptyDocument {
                name: source.to_string(),
            });
        }

        Ok(vec![Chunk {
            id: format!("{source}#0"),
            source: source.to_string(),
            text: trimmed.to_string(),
        }])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_text_rejects_empty_input() {
        let chunker = TextChunker::default();
        let err = chunker.chunk_text("doc.txt", "   ").unwrap_err();
        assert!(matches!(err, ChunkError::EmptyDocument { .. }));
    }

    // TODO(step-1): Uncomment and make pass after implementing sliding-window chunking.
    
    #[test]
    fn chunk_text_splits_long_documents() {
        let chunker = TextChunker::new(10, 2);
        let text = "abcdefghijklmnopqrstuvwxyz";
        let chunks = chunker.chunk_text("doc.txt", text).unwrap();
        assert!(chunks.len() > 1);
        assert!(chunks.iter().all(|c| c.text.len() <= 10));
    }
}
