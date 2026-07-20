# roqoqo exercises

Implement the functions marked `todo!()` in `src/exercises/`. Each exercise has unit tests.

**Verify:** `cargo test` (from `quantum-learning/`)

## Order

| # | File | Goal | Test |
|---|------|------|------|
| 1 | `ex01_circuit.rs` | Create an empty `Circuit` | `ex01_empty_circuit_has_no_operations` |
| 2 | `ex02_build_gates.rs` | Add `Hadamard` and `PauliX` gates | `ex02_*` |
| 3 | `ex03_zero_state.rs` | Simulate \|0⟩; P(\|0⟩) ≈ 1.0 | `ex03_zero_state_probability_is_one` |
| 4 | `ex04_x_gate.rs` | X gate flips to \|1⟩ | `ex04_x_gate_flips_to_one` |
| 5 | `ex05_hadamard.rs` | H gate → ~50/50 superposition | `ex05_hadamard_gives_equal_superposition` |

## Provided for you

- `src/sim.rs` — `occupation_probabilities(circuit, n_qubits)` runs roqoqo-quest and returns probability vector
- Tests in `src/exercises/tests.rs`

## Tips

- Import gates: `use roqoqo::operations::*;`
- Build circuits: `let mut c = Circuit::new(); c += Hadamard::new(0);`
- For 1-qubit sim: `occupation_probabilities(&c, 1)` → `[P(|0⟩), P(|1⟩)]`

Stop after each exercise when its test passes.
