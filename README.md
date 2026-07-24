<div align="center">

# 🚀 Codex Manager

**Cross-Platform Multi-Account & Profile Isolation Manager for Codex App**

[![Tauri 2.0](https://img.shields.io/badge/Tauri-2.0-blue?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app/)
[![Rust Engine](https://img.shields.io/badge/Rust-Backend-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Vite](https://img.shields.io/badge/Vite-Frontend-646CFF?style=for-the-badge&logo=vite&logoColor=white)](https://vitejs.dev/)
[![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey?style=for-the-badge)](https://github.com/)
[![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)](LICENSE)

<p align="center">
  <b>Concurrent Multi-Account Support</b> • <b>Zero Session Overwrite</b> • <b>Unrestricted Native Agent Capabilities</b>
</p>

---

[ **English** | [简体中文](README_CN.md) | [Español](README_ES.md) ]

</div>

---

## 💡 Why Codex Manager?

When running official **Codex Desktop Apps** (or Electron-based AI Agents), credentials and Device IDs are typically stored in fixed global directories (`~/.codex` or System Keyrings). 

Switching between Work and Personal accounts usually **forces session overwrites**, causing accounts to be kicked offline. Furthermore, running isolated sandboxes (like VMs) often cripples the Agent's ability to edit host project files or run terminal/Docker commands.

**Codex Manager** solves this dilemma elegantly using **Tauri 2 + Rust** by providing:

1. **Physical Credential & Profile Isolation**: Run multiple Codex Desktop App instances concurrently with zero session conflicts or kicks.
2. **Unrestricted Native Agent Capabilities**: The Agent retains full access to host project files, Shell environment, Git/SSH credentials, and Docker Socket.
3. **Ultra-Lightweight & Blazing Fast**: Native Rust engine with ~6MB package size and 0ms launch delay.

---

## ✨ Key Features

- 🔄 **Concurrent Multi-Account Instances**: Run Account A (Work) and Account B (Personal) side-by-side without getting logged out.
- 🛡️ **Complete Environment Isolation**: Isolated `HOME`, `CODEX_HOME`, `--user-data-dir`, `TMPDIR`, and Chromium caches per profile.
- 🔑 **Automatic Git & SSH Inheritance**: Automatically links host `~/.gitconfig` and `~/.ssh` into profile environments, preserving Agent CLI capabilities.
- ⚡ **Precision Process Tree Termination**: Cleanly stops master processes and all Electron Helper/Renderer sub-processes via path-matched `pkill -9 -f`.
- 📁 **Folder Drag & Drop**: Drag any project folder directly onto a profile card to launch Codex immediately.
- 🔍 **One-Click Data Directory Access**: Directly open the profile's isolated data directory in Finder / File Explorer.
- 🎨 **Modern Glassmorphic UI**: Gorgeous Dark/Light responsive UI with real-time PID status indicators and customizable badge colors.

---

## 📐 Architecture & Isolation Mechanics

```
                        +---------------------------------------+
                        |     Codex Manager GUI (Tauri App)     |
                        +-------------------+-------------------+
                                            |
                                            v
                        +---------------------------------------+
                        |        Rust Isolation Engine          |
                        +-------------------+-------------------+
                                            |
         +----------------------------------+----------------------------------+
         |                                  |                                  |
 [ Profile: Work ]                 [ Profile: Personal ]             [ Profile: Client-A ]
 - HOME: ~/.codex_manager/p1       - HOME: ~/.codex_manager/p2       - HOME: ~/.codex_manager/p3
 - UserData: .../p1/userdata       - UserData: .../p2/userdata       - UserData: .../p3/userdata
 - Temp Socket: .../p1/tmp         - Temp Socket: .../p2/tmp         - Temp Socket: .../p3/tmp
 - Auth Token: Session 1           - Auth Token: Session 2           - Auth Token: Session 3
         |                                  |                                  |
         +----------------------------------+----------------------------------+
                                            |
                                            v
                        [ Host System Resources & Git/Docker ]
```

### Isolation Specification

| Dimension | Mechanism | Purpose |
| :--- | :--- | :--- |
| **Auth & Session** | `--user-data-dir=<profile>/userdata` | Prevents credential overwrites and single-instance lock. |
| **Global Config** | `HOME=<profile>` & `CODEX_HOME=<profile>/.codex` | Isolates global CLI settings and Device ID generation. |
| **Shared IPC Socket**| `TMPDIR=<profile>/tmp` (macOS/Linux) / `TMP` (Win) | Prevents Electron instances from sharing IPC sockets. |
| **Keyring Override** | `NODE_KEYRING_DISABLE=1` & `--password-store=basic` | Forces file-based token storage, bypassing system Keyring collisions. |
| **Git/SSH Preservation**| Symlink `~/.gitconfig` & `~/.ssh` | Ensures Agent can commit code and access remotes seamlessly. |

---

## 🛠️ Prerequisites & Installation

### Prerequisites
- [Node.js](https://nodejs.org/) (v18+)
- [Rust & Cargo](https://www.rust-lang.org/) (1.75+)

### Quick Start (Development Mode)

```bash
# Clone the repository
git clone https://github.com/your-username/CodexManager.git
cd CodexManager

# Install dependencies
npm install

# Run application in development mode
npx tauri dev
```

### Production Build

To compile a standalone binary (`.app`, `.dmg`, `.exe`, `.msi`, `.deb`, `.AppImage`):

```bash
# Build frontend and compile Rust release binary
npm run build
npx tauri build
```

The output bundle will be located in `src-tauri/target/release/bundle/`.

---

## 📖 Usage Guide

1. **Add Profile**: Click **"新增账号 / Add Profile"**, enter the profile name (e.g., Work), and choose a color.
2. **Launch Instance**: Click **"▶ 独立启动 / Launch"** on any card. An isolated Codex Desktop App window will open.
3. **Log In**: Log in with your account. The Session Token will be locked into that profile's directory permanently.
4. **Drag & Drop Folder**: Drag any local project folder directly onto a profile card to launch Codex scoped to that project.
5. **Stop Instance**: Click **"■ 停止运行 / Stop"** to cleanly terminate the process tree for that profile.

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/your-username/CodexManager/issues).

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'feat: Add some AmazingFeature'`)
4. Push to the Branch (`git checkout -b feature/AmazingFeature`)
5. Open a Pull Request

---

## 📄 License

Distributed under the MIT License. See [`LICENSE`](LICENSE) for more information.
