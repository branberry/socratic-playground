# Project roadmap

Your long-term plan for **rust-webgpu** — paced for 2–4 sessions/week (~30–60 min each), with no hard deadline.

**Living checklist:** update [PROGRESS.md](PROGRESS.md) after each session.

**Vision:** [VISION.md](VISION.md) · **Implementation:** [STEPS.md](STEPS.md) · **Warm-up:** [WEBGPU_WARMUP.md](WEBGPU_WARMUP.md)

---

## Long-term goal

> **Build a from-scratch Rust WebGPU playground** — orbit camera + compute-driven particle field — **documented enough that I can share my knowledge and experience with others** via a blog series and a polished GitHub repository.

### What “done” looks like

| Deliverable | Success criteria |
|-------------|------------------|
| **Working app** | Resize-safe window; render + compute pipelines; orbit camera; mouse disturbs particles |
| **Blog series** | One post per major phase (see below) — problem-first, not tutorial dump |
| **Polished repo** | README tells the story; architecture diagram; verify commands work |
| **Peer value** | Another engineer learns *why* each WebGPU object exists, not just copies boilerplate |

### Why this matters (career fuel)

When motivation dips, reconnect to **who this helps**:

| Audience | Problem you're solving |
|----------|------------------------|
| **Engineers learning GPU programming** | Most WebGPU content is JavaScript or C++ — you're showing the same pipeline in Rust with tests |
| **Graphics-curious backend devs** | Buffers, shaders, and compute aren't magic — you built them line by line |
| **Future you** | GPU compute, real-time viz, and wgpu patterns — portable for games, simulations, or ML viz |

You're not "following a tutorial." You're building a **reference playground** you can stand behind in conversation.

---

## Pace & timeline

| Assumption | Value |
|------------|-------|
| Sessions per week | 2–4 |
| Session length | 30–60 min (15 min counts on bad days) |
| Hard deadline | None — consistency over speed |

**Rough calendar:** 3–5 months steady, 4–6 months with normal gaps.

| Phase | Focus | Sessions (est.) |
|-------|-------|-----------------|
| 0 | Finish warm-up | 1–2 |
| 1 | Window + clear | 2–3 |
| 2 | Triangle | 2–4 |
| 3 | Buffers + cube | 3–5 |
| 4 | Camera + uniforms | 3–5 |
| 5 | Instanced particles | 4–6 |
| 6 | Compute update | 5–8 |
| 7 | Repo polish + capstone post | 3–5 |

---

## Phases

### Phase 0 — Warm-up complete

**Code:** [WEBGPU_WARMUP.md](WEBGPU_WARMUP.md) — create `src/webgpu_warmup.rs`, exercises 1–5 (6 optional).

**Verify:**
```bash
cargo test -p rust-webgpu webgpu_warmup
```

**Demo checkpoint:** All warm-up tests green; one sentence on why GPU structs need `#[repr(C)]`.

**Career hook:** Buffer layout errors don't show up as friendly Rust errors — you learn to think in bytes before pixels.

**Blog:** No post yet. Optional: one note — "Why I'm learning WebGPU in Rust."

---

### Phase 1 — Window + clear color

**Code:** [STEPS.md](STEPS.md) Step 1 — `window.rs`, `gpu.rs`, event loop, surface clear.

**Verify:**
```bash
cargo run -p rust-webgpu
```

**Demo checkpoint:** Solid color window; resize doesn't crash.

**Career hook:** Every GPU app starts here — instance, adapter, device. Skipping this step means copying boilerplate you can't debug.

**Blog post 1 (draft title):** *"What happens before the first pixel — wgpu init in Rust"*

Cover: async init, surface config, why resize requires reconfigure.

---

### Phase 2 — Triangle

**Code:** [STEPS.md](STEPS.md) Step 2 — `shader.wgsl`, render pipeline, `draw(0..3)`.

**Verify:**
```bash
cargo run -p rust-webgpu
```

**Demo checkpoint:** Screenshot — triangle on screen.

**Career hook:** The render pipeline is the contract between your data and the GPU — same pattern in every engine.

