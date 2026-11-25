# SpotiRust

Standalone Spotify desktop client built with **Rust (Tauri v2)**, embedded **`librespot` 0.8** audio engine (`rodio` / WASAPI / ALSA), and a zero-npm vanilla HTML/CSS/SVG frontend using embedded **Plus Jakarta Sans** fonts.

## Features
- **Single Portable Executable** — built-in Spotify Connect audio speaker (**"SpotiRust"**) powered by `librespot` + `rodio` (no external player or second binary needed; requires Spotify Premium for playback).
- **Spotify Web API (OAuth 2.0 PKCE)** — login via browser with auto-refreshing tokens stored locally (`~/.config/spotirust/config.json` on Linux, `%APPDATA%\spotirust\config.json` on Windows).
- **Live Playback & Device Picker** — play, pause, next/prev, seek, volume, shuffle, repeat, queue, and seamless device switching.
- **Desktop UX** — resizable sidebars, collapsible *Your Library*, right-click context menus, sticky playlist header, inline playlist search, and dedicated Queue panel.

## Spotify Developer Setup
1. Add Redirect URI `http://127.0.0.1:8888/callback` in your Spotify Developer Dashboard app.
2. Open SpotiRust and click **Log in**.
3. Click the **Device** icon (bottom-right) → **Built-in Speaker (SpotiRust)** to play audio directly from your PC speakers.

## Build from Source

### Fedora Linux
```bash
sudo dnf install webkit2gtk4.1-devel libsoup3-devel librsvg2-devel alsa-lib-devel
cargo run --release
```

### Windows 10/11
Prebuilt portable `spotirust-windows-x64.exe` binaries are built automatically via GitHub Actions on every push and published to **GitHub Releases** on `v*` tags.

## Tests
```bash
cargo test
```
