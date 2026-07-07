# WebGPU warm-up — tutor guide

Build the Rust + math muscles you'll need **before** opening a GPU window.

**File:** `src/webgpu_warmup.rs` (you create this — it doesn't exist yet)

**Run tests:**
```bash
cargo test -p rust-webgpu webgpu_warmup
```

Every exercise starts with `todo!()` — **all tests will fail** until you implement them. That's correct. Fix one exercise at a time.

**Dependencies to add when you start Phase 0:**
```toml
glam = "0.29"
bytemuck = { version = "1", features = ["derive"] }
```

---

## How we'll work

Same contract as WebGPU steps:

1. Read the concept comment above each function
2. Try the implementation yourself
3. Run the single test: `cargo test -p rust-webgpu exercise_1` (replace number as needed)
4. Ask for hints only when stuck — tell the tutor what you tried first
5. The tutor won't paste full solutions unless you're truly blocked (3 honest attempts)

**Recommended order:** **1 → 2 → 4 → 3 → 5 → 6**

Exercise 4 reuses the `#[repr(C)]` struct from exercise 3. Exercise 5 reuses Vec3 from exercise 1 and matrix ideas from exercise 2. Exercise 6 is optional stretch.

---

## Exercise 1 — Vector math (`dot`, `cross`, `normalize`)

**Concept:** 3D graphics runs on `Vec3`. Dot product measures alignment; cross product gives a perpendicular vector; normalize makes unit length.

**Why it matters for WebGPU:** Camera look-at, lighting directions, and particle velocity all use vector math. `glam::Vec3` is the standard Rust choice.

**Your task:** Implement three pure functions using `glam::Vec3`:
- `vec3_dot(a, b)` → `f32`
- `vec3_cross(a, b)` → `Vec3`
- `vec3_normalize(v)` → `Vec3` (handle zero-length gracefully — return `Vec3::ZERO` or `Vec3::Y`)

**Verify:** `cargo test -p rust-webgpu exercise_1`

**Unlocks:** Step 4 orbit camera, Step 6 particle forces

---

## Exercise 2 — Matrix multiply (`mat4_mul`)

**Concept:** Transform matrices are 4×4, stored **column-major** in GPU memory. Multiplying view × projection produces the matrix you upload to a uniform buffer.

**Why it matters for WebGPU:** Your uniform buffer holds raw `f32` bytes — wrong layout = wrong rendering.

**Your task:** Implement `mat4_mul(a: &[f32; 16], b: &[f32; 16]) -> [f32; 16]` for column-major 4×4 matrices. You may use `glam::Mat4` internally and convert to/from arrays.

**Verify:** `cargo test -p rust-webgpu exercise_2`

**Unlocks:** Step 4 uniform buffer (view-projection)

---

## Exercise 3 — GPU struct layout (`vertex_size_and_align`)

**Concept:** GPU buffers interpret memory literally. Rust struct layout must match WGSL via `#[repr(C)]` and correct field order.

**Why it matters for WebGPU:** A mismatched vertex layout causes validation errors or garbage on screen — often with no helpful Rust compile error.

**Your task:** Define a `#[repr(C)]` struct `GpuVertex { position: [f32; 3], color: [f32; 3] }` and implement:
- `vertex_size() -> usize` → `std::mem::size_of::<GpuVertex>()`
- `vertex_align() -> usize` → `std::mem::align_of::<GpuVertex>()`

**Verify:** `cargo test -p rust-webgpu exercise_3`

**Unlocks:** Step 3 vertex buffer layout

---

## Exercise 4 — Bytes round-trip (`vertex_to_bytes`)

**Concept:** `bytemuck` safely casts `Pod` types to byte slices for `queue.write_buffer`.

**Why it matters for WebGPU:** You upload vertices and uniforms as raw bytes — the GPU doesn't know about Rust structs.

**Your task:** Given a `GpuVertex`, return `&[u8]` via `bytemuck::bytes_of`. Implement `vertices_to_bytes(vertices: &[GpuVertex]) -> Vec<u8>` for a slice.

**Verify:** `cargo test -p rust-webgpu exercise_4`

**Unlocks:** Step 3 buffer uploads, Step 4 uniform writes

---

## Exercise 5 — Orbit eye position (`orbit_eye_position`)

**Concept:** An orbit camera keeps a fixed radius from a target. Yaw (horizontal) and pitch (vertical) angles determine eye position.

**Why it matters for WebGPU:** Step 4 reads mouse input into yaw/pitch, then computes view matrix from eye position.

**Your task:** Implement `orbit_eye_position(target: Vec3, yaw: f32, pitch: f32, radius: f32) -> Vec3`:
- Yaw rotates around world Y axis
- Pitch rotates around local X (clamp pitch to avoid gimbal flip, e.g. ±89°)
- Eye = target + offset at `radius` distance

**Verify:** `cargo test -p rust-webgpu exercise_5`

**Unlocks:** Step 4 `camera.rs`

---

## Exercise 6 — AABB contains point (stretch)

**Concept:** Axis-aligned bounding boxes are the simplest culling primitive. "Is this point inside the box?"

**Why it matters for WebGPU:** Optional optimization before drawing thousands of particles — skip off-screen work on CPU or GPU.

**Your task:** Define `Aabb { min: Vec3, max: Vec3 }` and implement `aabb_contains(aabb: &Aabb, point: Vec3) -> bool`.

**Verify:** `cargo test -p rust-webgpu exercise_6`

**Unlocks:** Stretch goal in Step 7 polish

---

## Gate — Phase 0 complete

All six exercises pass (or 1–5 if skipping stretch):

```bash
cargo test -p rust-webgpu webgpu_warmup
```

**Demo checkpoint:** All tests green. Explain in one sentence why GPU structs need `#[repr(C)]`.

**Next:** [STEPS.md](STEPS.md) Step 1 — add `winit` + `wgpu`, open a window.

---

## Mapping table (warm-up → steps)

| Exercise | Skill | Unlocks step |
|----------|-------|--------------|
| 1 | Vec3 dot/cross/normalize | Step 4 camera, Step 6 forces |
| 2 | Column-major mat4 multiply | Step 4 uniforms |
| 3 | `#[repr(C)]` size/align | Step 3 vertex layout |
| 4 | `bytemuck` bytes | Step 3–4 buffer uploads |
| 5 | Orbit eye from yaw/pitch/radius | Step 4 camera |
| 6 | AABB contains (stretch) | Step 7 polish |
