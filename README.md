Below is a **clean, project-appropriate README** you can drop straight into the repo root as `README.md`.
It reflects the **current stable state** you just tagged (procedural skybox, full avatar, compass, renderer baseline) without over-promising.

---

# U-VR

**U-VR** is an experimental Rust-based 3D engine and client focused on **clean renderer architecture**, **procedural visuals**, and **low-level control** over the entire rendering pipeline.

This project is part of the broader **Unhidra** ecosystem and serves as a proving ground for renderer design, camera systems, and world interaction without relying on heavyweight game engines.

---

## ✨ Current Features

### Renderer

* Procedural **skybox** (no textures, no cubemaps)
* Depth-buffered world rendering
* Stable render pass ordering:

  1. Skybox
  2. World geometry
  3. Overlay / HUD

### World

* Grid floor
* Composite **avatar built from multiple primitives**
* Correct depth testing from all camera angles

### Camera

* Orbit camera with yaw control
* Target-following behavior
* Perspective projection

### Overlay

* On-screen **compass / axis indicator**
* Rendered as an overlay pass
* Independent of world depth

---

## 🧱 Architecture Overview

The renderer is intentionally **modular and explicit**:

```
renderer/
├─ context/        # Device, surface, depth, swapchain
├─ pipeline/       # World, overlay, skybox pipelines
├─ frame/          # FrameRenderer + render passes
├─ resources/      # Meshes, vertices, primitives
├─ uniforms/       # Camera and other GPU uniforms
├─ skybox/         # Procedural skybox system
└─ overlay/        # Compass / HUD rendering
```

### Key Design Principles

* No hidden global state
* Pipelines are created once, reused per frame
* Explicit render passes (no magic ordering)
* Minimal GPU abstractions on top of `wgpu`

---

## 🚀 Getting Started

### Prerequisites

* Rust (stable)
* Cargo
* A GPU with Vulkan / DX12 / Metal support

### Build & Run

```bash
cargo run --bin client
```

---

## 🏷️ Versioning

The current stable renderer milestone is tagged:

```
v0.3.0
```

This tag represents:

* Procedural skybox integrated
* Full avatar rendering restored
* Compass overlay working
* Renderer architecture stabilized

---

## 🧪 Experimental Workflow

Development follows a **branch → validate → merge → tag** flow:

* Experimental features are developed on feature branches
* Only validated, stable changes are merged into `main`
* Major milestones are tagged for easy rollback

---

## 🔮 Roadmap (Short Term)

* Avatar mesh caching
* Simple animation (idle / walk)
* Skybox time-of-day variation
* Debug gizmos using the overlay system
* World object streaming experiments

---

## 📜 License

MIT (or update if different).

---

## ✍️ Author

Bronson Manley
GitHub: [BronBron-Commits](https://github.com/BronBron-Commits)

---

If you want, next we can:

* tighten the README for public visibility
* add architecture diagrams
* write a CONTRIBUTING.md
* or tag this README update as part of a new release
