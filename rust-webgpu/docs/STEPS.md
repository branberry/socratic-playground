# Step-by-step implementation guide

Work through these steps in order. **You create each new module yourself** — the repo starts with only a scaffold `main.rs`.

After each step, run the verify commands listed below.

**Vision:** [VISION.md](VISION.md) · **Warm-up first:** [WEBGPU_WARMUP.md](WEBGPU_WARMUP.md) · **Track progress:** [PROGRESS.md](PROGRESS.md) · **Readings:** [READING.md](READING.md)

**Cognition pattern (every step):** Concept → (paper-before-code from Step 3+) → substeps → Discussion → Verify → Explain-back.

---

## Step 0 — Read the problem (no new code)

**Concept:** WebGPU is a low-level GPU API. The CPU records commands into queues; the GPU executes them asynchronously. Before writing code, understand the three roles you'll wire up first: **Device** (creates resources), **Queue** (submits work), **Surface** (presents to the window). Your long-term goal is a **minimal 3D engine** that ships Breakout — not a particle demo.

**Your task:**
1. Read [VISION.md](VISION.md) — know what you're building toward
2. Skim the wgpu docs mental model: [https://docs.rs/wgpu/latest/wgpu/](https://docs.rs/wgpu/latest/wgpu/)
3. Skim the Step 0 readings in [READING.md](READING.md) — at minimum the WebGPU Fundamentals piece
4. Run the scaffold:
   ```bash
   cargo run -p rust-webgpu
   ```
4. In your own words: *Why does WebGPU have separate `Device`, `Queue`, and `Surface`?*

**Discussion:** If you merged them into one object, what would break when you resize the window or separate “update world” from “draw frame”?

**Verify:**
```bash
cargo run -p rust-webgpu
```

You should see the scaffold message. No GPU yet.

**Explain-back:** One sentence — what problem does a Surface solve that Device alone does not?

**Next:** Complete [WEBGPU_WARMUP.md](WEBGPU_WARMUP.md) Phase 0 before Step 1.

---

## Step 1 — Window + clear color

**Concept:** Every wgpu app needs an event loop (`winit`), async initialization (`pollster`), and a configured surface. The simplest frame clears the screen to a solid color. This is the bottom of your engine’s render path.

**Dependencies to add:**
```toml
winit = "0.30"
wgpu = "24"
pollster = "0.4"
```

**Your task:** Build window + GPU init yourself. Suggested modules: `window.rs`, `gpu.rs`.

### Substep 1a — Event loop

Create a `winit` window (800×600 or similar). Handle redraw requests. Exit on close.

**Verify:** Window opens and closes cleanly.

### Substep 1b — Async GPU init

Use `pollster::block_on` to:
1. Create `wgpu::Instance`
2. Request `Adapter`
3. Request `Device` + `Queue`

Store handles in an `AppState` struct.

**Verify:** Compiles; no panic on startup.

### Substep 1c — Surface config

Create `Surface` from the window. Configure with `SurfaceConfiguration` (format, width, height, `present_mode`). Handle `WindowEvent::Resized`.

**Verify:** Resize the window — no crash.

### Substep 1d — Clear color render pass

Each frame:
1. `surface.get_current_texture()`
2. Begin render pass with a clear color (pick something visible — not black on black)
3. End pass, submit to queue, present

**Discussion:** What happens if you skip `configure` after a resize?

**Verify:**
```bash
cargo run -p rust-webgpu
```

Solid color fills the window. Resize works.

**Explain-back (3–5 sentences from memory):** What are Device, Queue, and Surface each responsible for?

---

## Step 2 — Triangle → colored cube mesh

**Concept:** Drawing requires a **shader module** (WGSL), a **render pipeline**, and a **draw call**. Start with a hard-coded triangle, then move geometry into GPU buffers as a colored **cube** — the building block for Breakout boxes.

**Dependencies** (if not already from warm-up):
```toml
bytemuck = { version = "1", features = ["derive"] }
glam = "0.29"
```

**Your task:** Create `shader.wgsl`, `pipeline.rs`, and start `mesh.rs`.

### Substep 2a — WGSL triangle

Write a vertex shader that outputs clip-space positions for 3 corners. Fragment shader: solid or interpolated color.

**Verify:** Shader compiles when you create `ShaderModule`.

### Substep 2b — Render pipeline + draw

Create `RenderPipeline`; `draw(0..3, 0..1)`.

**Verify:** Triangle on screen.

### Substep 2c — Cube mesh buffers

Define `#[repr(C)]` vertex (position + color). Upload cube vertices + indices. Update pipeline vertex layout. `draw_indexed`.

**Discussion:** Why `BufferUsages::VERTEX | COPY_DST` instead of `VERTEX` alone?

**Verify:**
```bash
cargo run -p rust-webgpu
```

**Demo checkpoint:** Colored cube on screen (may still be orthographic / no depth yet).

**Explain-back:** In one sentence, what does the render pipeline “lock in” that a draw call does not?

---

## Step 3 — Depth + perspective camera (static)

**Concept:** Real 3D needs a **depth buffer** and a **perspective projection**. Model–view–projection (MVP) uniforms place the cube in a 3D world. Camera can be static for this step.

**Paper-before-code:** Sketch (ASCII or paper): eye → look-at → view matrix → projection → clip space. Label where the uniform buffer sits.

**Your task:** Extend `mesh.rs` / `pipeline.rs`; create early `camera.rs` for static MVP.

### Substep 3a — Depth texture + depth state

Create a depth texture matching the surface size. Enable depth test/write in the pipeline. Recreate depth on resize.

**Verify:** Overlapping geometry sorts correctly (no random z-fighting).

### Substep 3b — Uniform MVP

`#[repr(C)]` uniform with `view_proj` (and optional `model`). Bind group layout + bind group. WGSL uses `mat4x4<f32>`.

**Verify:** Uniform size is a multiple of 16 bytes.

### Substep 3c — Static perspective view

Place the camera at a fixed eye looking at the origin (or playfield center). Draw the cube in perspective.

**Discussion:** What goes wrong if near/far planes are chosen poorly?

**Verify:**
```bash
cargo run -p rust-webgpu
```

Cube sits in 3D space; depth looks correct.

**Explain-back (3–5 sentences):** What is MVP, and why is depth testing separate from the projection matrix?

---

## Step 4 — Game loop + input + fixed timestep

**Concept:** An engine **owns the frame**: sample input, advance simulation on a **fixed timestep**, then render. Variable frame time alone makes physics jittery and hard to reason about.

**Paper-before-code:** Sketch the frame loop: events → accumulate dt → while accumulator ≥ step { update(step) } → render(alpha?).

**Decision note (before coding):** Write 3–5 lines: why fixed timestep for Breakout instead of “just use last frame’s dt”? Save it in chat or [PROGRESS.md](PROGRESS.md).

**Your task:** Create `time.rs`, `input.rs`, and structure `app.rs` (or expand `main.rs`).

### Substep 4a — Input state

Track keys pressed this frame (WASD / arrows). Clear or edge-detect as you prefer — document the choice.

**Verify:** Print or log key presses once (temporary).

### Substep 4b — Fixed timestep accumulator

`Instant` for real dt; accumulate; run `update(FIXED_DT)` zero or more times per frame.

**Verify:** Update count stays stable when you drag the window (within reason).

### Substep 4c — Move a cube with WASD

Apply input in `update` to a world position; rebuild model matrix; render.

**Discussion:** Who “owns” time in your architecture — the event loop, `time.rs`, or the game?

**Verify:**
```bash
cargo run -p rust-webgpu
```

You can move a cube with the keyboard at a stable rate.

**Explain-back (3–5 sentences from memory):** What problem does a fixed timestep solve that variable dt does not?

---

## Step 5 — Camera as a game system

**Concept:** The camera is not just math — it is a **system** that produces view (and projection) each frame for the playfield. For Breakout, a fixed chase / elevated view aimed at the board is enough; orbit control is optional polish.

**Paper-before-code:** Sketch playfield axes (where is the paddle? bricks? camera?). Mark eye and target.

**Your task:** Flesh out `camera.rs` — reuse warm-up exercise 5 if you use orbit; or a fixed elevated eye looking at board center.

### Substep 5a — Playfield-oriented view

Define board bounds in world space. Camera looks at board center from a comfortable angle.

**Verify:** Full playfield visible; cube/paddle prototype not clipped oddly.

### Substep 5b — Aspect ratio on resize

Update projection when the window resizes.

**Verify:** Resize — no permanent stretch.

### Substep 5c — Optional orbit polish

Mouse drag adjusts yaw/pitch around the board (clamp pitch). Do not block Breakout progress on this.

**Discussion:** Should gameplay input and camera input share one `InputState` or stay separate?

**Verify:**
```bash
cargo run -p rust-webgpu
```

Comfortable view of a 3D playfield.

**Explain-back:** One sentence — what does the camera system *output* that the renderer consumes?

---

## Step 6 — World + AABB collision

**Concept:** Breakout needs entities (paddle, ball, bricks) and **AABB tests**. Prefer a simple `World` with structs/`Vec`s for Year 1 — full ECS is a later stretch.

**Paper-before-code:** List entity fields (position, size, velocity if any). Draw paddle/ball/brick AABBs.

**Decision note:** One paragraph — why `Vec`/structs for Breakout instead of an ECS crate right now?

**Your task:** Create `world.rs` and `collision.rs`. Reuse warm-up exercise 6.

### Substep 6a — Entity types + AABBs

Paddle, ball, bricks with positions/sizes. Helper: entity → `Aabb`.

**Verify:** Unit tests for `aabb_intersects` still pass; add a couple world-level tests if useful.

### Substep 6b — Ball motion + wall bounce

Update ball in fixed timestep; reverse velocity on playfield bounds.

**Verify:** Ball stays in the board.

### Substep 6c — Paddle and brick hits

Paddle follows input; on ball–paddle / ball–brick overlap, bounce and deactivate brick.

**Discussion:** On overlap, do you resolve by flipping velocity, pushing out of penetration, or both?

**Verify:**
```bash
cargo run -p rust-webgpu
cargo test -p rust-webgpu
```

Paddle hits ball; bricks disappear on hit.

**Explain-back (3–5 sentences):** How does AABB overlap decide a “hit,” and what does your game do in the same frame after a hit?

---

## Step 7 — Ship 3D Breakout + polish

**Concept:** A game is rules + feedback: lives or fail on miss, win when bricks clear, readable colors, resize-safe present. Then document what you built.

**Your task:** Create `game.rs` or `breakout.rs`; harden and write up.

### Substep 7a — Rules + win/lose

Miss ball → lose life or reset; clear all bricks → win. Simple on-screen state is enough (title in window / clear-color change / println is fine for v1).

**Verify:** You can win and lose deliberately.

### Substep 7b — Render all entities as boxes

One cube mesh, different model matrices/colors per entity (or small instance buffer if you already know how — not required).

**Verify:** Playable round feels readable.

### Substep 7c — Repo polish + architecture note

Update README with architecture diagram. List limitations (no audio, no textures, no ECS). Optional: short screen recording.

**Verify:**
```bash
cargo test -p rust-webgpu
cargo run -p rust-webgpu
```

**Demo checkpoint:** Playable 3D Breakout on your engine.

**Explain-back (3–5 sentences from memory):** From events to pixels, who owns the frame in your engine?

**Capstone complete:** See [ROADMAP.md](ROADMAP.md) Phase 7. Optional Phase 8 (particles/compute) is off the critical path.

---

## Dependency summary

| Step | Add to `Cargo.toml` |
|------|---------------------|
| Phase 0 | `glam`, `bytemuck` |
| 1 | `winit`, `wgpu`, `pollster` |
| 2+ | (already have `glam`, `bytemuck`) |
| 7 (optional) | `env_logger`, `log` |

Add dependencies **when you reach that step** — not all at once.

---

## Module creation checklist

| Step | You create |
|------|------------|
| 0 | — (read only) |
| 1 | `window.rs`, `gpu.rs`, wire in `main.rs` |
| 2 | `shader.wgsl`, `pipeline.rs`, start `mesh.rs` |
| 3 | depth path; early `camera.rs` (static MVP) |
| 4 | `time.rs`, `input.rs`, `app.rs` structure |
| 5 | flesh out `camera.rs` |
| 6 | `world.rs`, `collision.rs` |
| 7 | `game.rs` / `breakout.rs` + polish |

The tutor may scaffold **empty files** from this list — you implement the logic.
