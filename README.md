# Illustrator RS v0.4 — Rust + Tailwind CSS

Desktop vector editor MVP for Windows. **No Next.js, React, or localhost server in release builds.** Rust (`document`, `editor`, Tauri commands) owns document mutations and history. A static HTML + Tailwind CSS interface uses a small vanilla JavaScript bridge to invoke Rust.

## GitHub Actions
Upload the **contents** of this directory to the repository root. Actions → Windows Desktop Build (Rust + Tailwind CSS) → Run workflow. Download the `IllustratorRS-Windows-x64` artifact. The portable executable requires Microsoft Edge WebView2 Runtime (normally present on Windows 11).

## Local build
```powershell
cd web
npm install
npm run build
cd ..
cargo test -p illustrator-document -p illustrator-editor
cargo build --release -p illustrator-desktop
```

`target/release/illustrator-desktop.exe` embeds the files from `web/dist` via Tauri. It **does not connect to localhost**. Run `npm run build` before `cargo build`.

## Controls
- V: Select / drag to move
- M: Draw rectangle
- H: Pan
- Ctrl+Z / Ctrl+Y: Undo / Redo
- Ctrl+Mouse wheel: Zoom

## Scope
Rectangle editor only; no SVG export, Pen tool, resizing, or save/load yet. The frontend requires a tiny vanilla JavaScript bridge; the document and editing engine remain in Rust.

## Notes
Build is configured for Windows GitHub Actions, but has not been executed on a Windows runner in this environment. `web/dist` is generated in CI and is intentionally not committed.
