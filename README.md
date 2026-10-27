# Illustrator RS — Sprint 01

Native vector editor prototype for Windows, built with Rust and egui/eframe.

## Requirements
- Rust stable via rustup (MSVC toolchain on Windows)
- Visual Studio Build Tools with C++ desktop workload and Windows SDK

## Run
```powershell
cargo test --workspace
cargo run -p illustrator-desktop
```

## Usage
- Rectangle: choose Rectangle tool and drag on the white artboard.
- Select: choose Select tool and click a rectangle.
- Move: drag a selected or unselected rectangle with the Select tool.
- Undo: Ctrl+Z; Redo: Ctrl+Y (or Ctrl+Shift+Z).

## Architecture
- `illustrator-document`: document objects and geometry.
- `illustrator-editor`: commands, selection, undo/redo.
- `illustrator-desktop`: native UI, mouse input, egui rendering.

## Sprint 01 limitations
No zoom/pan, resizing, serialization, paths, SVG, or custom wgpu renderer yet.

## Verification
Source files were generated, but not compiled in the authoring environment because Rust/Cargo were unavailable there. Run `cargo test --workspace` and `cargo run -p illustrator-desktop` on Windows to validate.

## GitHub Actions: build Windows executable without local Rust

1. Create a GitHub repository and upload **the contents** of this `illustrator-rs` directory to the repository root (including `.github/workflows/windows-build.yml`).
2. Push to `main` or `master`, or open **Actions → Windows Build → Run workflow**.
3. Wait for the `Test and build Windows x64` job to finish.
4. On the workflow run page, scroll to **Artifacts** and download `IllustratorRS-Windows-x64`.
5. Extract the downloaded artifact, then extract `IllustratorRS-Windows-x64.zip` inside it. Launch `IllustratorRS.exe` on Windows x64.

The workflow runs `cargo check --workspace`, `cargo test --workspace`, and a release build targeting `x86_64-pc-windows-msvc`. It uses the GitHub-hosted Windows runner, so no Rust installation on your own PC is necessary. The build artifact is **not** a GitHub Release and is not code-signed. Windows SmartScreen may show a warning for an unsigned executable.

For reproducible dependencies, generate and commit `Cargo.lock` once a successful build has created it. You can then add `--locked` to the Cargo commands in the workflow.

**Note:** The source and workflow have not been executed in this environment. GitHub Actions is the build verification source; inspect its logs if a step fails.
