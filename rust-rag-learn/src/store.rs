use crate::chunk::Chunk;

pub struct Document {
    pub chunk: Chunk,
    pub vector: Vec<f32>,
}
pub struct ScoredDocument {
    pub document: Document,
    pub score: f32,
}

pub struct InMemoryVectorStore {
    pub store: Vec<Document>,
}
