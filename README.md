# Vinyl

Vinyl is a compact Windows desktop music HUD built with Tauri 2. It reads the active Windows System Media Transport Controls (SMTC) session and provides basic playback controls.

## Build

Requires Node.js 22, Rust stable, and Windows build tools.

```bash
npm install
npm run tauri dev
```

## Windows installer

Push to `main` or manually run **Build Vinyl Windows installer** in the Actions tab. The workflow builds a 32-bit NSIS installer and uploads it as the `Vinyl-Setup-Windows-x86` artifact.

The installer is configured for per-machine installation. On 64-bit Windows, the 32-bit application is installed under **Program Files (x86)**.

## Notes

- Requires Windows 10/11 with an active SMTC-compatible media session.
- Media metadata and play/pause/next/previous are read through Windows SMTC.
- Album artwork extraction and desktop-edge drag/miniplayer behavior are not included in this initial build.