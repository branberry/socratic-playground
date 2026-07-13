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
        let mut documents: Vec<Document> = vec![];

        for chunk in chunks {
            let vector = embedder.embed(&chunk.text);
            let doc = Document {
                vector: vector,
                chunk: chunk,
            };

            documents.push(doc);
        }

        Self {
            documents: documents,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        chunk::TextChunker,
        embed::{Embedder, MockEmbedder},
        store::InMemoryVectorStore,
    };

    #[test]
    pub fn from_chunks_contains_documents() {
        let embedder = MockEmbedder::new(64);
        let chunker = TextChunker::default();
        let chunks = chunker
            .chunk_text("test.txt", "asdqweqweqweqweqweq")
            .expect("Could not chunk text");

        let store = InMemoryVectorStore::from_chunks(&embedder, chunks);
        assert!(store.documents.len() > 0)
    }
}
