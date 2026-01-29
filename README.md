# Turbo Waffle
<img width="848" height="507" alt="image" src="https://github.com/user-attachments/assets/f24aedf8-76e3-4a00-a851-a3a1e1f777fe" />

A cross-platform desktop application for managing connections (websites, SSH) with health monitoring and Wake-on-LAN support.

## Features

- **Connection Management**: Store and organize your websites and SSH servers
- **Health Monitoring**: Automatic health checks with real-time status indicators
- **Quick Connect**: Launch SSH connections in your terminal or open websites in your browser with one click
- **Wake-on-LAN**: Wake sleeping machines before connecting via SSH

## Supported Connection Types

| Type | Features |
|------|----------|
| **Website** | HTTP/HTTPS health checks, custom check paths, one-click browser launch |
| **SSH** | TCP health checks, terminal launch, Wake-on-LAN support |

## Tech Stack

- **Backend**: Rust + Tauri 2.0
- **Frontend**: Vue.js 3 + TypeScript
- **Platforms**: Linux, macOS

## Prerequisites

- Node.js 18+
- Rust 1.70+
- System dependencies for Tauri:
  - **Linux**: `webkit2gtk-4.1`, `libappindicator3`, `librsvg2`
  - **macOS**: Xcode Command Line Tools

## Development

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Configuration

Connections are stored in:
- **Linux**: `~/.config/turbo-waffle/connections.json`
- **macOS**: `~/Library/Application Support/com.turbowaffle.turbo-waffle/connections.json`

## Terminal Support (SSH)

On Linux, the app will automatically detect and use the first available terminal:
- GNOME Terminal
- Konsole
- Alacritty
- Kitty
- xterm

On macOS, connections open in Terminal.app.

## Contributors

This project was built with assistance from:

- **Claude** (Anthropic) - AI pair programming assistant
