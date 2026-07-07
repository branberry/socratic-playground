# rust-webgpu

Build an interactive **GPU playground** in Rust — from your first WebGPU triangle to a compute-driven particle field you orbit with the mouse.

Same monorepo principles as [rust-rag-learn](../rust-rag-learn/): Socratic tutoring, small steps, **you write the code**.

## Vision

> Orbit camera + thousands of instanced particles, positions updated each frame by a **compute shader**. Document the journey so another Rust learner can follow.

Full north star: [docs/VISION.md](docs/VISION.md)

## Start here

1. Read [docs/STEPS.md](docs/STEPS.md) **Step 0** (no new code)
2. Run:
   ```bash
   cargo run -p rust-webgpu
   ```
3. Answer: *Why does WebGPU have separate `Device`, `Queue`, and `Surface`?*
4. Begin [docs/WEBGPU_WARMUP.md](docs/WEBGPU_WARMUP.md) Phase 0 before adding `wgpu`

Track progress in [docs/PROGRESS.md](docs/PROGRESS.md).

## Learning path

| Doc | Purpose |
|-----|---------|
| [docs/VISION.md](docs/VISION.md) | North star + capstone demo |
| [docs/WEBGPU_WARMUP.md](docs/WEBGPU_WARMUP.md) | Rust/math exercises before the GPU |
| [docs/STEPS.md](docs/STEPS.md) | Steps 0–7 — implementation guide |
| [docs/ROADMAP.md](docs/ROADMAP.md) | Phases, pace, blog hooks, session template |
| [docs/PROGRESS.md](docs/PROGRESS.md) | Your checklist + session log |

## Quick verify

```bash
cargo test -p rust-webgpu
cargo run -p rust-webgpu
```

## Tutor mode

[AGENTS.md](AGENTS.md) · Shared workflow: [../docs/AI_LEARNING_WORKFLOW.md](../docs/AI_LEARNING_WORKFLOW.md)
