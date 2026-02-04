# Air-Link

Air-Link is a high-performance virtual mouse application written in Rust. It utilizes computer vision and AI inference to translate hand gestures into system-level mouse movements and actions. The project is specifically optimized for legacy hardware and supports cross-platform execution on Linux and Windows.

## Technical Architecture

The application is built using a modular architecture:
- **CLI Layer**: Command-line interface for configuration and headless operation.
- **GUI Layer**: Built with `eframe` (egui) for real-time video feed visualization and debugging.
- **Core Logic**: Handles frame capture via `nokhwa`, AI inference via `ort` (ONNX Runtime), and input injection via `enigo`.
- **Vision Module**: Implements a MediaPipe Hand Landmark pipeline for 21-point hand tracking.
- **Input Module**: Manages mouse movement and click gestures with exponential moving average (EMA) smoothing.

## Current Status

The project has completed the following implementation phases:
- **Modular Project Skeleton**: Established a clean module tree for error handling, CLI, core logic, and GUI.
- **AI Inference Pipeline**: Integrated ONNX Runtime for CPU-based hand tracking optimized for legacy AVX instructions.
- **Hardware Integration**: Implemented camera stream capture and multi-monitor coordinate mapping.
- **Gesture System**: Integrated a pinch-to-click gesture mechanism using Euclidean distance analysis.
- **Graphical Interface**: Developed a Wayland-compatible GUI for live monitoring and status reporting.

## Dependencies

### Linux (Fedora/Ubuntu)
The following system libraries are required for compilation and execution:
- `libxdo-devel` (or `libxdo-dev` on Debian-based systems)
- `onnxruntime` (automatically managed by the `ort` crate via dynamic loading)
- `v4l-utils` (for camera device management)

For Wayland support, ensure the user is part of the `input` group to enable `uinput` access:
`sudo usermod -aG input $USER`

## Usage

### CLI Mode (Headless)
To start the application in CLI mode for maximum performance:
```bash
cargo run -- run --camera-id 0 --screen-width 1920 --screen-height 1080
```

### GUI Mode
To start the application with the graphical control panel:
```bash
cargo run -- gui --camera-id 0
```

### Utility Commands
To list available camera devices and their indices:
```bash
cargo run -- list-cameras
```

## Configuration Parameters

- `--camera-id`: Index of the video device.

- `--sensitivity`: Multiplier for mouse movement speed.

- `--screen-width/height`: Resolution of the target display area.

- `--screen-x/y-offset`: Origin coordinates for the target display.



## Multi-Monitor Setup

In multi-monitor environments, the application maps normalized AI coordinates (0.0 to 1.0) to a specific screen region defined by dimensions and offsets.



### Finding your offsets (Linux/Hyprland)

Run `hyprctl monitors` to identify the geometry of your displays. For example, if your secondary monitor is 1920x1080 and positioned to the right of your primary 1080p screen, its coordinates will be `1920x1080@1920,0`.



### Example: Target the second monitor

```bash

cargo run -- run --screen-width 1920 --screen-height 1080 --screen-x-offset 1920

```

This command ensures the "Air Mouse" only operates within the bounds of the second screen.
