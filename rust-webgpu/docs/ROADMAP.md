# Project roadmap

Your long-term plan for **rust-webgpu** — a **3D-first minimal game engine** paced for 2–4 sessions/week (~30–60 min each), with no hard deadline.

**Living checklist:** update [PROGRESS.md](PROGRESS.md) after each session.

**Vision:** [VISION.md](VISION.md) · **Implementation:** [STEPS.md](STEPS.md) · **Warm-up:** [WEBGPU_WARMUP.md](WEBGPU_WARMUP.md)

---

## Long-term goal

> **Build a from-scratch Rust + WebGPU minimal 3D engine** — enough to ship **3D Breakout** — **documented enough that I can share knowledge and deepen expertise** via a blog series and a polished GitHub repository.

### What “done” looks like (Year 1)

| Deliverable | Success criteria |
|-------------|------------------|
| **Working engine + game** | Playable 3D Breakout: loop, input, fixed timestep, camera, meshes, AABB, win/lose |
| **Blog series** | One post per major phase — problem-first, not tutorial dump |
| **Polished repo** | README tells the story; architecture diagram; verify commands work |
| **Peer value** | Another engineer learns *why* each subsystem exists |

### Why this matters (career + cognition fuel)

| Audience | Problem you're solving |
|----------|------------------------|
| **Engineers learning GPU + engines** | Most “engine” content hides the frame; you show ownership of time, state, and present |
| **Graphics-curious backend devs** | Perspective, depth, MVP, and collision aren't magic — you built them |
| **Future you** | Mental models for real-time systems — portable beyond this repo |

You're not following a tutorial. You're building a **reference learning engine** you can stand behind in conversation.

---

## Pace & timeline

| Assumption | Value |
|------------|-------|
| Sessions per week | 2–4 |
| Session length | 30–60 min (15 min counts on bad days) |
| Hard deadline | None — consistency over speed |
| **~3-month target** | Phases 0–7 → playable 3D Breakout |

| Phase | Focus | Sessions (est.) |
|-------|-------|-----------------|
| 0 | Finish warm-up (incl. AABB) | 1–2 |
| 1 | Window + clear | 2–3 |
| 2 | Triangle → colored cube | 3–5 |
| 3 | Depth + static perspective | 3–5 |
| 4 | Loop + input + fixed timestep | 3–5 |
| 5 | Camera as a game system | 2–4 |
| 6 | World + AABB collision | 4–6 |
| 7 | Ship Breakout + polish | 4–6 |
| 8 | Optional: particles / compute | later |

---

## Phases

### Phase 0 — Warm-up complete

**Code:** [WEBGPU_WARMUP.md](WEBGPU_WARMUP.md) — create `src/webgpu_warmup.rs`, exercises 1–6 (AABB required).

**Verify:**
```bash
cargo test -p rust-webgpu webgpu_warmup
```

**Demo checkpoint:** All warm-up tests green.

**Explain-back:** One sentence — why GPU structs need `#[repr(C)]`.

**Career hook:** Layout bugs don't show up as friendly Rust errors — you learn to think in bytes before pixels.

**Blog:** Optional note — “Why I'm building a tiny 3D engine in Rust.”

---

### Phase 1 — Window + clear color

**Code:** [STEPS.md](STEPS.md) Step 1 — `window.rs`, `gpu.rs`, surface clear.

**Verify:**
```bash
cargo run -p rust-webgpu
```

**Demo checkpoint:** Solid color window; resize doesn't crash.

**Explain-back:** 3–5 sentences — Device, Queue, Surface roles.

**Blog post 1:** *"Before the first pixel — wgpu init in Rust"*

---

### Phase 2 — Triangle → cube

**Code:** [STEPS.md](STEPS.md) Step 2 — shaders, pipeline, cube mesh.

**Verify:**
```bash
cargo run -p rust-webgpu
```

**Demo checkpoint:** Colored cube on screen.

**Explain-back:** What the render pipeline locks in vs a draw call.

**Blog post 2:** *"First pipeline → first cube"*

---

### Phase 3 — Depth + perspective

**Code:** [STEPS.md](STEPS.md) Step 3 — depth buffer, MVP uniforms, static camera.

**Verify:**
```bash
cargo run -p rust-webgpu
```

**Demo checkpoint:** Cube in true 3D; paper sketch of MVP completed first.

**Explain-back:** 3–5 sentences — MVP and depth testing.

**Blog post 3:** *"Perspective and depth without an engine"*

---

### Phase 4 — Owning the frame

**Code:** [STEPS.md](STEPS.md) Step 4 — `time.rs`, `input.rs`, fixed timestep, WASD cube.

**Verify:**
```bash
cargo run -p rust-webgpu
```

**Demo checkpoint:** Keyboard moves a cube at a stable rate; decision note on fixed timestep saved.

**Explain-back:** 3–5 sentences — what fixed timestep solves.

**Blog post 4:** *"Owning the frame — fixed timestep and input"*

---

### Phase 5 — Camera as a game system

**Code:** [STEPS.md](STEPS.md) Step 5 — playfield-oriented perspective camera.

**Verify:**
```bash
cargo run -p rust-webgpu
```

**Demo checkpoint:** Comfortable view of a 3D playfield.

**Explain-back:** What the camera system outputs for the renderer.

**Blog post 5:** *"Camera as a game system"*

---

### Phase 6 — World + AABB

**Code:** [STEPS.md](STEPS.md) Step 6 — `world.rs`, `collision.rs`, paddle/ball/bricks.

