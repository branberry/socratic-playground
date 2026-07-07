# Step-by-step implementation guide

Work through these steps in order. **You create each new module yourself** — the repo starts with only a scaffold `main.rs`.

After each step, run the verify commands listed below.

**Vision:** [VISION.md](VISION.md) · **Warm-up first:** [WEBGPU_WARMUP.md](WEBGPU_WARMUP.md) · **Track progress:** [PROGRESS.md](PROGRESS.md)

---

## Step 0 — Read the problem (no new code)

**Concept:** WebGPU is a low-level GPU API. The CPU records commands into queues; the GPU executes them asynchronously. Before writing code, understand the three roles you'll wire up first: **Device** (creates resources), **Queue** (submits work), **Surface** (presents to the window).

**Your task:**
1. Read [VISION.md](VISION.md) — know what you're building toward
2. Skim the wgpu docs mental model: [https://docs.rs/wgpu/latest/wgpu/](https://docs.rs/wgpu/latest/wgpu/)
3. Run the scaffold:
   ```bash
   cargo run -p rust-webgpu
   ```
4. In your own words: *Why does WebGPU have separate `Device`, `Queue`, and `Surface`?*

**Discussion:** If you merged them into one object, what would break when you resize the window or submit compute work?

**Verify:**
```bash
cargo run -p rust-webgpu
```

You should see the scaffold message. No GPU yet.

**Next:** Complete [WEBGPU_WARMUP.md](WEBGPU_WARMUP.md) Phase 0 before Step 1.

---

## Step 1 — Window + clear color

**Concept:** Every wgpu app needs an event loop (`winit`), async initialization (`pollster`), and a configured surface. The simplest frame clears the screen to a solid color.

**Dependencies to add:**
```toml
winit = "0.30"
wgpu = "24"
pollster = "0.4"
```

**Your task:** Build window + GPU init yourself. Suggested modules: `window.rs`, `gpu.rs`.

### Substep 1a — Event loop

Create a `winit` window (800×600 or similar). Handle `Event::AboutToWait` to request redraws. Exit on close.

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

---

## Step 2 — Triangle (first render pipeline)

**Concept:** Drawing requires a **shader module** (WGSL), a **render pipeline** (fixed-function state + shader stages), and a **draw call**. Start with 3 hard-coded vertices in the vertex shader — no buffers yet.

**Your task:** Create `shader.wgsl` and `pipeline.rs` (or inline in `gpu.rs` at first).

### Substep 2a — WGSL shaders

Write a vertex shader that outputs clip-space positions for 3 corners of a triangle. Write a fragment shader that outputs a solid color (or per-vertex interpolated color if you pass it through).

**Verify:** Shader compiles when you create `ShaderModule`.

### Substep 2b — Render pipeline

Create `RenderPipeline` with:
- `vertex` + `fragment` entry points
- `PrimitiveTopology::TriangleList`
- Color target matching your surface format

**Verify:** Pipeline creation succeeds.

### Substep 2c — Draw call

In your render pass: `set_pipeline`, `draw(0..3, 0..1)`.

**Discussion:** Why is the triangle the "hello world" of GPU programming?

**Verify:**
```bash
cargo run -p rust-webgpu
```

**Demo checkpoint:** Triangle visible on screen — the universal WebGPU milestone.

Optional: `cargo test -p rust-webgpu pipeline` if you add compile-only unit tests.

---

## Step 3 — Buffers + colored geometry

**Concept:** Real geometry lives in **GPU buffers**. Vertex attributes describe layout; index buffers reuse vertices for shared corners.

**Dependencies to add:**
```toml
bytemuck = { version = "1", features = ["derive"] }
glam = "0.29"
```

**Your task:** Create `mesh.rs` with vertex struct + buffer upload.

### Substep 3a — Vertex struct

Define `#[repr(C)]` vertex with position + color (reuse warm-up `GpuVertex` pattern). Derive `bytemuck::Pod` + `Zeroable`.

**Verify:** `cargo test -p rust-webgpu exercise_3` still passes (warm-up).

### Substep 3b — Vertex + index buffers

Upload a cube (or quad) via `queue.write_buffer`. Create index buffer for two triangles (quad) or 12 triangles (cube).

**Verify:** Buffers created with `BufferUsages::VERTEX | COPY_DST` (and `INDEX` for index buffer).

### Substep 3c — Pipeline vertex layout

Update render pipeline with `vertex_buffers` layout matching your struct fields (location 0 = position, location 1 = color).

**Verify:** `set_vertex_buffer`, `set_index_buffer`, `draw_indexed`.

**Discussion:** Why `BufferUsages::VERTEX | COPY_DST` instead of `VERTEX` alone?

**Verify:**
```bash
cargo run -p rust-webgpu
```

Colored cube (or quad) on screen.

---

## Step 4 — Uniforms + orbit camera

**Concept:** The view-projection matrix changes every frame but the pipeline stays the same. **Uniform buffers** + **bind groups** upload per-frame data to shaders.

**Your task:** Create `camera.rs`. Reuse warm-up exercise 5 for eye position.

### Substep 4a — Uniform struct

Define `#[repr(C)]` struct with `view_proj: [[f32; 4]; 4]` (or `[f32; 16]`). Match WGSL `struct Uniforms { view_proj: mat4x4<f32> }`.

**Verify:** Size is multiple of 16 bytes (WGSL uniform alignment).

### Substep 4b — Bind group layout + bind group

Create bind group layout (uniform buffer, vertex + fragment visibility). Allocate uniform buffer. Create bind group.

**Verify:** Pipeline layout includes bind group layout 0.

### Substep 4c — Update camera from input

Mouse drag → yaw/pitch. Scroll → radius. Compute view × projection each frame, write to uniform buffer before draw.

**Verify:** Orbit feels smooth; no gimbal flip (clamp pitch).

**Discussion:** Why a bind group instead of pushing constants every draw?

**Verify:**
```bash
cargo run -p rust-webgpu
```

Cube stays centered; you orbit around it with mouse.

---

## Step 5 — Instanced particles (render only)

**Concept:** **Instancing** draws the same mesh many times with different per-instance data. Start with static random positions (CPU seed once) before compute animation.

**Your task:** Create `particles.rs`.

### Substep 5a — Particle struct

```rust
#[repr(C)]
struct Particle {
    position: [f32; 3],
    velocity: [f32; 3],
    // pad to 16-byte alignment if needed
}
```

Seed ~1000–10000 particles with random positions in a sphere or box.

**Verify:** Buffer uploads without validation errors.

### Substep 5b — Instance buffer + instanced draw

Use `draw_indexed(0..index_count, 0..instance_count)` or equivalent instanced API. Pass instance index to vertex shader via `@builtin(instance_index)` or instance vertex buffer.

**Verify:** Static particle field visible.

### Substep 5c — Billboard or point sprites

Expand quad per particle in vertex shader (billboard facing camera) or use small quads. Keep fragment shader simple (color by velocity magnitude or random hue).

**Demo checkpoint:** Field of particles you can fly through with orbit camera.

**Verify:**
```bash
cargo run -p rust-webgpu
```

Thousands of points visible; camera moves through them.

---

## Step 6 — Compute shader (capstone logic)

**Concept:** **Compute shaders** run general parallel work on the GPU. A separate pipeline updates particle state; the render pipeline reads the result.

**Your task:** Extend `shader.wgsl` with a compute entry point.

### Substep 6a — Compute pipeline

Add `@compute fn main(@builtin(global_invocation_id) id: vec3<u32>)`. One thread per particle. Create `ComputePipeline` + bind group for particle storage buffer.

**Verify:** Pipeline compiles; dispatch `(particle_count + 63) / 64` workgroups.

### Substep 6b — Simulation logic

Simple rules: velocity += gravity or attraction to origin; position += velocity * delta_time; damp velocity. Use `delta_time` from frame timing.

**Verify:** Particles move smoothly without exploding (tune constants).

### Substep 6c — Mouse interaction

On click, write mouse world position (or screen ray) into a small uniform. Compute shader applies repulsion or attraction within a radius.

**Discussion:** Render vs compute pipeline — when does each run in your frame loop?

**Verify:**
```bash
cargo run -p rust-webgpu
```

**Demo checkpoint:** Particles swirl and respond to mouse clicks.

---

## Step 7 — Polish + share

**Concept:** Production-ish demos handle edge cases: depth testing, frame timing, resize reconfigure, and honest documentation.

**Your task:** Harden the app and prepare to share.

### Substep 7a — Depth buffer

Add depth texture + `DepthStencilState`. Particles/cube render correctly when overlapping.

**Verify:** No z-fighting when geometry overlaps.

### Substep 7b — Delta time + resize

Use `Instant` for frame delta. On resize: update surface config, recreate depth texture if needed.

**Verify:** Resize during simulation — no crash, no stretch artifacts.

### Substep 7c — Repo polish

Update README with architecture diagram. List limitations (no PBR, no asset loading, platform quirks). Optional: record a short demo GIF.

**Verify:**
```bash
cargo test -p rust-webgpu
cargo run -p rust-webgpu
```

All warm-up tests still pass. App runs on a clean clone with documented deps.

**Capstone complete:** See [ROADMAP.md](ROADMAP.md) Phase 7 for blog checklist.

---

## Dependency summary

| Step | Add to `Cargo.toml` |
|------|---------------------|
| Phase 0 | `glam`, `bytemuck` |
| 1 | `winit`, `wgpu`, `pollster` |
| 3 | (already have `glam`, `bytemuck`) |
| 7 (optional) | `env_logger`, `log` |

Add dependencies **when you reach that step** — not all at once.

---

## Module creation checklist

| Step | You create |
|------|------------|
| 0 | — (read only) |
| 1 | `window.rs`, `gpu.rs`, wire in `main.rs` |
| 2 | `shader.wgsl`, `pipeline.rs` |
| 3 | `mesh.rs` |
| 4 | `camera.rs` |
| 5–6 | `particles.rs`, extend `shader.wgsl` |
| 7 | polish across existing modules |

The tutor may scaffold **empty files** from this list — you implement the logic.
