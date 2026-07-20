//! Exercise 2 — add gates to a circuit
//!
//! Goal: build small circuits using roqoqo gate types and the `+=` operator.
//!
//! Hints:
//! - `use roqoqo::operations::*;`
//! - `circuit += Hadamard::new(0);` applies H to qubit 0
//! - `circuit += PauliX::new(0);` applies X (NOT) to qubit 0

use roqoqo::{operations::Hadamard, Circuit};

/// Circuit with a single Hadamard gate on qubit 0.
pub fn hadamard_on_qubit_zero() -> Circuit {
    let mut circuit = Circuit::new();

    circuit += Hadamard::new(0);

    return circuit;
}

/// Circuit with a single Pauli-X gate on qubit 0.
pub fn x_gate_on_qubit_zero() -> Circuit {
    todo!("build Circuit with one PauliX on qubit 0")
}
