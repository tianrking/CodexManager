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
  <b>多账号并发在线</b> • <b>零凭据覆盖/0踢下线</b> • <b>Agent 原生能力零损耗</b>
</p>

---

[ [English](README.md) | **简体中文** | [Español](README_ES.md) ]

</div>

---

## 💡 项目背景与痛点解决

当运行官方的 **Codex 桌面客户端**（或 Electron 架构的 AI Agent）时，凭据与设备 ID 通常保存在固定的本地全局存储中（如 `~/.codex` 或系统 Keyring 凭据管理器）。

这带来了非常痛折的使用难题：
1. **账号强行互踢**：在同一台电脑上登录账号 B，账号 A 的 Session 就会在本地或服务端被直接覆盖踢下线。
2. **死沙箱阉割 Agent 能力**：传统虚拟机/容器隔离方案虽然隔离了凭据，但阉割了 Agent 的核心能力，使其无法读写宿主机项目文件、无法调用终端 Shell 或 Docker 引擎。

由 **[@tianrking](https://github.com/tianrking)** 开源的 **Codex Manager** 结合 **Tauri 2 + Rust** 物理级环境重定向，彻底解决了上述问题：**在多账号同时并发在线、零摩擦切换的同时，保全 Agent 100% 的宿主机自动化能力**！

---

## 🎯 核心使用场景分析

### 场景 1：工作账号与个人账号完全共存
- **痛点**：开发者需要一边在企业账号里开发公司项目，一边在个人账号里维护开源/私有项目。
- **解决方案**：在 Codex Manager 中分别建立 `Work` 和 `Personal` Profile，两个 Codex 桌面 App 窗口同时并发开启，各自保持在线，决不互相覆盖。

### 场景 2：多客户与多项目独立隔离
- **痛点**：接单开发者或独立开发者管理多个客户项目，需要避免 Auth Token 和配置交叉污染。
- **解决方案**：将指定项目目录绑定到专属的客户 Profile。启动该 Profile 自动带指定项目打开，凭据与历史会话绝对物理隔离。

### 场景 3：配额用尽后的秒级无缝轮换
- **痛点**：当前账号触发 Rate Limit 速率限制或配额用尽，导致开发流程被迫中断。
- **解决方案**：提前将备用账号配置为 Profile。在 UI 上一键启动备用 Profile，0 毫秒延时无缝接续开发。

### 场景 4：企业级 Agent 自动化无损工作流
- **痛点**：隔离目录导致 Agent 找不到宿主机的 `git` 配置或 `ssh` 密钥，无法提交代码。
- **解决方案**：Codex Manager 自动将宿主机的 `~/.gitconfig` 与 `~/.ssh` 建立软链接传入隔离区，让 Agent 拥有完整的 Git 提交、远程仓库推送与 Docker 控制能力。

---

## ✨ 核心功能矩阵

- 🌐 **原生软件界面三语无缝切换**：顶栏原生支持 **英文 (English)**、**简体中文** 与 **西班牙语 (Español)** 动态实时切换。
- 🔄 **多账号并发多开**：同时运行工作账号与个人账号，各自保持在线，不互相影响。
- 🛡️ **彻底的环境物理隔离**：为每个 Profile 独立隔离 `HOME`、`CODEX_HOME`、`--user-data-dir`、`TMPDIR` 以及 Chromium 缓存。
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
                        |          作者: @tianrking             |
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

### 隔离维度比对表

| 隔离维度 | 技术实现机制 | 解决的痛点 |
| :--- | :--- | :--- |
| **Auth & Session** | `--user-data-dir=<profile>/userdata` | 阻断凭据覆盖，解除 Electron 单例锁定。 |
| **全局配置** | `HOME=<profile>` & `CODEX_HOME=<profile>/.codex` | 隔离全局 CLI 配置与 Device ID 识别码。 |
| **共享 IPC Socket**| `TMPDIR=<profile>/tmp` (macOS/Linux) / `TMP` (Win) | 避免不同 Electron 实例共享临时 Socket。 |
| **Keyring 覆写** | `NODE_KEYRING_DISABLE=1` & `--password-store=basic` | 强制采用纯文件存储，避开系统 Keyring 冲突。 |
| **Git/SSH 保全** | 自动软链接 `~/.gitconfig` & `~/.ssh` | 保证 Agent 可以顺利提交代码与连接远程仓库。 |

---

## 🔒 隐私与安全承诺

- **完全本地优先 (Local-First)**：所有 Profile 配置、Session 凭据均 **100% 保存在你的本地硬盘** (`~/.codex_manager/`)。
- **无任何遥测 / 追踪**：Codex Manager 不包含任何追踪代码，不会上传任何数据至第三方。
- **代码完全开源透明**：基于 Rust 与 Tauri 2.0，代码清晰，随时接受审计。

---

## 🛠️ 环境准备与安装

### 环境要求
- [Node.js](https://nodejs.org/) (v18+)
- [Rust & Cargo](https://www.rust-lang.org/) (1.75+)

### 快速开始 (开发模式)

```bash
# 克隆仓库
git clone https://github.com/tianrking/CodexManager.git
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

## 📄 开源协议

本项目采用 [MIT License](LICENSE) 开源协议。作者与维护者：**[@tianrking](https://github.com/tianrking)**。
