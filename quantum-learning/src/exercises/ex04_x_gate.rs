//! Exercise 4 — Pauli-X flips |0⟩ → |1⟩
//!
//! Goal: apply X and read occupation probabilities.
//!
//! Hints:
//! - Reuse patterns from ex02 and ex03
//! - After X on |0⟩, P(|0⟩) should be ≈ 0 and P(|1⟩) should be ≈ 1

use roqoqo::Circuit;

/// Circuit that applies X to qubit 0.
pub fn x_flip_circuit() -> Circuit {
    todo!("build circuit with PauliX on qubit 0")
}

/// Return P(|0⟩) after applying X to |0⟩ (should be ≈ 0.0).
pub fn prob_zero_after_x() -> f64 {
    todo!("simulate x_flip_circuit() and return prob(|0⟩)")
}

/// Return P(|1⟩) after applying X to |0⟩ (should be ≈ 1.0).
pub fn prob_one_after_x() -> f64 {
    todo!("simulate x_flip_circuit() and return prob(|1⟩)")
}
