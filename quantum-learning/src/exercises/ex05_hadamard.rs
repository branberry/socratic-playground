//! Exercise 5 — Hadamard superposition (your current edge)
//!
//! Goal: apply H to |0⟩ and read ~50/50 occupation probabilities.
//!
//! Hints:
//! - H on |0⟩ creates equal superposition
//! - Return both P(|0⟩) and P(|1⟩); each should be ≈ 0.5

use roqoqo::Circuit;

/// Circuit with Hadamard on qubit 0 (no measurement yet).
pub fn hadamard_circuit() -> Circuit {
    todo!("build circuit with Hadamard on qubit 0")
}

/// Return (P(|0⟩), P(|1⟩)) after applying H to |0⟩.
pub fn hadamard_probabilities() -> (f64, f64) {
    todo!("simulate hadamard_circuit() and return (prob0, prob1)")
}
