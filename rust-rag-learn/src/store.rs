use crate::{chunk::Chunk, embed::Embedder};

pub struct Document {
    pub chunk: Chunk,
    pub vector: Vec<f32>,
}
pub struct ScoredDocument {
    pub document: Document,
    pub score: f32,
}

pub struct InMemoryVectorStore {
    pub documents: Vec<Document>,
}

impl InMemoryVectorStore {
    pub fn from_chunks<TEmbedder: Embedder>(embedder: &TEmbedder, chunks: Vec<Chunk>) -> Self {
        Self {
            documents: Vec::new(),
        }
    }
}
