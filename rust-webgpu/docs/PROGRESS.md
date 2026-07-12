# Progress tracker

Update this file at the **end of every session** — especially the “Next 5-min task” line. Future-you re-enters here.

**Roadmap:** [ROADMAP.md](ROADMAP.md) · **Implementation:** [STEPS.md](STEPS.md) · **Vision:** [VISION.md](VISION.md)

---

## Long-term goal

Build a from-scratch Rust + WebGPU **minimal 3D engine** and ship **3D Breakout**, documented enough to share knowledge and deepen expertise — blog series + polished GitHub repo.

---

## Current status

| Field | Value |
|-------|-------|
| **Phase** | 0 — Not started |
| **Last updated** | _(set date when you edit)_ |
| **Last session** | _(what you did)_ |
| **Next 5-min task** | Read [STEPS.md](STEPS.md) Step 0, run `cargo run -p rust-webgpu`, answer Device/Queue/Surface question |

---

## Phase checklist

### Phase 0 — Warm-up

- [ ] Created `src/webgpu_warmup.rs` with exercise stubs
- [ ] Added `glam` + `bytemuck` to `Cargo.toml`
- [ ] Exercise 1 — Vec3 dot/cross/normalize
- [ ] Exercise 2 — mat4 multiply
- [ ] Exercise 4 — vertex bytes (after ex 3)
- [ ] Exercise 3 — `#[repr(C)]` size/align
- [ ] Exercise 5 — orbit eye position
- [ ] Exercise 6 — AABB contains + intersects (**required**)
- [ ] `cargo test -p rust-webgpu webgpu_warmup` — all green
- [ ] Explain-back: why GPU structs need `#[repr(C)]` (one sentence)

### Phase 1 — Window + clear

- [ ] Step 0 complete (scaffold run, Device/Queue/Surface answered)
- [ ] Step 1a — event loop
- [ ] Step 1b — async GPU init (`pollster`)
- [ ] Step 1c — surface config + resize
- [ ] Step 1d — clear color render pass
- [ ] `cargo run -p rust-webgpu` — colored window, resize OK
- [ ] Explain-back: Device / Queue / Surface (3–5 sentences)
- [ ] Demo checkpoint captured
- [ ] Blog post 1 drafted or published

### Phase 2 — Triangle → cube

- [ ] Step 2 — WGSL + pipeline + cube mesh
- [ ] Colored cube on screen
- [ ] Explain-back: pipeline vs draw call
- [ ] Demo checkpoint captured
- [ ] Blog post 2 drafted or published

### Phase 3 — Depth + perspective

- [ ] Paper sketch of MVP completed before coding
- [ ] Step 3 — depth buffer + static perspective camera
- [ ] Cube in true 3D space
- [ ] Explain-back: MVP + depth (3–5 sentences)
- [ ] Demo checkpoint captured
- [ ] Blog post 3 drafted or published

### Phase 4 — Game loop + input

- [ ] Decision note: fixed vs variable timestep
- [ ] Step 4 — `time.rs`, `input.rs`, WASD-moving cube
- [ ] Explain-back: what fixed timestep solves (3–5 sentences)
- [ ] Demo checkpoint captured
- [ ] Blog post 4 drafted or published

### Phase 5 — Camera system

- [ ] Paper sketch of playfield + camera
- [ ] Step 5 — playfield-oriented view + resize aspect
- [ ] Explain-back: camera system output
- [ ] Demo checkpoint captured
- [ ] Blog post 5 drafted or published

### Phase 6 — World + AABB

- [ ] Decision note: `Vec` world vs ECS for Breakout
- [ ] Step 6 — paddle / ball / bricks + collision
- [ ] Bricks clear on hit; ball stays in bounds
- [ ] Explain-back: hit detection + same-frame response
- [ ] Demo checkpoint captured
- [ ] Blog post 6 drafted or published

### Phase 7 — Ship Breakout + polish

- [ ] Step 7 — win/lose rules
- [ ] Playable 3D Breakout round
- [ ] Explain-back: who owns the frame (3–5 sentences)
- [ ] README architecture + limitations
- [ ] Clean-clone verify commands tested
- [ ] Blog capstone drafted or published
- [ ] Optional: screen recording

### Phase 8 — Optional GPU deep dive

- [ ] _(only after Phase 7)_ particles / compute — not required for Year 1

---

## Session log

_Newest first. One line per session is enough._

| Date | Phase | Done | Next 5-min task |
|------|-------|------|-----------------|
| _(example)_ | 0 | Read Step 0 | Create `webgpu_warmup.rs`, add deps, start exercise 1 |

---

## Re-entry (copy when you've been away)

1. Read **Next 5-min task** above
2. Pick one [tangential quiz from ROADMAP.md](ROADMAP.md#tangential-task--quiz-bank) for your phase
3. Do the 5-min task, then decide if you have energy for more

**Career / cognition reminder:** _Who benefits when this ships? Engineers learning real-time systems in Rust; peers who want grounded GPU+engine intuition; future you explaining the frame without hand-waving._
