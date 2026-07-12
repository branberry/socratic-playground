use std::hash::{DefaultHasher, Hash, Hasher};

pub fn normalize(vector: &mut [f32]) {
    let l2_norm: f32 = vector.iter().map(|x| x * x).sum::<f32>().sqrt();

    if l2_norm == 0.0 {
        return;
    }
    for i in 0..vector.len() {
        vector[i] = vector[i] / l2_norm;
    }
}

pub fn cosine_similarity(vec_a: &[f32], vec_b: &[f32]) -> f32 {
    let numerator = vec_a.iter().zip(vec_b).map(|(a, b)| a * b).sum::<f32>();
    let l2_norm_a = vec_a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let l2_norm_b = vec_b.iter().map(|x| x * x).sum::<f32>().sqrt();

    numerator / (l2_norm_a * l2_norm_b)
}

pub trait Embedder {
    fn new(dim: usize) -> Self;
    fn embed(&self, text: &str) -> Vec<f32>;
}
pub struct MockEmbedder {
    dimension: usize,
}

impl Embedder for MockEmbedder {
    fn new(dim: usize) -> Self {
        Self { dimension: dim }
    }

    fn embed(&self, text: &str) -> Vec<f32> {
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

    #[test]
    fn cosine_similarity_identical_vectors_is_one() {
        let v = vec![1.0, 2.0, 3.0];
        let score = cosine_similarity(&v, &v);
        assert!((score - 1.0).abs() < 1e-5, "expected 1.0, got {score}");
    }

    #[test]
    fn cosine_similarity_orthogonal_vectors_is_zero() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        let score = cosine_similarity(&a, &b);
        assert!(score.abs() < 1e-5, "expected 0.0, got {score}");
    }

    #[test]
    fn cosine_similarity_opposite_vectors_is_negative_one() {
        let a = vec![1.0, 0.0];
        let b = vec![-1.0, 0.0];
        let score = cosine_similarity(&a, &b);
        assert!((score + 1.0).abs() < 1e-5, "expected -1.0, got {score}");
    }

    #[test]
    fn cosine_similarity_same_direction_different_magnitude_is_one() {
        let a = vec![3.0, 4.0];
        let b = vec![6.0, 8.0];
        let score = cosine_similarity(&a, &b);
        assert!((score - 1.0).abs() < 1e-5, "expected 1.0, got {score}");
    }

    #[test]
    fn cosine_similarity_normalized_vectors_is_dot_product() {
        let mut a = vec![3.0, 4.0];
        let mut b = vec![5.0, 12.0];
        normalize(&mut a);
        normalize(&mut b);
        let score = cosine_similarity(&a, &b);
        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        assert!((score - dot).abs() < 1e-5);
    }

    #[test]
    fn similar_texts_score_higher_than_unrelated() {
        let embedder = MockEmbedder::new(32);
        let cat = embedder.embed("the cat sat on the mat");
        let dog = embedder.embed("the dog sat on the log");
        let unrelated = embedder.embed("quantum physics equations");
        let similar_score = cosine_similarity(&cat, &dog);
        let unrelated_score = cosine_similarity(&cat, &unrelated);
        assert!(
            similar_score > unrelated_score,
            "expected similar ({similar_score}) > unrelated ({unrelated_score})"
        );
    }
}