**Verify:**
```bash
cargo run -p rust-webgpu
cargo test -p rust-webgpu
```

**Demo checkpoint:** Collisions work; bricks clear on hit; decision note on `Vec` world vs ECS.

**Explain-back:** 3–5 sentences — hit detection and same-frame response.

**Blog post 6:** *"AABB and a playable 3D Breakout"* (may finish in Phase 7)

---

### Phase 7 — Ship Breakout + polish

**Code:** [STEPS.md](STEPS.md) Step 7 — rules, win/lose, README architecture.

**Verify:**
```bash
cargo test -p rust-webgpu
cargo run -p rust-webgpu
```

**Demo checkpoint:** Playable win/lose Breakout; architecture writeup.

**Explain-back:** 3–5 sentences — who owns the frame from events to pixels.

**Repo polish checklist:**

- [ ] README: problem statement, architecture diagram, quick start, blog links
- [ ] [STEPS.md](STEPS.md) still accurate
- [ ] Limitations section (no audio/textures/ECS/PBR)
- [ ] Verify commands pass on a clean clone
- [ ] Optional: short screen recording

**Blog capstone:** *"Retrospective: what a minimal 3D engine taught me"*

---

### Phase 8 — Optional GPU deep dive (off critical path)

Instancing and/or compute particles — only after Breakout ships. Do not let this delay Phase 7.

---

## Session template (30–60 min)

```
Session start
- Phase: [N]
- Goal (one thing): [e.g. "fixed timestep moves cube" or "cargo test exercise_6"]
- Energy: low / medium / high
- Cognition reminder: [explain-back or paper-before-code for this step]
```

At session end, update [PROGRESS.md](PROGRESS.md):

```
Session end
- Done: [visual milestone / test / decision note]
- Stuck on: [optional]
- Next 5-min task: [exact re-entry point]
```

**Minimum session (15 min):** One substep, one warm-up test, or three sentences of explain-back / blog draft. Counts.

---

## Re-entry ritual (after a gap)

### Step 1 — Orient (2 min)

1. Open [PROGRESS.md](PROGRESS.md)
2. Read “Last session” and “Next 5-min task”
3. Run one verify command for your current phase

### Step 2 — Tangential spark (5–10 min)

Pick **one** quiz from the bank below.

### Step 3 — Main work (15–40 min)

One goal only. Stop when the timer ends.

---

## Tangential task & quiz bank

### Any phase

- Explain out loud (2 min): *“Who owns the frame in a game engine?”*
- Draw events → update → render → present from memory
- Find one wgpu validation error story; two sentences on the cause
- Write three sentences for next blog post without opening code

### Phase 0 (warm-up)

- Normalize `Vec3::new(3.0, 4.0, 0.0)` by hand
- Column-major vs row-major — which does WGSL expect?
- When do two AABBs *not* intersect?

### Phase 1–2 (window + cube)

- List objects created before the first draw call
- What is clip space? Where is (0, 0)?
- Why index buffers for a cube?

### Phase 3 (depth + perspective)

- Near plane too small — what fails?
- Sketch MVP multiply order
- Depth write vs depth test — difference?

### Phase 4 (loop + input)

- Fixed vs variable timestep — one failure mode of each
- Does input belong in update or event handlers? Why?
- What is spiral-of-death with an accumulator?

### Phase 5 (camera)

- View vs projection — one sentence each
- Aspect ratio change — which matrix updates?
- Should Breakout use free orbit during play? Why / why not?

### Phase 6–7 (collision + game)

- Separate axes theorem in one sentence (AABB special case)
- On overlap: flip velocity vs resolve penetration
- Who should not use this repo as a production engine?

---

## Blog series outline

| # | Working title | Publish when |
|---|---------------|--------------|
| 1 | Before the first pixel — wgpu init in Rust | Phase 1 demo |
| 2 | First pipeline → first cube | Phase 2 demo |
| 3 | Perspective and depth without an engine | Phase 3 demo |
| 4 | Owning the frame — fixed timestep and input | Phase 4 demo |
| 5 | Camera as a game system | Phase 5 demo |
| 6 | AABB and a playable 3D Breakout | Phase 6–7 demo |
| 7 | Retrospective: what a minimal 3D engine taught me | Phase 7 polish |

**Draft location:** `docs/blog/` or external blog — link from README.

---

## Related docs

| Doc | Use |
|-----|-----|
| [PROGRESS.md](PROGRESS.md) | Checkboxes + last session notes |
| [STEPS.md](STEPS.md) | Implementation tasks per step |
| [WEBGPU_WARMUP.md](WEBGPU_WARMUP.md) | Warm-up concepts |
| [VISION.md](VISION.md) | North star + cognition principles |
| [AI_LEARNING_WORKFLOW.md](../../docs/AI_LEARNING_WORKFLOW.md) | AI tutor sessions & stuck ladder |
| [SOCRATIC_METHOD.md](../../docs/SOCRATIC_METHOD.md) | Tutor contract |

---

## When you finish Year 1

Celebrate explicitly.

- [ ] All phase checkboxes in PROGRESS.md green (0–7)
- [ ] Blog series linked from README
- [ ] One peer walkthrough of Breakout or a post
- [ ] Note in PROGRESS.md: *“What I'd build next”* (ECS stretch, audio, textures, Phase 8 particles)

Then decide: archive as reference, extend (ECS, 3D lighting), or fork for a bigger game.
