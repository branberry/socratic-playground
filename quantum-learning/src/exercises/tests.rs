//! Exercise verification tests — turn green one exercise at a time.

use super::ex01_circuit::empty_circuit;
use super::ex02_build_gates::{hadamard_on_qubit_zero, x_gate_on_qubit_zero};
use super::ex03_zero_state::{zero_state_circuit, zero_state_probability};
use super::ex04_x_gate::{prob_one_after_x, prob_zero_after_x, x_flip_circuit};
use super::ex05_hadamard::{hadamard_circuit, hadamard_probabilities};

const EPS: f64 = 1e-6;

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
}

#[test]
fn ex01_empty_circuit_has_no_operations() {
    let circuit = empty_circuit();
    assert_eq!(circuit.len(), 0);
}

#[test]
fn ex02_hadamard_circuit_has_one_gate() {
    let circuit = hadamard_on_qubit_zero();
    assert_eq!(circuit.len(), 1);
}

#[test]
fn ex02_x_gate_circuit_has_one_gate() {
    let circuit = x_gate_on_qubit_zero();
    assert_eq!(circuit.len(), 1);
}

#[test]
fn ex03_zero_state_probability_is_one() {
    let circuit = zero_state_circuit();
    assert_eq!(circuit.len(), 0);
    assert!(approx_eq(zero_state_probability(), 1.0));
}

#[test]
fn ex04_x_gate_flips_to_one() {
    let circuit = x_flip_circuit();
    assert_eq!(circuit.len(), 1);
    assert!(approx_eq(prob_zero_after_x(), 0.0));
    assert!(approx_eq(prob_one_after_x(), 1.0));
}

#[test]
fn ex05_hadamard_gives_equal_superposition() {
    let circuit = hadamard_circuit();
    assert_eq!(circuit.len(), 1);
    let (p0, p1) = hadamard_probabilities();
    assert!(approx_eq(p0, 0.5), "P(|0⟩) expected 0.5, got {p0}");
    assert!(approx_eq(p1, 0.5), "P(|1⟩) expected 0.5, got {p1}");
    assert!(approx_eq(p0 + p1, 1.0), "probabilities should sum to 1");
}
