# Unit guide: Superposition and the Hadamard (H) gate

**Material unit:** `unit-theory-hadamard`  
**Difficulty:** medium · **Prerequisites:** `unit-theory-measurement`

## What you need to know

The **Hadamard gate (H)** is the standard “create superposition” gate on one qubit. Applied to |0⟩, it produces an **equal superposition** — so **P(`0`) = P(`1`) = ½** when you measure in the standard basis.

| Before H | After H (before measure) | After measure (one shot) |
|----------|--------------------------|---------------------------|
| Definite |0⟩ | Equal superposition | Random `0` or `1` |
| P(0)=1, P(1)=0 | P(0)=½, P(1)=½ | One bit — not the full distribution |

**No classical analogue:** H is not “maybe flip the bit.” It creates *quantum* superposition — the amplitudes interfere until measurement.

**Toy analogy (imperfect but useful):** |0⟩ is a coin flat on the table (heads up). H is a spin. Measurement is letting it land — one landing does not prove fairness; many trials (or reading simulator probabilities) do.

## Claims & supporting resources

1. **H puts |0⟩ into an equal superposition** (up to global phase).  
   - [freeCodeCamp — Creating Superposition](https://www.freecodecamp.org/news/how-to-write-your-first-quantum-circuit-in-python-a-beginner-s-step-by-step-guide/)  
   - [Real Python — Hadamard gate section](https://realpython.com/quantum-computing-basics/)

2. **Hadamard then measure is the minimal “quantum behavior” demo** — the hello-world step before Bell states / entanglement.  
   - [Qiskit_qubit_Measure Task 3](https://github.com/arunpandianj/Qiskit_qubit_Measure) — H before measure vs |0⟩ baseline  
   - [qoqo_examples — Intro to roqoqo](https://github.com/HQSquantumsimulations/qoqo_examples/tree/main/roqoqo/standalone/1_Intro_to_roqoqo) — `Hadamard::new(0)` in Rust

3. **H is a unitary gate** (reversible, no cloning). Deeper treatment when you want math:  
   - [Waterloo QIC-710 Part I primer (PDF)](https://cleve.iqc.uwaterloo.ca/resources/QIC-710-F25/Qic710LectureNotes2025V1.pdf)  
   - [Nielsen & Chuang — Ch. 1](https://www.cambridge.org/highereducation/books/quantum-computation-and-quantum-information/DAE6217BEAC4AEA91D68EA689ECE9443)

## Your current edge

> Add roqoqo and run a single-qubit Hadamard circuit that **prints measurement probabilities**.

Theory checklist before coding ex05:

- [ ] You expect **~0.5 / ~0.5**, not a fixed outcome  
- [ ] You verify |0⟩ baseline **first** (ex03)  
- [ ] You use **simulator probabilities**, not one random sample

## Math appendix

**Hadamard matrix** (up to global phase conventions used in most texts):

```
      1   ( 1   1 )
H = ───── ( 1  -1 )
     √2
```

**Action on basis states:**

```
H|0⟩ = (|0⟩ + |1⟩) / √2     →  P(0) = P(1) = ½
H|1⟩ = (|0⟩ − |1⟩) / √2     →  P(0) = P(1) = ½
```

**Unitarity:** H†H = I (gates are reversible; H is its own inverse up to phase: H² = I).

**Born rule check for H|0⟩.** State (1/√2, 1/√2) in the (|0⟩, |1⟩) basis:

```
P(0) = |1/√2|² = ½
P(1) = |1/√2|² = ½
```

**Related gate (ex04 sanity check) — Pauli-X:**

```
      ( 0  1 )
X =   ( 1  0 )        X|0⟩ = |1⟩,  X|1⟩ = |0⟩
```

**Further reading:**
- [Linear Algebra for QC — Lecture 10: gates as matrices](https://the-singularity-research.github.io/linear_algebra_for_quantum_computing/)
- [Nielsen & Chuang — Ch. 1.3.1 single-qubit gates](https://www.cambridge.org/highereducation/books/quantum-computation-and-quantum-information/DAE6217BEAC4AEA91D68EA689ECE9443)
- [UCSD CSE 190 — Lecture 1–2 (qubits, gates)](https://www.danielgrier.com/courses/CSE190/Sp25)

## Stop point

Answer: *What changes in P(|0⟩) and P(|1⟩) when you apply H to |0⟩?*
