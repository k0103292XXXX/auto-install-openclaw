# auto-install-openclaw

One-click installer for [OpenClaw](https://github.com/openclaw/openclaw) — making AI assistant setup accessible to everyone.

## What is this?

OpenClaw is a powerful open-source AI assistant platform, but installing it requires terminal commands, Node.js setup, and manual configuration. **auto-install-openclaw** is a lightweight GUI installer (~3MB) that handles everything automatically.

## How it works

1. **Download & run** `Setup.exe`
2. **Sign in** with your GitHub account (OAuth)
3. **Auto-check** GitHub Copilot subscription status
4. **Click Install** — Node.js, OpenClaw, and configuration are set up automatically
5. **Done** — Connect via Telegram, Discord, or web chat

## Features

- 🚀 One-click installation — no terminal required
- 🔐 Secure GitHub OAuth (Device Flow)
- ✅ Automatic Copilot subscription verification
- 📦 Auto-installs Node.js if not present
- ⚙️ Auto-generates OpenClaw config with Claude Opus 4 as default LLM
- 🖥️ Optional system startup registration
- 🪶 Lightweight stub installer (~3MB)

## Requirements

- Windows 10/11 (64-bit)
- Internet connection
- GitHub account with [Copilot subscription](https://github.com/features/copilot)

## Tech Stack

- **[Tauri v2](https://v2.tauri.app/)** — Lightweight desktop app framework
- **Rust** — Backend installer logic
- **HTML/CSS/JS** — Frontend UI

## Development

### Prerequisites

- [Rust](https://rustup.rs/)
- [Node.js](https://nodejs.org/) (for Tauri CLI)
- [Tauri CLI](https://v2.tauri.app/start/create-project/)

### Setup

```bash
git clone https://github.com/k0103292xxxx/auto-install-openclaw.git
cd auto-install-openclaw
npm install
cargo tauri dev
```

### Build

```bash
cargo tauri build
```

The installer will be generated in `src-tauri/target/release/bundle/`.

## Roadmap

- [x] Windows installer
- [ ] macOS support
- [ ] Linux support
- [ ] Auto Telegram bot setup
- [ ] Update checker
- [ ] Multi-language UI

## Contributing

PRs welcome! See [SPEC.md](SPEC.md) for the full specification.

## License

[MIT](LICENSE)
