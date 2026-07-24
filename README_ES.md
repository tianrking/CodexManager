<div align="center">

# 🚀 Codex Manager

**Administrador multiplataforma de aislamiento de cuentas y perfiles para Codex App**

Creado y mantenido por **[@tianrking](https://github.com/tianrking)**

[![Author](https://img.shields.io/badge/Autor-tianrking-black?style=for-the-badge&logo=github)](https://github.com/tianrking)
[![CI Status](https://img.shields.io/github/actions/workflow/status/tianrking/CodexManager/ci.yml?branch=main&style=for-the-badge&logo=github-actions&logoColor=white&label=CI)](https://github.com/tianrking/CodexManager/actions)
[![Tauri 2.0](https://img.shields.io/badge/Tauri-2.0-blue?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app/)
[![Rust Engine](https://img.shields.io/badge/Rust-Backend-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Vite](https://img.shields.io/badge/Vite-Frontend-646CFF?style=for-the-badge&logo=vite&logoColor=white)](https://vitejs.dev/)
[![Platform](https://img.shields.io/badge/Plataforma-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey?style=for-the-badge)](https://github.com/tianrking/CodexManager)
[![License](https://img.shields.io/badge/Licencia-MIT-green?style=for-the-badge)](LICENSE)

<p align="center">
  <b>Soporte multicuenta concurrente</b> • <b>Cero sobrescritura de sesión</b> • <b>Capacidades nativas de Agent sin restricciones</b>
</p>

---

[ [English](README.md) | [简体中文](README_CN.md) | **Español** ]

</div>

---

## 📖 Descripción General y Problemas Solucionados

Las aplicaciones oficiales **Codex Desktop Apps** (y agentes de IA basados en Electron) almacenan credenciales y ID de dispositivo en directorios globales fijos (`~/.codex` o depósitos de llaves del sistema).

Esto genera problemas frecuentes:
1. **Conflictos de Cuentas**: Iniciar sesión en la Cuenta B sobrescribe la sesión local de la Cuenta A, desconectando la Cuenta A.
2. **Entornos Limitados**: Los entornos virtuales tradicionales impiden que el agente de IA edite archivos del sistema anfitrión o ejecute comandos de consola/Docker.

**Codex Manager** por **[@tianrking](https://github.com/tianrking)** resuelve estos desafíos combinando **Tauri 2 + Rust** con redirección física de entorno, permitiendo la **ejecución simultánea multicuenta** mientras preserva el **100% de las capacidades nativas del agente de IA**.

---

## 🎯 Casos de Uso y Escenarios

### Escenario 1: Coexistencia de Cuentas de Trabajo y Personales
- **Problema**: Los desarrolladores necesitan ejecutar cuentas de empresa para proyectos corporativos y cuentas personales para proyectos propios de forma simultánea.
- **Solución**: Cree perfiles de `Trabajo` y `Personal` en Codex Manager. Ambas instancias se ejecutan lado a lado sin cerrarse la sesión entre sí.

### Escenario 2: Aislamiento por Proyectos y Clientes
- **Problema**: Freelancers y agencias que gestionan repositorios de múltiples clientes necesitan separación estricta de credenciales.
- **Solución**: Vincule directorios de proyectos a perfiles dedicados. Abrir un perfil iniciará automáticamente Codex en el repositorio del cliente correspondiente con credenciales aisladas.

### Escenario 3: Rotación Instantánea por Límite de Cuota
- **Problema**: Alcanzar el límite de velocidad o cuota en una cuenta detiene el desarrollo activo.
- **Solución**: Mantenga cuentas secundarias configuradas como perfiles. Inicie un perfil alternativo en 0 milisegundos sin volver a autenticarse.

### Escenario 4: Flujo de Trabajo Empresarial del Agente
- **Problema**: Los entornos aislados que impiden el acceso a las credenciales Git o SSH del sistema anfitrión rompen la automatización.
- **Solución**: Codex Manager enlaza automáticamente `~/.gitconfig` y `~/.ssh` a los entornos de los perfiles, permitiendo crear commits y gestionar repositorios remotos.

---

## ✨ Características Principales

- 🌐 **GUI Nativa Trilingüe (i18n)**: Cambie instantáneamente entre **Inglés**, **Chino simplificado** y **Español** en la barra superior.
- 🔄 **Ejecución multicuenta concurrente**: Abra múltiples ventanas oficiales de Codex Desktop App con sesiones independientes.
- 🛡️ **Aislamiento físico completo**: `HOME`, `CODEX_HOME`, `--user-data-dir`, `TMPDIR` y cachés aislados por perfil.
- 🔑 **Heredado de Git y SSH**: Enlaza automáticamente `~/.gitconfig` y `~/.ssh` del sistema anfitrión.
- ⚡ **Terminación precisa de procesos**: Elimina procesos principales y subprocesos Helper mediante `pkill -9 -f`.
- 📁 **Arrastrar y soltar carpetas**: Arrastre cualquier carpeta sobre una tarjeta de perfil para iniciar Codex en ese directorio.
- 🔍 **Acceso directo al directorio de datos**: Abra el directorio de datos aislado del perfil en Finder / Explorador de archivos.

---

## 📐 Arquitectura y Mecanismos de Aislamiento

```
                        +---------------------------------------+
                        |     Codex Manager GUI (Tauri App)     |
                        |          Autor: @tianrking            |
                        +-------------------+-------------------+
                                            |
                                            v
                        +---------------------------------------+
                        |        Motor de Aislamiento Rust      |
                        +-------------------+-------------------+
                                            |
         +----------------------------------+----------------------------------+
         |                                  |                                  |
 [ Perfil: Trabajo ]                [ Perfil: Personal ]              [ Perfil: Cliente-A ]
 - HOME: ~/.codex_manager/p1       - HOME: ~/.codex_manager/p2       - HOME: ~/.codex_manager/p3
 - UserData: .../p1/userdata       - UserData: .../p2/userdata       - UserData: .../p3/userdata
 - Temp Socket: .../p1/tmp         - Temp Socket: .../p2/tmp         - Temp Socket: .../p3/tmp
 - Auth Token: Sesión 1            - Auth Token: Sesión 2            - Auth Token: Sesión 3
         |                                  |                                  |
         +----------------------------------+----------------------------------+
                                            |
                                            v
                        [ Recursos del Sistema y Git/SSH/Docker ]
```

---

## 🔒 Garantía de Privacidad y Seguridad

- **Arquitectura Local Primero (Local-First)**: Todas las configuraciones y credenciales permanecen **100% en su disco local** (`~/.codex_manager/`).
- **Cero Telemetría**: Codex Manager no recopila, transmite ni almacena credenciales ni telemetría personal.
- **Seguridad de Código Abierto**: Construido con Rust y Tauri 2.0 con dependencias mínimas para total transparencia.

---

## 🛠️ Instalación

```bash
# Clonar el repositorio
git clone https://github.com/tianrking/CodexManager.git
cd CodexManager

# Instalar dependencias
npm install

# Modo desarrollo
npx tauri dev

# Compilar para producción
npm run build
npx tauri build
```

---

## 📄 Licencia

Distribuido bajo la Licencia MIT. Creado y mantenido por **[@tianrking](https://github.com/tianrking)**. Consulte [`LICENSE`](LICENSE).
