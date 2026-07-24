<div align="center">

# 🚀 Codex Manager

**Cross-Platform Multi-Account & Profile Isolation Manager for Codex App**

Created & Maintained by **[@tianrking](https://github.com/tianrking)**

[![Author](https://img.shields.io/badge/Author-tianrking-black?style=for-the-badge&logo=github)](https://github.com/tianrking)
[![CI Status](https://img.shields.io/github/actions/workflow/status/tianrking/CodexManager/ci.yml?branch=main&style=for-the-badge&logo=github-actions&logoColor=white&label=CI)](https://github.com/tianrking/CodexManager/actions)
[![Tauri 2.0](https://img.shields.io/badge/Tauri-2.0-blue?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app/)
[![Rust Engine](https://img.shields.io/badge/Rust-Backend-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Vite](https://img.shields.io/badge/Vite-Frontend-646CFF?style=for-the-badge&logo=vite&logoColor=white)](https://vitejs.dev/)
[![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey?style=for-the-badge)](https://github.com/tianrking/CodexManager)
[![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)](LICENSE)

<p align="center">
  <b>Concurrent Multi-Account Support</b> • <b>Zero Session Overwrite</b> • <b>Unrestricted Native Agent Capabilities</b>
</p>

---

[ **English** | [简体中文](README_CN.md) | [Español](README_ES.md) ]

</div>

---

## 📖 Overview & Problem Statement

Official **Codex Desktop Apps** (and Electron-based AI coding agents) store authentication tokens, session states, and Device IDs in fixed global system locations (such as `~/.codex` or System Keyrings/Keychain).

This architecture causes significant friction for developers:
1. **Account Conflict**: Logging into Account B instantly overwrites the local session for Account A, forcing account A offline.
2. **Handicapped Sandboxes**: Traditional virtualization or container sandboxes (like VMs) isolate accounts but **cripple the AI Agent's core capabilities**—preventing it from editing host files, running shell commands, or interacting with host Docker daemons.

**Codex Manager** by **[@tianrking](https://github.com/tianrking)** addresses these challenges by combining **Tauri 2 + Rust** with physical environment redirection. It enables **concurrent multi-account execution** while preserving **100% of the AI Agent's native operating system capabilities**.

---

## 🎯 Target Use Cases & Scenarios

### Scenario 1: Work vs. Personal Account Coexistence
- **Problem**: Developers need to run enterprise Codex accounts for company projects and personal accounts for side projects simultaneously.
- **Solution**: Create a `Work` profile and a `Personal` profile in Codex Manager. Both instances run side-by-side without kicking each other offline.

### Scenario 2: Multi-Client Project Isolation
- **Problem**: Freelancers and agencies managing multiple client repositories need strict credential separation to avoid cross-project token leakage.
- **Solution**: Bind specific project directories to dedicated client profiles. Launching a profile automatically opens Codex scoped to that client's repository with isolated tokens.

### Scenario 3: Instant Account Rotation Upon Quota Limits
- **Problem**: Encountering rate limits or quota depletion on one account halts active development.
- **Solution**: Keep secondary fallback accounts configured as profiles. Switch or launch an alternate profile in 0 milliseconds without re-authenticating.

### Scenario 4: Enterprise Unrestricted Agent Workflow
- **Problem**: Agent sandboxes that prevent access to host terminal or Git configs break automated workflows.
- **Solution**: Codex Manager automatically links host `~/.gitconfig` and `~/.ssh` into profile environments, allowing Agents to commit code, push to remote repos, and manage host Docker containers natively.

---

## ✨ Features Breakdown

- 🔄 **Concurrent Multi-Instance Execution**: Open multiple official Codex Desktop App windows simultaneously with independent Auth sessions.
- 🛡️ **Complete Physical Isolation**: Isolates `HOME`, `CODEX_HOME`, `--user-data-dir`, `TMPDIR`, Chromium caches, and crash dumps per profile.
- 🔑 **Automatic Git & SSH Inheritance**: Preserves host `~/.gitconfig` and `~/.ssh` credentials, ensuring seamless CLI, Git, and remote operations for AI Agents.
- ⚡ **Precision Process Tree Cleanup**: Employs path-matched `pkill -9 -f` regex termination to cleanly kill master processes and all Electron Helper/Renderer sub-processes.
- 📁 **Drag & Drop Folder Launch**: Drag any local project folder directly onto a profile card to launch Codex scoped to that directory.
- 🔍 **One-Click Data Folder Access**: Instantly open the profile's physical storage directory in Finder (macOS), Explorer (Windows), or xdg-open (Linux).
- 🎨 **Modern Glassmorphic UI**: Responsive Dark/Light UI with real-time PID status badges and customizable profile icons.

---

## 📐 Architecture & Isolation Mechanics

```
                        +---------------------------------------+
                        |     Codex Manager GUI (Tauri App)     |
                        |          Author: @tianrking           |
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

### Deep Technical Isolation Matrix

| Dimension | Technical Redirection | Solved Pain Point |
| :--- | :--- | :--- |
| **Auth & Session** | `--user-data-dir=<profile>/userdata` | Prevents credential overwrites and single-instance locks. |
| **Global Config** | `HOME=<profile>` & `CODEX_HOME=<profile>/.codex` | Isolates global CLI settings and Device ID generation. |
| **Shared IPC Socket**| `TMPDIR=<profile>/tmp` (macOS/Linux) / `TMP` (Win) | Prevents Electron instances from sharing IPC sockets. |
| **Keyring Collision** | `NODE_KEYRING_DISABLE=1` & `--password-store=basic` | Forces file-based token storage, bypassing Keyring collisions. |
| **Git/SSH Preservation**| Symlink `~/.gitconfig` & `~/.ssh` | Ensures Agent can commit code and access remotes seamlessly. |

---

## 🔒 Privacy & Security Guarantee

- **Local-First Architecture**: All profile configurations, session directories, and tokens remain **100% on your local machine** (`~/.codex_manager/`).
- **Zero Telemetry / Analytics**: Codex Manager does not collect, transmit, or store any personal credentials or telemetry.
- **Open Source Security**: Built with Rust and Tauri 2.0 with minimal external dependencies, ensuring full transparency and auditability.

---

## 🛠️ Prerequisites & Installation

### Prerequisites
- [Node.js](https://nodejs.org/) (v18+)
- [Rust & Cargo](https://www.rust-lang.org/) (1.75+)

### Quick Start (Development Mode)

```bash
# Clone repository
git clone https://github.com/tianrking/CodexManager.git
cd CodexManager

# Install dependencies
npm install

# Run desktop app in dev mode
npx tauri dev
```

### Production Build

To compile a standalone binary (`.app`, `.dmg`, `.exe`, `.msi`, `.deb`, `.AppImage`):

```bash
# Build frontend and compile Rust release binary
npm run build
npx tauri build
```

The output bundle will be generated in `src-tauri/target/release/bundle/`.

---

## 📄 License

Distributed under the MIT License. Created & Maintained by **[@tianrking](https://github.com/tianrking)**. See [`LICENSE`](LICENSE) for details.
