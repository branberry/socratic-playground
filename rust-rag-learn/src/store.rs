use crate::{
    chunk::Chunk,
    embed::{cosine_similarity, Embedder},
};

pub struct Document {
    pub chunk: Chunk,
    pub vector: Vec<f32>,
}
#[derive(Clone)]
pub struct ScoredDocument<'a> {
    pub document: &'a Document,
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

    pub fn search<'a>(&self, query_vector: Vec<f32>, top_k: usize) -> Vec<ScoredDocument<'_>> {
        let mut scored_docs: Vec<ScoredDocument> = self
            .documents
            .iter()
            .map(|doc| ScoredDocument {
                score: cosine_similarity(&query_vector, &doc.vector),
                document: doc,
            })
            .collect();
        scored_docs.sort_by(|a, b| b.score.total_cmp(&a.score));
        let docs_num = top_k.min(scored_docs.len());
        return scored_docs[..docs_num].to_vec();
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

    #[test]
    pub fn search_returns_at_most_top_k_results() {
        let embedder = MockEmbedder::new(64);
        let chunker = TextChunker::default();
        let chunks: Vec<crate::chunk::Chunk> = ["cat cat cat", "dog dog dog", "fish fish fish"]
            .into_iter()
            .enumerate()
            .flat_map(|(i, t)| {
                chunker
                    .chunk_text(&format!("doc{i}.txt"), t)
                    .expect("Could not chunk text")
            })
            .collect();
        let store = InMemoryVectorStore::from_chunks(&embedder, chunks);
        assert!(store.documents.len() >= 3, "need >=3 docs for this test");

        let query = embedder.embed("cat");
        let hits = store.search(query, 2);
        assert!(
            hits.len() <= 2,
            "search should return at most top_k, got {}",
            hits.len()
        );
        assert!(!hits.is_empty(), "search should return some hits");
    }

    #[test]
    pub fn search_results_sorted_by_score_descending() {
        let embedder = MockEmbedder::new(64);
        let chunker = TextChunker::default();
        let chunks: Vec<crate::chunk::Chunk> = ["cat cat cat", "dog dog dog", "fish fish fish"]
            .into_iter()
            .enumerate()
            .flat_map(|(i, t)| {
                chunker
                    .chunk_text(&format!("doc{i}.txt"), t)
                    .expect("Could not chunk text")
            })
            .collect();
        let store = InMemoryVectorStore::from_chunks(&embedder, chunks);

        let query = embedder.embed("cat");
        let hits = store.search(query, 10);
        for window in hits.windows(2) {
            assert!(
                window[0].score >= window[1].score,
                "hits must be sorted by score descending; saw {} then {}",
                window[0].score,
                window[1].score
            );
        }
    }

    #[test]
    pub fn search_top_hit_matches_query_text() {
        let embedder = MockEmbedder::new(64);
        let chunker = TextChunker::default();
        let chunks: Vec<crate::chunk::Chunk> = ["dog dog dog", "cat cat cat", "fish fish fish"]
            .into_iter()
            .enumerate()
            .flat_map(|(i, t)| {
                chunker
                    .chunk_text(&format!("doc{i}.txt"), t)
                    .expect("Could not chunk text")
            })
            .collect();
        let store = InMemoryVectorStore::from_chunks(&embedder, chunks);

        let query = embedder.embed("cat cat cat");
        let hits = store.search(query, 1);
        assert_eq!(hits.len(), 1);
        assert!(
            hits[0].document.chunk.text.contains("cat"),
            "top hit should be the cat doc, got: {:?}",
            hits[0].document.chunk.text
        );
    }

    #[test]
    pub fn search_identical_query_vector_scores_one() {
        let embedder = MockEmbedder::new(64);
        let chunker = TextChunker::default();
        let chunks: Vec<crate::chunk::Chunk> = ["cat cat cat", "dog dog dog"]
            .into_iter()
            .enumerate()
            .flat_map(|(i, t)| {
                chunker
                    .chunk_text(&format!("doc{i}.txt"), t)
                    .expect("Could not chunk text")
            })
            .collect();
        let store = InMemoryVectorStore::from_chunks(&embedder, chunks);

        let target = &store.documents[0];
        let query = target.vector.clone();
        let hits = store.search(query, 1);
        assert_eq!(hits.len(), 1);
        assert!(
            (hits[0].score - 1.0).abs() < 1e-4,
            "identical vector should score ~1.0, got {}",
            hits[0].score
        );
    }

    #[test]
    pub fn search_top_k_larger_than_store_returns_all() {
        let embedder = MockEmbedder::new(64);
        let chunker = TextChunker::default();
        let chunks: Vec<crate::chunk::Chunk> = ["cat cat cat", "dog dog dog"]
            .into_iter()
            .enumerate()
            .flat_map(|(i, t)| {
                chunker
                    .chunk_text(&format!("doc{i}.txt"), t)
                    .expect("Could not chunk text")
            })
            .collect();
        let store = InMemoryVectorStore::from_chunks(&embedder, chunks);
        let n = store.documents.len();
        let n = store.documents.len();

        let query = embedder.embed("anything");
        let hits = store.search(query, 100);
        assert_eq!(
            hits.len(),
            n,
            "top_k > doc count should return all {} docs, got {}",
            n,
            hits.len()
        );
    }

    #[test]
    pub fn search_empty_store_returns_empty() {
        let embedder = MockEmbedder::new(64);
        let store = InMemoryVectorStore { documents: vec![] };

        let query = embedder.embed("anything");
        let hits = store.search(query, 5);
        assert!(hits.is_empty(), "empty store should return no hits");
    }
}
