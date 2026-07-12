use crate::{chunk::Chunk, embed::MockEmbedder};

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
    pub fn from_chunks(embedder: MockEmbedder, chunks: &[f32]) {}
}
