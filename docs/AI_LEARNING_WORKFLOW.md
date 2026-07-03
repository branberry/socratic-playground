# AI learning workflow — Rust & RAG

How to use AI (Cursor, ChatGPT, Claude, etc.) as a **patient tutor** while working through this repo — without letting it do the learning for you.

**Related files:**

| File | Purpose |
|------|---------|
| [AGENTS.md](../AGENTS.md) | Tutor system prompt — Cursor reads this automatically |
| [.cursor/rules/rust-tutor.mdc](../.cursor/rules/rust-tutor.mdc) | Cursor rule when editing `src/**/*.rs` |
| [RUST_WARMUP.md](RUST_WARMUP.md) | Exercise concepts and verify commands |
| [STEPS.md](STEPS.md) | RAG implementation walkthrough |
| [WEEKLY_ROUTINE.md](WEEKLY_ROUTINE.md) | Weekly schedule, retrieval days, Sunday review |

---

## The one rule

Think of AI as a **patient tutor sitting next to you**, not a **homework machine**.

- ✅ "Help me understand why this doesn't compile."
- ✅ "What's the smallest next step?"
- ✅ "Quiz me on what I just implemented."
- ❌ "Just write exercise 9 for me."

This repo's tutor contract (see [README](../README.md#tutor-mode)) applies to humans and AI alike.

---

## One-time setup (~10 minutes)

### 1. Save the tutor system prompt

The canonical prompt lives in **[AGENTS.md](../AGENTS.md)** at the repo root.

**Where to use it:**

| Tool | How |
|------|-----|
| **Cursor Agent / Ask** | Automatic — reads `AGENTS.md` and `.cursor/rules/` |
| **ChatGPT** | Create a Custom GPT → paste from `AGENTS.md` into Instructions |
| **Claude** | Create a Project → paste into Custom Instructions |
| **Gemini** | Create a Gem → paste into system instructions |

Update `AGENTS.md` when your progress changes (e.g. "currently on Step 2 embed.rs").

### 2. Pick your two modes

| Mode | Tool | When |
|------|------|------|
| **Think & learn** | Cursor **Ask** or ChatGPT | Concepts, hints, "why did this fail?" |
| **Do with guardrails** | Cursor **Agent** | Boilerplate files, running tests, refactors *you* direct |

For skill-building exercises, **default to Ask** until tests pass.

---

**Weekly rhythm:** [WEEKLY_ROUTINE.md](WEEKLY_ROUTINE.md) — Mon/Wed/Fri deep sessions, Tue retrieval, Sun review.

---

## Every study session (ADHD-friendly, ~25–45 min)

### Before you open the editor (2 min)

```
I'm starting a Rust session. I have [25/45] minutes.
My only goal today: [one thing — e.g. "finish exercise 9" or "implement chunk_text"].
Energy: low / medium / high.
Give me a 3-step plan and the FIRST physical action (one sentence).
```

Example first action: *"Open `rust_warmup.rs`, scroll to `borrow_first_window`, replace `todo!()` with a return type in mind."*

### During (the loop)

```
1. Read the concept comment in the file (30 sec)
2. Try something yourself (5–15 min)
3. cargo test exercise_N
4. If red → paste ERROR ONLY to AI (not "fix it for me")
5. If green → explain in one sentence what you did (to AI or out loud)
6. Stop when the timer ends — even mid-exercise
```

**Stopping mid-exercise is a feature.** Leave a one-line note in the chat: *"Stopped at: need to wire first_n_bytes into BorrowedPassage."* That's your re-entry ramp tomorrow.

### After (3 min)

```
Session debrief:
- What passed? (test name)
- What concept clicked?
- What's still fuzzy?
- One 5-minute task for next time.
```

Momentum beats perfection.

---

## The "stuck ladder" (copy-paste prompts)

Use these **in order**. Don't skip to level 4.

### Level 1 — You haven't typed anything yet (task paralysis)

```
I'm stuck starting [exercise N / Step M] in [file].
Don't give me code. Tell me the smallest first line I could write,
and which earlier exercise or function I'm supposed to reuse.
```

### Level 2 — You tried, test failed

```
Here's my attempt and the compiler/test output:

[paste YOUR code]
[paste cargo test output]

What is ONE thing wrong? Ask me a question before hinting.
```

### Level 3 — Concept confusion

```
Explain [concept] like I'm tired.
Why does [related thing in this repo] work that way?
One paragraph, then one quiz question for me.
```

### Level 4 — Truly blocked (after 3 honest tries)

```
I've tried 3 times. Attempts:
1. ...
2. ...
3. ...

Show me a minimal solution with comments on ONLY the lines
that teach the concept — I'll retype it myself.
```

---

