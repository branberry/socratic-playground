//! Exercise 3 — simulate |0⟩ (baseline)
//!
//! Goal: confirm a 1-qubit register with *no gates* stays in |0⟩.
//!
//! Hints:
//! - Use `crate::sim::occupation_probabilities(&circuit, 1)`
//! - Return index `0` from the probability vector (P(|0⟩))

use roqoqo::Circuit;

/// Build a 1-qubit circuit with no gates (|0⟩ baseline).
pub fn zero_state_circuit() -> Circuit {
    let circuit = Circuit::new();

    return circuit;
}

/// Return P(|0⟩) for the default |0⟩ state (should be ≈ 1.0).
pub fn zero_state_probability() -> f64 {
    todo!("simulate zero_state_circuit() and return prob(|0⟩)")
}
