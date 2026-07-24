<div align="center">

# 🚀 Codex Manager

**Codex 桌面 App 跨平台多账号隔离管理器**

项目作者与维护者：**[@tianrking](https://github.com/tianrking)**

[![Author](https://img.shields.io/badge/作者-tianrking-black?style=for-the-badge&logo=github)](https://github.com/tianrking)
[![CI Status](https://img.shields.io/github/actions/workflow/status/tianrking/CodexManager/ci.yml?branch=main&style=for-the-badge&logo=github-actions&logoColor=white&label=CI)](https://github.com/tianrking/CodexManager/actions)
[![Tauri 2.0](https://img.shields.io/badge/Tauri-2.0-blue?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app/)
[![Rust Engine](https://img.shields.io/badge/Rust-后端引擎-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Vite](https://img.shields.io/badge/Vite-前端框架-646CFF?style=for-the-badge&logo=vite&logoColor=white)](https://vitejs.dev/)
[![Platform](https://img.shields.io/badge/平台-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey?style=for-the-badge)](https://github.com/tianrking/CodexManager)
[![License](https://img.shields.io/badge/开源协议-MIT-green?style=for-the-badge)](LICENSE)

<p align="center">
  多账号并发支持 • 独立 Session 存储 • 宿主机原生能力保留
</p>

---

[ [English](README.md) | **简体中文** | [Español](README_ES.md) ]

</div>

---

## 📖 项目简介

官方 Codex 桌面客户端默认将 Token、配置文件和 Device ID 存在全局固定目录中（如 `~/.codex` 或系统 Keyring）。如果在同一台电脑上登录不同账号，本地 Session 会被覆盖，导致已打开的窗口掉线。

**Codex Manager** 是一个基于 **Tauri 2** 和 **Rust** 开发的桌面小工具。它通过按 Profile 重定向环境变量（`HOME`、`CODEX_HOME`、`TMPDIR` 以及 `--user-data-dir`），实现多个 Codex 桌面 App 窗口的独立运行与并发使用。

工具在隔离凭据的同时，会自动保留宿主机项目文件、终端环境、Git/SSH 配置和 Docker Socket 的访问能力，不影响 Agent 的日常自动化操作。

---

## 🎯 常用场景

- **工作与个人账号共存**：同时打开公司账号与个人账号窗口，互不影响。
- **多项目/客户隔离**：为不同客户或项目独立绑定 Profile，隔离 Auth 凭据。
- **配额用尽切换**：当某个账号触发速率限制时，一键切到备用 Profile 窗口继续开发。
- **保留本地 Git/SSH 环境**：自动关联宿主机的 `~/.gitconfig` 与 `~/.ssh`，确保 Agent 能正常提交代码和连接远程仓库。

---

## ✨ 主要功能

- 🌐 **界面多语言支持**：支持英文、简体中文与西班牙语一键切换。
- 🔄 **多窗口并发**：支持独立调起多个 Codex 桌面 App 窗口。
- 🛡️ **物理目录隔离**：每个 Profile 独立隔离 `HOME`、`CODEX_HOME`、`--user-data-dir` 与 `TMPDIR`。
- 🔑 **Git & SSH 自动关联**：自动为隔离 Profile 软链接宿主机的 `~/.gitconfig` 与 `~/.ssh`。
- ⚡ **进程清理**：根据 Profile 路径精准识别主进程，并清理完整的 Helper/Renderer 子进程树。
- 🖥️ **系统托盘交互**：Windows 右下角托盘可直接启动/停止账号、全部停止、显示或隐藏主窗口；关闭主窗口时自动隐藏到托盘。
- 📁 **拖拽文件夹启动**：将项目文件夹拖放到卡片上即可直接带路径调起。
- 🔍 **查看数据目录**：可在 Finder / 文件资源管理器中打开当前 Profile 的物理存储文件夹。

---

## 📐 隔离机制说明

```
                        +---------------------------------------+
                        |     Codex Manager GUI (Tauri 应用)    |
                        +-------------------+-------------------+
                                            |
                                            v
                        +---------------------------------------+
                        |         Rust 进程调度引擎             |
                        +-------------------+-------------------+
                                            |
         +----------------------------------+----------------------------------+
         |                                  |                                  |
 [ Profile: 工作账号 ]              [ Profile: 个人账号 ]             [ Profile: 客户账号 ]
 - HOME: ~/.codex_manager/p1       - HOME: ~/.codex_manager/p2       - HOME: ~/.codex_manager/p3
 - UserData: .../p1/userdata       - UserData: .../p2/userdata       - UserData: .../p3/userdata
 - Temp Socket: .../p1/tmp         - Temp Socket: .../p2/tmp         - Temp Socket: .../p3/tmp
         |                                  |                                  |
         +----------------------------------+----------------------------------+
                                            |
                                            v
                        [ 宿主机资源 & Git/SSH/Docker 管道 ]
```

### 关键参数说明

| 隔离维度 | 参数/实现方式 | 作用 |
| :--- | :--- | :--- |
| **Auth 与 Session** | `--user-data-dir=<profile>/userdata` | 隔离凭据存储，解除 Electron 单例锁定。 |
| **全局配置** | `HOME=<profile>` & `CODEX_HOME=<profile>/.codex` | 隔离全局 CLI 配置与 Device ID。 |
| **IPC Socket**| `TMPDIR=<profile>/tmp` (macOS/Linux) / `TMP` (Win) | 避免 Electron 实例共享临时 Socket。 |
| **Keyring 冲突** | `NODE_KEYRING_DISABLE=1` & `--password-store=basic` | 使用文件存储凭据，避开系统 Keyring 冲突。 |
| **Git/SSH 保留** | 软链接 `~/.gitconfig` & `~/.ssh` | 保留宿主机 Git/SSH 配置，方便提交代码。 |

---

## 🛠️ 构建与运行

### 环境要求
- [Node.js](https://nodejs.org/) (v20.19+)
- [Rust & Cargo](https://www.rust-lang.org/) (1.75+)
- Windows：已安装 Microsoft Store 版 Codex Desktop，或通过 `CODEX_DESKTOP_PATH` 指定独立版 `ChatGPT.exe` / `Codex.exe`

### Windows 首次启动说明

Microsoft Store 应用的受保护安装目录不允许普通程序直接携带自定义环境启动。Codex Manager 第一次在 Windows 启动 Profile 时，会把当前安装版本的 Codex 程序复制到：

```text
%USERPROFILE%\.codex_manager\runtime\<Store 包版本>\app
```

这份运行时由所有 Profile 共用，账号的 `CODEX_HOME`、AppData、缓存和 Session 仍分别保存在各自的 Profile 目录。首次准备所需空间约等于已安装的 Codex Desktop（当前版本约 1.8 GiB），后续启动无需重复复制；Microsoft Store 更新后，下次启动会按新版本准备一份运行时。

### 开发模式

```bash
git clone https://github.com/tianrking/CodexManager.git
cd CodexManager

npm install
npx tauri dev
```

### 打包 Release

```bash
npm run build
npx tauri build
```

打包文件存放在 `src-tauri/target/release/bundle/` 目录中。

---

## 📄 开源协议

项目采用 [MIT License](LICENSE) 协议。维护者：**[@tianrking](https://github.com/tianrking)**。
