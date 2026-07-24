<div align="center">

# 🚀 Codex Manager

**Administrador multiplataforma de aislamiento de perfiles para Codex App**

Creado y mantenido por **[@tianrking](https://github.com/tianrking)**

[![Author](https://img.shields.io/badge/Autor-tianrking-black?style=for-the-badge&logo=github)](https://github.com/tianrking)
[![CI Status](https://img.shields.io/github/actions/workflow/status/tianrking/CodexManager/ci.yml?branch=main&style=for-the-badge&logo=github-actions&logoColor=white&label=CI)](https://github.com/tianrking/CodexManager/actions)
[![Tauri 2.0](https://img.shields.io/badge/Tauri-2.0-blue?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app/)
[![Rust Engine](https://img.shields.io/badge/Rust-Backend-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Vite](https://img.shields.io/badge/Vite-Frontend-646CFF?style=for-the-badge&logo=vite&logoColor=white)](https://vitejs.dev/)
[![Platform](https://img.shields.io/badge/Plataforma-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey?style=for-the-badge)](https://github.com/tianrking/CodexManager)
[![License](https://img.shields.io/badge/Licencia-MIT-green?style=for-the-badge)](LICENSE)

<p align="center">
  Soporte multicuenta concurrente • Almacenamiento independiente • Acceso nativo al sistema
</p>

---

[ [English](README.md) | [简体中文](README_CN.md) | **Español** ]

</div>

---

## 📖 Introducción

La aplicación oficial Codex Desktop almacena ficheros de autenticación, configuración y el ID del dispositivo en un directorio global compartido (`~/.codex` o depósitos del sistema). Al iniciar sesión con otra cuenta, la sesión local previa se sobrescribe.

**Codex Manager** es una herramienta de escritorio desarrollada con **Tauri 2** y **Rust**. Permite ejecutar varias instancias de Codex Desktop App de forma simultánea redirigiendo variables de entorno (`HOME`, `CODEX_HOME`, `TMPDIR` y `--user-data-dir`) por cada perfil.

La herramienta aísla las sesiones de autenticación manteniendo el acceso a los archivos de proyectos locales, la consola, las configuraciones Git/SSH y el socket de Docker.

---

## 🎯 Casos de Uso

- **Cuentas de Trabajo y Personales**: Inicie sesión en ambas cuentas al mismo tiempo sin cerrar sesiones previas.
- **Aislamiento por Proyectos**: Separe las credenciales de diferentes clientes o proyectos en perfiles independientes.
- **Rotación por Límites de Cuota**: Cambie de perfil cuando alcance el límite de velocidad de una cuenta.
- **Acceso a Git/SSH**: Enlaza automáticamente `~/.gitconfig` y `~/.ssh` para mantener las credenciales locales de Git.

---

## ✨ Características

- 🌐 **Soporte Multilingüe**: Cambio de idioma entre Inglés, Chino y Español desde la barra superior.
- 🔄 **Multiproceso**: Abra varias ventanas de Codex con directorios de sesión independientes.
- 🛡️ **Aislamiento de Entorno**: Aísla `HOME`, `CODEX_HOME`, `--user-data-dir` y `TMPDIR` por perfil.
- 🔑 **Enlace Git & SSH**: Vincula automáticamente `~/.gitconfig` y `~/.ssh` del sistema anfitrión.
- ⚡ **Limpieza de Procesos**: Finaliza procesos principales y auxiliares mediante `pkill -9 -f`.
- 📁 **Arrastrar Carpetas**: Arrastre una carpeta de proyecto a una tarjeta de perfil para abrir Codex en esa ruta.
- 🔍 **Directorio de Datos**: Abra la carpeta de datos del perfil en el explorador de archivos.

---

## 🛠️ Instalación y Compilación

### Requisitos previos
- [Node.js](https://nodejs.org/) (v20.19+)
- [Rust & Cargo](https://www.rust-lang.org/) (1.75+)

### Modo Desarrollo

```bash
git clone https://github.com/tianrking/CodexManager.git
cd CodexManager

npm install
npx tauri dev
```

### Compilar Release

```bash
npm run build
npx tauri build
```

Los paquetes (`.app`, `.dmg`, `.exe`, `.msi`, `.deb`, `.AppImage`) se generarán en `src-tauri/target/release/bundle/`.

---

## 📄 Licencia

Distribuido bajo la Licencia MIT. Mantenido por **[@tianrking](https://github.com/tianrking)**. Consulte [`LICENSE`](LICENSE) para más información.
