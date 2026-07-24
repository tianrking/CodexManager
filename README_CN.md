<div align="center">

# 🚀 Codex Manager

**Codex 桌面 App 跨平台多账号隔离管理器**

[![Tauri 2.0](https://img.shields.io/badge/Tauri-2.0-blue?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app/)
[![Rust Engine](https://img.shields.io/badge/Rust-后端引擎-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Vite](https://img.shields.io/badge/Vite-前端框架-646CFF?style=for-the-badge&logo=vite&logoColor=white)](https://vitejs.dev/)
[![Platform](https://img.shields.io/badge/平台-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey?style=for-the-badge)](https://github.com/)
[![License](https://img.shields.io/badge/开源协议-MIT-green?style=for-the-badge)](LICENSE)

<p align="center">
  <b>多账号并发在线</b> • <b>零凭据覆盖/0踢下线</b> • <b>Agent 原生能力零损耗</b>
</p>

---

[ [English](README.md) | **简体中文** | [Español](README_ES.md) ]

</div>

---

## 💡 为什么选择 Codex Manager？

当运行官方的 **Codex 桌面客户端**（或 Electron 架构的 AI Agent）时，凭据与设备 ID 通常保存在固定的本地全局存储中（如 `~/.codex` 或系统 Keyring 凭据管理器）。

在工作账号与个人账号之间切换往往会**强制覆盖本地 Session**，导致已登录的账号被踢下线。此外，传统的虚拟机或沙箱隔离方案往往会阉割 Agent 的核心能力，使其无法读写宿主机项目文件或调用终端/Docker 引擎。

**Codex Manager** 采用 **Tauri 2 + Rust** 优雅地解决了这一难题：

1. **凭据与 Profile 物理隔绝**：在同一台电脑上并发运行多个 Codex 桌面 App 实例，凭据互不干扰，零踢下线。
2. **保全 Agent 原生能力**：Agent 继承对宿主机项目文件、Shell 环境、Git/SSH 凭据以及 Docker Socket 的完整访问权限。
3. **极致轻量与极速**：基于原生 Rust 引擎，打包体积仅约 **6MB**，启动延迟 **0 毫秒**。

---

## ✨ 核心特性

- 🔄 **多账号并发多开**：同时运行工作账号与个人账号，各自保持在线，不互相影响。
- 🛡️ **彻底的环境隔离**：为每个 Profile 独立隔离 `HOME`、`CODEX_HOME`、`--user-data-dir`、`TMPDIR` 以及 Chromium 缓存。
- 🔑 **自动继承 Git & SSH 配置**：自动为 Profile 软链接宿主机的 `~/.gitconfig` 与 `~/.ssh`，确保 Agent 命令行提交与远程仓库访问无缝顺畅。
- ⚡ **精准进程树清理**：通过命令行全路径正则匹配与 `pkill -9 -f`，干净干掉主进程及所有 Helper/Renderer 子进程。
- 📁 **拖拽文件夹启动**：将任意本地项目文件夹直接拖放到 Profile 卡片上即可立即调起对应账号。
- 🔍 **数据目录一键直达**：在 Finder / 文件资源管理器中一键定位并打开该 Profile 的数据存储路径。
- 🎨 **现代毛玻璃 UI**：支持暗黑/明亮自适应主题、PID 状态指示灯与自定义色彩标记。

---

## 📐 架构与隔离机制

```
                        +---------------------------------------+
                        |     Codex Manager GUI (Tauri 应用)    |
                        +-------------------+-------------------+
                                            |
                                            v
                        +---------------------------------------+
                        |         Rust 物理隔离调度引擎         |
                        +-------------------+-------------------+
                                            |
         +----------------------------------+----------------------------------+
         |                                  |                                  |
 [ Profile: 工作账号 ]              [ Profile: 个人账号 ]             [ Profile: 客户账号 ]
 - HOME: ~/.codex_manager/p1       - HOME: ~/.codex_manager/p2       - HOME: ~/.codex_manager/p3
 - UserData: .../p1/userdata       - UserData: .../p2/userdata       - UserData: .../p3/userdata
 - Temp Socket: .../p1/tmp         - Temp Socket: .../p2/tmp         - Temp Socket: .../p3/tmp
 - Auth Token: Session 1           - Auth Token: Session 2           - Auth Token: Session 3
         |                                  |                                  |
         +----------------------------------+----------------------------------+
                                            |
                                            v
                        [ 宿主机资源 & Git/SSH/Docker 管道 ]
```

### 隔离维度说明

| 隔离维度 | 技术实现机制 | 解决的痛点 |
| :--- | :--- | :--- |
| **Auth & Session** | `--user-data-dir=<profile>/userdata` | 阻断凭据覆盖，解除 Electron 单例锁定。 |
| **全局配置** | `HOME=<profile>` & `CODEX_HOME=<profile>/.codex` | 隔离全局 CLI 配置与 Device ID 识别码。 |
| **共享 IPC Socket**| `TMPDIR=<profile>/tmp` (macOS/Linux) / `TMP` (Win) | 避免不同 Electron 实例共享临时 Socket。 |
| **Keyring 覆写** | `NODE_KEYRING_DISABLE=1` & `--password-store=basic` | 强制采用纯文件存储，避开系统 Keyring 冲突。 |
| **Git/SSH 保全** | 自动软链接 `~/.gitconfig` & `~/.ssh` | 保证 Agent 可以顺利提交代码与连接远程仓库。 |

---

## 🛠️ 环境准备与安装

### 环境要求
- [Node.js](https://nodejs.org/) (v18+)
- [Rust & Cargo](https://www.rust-lang.org/) (1.75+)

### 快速开始 (开发模式)

```bash
# 克隆仓库
git clone https://github.com/your-username/CodexManager.git
cd CodexManager

# 安装依赖
npm install

# 启动桌面 GUI 开发模式
npx tauri dev
```

### 打包编译

打包可执行程序（包含 `.app`, `.dmg`, `.exe`, `.msi`, `.deb`, `.AppImage`）：

```bash
npm run build
npx tauri build
```

打包文件将生成在 `src-tauri/target/release/bundle/` 目录中。

---

## 📖 使用指南

1. **新建账号**：点击 **“新增账号”**，输入 Profile 名称（如 Work），挑选代表色彩。
2. **独立启动**：点击卡片上的 **“▶ 独立启动”**，系统会弹出隔离的 Codex 桌面 App 窗口。
3. **登录账号**：在弹出的窗口中登录账号，Auth Token 将永久锁定在该 Profile 目录中。
4. **拖拽文件夹**：拖拽任意本地项目文件夹到卡片上，自动用该账号打开指定项目。
5. **停止运行**：点击 **“■ 停止运行”**，精准清理该 Profile 的全套子进程。

---

## 📄 开源协议

本项目采用 [MIT License](LICENSE) 开源协议。
