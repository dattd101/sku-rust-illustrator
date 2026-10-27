# Illustrator RS v0.3 — Light Gray UI + Next.js 15 + Rust + Tauri 2

Windows native desktop app using a statically exported Next.js UI and Rust document/editor core.

## GitHub Actions
Push the repository contents (including the `web/` folder) to `main`, then open **Actions → Windows Desktop Build**. Download the `IllustratorRS-Windows-x64` artifact. The EXE uses Microsoft Edge WebView2 runtime (typically preinstalled on Windows 10/11).

## Development
- Install Node.js 22, Rust stable MSVC, Visual Studio C++ Build Tools, WebView2.
- `npm install --prefix web`
- `npm run build --prefix web`
- `cargo test -p illustrator-document -p illustrator-editor`
- `cargo run -p illustrator-desktop`

## Features
Rectangle creation (M + drag), select/move (V), pan (Hand tool or middle mouse), wheel pan, Ctrl+wheel zoom, layers selection, Ctrl+Z/Ctrl+Y history.

Next.js is exported as static assets. No Next server or Node runtime is required for the packaged desktop app.

## Known limitations
Document is in-memory; no save/load yet. Browser-only preview cannot mutate Rust state. Tauri packaging and full Windows compilation must be validated in GitHub Actions.

## Light Gray UI
The editor now uses a light-gray workspace, white artboard, soft-gray panels, blue selection accents, and light native window background. Existing Rust commands, canvas interactions, and Windows GitHub Actions are retained.

## GitHub repository layout

Upload the **contents** of `illustrator-rs/` to the repository root. The repository root must contain `Cargo.toml`, `web/package.json`, and `.github/workflows/windows-build.yml`. The workflow uses `working-directory: web` for npm commands.
