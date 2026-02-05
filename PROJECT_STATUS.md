# 📡 PROJECT STATUS: AIR-LINK (Air Mouse)

**Tech Stack:** Rust (2024 Edition)
**Runtime:** ONNX Runtime (ort 2.0.0-rc.11)
**Architecture:** Modular (CLI, GUI, Core, Vision, Input)
**Last Update:** Feb 4, 2026 - Session Closed

## 📋 TODO LIST
- [x] [INIT] Project Skeleton & Modular Architecture
- [x] [DEPS] Setup Nokhwa, Ort, uinput (replaced Enigo), Eframe
- [x] [VISION] AI Pipeline (Hand Landmark ONNX) + Confidence Score (>0.7)
- [x] [INPUT] Native Linux uinput Driver (Relative Delta Logic)
- [x] [INPUT] Multi-monitor support via coordinate offsets
- [x] [ALGO] EMA Smoothing Filter + Central Active Zone (Turbo Mode)
- [x] [GESTURE] Pinch-to-Click implementation
- [x] [GUI] Egui integration for Camera Stream & Monitoring
- [ ] [OPT] Threaded Architecture: Move AI to a separate thread
- [ ] [OPT] Kalman Filter for ultra-smooth tracking

## 🔍 NEXT SESSION: CODE DISSECTION (Mổ Bụng Code)
*   **Topic 1: Ownership & Borrowing**: Review `src/core/app.rs` to understand why structs own their components and how `&mut` references work in the loop.
*   **Topic 2: Error Handling**: Deep dive into `src/error.rs` and the use of `thiserror` + `?` operator for robust code.
*   **Topic 3: AI Data Flow**: Analyze `src/core/vision.rs` - understand CHW layout, normalization (/255.0), and Tensor shapes.
*   **Topic 4: System IO**: Examine `src/core/input.rs` to see how Rust talks to the Linux Kernel via `/dev/uinput`.

## 🔄 RECENT LOGS
| Date | Action | Context |
|---|---|---|
| 2026-02-04 | Turbo Mode | Implemented 0.2-0.8 central zone & sensitivity boost |
| 2026-02-04 | Driver Fix | Replaced Enigo with native `uinput` for Wayland compatibility |
| 2026-02-04 | Phase 1 Done | Completed Core, CLI, and GUI integration |

## ⚙️ CURRENT CONFIGURATION
- **Active Zone**: Center 60% of camera view (0.2 to 0.8)
- **Smoothing**: EMA Alpha 0.2
- **Backend**: Native Linux `uinput` (Wayland/Hyprland Ready)