<div align="center">

# 🚀 Codex Manager

**Administrador multiplataforma de aislamiento de cuentas y perfiles para Codex App**

[![Tauri 2.0](https://img.shields.io/badge/Tauri-2.0-blue?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app/)
[![Rust Engine](https://img.shields.io/badge/Rust-Backend-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Vite](https://img.shields.io/badge/Vite-Frontend-646CFF?style=for-the-badge&logo=vite&logoColor=white)](https://vitejs.dev/)
[![Platform](https://img.shields.io/badge/Plataforma-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey?style=for-the-badge)](https://github.com/)
[![License](https://img.shields.io/badge/Licencia-MIT-green?style=for-the-badge)](LICENSE)

<p align="center">
  <b>Soporte multicuenta concurrente</b> • <b>Cero sobrescritura de sesión</b> • <b>Capacidades nativas de Agent sin restricciones</b>
</p>

---

[ [English](README.md) | [简体中文](README_CN.md) | **Español** ]

</div>

---

## 💡 ¿Por qué Codex Manager?

Al ejecutar la aplicación de escritorio oficial **Codex Desktop App** (o agentes de IA basados en Electron), las credenciales y los ID de dispositivo se almacenan en directorios globales fijos (`~/.codex` o depósitos de llaves del sistema).

Cambiar entre cuentas de trabajo y personales generalmente **fuerza la sobrescritura de la sesión**, cerrando la sesión de la cuenta anterior. Además, las soluciones tradicionales basadas en máquinas virtuales suelen mermar las capacidades del agente, impidiendo que modifique archivos de proyectos en el sistema anfitrión o ejecute comandos de terminal/Docker.

**Codex Manager** resuelve este dilema de manera elegante mediante **Tauri 2 + Rust**:

1. **Aislamiento físico de credenciales y perfiles**: Ejecute múltiples instancias de Codex simultáneamente sin conflictos de sesión ni cierres forzados.
2. **Capacidades nativas del agente intactas**: El agente conserva acceso completo a los archivos del proyecto anfitrión, entorno Shell, credenciales Git/SSH y el Socket de Docker.
3. **Ultraligero y extremadamente rápido**: Motor nativo en Rust con un tamaño de paquete de ~6MB y 0ms de retardo al iniciar.

---

## ✨ Características Principales

- 🔄 **Instancias multicuenta concurrentes**: Ejecute la Cuenta de Trabajo y la Cuenta Personal lado a lado sin cerrar sesión.
- 🛡️ **Aislamiento completo del entorno**: `HOME`, `CODEX_HOME`, `--user-data-dir`, `TMPDIR` y cachés de Chromium aislados por perfil.
- 🔑 **Heredado automático de Git y SSH**: Enlaza automáticamente `~/.gitconfig` y `~/.ssh` del sistema anfitrión a cada perfil.
- ⚡ **Terminación precisa de procesos**: Elimina limpiamente los procesos principales y todos los subprocesos Helper/Renderer mediante `pkill -9 -f`.
- 📁 **Arrastrar y soltar carpetas**: Arrastre cualquier carpeta de proyecto directamente sobre la tarjeta de un perfil para abrir Codex en ese proyecto.
- 🔍 **Acceso directo al directorio de datos**: Abra el directorio de datos aislado del perfil en Finder / Explorador de archivos con un solo clic.
- 🎨 **Interfaz Glassmorphic moderna**: Elegante interfaz adaptativa claro/oscuro con indicadores de estado PID en tiempo real.

---

## 📐 Arquitectura y Mecanismos de Aislamiento

```
                        +---------------------------------------+
                        |     Codex Manager GUI (Tauri App)     |
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

## 🛠️ Instalación y Requisitos

### Requisitos previos
- [Node.js](https://nodejs.org/) (v18+)
- [Rust & Cargo](https://www.rust-lang.org/) (1.75+)

### Inicio Rápido (Modo Desarrollo)

```bash
# Clonar el repositorio
git clone https://github.com/tu-usuario/CodexManager.git
cd CodexManager

# Instalar dependencias
npm install

# Ejecutar aplicación en modo desarrollo
npx tauri dev
```

### Compilar para Producción

Para compilar un ejecutable independiente (`.app`, `.dmg`, `.exe`, `.msi`, `.deb`, `.AppImage`):

```bash
npm run build
npx tauri build
```

El paquete resultante se ubicará en `src-tauri/target/release/bundle/`.

---

## 📄 Licencia

Distribuido bajo la Licencia MIT. Consulte [`LICENSE`](LICENSE) para obtener más información.
