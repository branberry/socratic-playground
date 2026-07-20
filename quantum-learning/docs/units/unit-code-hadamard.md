# Unit guide: Hadamard circuit + measurement probabilities

**Material unit:** `unit-code-hadamard`  
**Difficulty:** medium · **Prerequisites:** `unit-theory-hadamard`, `unit-code-baseline`

## What you need to know

This unit completes your **track edge**: build a 1-qubit circuit with **H**, simulate it, and **print or assert** P(|0⟩) and P(|1⟩).

### Circuit (conceptual)

```text
|0⟩ ──[ H ]── (superposition) ── (read probs) ── P(0)≈0.5, P(1)≈0.5
```

In roqoqo (ex05):

```rust
use roqoqo::operations::*;
use roqoqo::Circuit;

let mut circuit = Circuit::new();
circuit += Hadamard::new(0);
// simulate with occupation_probabilities(&circuit, 1)
```

### What “print measurement probabilities” means here

For learning + unit tests, **occupation probabilities from the simulator** are the right object — they match the Born rule without sampling noise. Optional stretch: add `PragmaRepeatedMeasurement` for shot histograms (see qoqo examples); not required for ex05.

## Claims & supporting resources

1. **H on qubit 0 in roqoqo:** `circuit += Hadamard::new(0);`  
   - [roqoqo operations docs](https://docs.rs/roqoqo/latest/roqoqo/operations/index.html)  
   - [freeCodeCamp H gate section](https://www.freecodecamp.org/news/how-to-write-your-first-quantum-circuit-in-python-a-beginner-s-step-by-step-guide/) (Python, same story)

2. **Expected outcome after H on |0⟩:** P(0) = P(1) = ½ within numerical tolerance (≈ 1e-6 in tests).  
   - [Qiskit_qubit_Measure Task 3](https://github.com/arunpandianj/Qiskit_qubit_Measure)  
   - [Real Python — superposition measurement](https://realpython.com/quantum-computing-basics/)

3. **Pauli-X baseline (sanity check):** X on |0⟩ gives P(0)=0, P(1)=1 — ex04 confirms gates work before H.  
   - Exercise: `src/exercises/ex04_x_gate.rs`

## Exercise & verify

| Step | Test |
|------|------|
| ex04 X gate | `cargo test ex04_x_gate` |
| ex05 Hadamard | `cargo test ex05_hadamard` |
| Full suite | `cargo test` |

## Success criteria (edge complete)

- [ ] `hadamard_probabilities()` returns `(≈0.5, ≈0.5)`  
- [ ] You can explain *why* in one sentence (superposition + Born rule)  
- [ ] Track verify command green: `cargo test`

## Math appendix

**Circuit as unitary composition.** A sequence of gates G₁, G₂, … acts as **G = Gₙ … G₂ G₁** on the statevector (order matches circuit application in roqoqo).

**Hadamard edge (ex05).** Starting from |0⟩ = (1, 0)ᵀ:

```
|c⟩ = H|0⟩ = (1/√2, 1/√2)ᵀ
```

**Expected probability vector** returned by `occupation_probabilities`:

```
P = ( |c₀|², |c₁|² ) = ( 0.5, 0.5 )
```

**Test tolerance.** Floating-point simulation may differ slightly from exact ½; your tests use ε ≈ 10⁻⁶:

```
|P(0) − 0.5| < ε   and   |P(1) − 0.5| < ε
```

**Optional: shot histogram.** If you later add repeated measurements with N shots, outcome counts (n₀, n₁) estimate P(0) ≈ n₀/N, P(1) ≈ n₁/N. Variance decreases as O(1/√N) — why one shot is insufficient for distribution claims.

**Further reading:**
- [Qiskit_qubit_Measure Task 3 — H then measure](https://github.com/arunpandianj/Qiskit_qubit_Measure)
- [roqoqo Hadamard operation](https://docs.rs/roqoqo/latest/roqoqo/operations/struct.Hadamard.html)

## Stop point

After green tests: run `/learn-reflect` — what surprised you about simulator probabilities vs single-shot measurement?