**Blog post 2:** *"Your first render pipeline in wgpu"*

Cover: WGSL stages, clip space, pipeline vs draw call.

---

### Phase 3 — Buffers + colored geometry

**Code:** [STEPS.md](STEPS.md) Step 3 — `mesh.rs`, vertex/index buffers, instanced layout prep.

**Verify:**
```bash
cargo run -p rust-webgpu
```

**Demo checkpoint:** Colored cube (or quad) on screen.

**Career hook:** Vertex layout mismatches are silent killers — `#[repr(C)]` and attribute locations matter.

**Blog post 3:** *"Vertex layouts without magic — bytemuck and WGSL alignment"*

Cover: Pod types, buffer usages, index buffers.

---

### Phase 4 — Camera + uniforms

**Code:** [STEPS.md](STEPS.md) Step 4 — `camera.rs`, uniform buffer, bind groups, mouse orbit.

**Verify:**
```bash
cargo run -p rust-webgpu
```

**Demo checkpoint:** Screen recording — orbit around cube with mouse.

**Career hook:** Uniforms are how per-frame state reaches shaders — same pattern for lighting, time, mouse position.

**Blog post 4:** *"Orbit camera without a game engine"*

Cover: view-projection, bind group layout, input → matrix.

---

### Phase 5 — Instanced particles (render only)

**Code:** [STEPS.md](STEPS.md) Step 5 — `particles.rs`, static random field, instanced draw.

**Verify:**
```bash
cargo run -p rust-webgpu
```

**Demo checkpoint:** Fly through thousands of static particles.

**Career hook:** Instancing is how engines draw foliage, crowds, and debris — one mesh, many transforms.

**Blog post 5:** *"Drawing thousands of things — instancing in WebGPU"*

Cover: instance index, billboards, buffer sizing.

---

### Phase 6 — Compute-driven simulation

**Code:** [STEPS.md](STEPS.md) Step 6 — compute pipeline, particle update, mouse interaction.

**Verify:**
```bash
cargo run -p rust-webgpu
```

**Demo checkpoint:** Particles swirl; click disturbs the field.

**Career hook:** Compute shaders power physics, ML inference, and post-processing — render is only half the GPU story.

**Blog post 6:** *"Simulation on the GPU — compute + render in one frame"*

Cover: dispatch, storage buffers, sync between pipelines.

---

### Phase 7 — Peer polish (part of the goal, not optional)

**Repo polish checklist:**

- [ ] README: problem statement, architecture diagram, quick start, link to blog series
- [ ] [STEPS.md](STEPS.md) still accurate; add "I built this" narrative in README
- [ ] Architecture diagram matches final code
- [ ] Limitations section (platform quirks, no PBR, wgpu version pin)
- [ ] All verify commands pass on a clean clone
- [ ] Optional: short screen recording of particle demo

**Blog capstone:** *"Building a WebGPU playground in Rust: what I'd do differently"*

Retrospective: timeline, mistakes, surprises, who should copy this approach.

**Order:** Blog series can publish **as you finish each phase** — don't wait until Phase 7. Repo polish is the final pass once posts exist.

---

## Session template (30–60 min)

Copy into your notes or Cursor chat at session start:

```
Session start
- Phase: [N]
- Goal (one thing): [e.g. "triangle on screen" or "cargo test exercise_3"]
- Energy: low / medium / high
- Career reminder: [one line — who does this help?]
```

At session end, update [PROGRESS.md](PROGRESS.md):

```
Session end
- Done: [visual milestone / test name / paragraph drafted]
- Stuck on: [optional]
- Next 5-min task: [exact re-entry point]
```

**Minimum session (15 min):** One substep, one warm-up test, or three sentences of blog draft. Counts.

---

## Re-entry ritual (after a gap)

Motivation fades when the project feels abstract. **Don't restart with guilt — restart with curiosity.**

### Step 1 — Orient (2 min)

1. Open [PROGRESS.md](PROGRESS.md)
2. Read "Last session" and "Next 5-min task"
3. Run one verify command for your current phase

### Step 2 — Tangential spark (5–10 min)

