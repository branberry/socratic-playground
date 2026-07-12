# rust-webgpu

Build a **minimal 3D game engine** in Rust + WebGPU — from your first triangle to a playable **3D Breakout** on *your* loop, input, camera, meshes, and collision.

Same monorepo principles as [rust-rag-learn](../rust-rag-learn/): Socratic tutoring, small steps, **you write the code**, explain-back for deep understanding.

## Vision

> App loop, fixed timestep, perspective camera, box meshes, AABB — enough to ship 3D Breakout and document *why* each subsystem exists.

Full north star + cognition principles: [docs/VISION.md](docs/VISION.md)

## Start here

1. Read [docs/STEPS.md](docs/STEPS.md) **Step 0** (no new code)
2. Run:
   ```bash
   cargo run -p rust-webgpu
   ```
3. Answer: *Why does WebGPU have separate `Device`, `Queue`, and `Surface`?*
4. Begin [docs/WEBGPU_WARMUP.md](docs/WEBGPU_WARMUP.md) Phase 0 before adding `wgpu`

Track progress in [docs/PROGRESS.md](docs/PROGRESS.md). ~3-month target: playable Breakout ([docs/ROADMAP.md](docs/ROADMAP.md)).

## Learning path

| Doc | Purpose |
|-----|---------|
| [docs/VISION.md](docs/VISION.md) | North star, engine scope, cognition rules |
| [docs/WEBGPU_WARMUP.md](docs/WEBGPU_WARMUP.md) | Math/layout/AABB exercises before the GPU window |
| [docs/STEPS.md](docs/STEPS.md) | Steps 0–7 — implementation guide |
| [docs/ROADMAP.md](docs/ROADMAP.md) | Phases, pace, quizzes, blog hooks |
| [docs/PROGRESS.md](docs/PROGRESS.md) | Your checklist + session log |

## Quick verify

```bash
cargo test -p rust-webgpu
cargo run -p rust-webgpu
```

## Tutor mode

[AGENTS.md](AGENTS.md) · Shared workflow: [../docs/AI_LEARNING_WORKFLOW.md](../docs/AI_LEARNING_WORKFLOW.md)
