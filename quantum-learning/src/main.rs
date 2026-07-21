// coding_with_quantum_computing_and_qubits — learning sandbox for quantum computing in Rust.
// Implement exercises in src/exercises/; verify with: cargo test

use quantum_learning as quantum;

fn main() {
    println!("Quantum learning sandbox — run `cargo test` to verify exercises.");
    println!("Modules: sim (provided), exercises/ex01 … ex05 (your code).");
    let _ = quantum::sim::occupation_probabilities;
}