Pick **one** random task from the bank below (or ask Cursor: *"Give me a tangential 5-min quiz for Phase [N] WebGPU"*).

Do it **before** main work — it's the ramp, not procrastination.

### Step 3 — Main work (15–40 min)

One goal only. Stop when the timer ends.

---

## Tangential task & quiz bank

Use when returning after a break. Related enough to reconnect; novel enough to feel fresh.

### Any phase

- Explain to an imaginary junior dev: *"Why does WebGPU need a separate Surface?"* (out loud, 2 min)
- Draw the frame loop on paper: event → compute → render → present
- Find one wgpu validation error story online; write two sentences on what caused it
- Watch a 5-min WebGPU intro video; note one term you couldn't define before

### Phase 0 (warm-up)

- Normalize `Vec3::new(3.0, 4.0, 0.0)` by hand — what's the length?
- Without looking: what is `size_of::<GpuVertex>()` if position + color are each `[f32; 3]`?
- Quiz: column-major vs row-major — which does WGSL expect?

### Phase 1–2 (window + triangle)

- List the 5 objects you create before the first draw call
- What is NDC? Where is (0, 0) on screen in clip space?
- Change clear color to something ugly — confirm you control the render pass

### Phase 3 (buffers)

- Why `VERTEX | COPY_DST` on a vertex buffer?
- Sketch vertex layout for position + UV — how many `@location`s?
- What breaks if index buffer uses `u16` but you pass `u32` indices?

### Phase 4 (camera)

- Write view matrix ingredients on paper: eye, target, up
- What happens if pitch isn't clamped?
- One paragraph: bind group vs push constants — when would you use each?

### Phase 5 (instancing)

- How many draw calls for 10,000 particles with instancing vs without?
- What does `@builtin(instance_index)` give you in WGSL?
- Sketch: billboard quad expansion in vertex shader (4 corners)

### Phase 6 (compute)

- Render vs compute — which runs first in your frame loop and why?
- What is workgroup size? Why `(n + 63) / 64`?
- One sentence: what sync do you need between compute write and render read?

### Phase 7 (polish)

- Read your README as a stranger — what's missing in the first 30 seconds?
- One paragraph: who is this repo **not** for?
- Outline the capstone blog post in five bullet headers only

---

## Blog series outline

Publish when each phase's demo checkpoint passes — repo can stay "in progress" until Phase 7.

| # | Working title | Publish when |
|---|---------------|--------------|
| 1 | What happens before the first pixel — wgpu init in Rust | Phase 1 demo |
| 2 | Your first render pipeline in wgpu | Phase 2 demo |
| 3 | Vertex layouts without magic — bytemuck and alignment | Phase 3 demo |
| 4 | Orbit camera without a game engine | Phase 4 demo |
| 5 | Drawing thousands of things — instancing in WebGPU | Phase 5 demo |
| 6 | Simulation on the GPU — compute + render in one frame | Phase 6 demo |
| 7 | Building a WebGPU playground in Rust: retrospective | Phase 7 polish |

**Draft location suggestion:** `docs/blog/` (create when you publish post 1) or your external blog — link from README.

---

## Related docs

| Doc | Use |
|-----|-----|
| [PROGRESS.md](PROGRESS.md) | Checkboxes + last session notes |
| [STEPS.md](STEPS.md) | Implementation tasks per step |
| [WEBGPU_WARMUP.md](WEBGPU_WARMUP.md) | Warm-up concepts |
| [VISION.md](VISION.md) | North star + capstone |
| [AI_LEARNING_WORKFLOW.md](../docs/AI_LEARNING_WORKFLOW.md) | AI tutor sessions & stuck ladder |

---

## When you finish

Celebrate explicitly — this is a multi-month build.

- [ ] All phase checkboxes in PROGRESS.md green
- [ ] Blog series linked from README
- [ ] One peer conversation: walk someone through the particle demo or a post
- [ ] Note in PROGRESS.md: *"What I'd build next"* (e.g. texture sampling, egui controls, Game of Life compute)

Then decide: archive as reference, extend (PBR, mesh loading), or fork for a visualization tool.
