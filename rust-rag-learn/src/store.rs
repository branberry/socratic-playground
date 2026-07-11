use crate::chunk::Chunk;

pub struct Document {
    chunk: Chunk,
    vector: Vec<f32>,
}
pub struct ScoredDocument {
    document: Document,
    pub score: f32,
}
