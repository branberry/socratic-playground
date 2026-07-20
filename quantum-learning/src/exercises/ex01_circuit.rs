//! Exercise 1 — create a circuit
//!
//! Goal: construct an empty `Circuit` and inspect it with `len()`.
//!
//! Hints:
//! - `use roqoqo::Circuit;`
//! - `Circuit::new()` starts with no operations.

use roqoqo::Circuit;

/// Return a new, empty roqoqo circuit.
pub fn empty_circuit() -> Circuit {
    let circuit = Circuit::new();

    return circuit;
}