## Prompts by project phase

### Warm-up exercises (`src/rust_warmup.rs`)

Recommended order: **1 → 2 → 8 → 9 → 3 → 4 → 7 → 5 → 6** (see [RUST_WARMUP.md](RUST_WARMUP.md)).

**Exercise 9 example (before looking at solutions):**

```
Exercise 9 in rust_warmup.rs: borrow_first_window.

I already implemented first_n_bytes (exercise 2) and longer (exercise 8).

Questions for you (don't answer all at once — one at a time):
1. What type should the `text` field in BorrowedPassage be?
2. How do I call first_n_bytes inside borrow_first_window?
3. Why does the struct need `'a` on both fields?

I'll implement after each answer.
```

**Before running tests:**

```
I'm about to run cargo test exercise_9.
Before I do: what's one edge case the tests check that I might have missed?
```

### Finishing warm-up → Step 1 chunking

```
All rust_warmup tests pass. I'm moving to chunk.rs Step 1.

Read docs/STEPS.md Step 1 requirements.
Break chunk_text into 4 substeps I can do in separate 15-min sessions.
Don't write chunk.rs — just the substeps and which warm-up exercise each maps to.
```

### Implementing `chunk_text` (scaffold, not autopilot)

```
I'm implementing TextChunker::chunk_text in chunk.rs.

My plan:
[paste your plan in English]

Does this match window_starts + split_windows from the warm-up?
Point out ONE gap — no code yet.
```

Agent mode is fine for **"run cargo test chunk and show me the failure"** — you still write the loop logic.

### Later RAG steps (2–6)

Same pattern: read [STEPS.md](STEPS.md) concept → try → test → stuck ladder. Ask AI to map each step to warm-up exercises (e.g. dot product → cosine similarity in `embed.rs`).

---

## Weekly upkeep (15 min)

```
Rust-RAG weekly review.

I worked on: [files / exercises]
Tests passing: [cargo test summary or "not sure"]

1. What concept should I consolidate this week?
2. One 25-min session plan for next week (max 2 goals)
3. Am I using AI as tutor or replacement? Be honest.
```

---

## How to use Cursor

| Situation | Use |
|-----------|-----|
| "Why does the borrow checker hate this?" | **Ask** |
| "Explain this error" + paste compiler output | **Ask** |
| "Create embed.rs skeleton matching STEPS.md" | **Agent** — then *you* fill in logic |
| "Implement chunk_text for me" | **Avoid** on learning days |
| Red squiggles you don't understand | **Ask** with file + line number |

**Tip:** Keep one Cursor chat thread per exercise (`ex9-lifetimes`, `step1-chunking`). Context doesn't evaporate, and you don't re-explain the project every time.

---

## Scaffold vs replacement — Rust edition

| Task | AI can do more | You should do the thinking |
|------|----------------|----------------------------|
| Email, scheduling, README wording | ✅ | — |
| `cargo test` output interpretation | ✅ | Read the error once yourself first |
| Explaining lifetimes / iterators | ✅ | Explain back in one sentence |
| Warm-up functions, `chunk_text`, `cosine_similarity` | Hints only | ✅ |
| Creating empty `embed.rs` with struct stubs | ✅ Agent | ✅ Implement methods |
| Debugging *your* off-by-one in window loop | Guided hints | ✅ Fix the loop |

**80/20 for this repo:** AI handles structure, explanations, and "what's next" — you handle typing, running tests, and fixing red tests.

### Why this matters (research snapshot)

- **Scaffold mode:** AI breaks tasks, explains errors, plans sessions — you do the reps. Good for ADHD executive function and for learning.
- **Replacement mode:** AI writes the solution; you paste and move on. Fine for life admin; risky for skills you want to retain.
- MIT Media Lab (2025) found heavy LLM use during *learning tasks* can reduce engagement and recall when the AI does the thinking — use AI to **get into** the work, not to **skip** it.

---

## Signs it's working

- You run `cargo test exercise_N` before asking for help
- You can say *why* a test passed, not just *that* it passed
- Sessions end with one clear "next micro-step"
- Occasionally you solve something **without** opening chat first

## Signs to dial back AI

- You paste `todo!()` and ask for the whole function immediately
- Tests pass but you can't explain the code
- You feel productive but can't reproduce it the next day

No shame — switch back to Level 1–2 prompts for a session or two.

---

## Quick reference — verify commands

```bash
cargo test rust_warmup          # all 9 warm-up exercises
cargo test exercise_9           # single exercise
cargo test chunk                # Step 1
cargo test                      # full suite
cargo run -- ingest             # CLI smoke test
```

When all warm-up tests pass, tackle Step 1 in [STEPS.md](STEPS.md) with confidence.
