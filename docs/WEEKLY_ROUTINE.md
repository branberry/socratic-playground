# Weekly learning routine — Rust & RAG

Evidence-backed habits (sleep, retrieval practice, plan–monitor–evaluate) mapped to this repo. Pair with [AI_LEARNING_WORKFLOW.md](AI_LEARNING_WORKFLOW.md) for session prompts and the stuck ladder.

**Living state:** [PROGRESS.md](PROGRESS.md) — update “Next 5-min task” after every session and at Sunday review.

---

## Weekly structure

| Day | Focus | Time |
|-----|--------|------|
| **Mon** | Deep Rust session | 45 min |
| **Tue** | Light session + retrieval | 25 min |
| **Wed** | Deep Rust session | 45 min |
| **Thu** | Rest or 10-min review only | 0–10 min |
| **Fri** | Deep Rust session | 45 min |
| **Sat** | Optional catch-up / blog notes | 25 min |
| **Sun** | Weekly metacognitive review | 15 min |

**Minimum viable week:** Mon + Wed + Fri (3 × 45 min) + Sun review. Tue/Sat are bonus.

---

## Daily foundation

Protect baseline cognition so Rust sessions stick. No extra “study” time required — these are life habits.

| Habit | Why |
|-------|-----|
| **Sleep** (consistent 7–9 h for most adults) | Sleep restriction impairs working memory and executive function most ([meta-analyses](https://doi.org/10.31586/ojp.2025.6215)). |
| **Movement** 2–3×/week (20–30 min) | Among the strongest non-drug cognition supports; combine aerobic + resistance when possible. |
| **Phone boundary** | First 30 min of a deep session = editor only. |

---

## Every Rust session — Plan → Do → Monitor → Evaluate

Same contract as [AI_LEARNING_WORKFLOW.md](AI_LEARNING_WORKFLOW.md#every-study-session-adhd-friendly-2545-min).

### Before (2 min) — Plan

Paste to Cursor **Ask** (or say out loud):

```
I'm starting a Rust session. I have [25/45] minutes.
My only goal: [one thing].
Energy: low / medium / high.
Give me a 3-step plan and the FIRST physical action (one sentence).
```

Open [PROGRESS.md](PROGRESS.md) and confirm “Next 5-min task” matches today’s goal.

### During (25–45 min) — Monitor

```
1. Read the concept comment (30 sec)
2. Try implementation yourself (5–20 min)
3. cargo test exercise_N   (or cargo test chunk / embed / …)
4. If red → paste ERROR ONLY to AI (stuck ladder level 2)
5. If green → explain in one sentence what you did (out loud or in chat)
6. Stop when timer ends — even mid-exercise
```

**Metacognition check** (once per session, ~30 sec): *“Do I understand this, or am I pattern-matching compiler errors?”* If pattern-matching, ask one “why” question before continuing.

**Stopping mid-exercise is a feature.** Leave a one-line re-entry note in chat or PROGRESS.md.

### After (3 min) — Evaluate

```
Session debrief:
- What passed? (test name)
- What concept clicked?
- What's still fuzzy?
- One 5-minute task for next time.
```

Write the last line into [PROGRESS.md](PROGRESS.md).

---

## Tuesday — retrieval practice (25 min)

No new features. **Quiz yourself** — highest-utility learning technique (Dunlosky et al., 2013) plus metacognitive monitoring.

1. Close the editor.
2. Answer from memory (paper or notes). See **Retrieval questions by phase** below.
3. Open source files — check answers. Mark ✅ / ❌.
4. Re-run one or two old tests cold (e.g. `cargo test exercise_2`).
5. One sentence: *“What did I think I knew but didn’t?”*

### Thursday (optional, 10 min)

- Re-read session debriefs from Mon/Wed.
- Re-answer Tuesday’s questions without looking.

---

## Sunday — weekly metacognitive review (15 min)

With [PROGRESS.md](PROGRESS.md) open:

1. **What did I finish?** (tests green, concepts named)
2. **What strategy worked?** (e.g. error-only prompts, 25 min timers)
3. **What strategy failed?** (e.g. Agent wrote too much, skipped debrief)
4. **One adjustment for next week** (single change only)
5. **Next week’s first 5-min task** (one line in PROGRESS.md)

Optional Cursor prompt:

```
Weekly Rust debrief. Here's what I did: [paste PROGRESS.md session notes].
What pattern do you see in what stuck vs what didn't?
One habit to keep, one to drop, one quiz question for Monday.
Don't write code.
```

---

## Spaced review (after a test passes)

| When | Action |
|------|--------|
| Same day | One-sentence explanation out loud |
| +2 days | Re-run that exercise’s test; explain one line of your code |
| +1 week | Tuesday retrieval ritual |
| +2 weeks | Teach it in one paragraph (blog draft or note in PROGRESS.md) |

---

## AI usage rules

| When | Mode | Prompt style |
|------|------|----------------|
| Starting / stuck on *starting* | Ask | Stuck ladder level 1 |
| Compiler/test errors | Ask | Level 2 — your code + error only |
| “Why does Rust do X?” | Ask | Level 3 |
| Scaffolding files, running tests | Agent | You direct; no core exercise implementations |
| After 3 honest tries | Ask | Level 4 — minimal commented solution, you retype |

**Default to Ask** until tests pass. See [AI_LEARNING_WORKFLOW.md](AI_LEARNING_WORKFLOW.md#the-stuck-ladder-copy-paste-prompts).

---

## Low-energy day protocol

1. Open [PROGRESS.md](PROGRESS.md), do the “Next 5-min task” only.
2. Run one relevant test.
3. One-line debrief. Done.

---

## Retrieval questions by phase

Update your Tuesday quiz as you advance. Only quiz concepts you’ve already implemented.

### Phase 0 — Warm-up

- Why does `Chunk` use `String` not `&str`?
- What does `'a` on a struct mean?
- What’s the difference between byte indices and char indices in `&str`?
- When do you use `Option` vs `Result`?

**Verify commands:** `cargo test rust_warmup` · `cargo test exercise_N`

### Phase 1 — Chunking

- Why not one vector per whole book?
- How do `chunk_size` and `overlap` interact in the window loop?
- What metadata does each `Chunk` carry and why?

**Verify:** `cargo test chunk` · `cargo run -- ingest`

### Phase 2 — Embeddings & store

- What does cosine similarity measure (and what doesn’t it capture)?
- Why batch embed calls?
- What lives in Qdrant vs what stays in Rust structs?

**Verify:** `cargo test embed` · `cargo test store`

### Phase 3 — Retrieval

- Query embedding vs document embedding — same model, same space?
- What does `top_k` control?
- How would you explain a “false positive” retrieval?

**Verify:** `cargo test retrieve` · `cargo run -- search "..."`

### Phase 4 — RAG + Ollama

- What goes in the prompt vs what stays out?
- Where can hallucination still happen even with good retrieval?

**Verify:** `cargo test` · `cargo run -- ask "..."`

---

## Phase session maps (typical week)

Pick the section matching [PROGRESS.md](PROGRESS.md) **Phase**. Adjust days to your calendar; keep Mon/Wed/Fri as deep work when possible.

### Phase 0 — Warm-up (current)

Recommended exercise order: **1 → 2 → 8 → 9 → 3 → 4 → 7 → 5 → 6** ([RUST_WARMUP.md](RUST_WARMUP.md)).

| Session | Target | Verify |
|---------|--------|--------|
| **Mon** | Next failing exercise from PROGRESS | `cargo test exercise_N` |
| **Tue** | Retrieval (Phase 0 questions) | — |
| **Wed** | Next failing exercise | `cargo test exercise_N` |
| **Thu** | Rest or flashcards | — |
| **Fri** | Finish remaining warm-up | `cargo test rust_warmup` |
| **Sat** | One-sentence explanations for ex 8–9; read STEPS.md Step 1 | — |
| **Sun** | Weekly review | Update PROGRESS.md |

**Week success criteria:**

- [ ] `cargo test rust_warmup` — all green
- [ ] Can explain why `Chunk` owns `String` (one sentence)
- [ ] At least 2 session debriefs in PROGRESS.md
- [ ] Tuesday retrieval done once

### Phase 1 — Chunking

| Session | Target | Verify |
|---------|--------|--------|
| **Mon** | `chunk_text` substep 1–2 (loop skeleton) | `cargo test chunk` |
| **Tue** | Retrieval (Phase 1 questions) | — |
| **Wed** | Substeps 3–4 (overlap, `Chunk` assembly) | `cargo test chunk` |
| **Fri** | Edge cases + `cargo run -- ingest` | multiple chunks on long docs |
| **Sun** | Review + demo checkpoint note | PROGRESS.md |

### Phase 2 — Embeddings & store

Alternate deep sessions between `embed.rs` and `store.rs`. Tuesday quizzes Phase 2 questions. End week with similarity sanity check (see [STEPS.md](STEPS.md)).

### Phase 3+ — Retrieval, RAG, polish

Follow [ROADMAP.md](ROADMAP.md) milestones. Keep the same weekly skeleton; swap **Mon/Wed/Fri** targets for the active module under test.

---

## Copy-paste — Monday kickoff

```
Weekly Rust session 1/3. 45 minutes. Energy: [level].
Goal: [from PROGRESS.md "Next 5-min task"].

Give me:
1. The first physical action (one sentence)
2. One edge case the tests might check
3. One metacognition question to ask myself mid-session

Don't write the solution.
```

---

## Research snapshot (why this routine)

| Practice | Evidence |
|----------|----------|
| Plan–monitor–evaluate | Metacognitive instruction improves learning (*g* ≈ 0.48–0.60 in school meta-analyses); embed in real tasks ([EEF](https://educationendowmentfoundation.org.uk/education-evidence/guidance-reports/metacognition)). |
| Retrieval + spacing | Highest-utility study techniques ([Dunlosky et al., 2013](https://journals.sagepub.com/doi/abs/10.1177/1529100612453266)). |
| Sleep & exercise | Strongest general cognition levers; brain-training apps show poor far transfer. |
| AI as tutor not author | Scaffold mode preserves learning; replacement mode risks shallow retention ([AI_LEARNING_WORKFLOW.md](AI_LEARNING_WORKFLOW.md#scaffold-vs-replacement-mode)). |

---

## Quick reference — verify commands

```bash
cargo test rust_warmup          # all 9 warm-up exercises
cargo test exercise_9           # single exercise
cargo test chunk                # Step 1
cargo test                      # full suite
cargo run -- ingest             # CLI smoke test
```
