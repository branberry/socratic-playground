# WebGPU warm-up — tutor guide

Build the Rust + math muscles you'll need **before** opening a GPU window — and before writing engine systems.

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

Same contract as engine steps ([VISION.md](VISION.md) cognition principles):

1. Read the concept comment above each function
2. Try the implementation yourself
3. Run the single test: `cargo test -p rust-webgpu exercise_1` (replace number as needed)
4. Ask for hints only when stuck — tell the tutor what you tried first
5. The tutor won't paste full solutions unless you're truly blocked (3 honest attempts)

**Recommended order:** **1 → 2 → 4 → 3 → 5 → 6**

Exercise 4 reuses the `#[repr(C)]` struct from exercise 3. Exercise 5 reuses Vec3 from exercise 1 and matrix ideas from exercise 2. **Exercise 6 (AABB) is required** — it unlocks Breakout collision.

---

## Exercise 1 — Vector math (`dot`, `cross`, `normalize`)

**Concept:** 3D graphics and engines run on `Vec3`. Dot product measures alignment; cross product gives a perpendicular vector; normalize makes unit length.

**Why it matters for the engine:** Camera look-at, paddle/ball motion, and collision normals all use vector math. `glam::Vec3` is the standard Rust choice.

**Your task:** Implement three pure functions using `glam::Vec3`:
- `vec3_dot(a, b)` → `f32`
- `vec3_cross(a, b)` → `Vec3`
- `vec3_normalize(v)` → `Vec3` (handle zero-length gracefully — return `Vec3::ZERO` or `Vec3::Y`)

**Verify:** `cargo test -p rust-webgpu exercise_1`

**Unlocks:** Step 5 camera, Step 6–7 motion and collision

---

## Exercise 2 — Matrix multiply (`mat4_mul`)

**Concept:** Transform matrices are 4×4, stored **column-major** in GPU memory. Multiplying model × view × projection produces the matrix you upload to a uniform buffer.

**Why it matters for the engine:** Your uniform buffer holds raw `f32` bytes — wrong layout = wrong rendering.

**Your task:** Implement `mat4_mul(a: &[f32; 16], b: &[f32; 16]) -> [f32; 16]` for column-major 4×4 matrices. You may use `glam::Mat4` internally and convert to/from arrays.

**Verify:** `cargo test -p rust-webgpu exercise_2`

**Unlocks:** Step 3–5 MVP uniforms

---

## Exercise 3 — GPU struct layout (`vertex_size_and_align`)

**Concept:** GPU buffers interpret memory literally. Rust struct layout must match WGSL via `#[repr(C)]` and correct field order.

**Why it matters for the engine:** A mismatched vertex layout causes validation errors or garbage on screen — often with no helpful Rust compile error.

**Your task:** Define a `#[repr(C)]` struct `GpuVertex { position: [f32; 3], color: [f32; 3] }` and implement:
- `vertex_size() -> usize` → `std::mem::size_of::<GpuVertex>()`
- `vertex_align() -> usize` → `std::mem::align_of::<GpuVertex>()`

**Verify:** `cargo test -p rust-webgpu exercise_3`

**Unlocks:** Step 2–3 cube mesh layout

---

## Exercise 4 — Bytes round-trip (`vertex_to_bytes`)

**Concept:** `bytemuck` safely casts `Pod` types to byte slices for `queue.write_buffer`.

**Why it matters for the engine:** You upload vertices and uniforms as raw bytes — the GPU doesn't know about Rust structs.

**Your task:** Given a `GpuVertex`, return `&[u8]` via `bytemuck::bytes_of`. Implement `vertices_to_bytes(vertices: &[GpuVertex]) -> Vec<u8>` for a slice.

**Verify:** `cargo test -p rust-webgpu exercise_4`

**Unlocks:** Step 2–5 buffer uploads

---

## Exercise 5 — Orbit eye position (`orbit_eye_position`)

**Concept:** An orbit camera keeps a fixed radius from a target. Yaw (horizontal) and pitch (vertical) angles determine eye position.

**Why it matters for the engine:** Step 5 aims a perspective camera at the Breakout playfield — orbit or chase views both start from eye position math.

**Your task:** Implement `orbit_eye_position(target: Vec3, yaw: f32, pitch: f32, radius: f32) -> Vec3`:
- Yaw rotates around world Y axis
- Pitch rotates around local X (clamp pitch to avoid gimbal flip, e.g. ±89°)
- Eye = target + offset at `radius` distance

**Verify:** `cargo test -p rust-webgpu exercise_5`

**Unlocks:** Step 5 `camera.rs`

---

## Exercise 6 — AABB overlap (required)

**Concept:** Axis-aligned bounding boxes are the simplest collision volume. Breakout needs “do these two boxes overlap?” and often “does this point lie inside this box?”

**Why it matters for the engine:** Paddle–ball and ball–brick hits are AABB tests. Without this, you cannot ship the Year-1 game.

**Your task:**
1. Define `Aabb { min: Vec3, max: Vec3 }`
2. Implement `aabb_contains(aabb: &Aabb, point: Vec3) -> bool`
3. Implement `aabb_intersects(a: &Aabb, b: &Aabb) -> bool` (overlap on all three axes)

**Verify:** `cargo test -p rust-webgpu exercise_6`

**Unlocks:** Step 6 `collision.rs` and Breakout

**Explain-back (Phase 0 gate):** In one sentence, why do GPU structs need `#[repr(C)]`?

---

## Gate — Phase 0 complete

All **six** exercises pass:

```bash
cargo test -p rust-webgpu webgpu_warmup
```

**Demo checkpoint:** All tests green + explain-back sentence above.

**Next:** [STEPS.md](STEPS.md) Step 1 — add `winit` + `wgpu`, open a window.

---

## Mapping table (warm-up → steps)

| Exercise | Skill | Unlocks step |
|----------|-------|--------------|
| 1 | Vec3 dot/cross/normalize | Step 5 camera, Step 6–7 motion |
| 2 | Column-major mat4 multiply | Step 3–5 MVP uniforms |
| 3 | `#[repr(C)]` size/align | Step 2–3 mesh layout |
| 4 | `bytemuck` bytes | Step 2–5 buffer uploads |
| 5 | Orbit eye from yaw/pitch/radius | Step 5 camera |
| 6 | AABB contains + intersects (**required**) | Step 6 collision / Breakout |
