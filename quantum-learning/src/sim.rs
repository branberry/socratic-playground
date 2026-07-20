//! Simulation helpers — provided so exercises focus on circuits, not register plumbing.

use roqoqo::Circuit;
use roqoqo::registers::{
    BitOutputRegister, BitRegister, ComplexRegister, FloatRegister,
};
use roqoqo_quest::{call_circuit, Qureg};
use std::collections::HashMap;

/// Run `circuit` on `n_qubits` (initialized to |0…0⟩) and return occupation probabilities.
///
/// Index `0` is P(|0⟩), index `1` is P(|1⟩) for a single-qubit register.
pub fn occupation_probabilities(circuit: &Circuit, n_qubits: u32) -> Vec<f64> {
    let mut qureg = Qureg::new(n_qubits, false);
    let mut bit_registers: HashMap<String, BitRegister> = HashMap::new();
    let mut float_registers: HashMap<String, FloatRegister> = HashMap::new();
    let mut complex_registers: HashMap<String, ComplexRegister> = HashMap::new();
    let mut bit_registers_output: HashMap<String, BitOutputRegister> = HashMap::new();

    call_circuit(
        circuit,
        &mut qureg,
        &mut bit_registers,
        &mut float_registers,
        &mut complex_registers,
        &mut bit_registers_output,
    )
    .expect("simulation failed");

    qureg.probabilites()
}
