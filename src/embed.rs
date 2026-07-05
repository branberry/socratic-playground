use std::hash::{DefaultHasher, Hash, Hasher};

pub fn normalize(vector: &mut [f32]) {}

pub fn cosine_similarity() {}

pub struct MockEmbedder {
    dimension: usize,
}

impl MockEmbedder {
    pub fn new(dim: usize) -> Self {
        Self { dimension: dim }
    }

    pub fn embed(&self, text: &str) -> Vec<f32> {
        let mut text_vector: Vec<f32> = vec![0.0; self.dimension];

        for word in text.split(" ") {
            let mut hasher = DefaultHasher::new();

            word.hash(&mut hasher);
            let index = hasher.finish() % self.dimension as u64;
            text_vector[index as usize] += 1.0;
        }

        return text_vector;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embed_returns_vector_of_requested_dimension() {
        let embedder = MockEmbedder::new(64);
        let vector = embedder.embed("the cat sat on the mat");
        assert_eq!(vector.len(), 64);
    }

    #[test]
    fn embed_is_deterministic() {
        let embedder = MockEmbedder::new(32);
        let a = embedder.embed("the quick brown fox");
        let b = embedder.embed("the quick brown fox");
        assert_eq!(a, b);
    }

    #[test]
    fn embed_different_text_produces_different_vector() {
        let embedder = MockEmbedder::new(32);
        let a = embedder.embed("the quick brown fox");
        let b = embedder.embed("completely unrelated content here");
        assert_ne!(a, b);
    }

    #[test]
    fn embed_produces_nonzero_vector_for_nonempty_text() {
        let embedder = MockEmbedder::new(16);
        let vector = embedder.embed("hello world");
        assert!(vector.iter().any(|&value| value != 0.0));
    }
}
