<div align="center">

# 🚀 Codex Manager

**Cross-Platform Multi-Account Isolation Manager for Codex App**

Created & Maintained by **[@tianrking](https://github.com/tianrking)**

[![Author](https://img.shields.io/badge/Author-tianrking-black?style=for-the-badge&logo=github)](https://github.com/tianrking)
[![CI Status](https://img.shields.io/github/actions/workflow/status/tianrking/CodexManager/ci.yml?branch=main&style=for-the-badge&logo=github-actions&logoColor=white&label=CI)](https://github.com/tianrking/CodexManager/actions)
[![Tauri 2.0](https://img.shields.io/badge/Tauri-2.0-blue?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app/)
[![Rust Engine](https://img.shields.io/badge/Rust-Backend-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Vite](https://img.shields.io/badge/Vite-Frontend-646CFF?style=for-the-badge&logo=vite&logoColor=white)](https://vitejs.dev/)
[![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey?style=for-the-badge)](https://github.com/tianrking/CodexManager)
[![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)](LICENSE)

<p align="center">
  Concurrent Multi-Account Support • Independent Session Storage • Native System Access
</p>

---

[ **English** | [简体中文](README_CN.md) | [Español](README_ES.md) ]

</div>

---

## 📖 Introduction

Official Codex Desktop Apps store authentication tokens, configuration, and Device IDs in a shared global directory (`~/.codex` or system Keyrings). Logging into a different account overwrites the local session and forces existing instances offline.

**Codex Manager** is a desktop utility built with **Tauri 2** and **Rust**. It allows running multiple Codex Desktop App instances concurrently by redirecting environment variables (`HOME`, `CODEX_HOME`, `TMPDIR`, and `--user-data-dir`) per profile.

The tool isolates authentication sessions while allowing instances to retain access to host project files, terminal environment, Git/SSH configurations, and Docker socket.

---

## 🎯 Use Cases

- **Work & Personal Accounts**: Run enterprise and personal accounts simultaneously without logging out.
- **Client Project Isolation**: Keep client repositories and authentication tokens separated into distinct profiles.
- **Rate Limit & Quota Rotation**: Switch to an alternative account profile when reaching quota or rate limits.
- **Unrestricted Local Environment**: Keep host `~/.gitconfig` and `~/.ssh` access so AI Agents can commit code and access remotes normally.

---

## ✨ Features

- 🌐 **GUI Language Switcher**: Built-in support for English, 简体中文, and Español.
- 🔄 **Multi-Instance Support**: Open multiple Codex windows with independent session directories.
- 🛡️ **Environment Isolation**: Separate `HOME`, `CODEX_HOME`, `--user-data-dir`, `TMPDIR`, and cache folders per profile.
- 🔑 **Git & SSH Linking**: Automatically links host `~/.gitconfig` and `~/.ssh` into profile directories.
- ⚡ **Process Termination**: Stops master processes and helper sub-processes via path-matched `pkill -9 -f`.
- 📁 **Folder Drag & Drop**: Drag a project folder onto a profile card to launch Codex scoped to that directory.
- 🔍 **Data Directory Access**: Open a profile's physical data folder in Finder, File Explorer, or xdg-open.

---

## 📐 Isolation Mechanics

```
                        +---------------------------------------+
                        |     Codex Manager GUI (Tauri App)     |
                        +-------------------+-------------------+
                                            |
                                            v
                        +---------------------------------------+
                        |        Rust Process Engine            |
                        +-------------------+-------------------+
                                            |
         +----------------------------------+----------------------------------+
         |                                  |                                  |
 [ Profile: Work ]                 [ Profile: Personal ]             [ Profile: Client-A ]
 - HOME: ~/.codex_manager/p1       - HOME: ~/.codex_manager/p2       - HOME: ~/.codex_manager/p3
 - UserData: .../p1/userdata       - UserData: .../p2/userdata       - UserData: .../p3/userdata
 - Temp Socket: .../p1/tmp         - Temp Socket: .../p2/tmp         - Temp Socket: .../p3/tmp
         |                                  |                                  |
         +----------------------------------+----------------------------------+
                                            |
                                            v
                        [ Host System Resources & Git/Docker ]
```

### Technical Redirections

| Dimension | Redirection Parameter | Purpose |
| :--- | :--- | :--- |
| **Auth & Session** | `--user-data-dir=<profile>/userdata` | Isolates session storage and bypasses single-instance lock. |
| **Global Config** | `HOME=<profile>` & `CODEX_HOME=<profile>/.codex` | Isolates global CLI settings and Device ID generation. |
| **Shared IPC Socket**| `TMPDIR=<profile>/tmp` (macOS/Linux) / `TMP` (Win) | Prevents Electron instances from sharing IPC sockets. |
| **Keyring Collision** | `NODE_KEYRING_DISABLE=1` & `--password-store=basic` | Uses file-based token storage to prevent keyring collisions. |
| **Git/SSH Preservation**| Symlink `~/.gitconfig` & `~/.ssh` | Maintains host Git and SSH credentials for local commits. |

---

## 🛠️ Build & Installation

### Prerequisites
- [Node.js](https://nodejs.org/) (v18+)
- [Rust & Cargo](https://www.rust-lang.org/) (1.75+)

### Development Mode

```bash
git clone https://github.com/tianrking/CodexManager.git
cd CodexManager

npm install
npx tauri dev
```

### Building Release

```bash
npm run build
npx tauri build
```

Standalone packages (`.app`, `.dmg`, `.exe`, `.msi`, `.deb`, `.AppImage`) will be generated under `src-tauri/target/release/bundle/`.

---

## 📄 License

Distributed under the MIT License. Maintained by **[@tianrking](https://github.com/tianrking)**. See [`LICENSE`](LICENSE) for details.
