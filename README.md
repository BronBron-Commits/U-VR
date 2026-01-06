# U-VR

**U-VR** is a minimal third-person 3D prototype built in **Rust + wgpu + winit**. It focuses on establishing a clean, extensible foundation for movement, camera control, and rendering without the overhead of a heavy game engine.

This repository prioritizes **architectural correctness** and **input separation**, making it a solid baseline for engine development, physics experimentation, or custom game logic.

---

## 🚀 Current Features

### 🎥 Camera System

* **Orbit Camera:** Traditional third-person behavior.
* **Mouse Interaction:** Orbital rotation (Middle Mouse) and smooth zooming (Scroll).
* **Clamped Pitch:** Prevents gimbal lock or unnatural flipping.
* **Decoupled Logic:** The camera follows the player position but maintains its own independent orientation.

### 🏃 Player Movement

* **Camera-Relative WASD:** Movement is calculated based on the camera’s current yaw.
* **Directional Alignment:** Player avatar (cube) rotates to face the direction of movement.
* **Verticality:** Jump mechanics with gravity and grounded detection.

### 🛠️ Rendering & Architecture

* **wgpu Pipeline:** Modern, explicit graphics API usage.
* **Input Separation:** Clean boundaries between raw window events, movement logic, and camera state.
* **Spatial Reference:** Includes a procedural grid-based ground plane for depth perception.

---

## 🎮 Controls

| Input | Action |
| --- | --- |
| **W / A / S / D** | Move (relative to camera) |
| **Space** | Jump |
| **Middle Mouse (Hold)** | Rotate Camera |
| **Mouse Wheel** | Zoom In/Out |
| **Esc / Window Close** | Exit Application |

---

## 📂 Project Structure

```text
U-VR/
├── game/
│   └── client/
│       └── src/
│           └── main.rs         # Entry point (Thin layer)
└── engine/
    └── render/
        └── src/
            ├── app.rs          # Event loop & Input orchestration
            └── renderer/
                ├── mod.rs      # Player movement & Physics state
                ├── frame/      # Per-frame rendering logic
                ├── uniforms/   # Camera buffers & GPU data
                ├── resources/  # Procedural mesh generation
                └── pipeline/   # wgpu pipeline configuration

```

---

## 🧠 Design Philosophy

1. **No Magic:** Every transformation is explicit; no hidden engine "black boxes."
2. **Input as Data:** Movement and Camera systems consume input data rather than owning it.
3. **Independence:** The camera never infers intent from movement keys.
4. **Clarity over Abstraction:** CPU-side correctness is prioritized over premature GPU optimizations.

---

## 🛠 Building & Running

### Requirements

* **Rust** (Stable)
* **GPU:** Vulkan, DX12, or Metal capable hardware.
* **OS:** Windows, Linux, or macOS.

### Execution

```bash
# Run the client application
cargo run --bin client

```

---

## 📈 Roadmap & Extensions

* [ ] **Camera Damping:** Add interpolation for smoother motion.
* [ ] **Collision:** Prevent camera/player clipping through geometry.
* [ ] **Character Controller:** Transition from a simple cube to a capsule controller.
* [ ] **Instancing:** Optimized rendering for many objects.
* [ ] **Physics Integration:** Plug in a crate like `rapier3d`.

---

## ⚖️ License

Distributed under the **MIT License**. See `LICENSE` for more information.

---

**Status:** 🟢 *Stable baseline. Architecturally decoupled and ready for expansion.*
