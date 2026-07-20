# Unit guide: |0⟩ baseline test (roqoqo + roqoqo-quest)

**Material unit:** `unit-code-baseline`  
**Difficulty:** medium · **Prerequisites:** `unit-theory-qubit`

## What you need to know

This unit connects **theory → Rust**. The roqoqo stack splits into two roles:

| Crate | Role |
|-------|------|
| **[roqoqo](https://docs.rs/roqoqo/)** | Build circuits: `Circuit`, gates (`Hadamard`, `PauliX`, …) |
| **[roqoqo-quest](https://docs.rs/roqoqo-quest/)** | Simulate circuits via QuEST; read state occupation probabilities |

**You do not implement a simulator.** You describe a circuit in roqoqo and hand it to roqoqo-quest.

### Baseline workflow (ex03)

1. `let mut circuit = Circuit::new();` — empty circuit, no gates  
2. `occupation_probabilities(&circuit, 1)` — simulate on 1 qubit (provided in `src/sim.rs`)  
3. Assert `prob[0] ≈ 1.0` — confirms |0⟩ before adding H

```text
roqoqo::Circuit  ──►  roqoqo_quest::call_circuit  ──►  Qureg::probabilites()
     (describe)              (simulate)                      (read P(|0⟩), P(|1⟩))
```

## Claims & supporting resources

1. **roqoqo does not run circuits by itself** — backends live in separate crates.  
   - [HQS qoqo architecture docs](https://hqsquantumsimulations.github.io/qoqo/)  
   - [roqoqo-quest crate description](https://crates.io/crates/roqoqo-quest)

2. **Circuit building uses `+=` with gate operations.**  
   - [qoqo_examples — Intro to roqoqo (Rust)](https://github.com/HQSquantumsimulations/qoqo_examples/tree/main/roqoqo/standalone/1_Intro_to_roqoqo)

3. **Exercise path in this repo:** `quantum-learning/src/exercises/ex01` … `ex05`, verify with `cargo test`.  
   - See `quantum-learning/EXERCISES.md`

## Files to work in

| File | Goal |
|------|------|
| `src/exercises/ex01_circuit.rs` | `Circuit::new()` |
| `src/exercises/ex02_build_gates.rs` | Add `Hadamard`, `PauliX` |
| `src/exercises/ex03_zero_state.rs` | **Baseline:** P(|0⟩) ≈ 1.0 |
| `src/sim.rs` | Provided helper — use, don’t rewrite yet |

## Verify

```bash
cd quantum-learning
cargo test ex03_zero_state
```

## Math appendix

**Statevector indexing (1 qubit).** QuEST / roqoqo-quest stores amplitudes in a vector **c** of length 2:

```
index 0  ↔  |0⟩  amplitude c₀
index 1  ↔  |1⟩  amplitude c₁
```

**Occupation probabilities** (what `Qureg::probabilites()` returns for a pure state):

```
P(0) = |c₀|²
P(1) = |c₁|²
```

**Empty circuit on 1 qubit** ⇒ no unitary applied ⇒ **c = (1, 0)** ⇒ **P(0) = 1**.

**Gate application (conceptual).** Each gate G is a 2×2 unitary; the state updates as **c' = G · c**. Your ex03 test checks G = I (identity).

**Code ↔ math mapping:**

| Math | Rust (this repo) |
|------|------------------|
| Describe circuit | `roqoqo::Circuit` |
| Apply gates to statevector | `roqoqo_quest::call_circuit` |
| Read P(0), P(1) | `occupation_probabilities(...)[0]`, `[1]` |

**Further reading:**
- [roqoqo-quest — Qureg::probabilites](https://docs.rs/roqoqo-quest/latest/roqoqo_quest/struct.Qureg.html)
- [qoqo architecture — backends trait](https://hqsquantumsimulations.github.io/qoqo/)

## Stop point

Green `ex03_zero_state_probability_is_one` — then you have proof the simulator pipeline works before Hadamard.
