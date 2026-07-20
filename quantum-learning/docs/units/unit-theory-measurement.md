# Unit guide: Measurement and probabilities

**Material unit:** `unit-theory-measurement`  
**Difficulty:** easy · **Prerequisites:** `unit-theory-qubit`

## What you need to know

**Measurement** is how quantum information becomes classical information you can log, print, or assert in a test. For a single qubit measured in the standard (Z) basis, the outcome is **`0` or `1`**.

Key ideas:

1. **Born rule (informal):** if the qubit state is |ψ⟩ = α|0⟩ + β|1⟩, then measuring gives `0` with probability |α|² and `1` with probability |β|². Probabilities sum to 1 when the state is normalized.
2. **Collapse:** after a measurement, the qubit is no longer in superposition — it is projected onto the outcome you observed (for that shot).
3. **One shot ≠ the distribution:** a single run can return `0` even when P(`0`) = 0.5. You need many **shots** (repeated runs) or direct access to **simulator occupation probabilities** to see the underlying distribution.

| Situation | Single measurement | Many shots / simulator probs |
|-----------|-------------------|------------------------------|
| |0⟩ (no gates) | Always `0` | 100% `0` |
| Superposition (later) | Random `0` or `1` | Histogram approaches P(0), P(1) |

## Claims & supporting resources

1. **Measurement outcomes are probabilistic for superposition states, deterministic for basis states.**  
   - [Real Python — measurement section](https://realpython.com/quantum-computing-basics/)  
   - [JKU notes (2025)](https://www.jku.at/fileadmin/gruppen/180/2025_Quantum_Information_Lecture_Notes.pdf) — normalization |α|² + |β|² = 1

2. **Simulators expose probabilities without sampling noise** — useful for unit tests. roqoqo-quest’s `occupation_probabilities` (your `sim.rs` helper) reads P(|0⟩), P(|1⟩) directly from the statevector.  
   - [HQS qoqo user docs — backends](https://hqsquantumsimulations.github.io/qoqo/)  
   - [roqoqo-quest API](https://docs.rs/roqoqo-quest/)

3. **Repeated shots build a histogram** — same idea as running `AerSimulator` with `shots=1024`.  
   - [Qiskit_qubit_Measure Task 2](https://github.com/arunpandianj/Qiskit_qubit_Measure) — varying shot counts

## Connection to code

- **Test style:** assert `prob[0] ≈ 1.0` for |0⟩ baseline (deterministic in simulator).  
- **Later:** assert `prob[0] ≈ prob[1] ≈ 0.5` after Hadamard — that is a *distribution* claim, not a single-shot claim.

## Math appendix

**Born rule (computational basis).** For |ψ⟩ = α|0⟩ + β|1⟩:

```
P(0) = |α|² = |⟨0|ψ⟩|²
P(1) = |β|²  = |⟨1|ψ⟩|²
```

**Normalization constraint:**

```
|α|² + |β|² = 1
```

**Projective measurement.** Measuring in the Z basis applies projectors P₀ = |0⟩⟨0| and P₁ = |1⟩⟨1|. Outcome `0` collapses the state to |0⟩; outcome `1` collapses to |1⟩.

**Simulator vs sampling.** A statevector simulator stores amplitudes (α, β) and reports occupation probabilities |α|², |β|² directly. Shot-based simulation estimates the same values by counting outcomes over N runs (law of large numbers).

**For |0⟩ baseline:** α = 1, β = 0 ⇒ P(0) = 1, P(1) = 0 exactly.

**Further reading:**
- [JKU notes — measurement postulates](https://www.jku.at/fileadmin/gruppen/180/2025_Quantum_Information_Lecture_Notes.pdf)
- [Waterloo QIC-710 Part I — measurement](https://cleve.iqc.uwaterloo.ca/resources/QIC-710-F25/Qic710LectureNotes2025V1.pdf)
- [Nielsen & Chuang — Ch. 1.3](https://www.cambridge.org/highereducation/books/quantum-computation-and-quantum-information/DAE6217BEAC4AEA91D68EA689ECE9443)

## Stop point

Answer: *Why is one measurement not enough to verify a 50/50 superposition?*
