# Unit guide: Qubit and |0⟩ (default state)

**Material unit:** `unit-theory-qubit`  
**Difficulty:** easy · **Prerequisites:** none

## What you need to know

A **qubit** is the basic unit of quantum information. Where a classical bit is always `0` or `1`, a qubit is described by a *state* that can be prepared, transformed by gates, and eventually measured.

When you create a fresh qubit in a circuit toolkit (Qiskit, roqoqo, etc.) and apply **no gates**, it is in the **computational basis state |0⟩** — the quantum analogue of “definitely 0” before any quantum operation mixes it with |1⟩.

| Concept | Classical bit | Qubit |
|---------|---------------|-------|
| Default prepared state | `0` (by convention) | **|0⟩** |
| Can be “both at once”? | No | Yes, after certain gates (later units) |
| Readout | Deterministic | Often probabilistic (see measurement unit) |

**Dirac (ket) notation:** |0⟩ and |1⟩ are basis states. You will see them in every quantum SDK and textbook; they label the two outcomes of a single-qubit measurement in the standard basis.

## Claims & supporting resources

1. **A qubit is a two-level quantum system** (not just a fuzzy classical bit).  
   - [IBM Quantum Learning — Fundamentals](https://learning.quantum.ibm.com/)  
   - [JKU Quantum Information notes (2025)](https://www.jku.at/fileadmin/gruppen/180/2025_Quantum_Information_Lecture_Notes.pdf) — § on computational basis |0⟩, |1⟩

2. **|0⟩ is the usual initial state** when no preparation gate is applied.  
   - [Real Python — Quantum Computing Basics](https://realpython.com/quantum-computing-basics/) — qubit section  
   - [Qiskit_qubit_Measure Task 1](https://github.com/arunpandianj/Qiskit_qubit_Measure) — verify |0⟩ before adding gates

3. **Intuition before math:** think “arrow in 2D space” rather than “coin on table” for superposition (superposition comes in a later unit).  
   - [3Blue1Brown — Quantum Computing playlist](https://www.youtube.com/playlist?list=PL192E968792606FD8) — first 1–2 videos

## Why this matters for your track

Your Rust exercises and edge work assume: **1 qubit, no gates ⇒ still |0⟩**. The ex03 baseline test (`P(|0⟩) ≈ 1.0`) is checking exactly this claim in roqoqo-quest.

## Math appendix

**State space.** A single qubit lives in a 2-dimensional complex vector space (Hilbert space) ℋ ≅ ℂ².

**Computational basis** (standard measurement basis):

```
|0⟩ = (1)    |1⟩ = (0)
      (0)          (1)
```

**General pure state** (superposition — preview for later units):

```
|ψ⟩ = α|0⟩ + β|1⟩   where α, β ∈ ℂ
```

**Normalization** (probabilities must sum to 1):

```
|α|² + |β|² = 1
```

**Default initialization.** With no preparation gates, simulators initialize the statevector to index 0:

```
|ψ⟩ = |0⟩  ⟺  (α, β) = (1, 0)
```

**Further reading:**
- [JKU notes — qubit as two-level system](https://www.jku.at/fileadmin/gruppen/180/2025_Quantum_Information_Lecture_Notes.pdf)
- [Linear Algebra for QC — Lecture 5: Bloch sphere](https://the-singularity-research.github.io/linear_algebra_for_quantum_computing/)

## Stop point

You can explain in one sentence: *What is |0⟩, and what state is a qubit in when the circuit has zero gates?*
