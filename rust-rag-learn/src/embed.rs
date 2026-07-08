use std::hash::{DefaultHasher, Hash, Hasher};

pub fn normalize(vector: &mut Vec<f32>) {
    let l2_norm: f32 = vector.iter().map(|x| x * x).sum::<f32>().sqrt();

    if l2_norm == 0.0 {
        return;
    }
    for i in 0..vector.len() {
        vector[i] = vector[i] / l2_norm;
    }
}

pub fn cosine_similarity(vec_a: Vec<f32>, vec_b: Vec<f32>) {
    let numerator = vec_a.iter().zip(vec_b).map(|(a, b)| a * b).sum::<f32>();
    let l2_norm_a = vec_a.iter().map(|x| x * x).sum::<f32>().sqrt();
}

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

    fn magnitude(v: &[f32]) -> f32 {
        v.iter().map(|x| x * x).sum::<f32>().sqrt()
    }

    #[test]
    fn normalize_produces_unit_length() {
        let mut v = vec![3.0, 4.0];
        normalize(&mut v);
        let len = magnitude(&v);
        assert!((len - 1.0).abs() < 1e-5, "expected unit length, got {len}");
    }

    #[test]
    fn normalize_known_vector() {
        let mut v = vec![3.0, 4.0];
        normalize(&mut v);
        assert!((v[0] - 0.6).abs() < 1e-5);
        assert!((v[1] - 0.8).abs() < 1e-5);
    }

    #[test]
    fn normalize_preserves_direction() {
        let mut v = vec![2.0, 6.0, 4.0];
        let ratio_before = v[1] / v[0];
        normalize(&mut v);
        let ratio_after = v[1] / v[0];
        assert!((ratio_before - ratio_after).abs() < 1e-5);
    }

    #[test]
    fn normalize_empty_vector_does_not_panic() {
        let mut v: Vec<f32> = vec![];
        normalize(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn normalize_empty_vector_does_not_panic_on_all_zeroes() {
        let mut v: Vec<f32> = vec![0.0, 0.0, 0.0];
        normalize(&mut v);
        assert!(v.iter().sum::<f32>() == 0.0);
    }
}
