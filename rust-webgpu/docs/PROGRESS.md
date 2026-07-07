# Progress tracker

Update this file at the **end of every session** — especially the "Next 5-min task" line. Future-you re-enters here.

**Roadmap:** [ROADMAP.md](ROADMAP.md) · **Implementation:** [STEPS.md](STEPS.md) · **Vision:** [VISION.md](VISION.md)

---

## Long-term goal

Build a from-scratch Rust WebGPU playground (orbit camera + compute-driven particles), **documented enough that I can share my knowledge and experience with others** — blog series + polished GitHub repo.

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
- [ ] Exercise 6 — AABB contains (optional stretch)
- [ ] `cargo test -p rust-webgpu webgpu_warmup` — all green
- [ ] Can explain why GPU structs need `#[repr(C)]` (one sentence)

### Phase 1 — Window + clear

- [ ] Step 0 complete (scaffold run, Device/Queue/Surface question answered)
- [ ] Step 1a — event loop
- [ ] Step 1b — async GPU init (`pollster`)
- [ ] Step 1c — surface config + resize
- [ ] Step 1d — clear color render pass
- [ ] `cargo run -p rust-webgpu` — colored window, resize OK
- [ ] Demo checkpoint captured (screenshot)
- [ ] Blog post 1 drafted or published

### Phase 2 — Triangle

- [ ] Step 2 — WGSL + render pipeline + draw
- [ ] Triangle visible on screen
- [ ] Demo checkpoint captured
- [ ] Blog post 2 drafted or published

### Phase 3 — Buffers + cube

- [ ] Step 3 — `mesh.rs`, vertex/index buffers
- [ ] Colored cube (or quad) on screen
- [ ] Demo checkpoint captured
- [ ] Blog post 3 drafted or published

### Phase 4 — Camera + uniforms

- [ ] Step 4 — `camera.rs`, bind groups, mouse orbit
- [ ] Orbit camera works smoothly
- [ ] Demo checkpoint captured (screen recording)
- [ ] Blog post 4 drafted or published

### Phase 5 — Instanced particles

- [ ] Step 5 — `particles.rs`, static field
- [ ] Thousands of particles visible
- [ ] Demo checkpoint captured
- [ ] Blog post 5 drafted or published

### Phase 6 — Compute simulation

- [ ] Step 6 — compute pipeline + dispatch
- [ ] Particles animate each frame
- [ ] Mouse click disturbs field
- [ ] Demo checkpoint captured
- [ ] Blog post 6 drafted or published

### Phase 7 — Peer polish

- [ ] Step 7 — depth buffer, delta time, resize hardening
- [ ] README tells the full story + links to blog
- [ ] Architecture diagram matches code
- [ ] Limitations section written
- [ ] Clean-clone verify commands documented and tested
- [ ] Blog capstone (post 7) drafted or published
- [ ] Optional: screen recording of particle demo

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

**Career reminder:** _Who benefits when this ships? Engineers learning WebGPU in Rust; peers evaluating GPU compute patterns; future you on real-time visualization._
