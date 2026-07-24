//! 系统托盘 / 菜单栏（形态 A：原生菜单）
//!
//! 跨平台同一套代码：
//! - macOS：顶部菜单栏状态项（template 单色图标，自动适配明暗）
//! - Windows：右下角系统托盘
//! - Linux：状态通知区（AppIndicator）
//!
//! 菜单内容随 profile 列表与运行状态动态重建。

use crate::launcher::LauncherEngine;
use crate::AppState;
use tauri::{
    menu::{MenuBuilder, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, PhysicalPosition, WebviewUrl, WebviewWindowBuilder,
};

const TRAY_ID: &str = "main-tray";

/// 构建并注册托盘图标 + 原生菜单。
pub fn build_tray(app: &AppHandle) -> tauri::Result<()> {
    // macOS 用单色 template 图标（自动适配菜单栏明暗）；其余平台用 app 彩色图标
    #[cfg(target_os = "macos")]
    let icon = tauri::image::Image::from_bytes(include_bytes!("../icons/tray-template.png"))
        .expect("tray-template.png is a valid PNG");
    #[cfg(not(target_os = "macos"))]
    let icon = app
        .default_window_icon()
        .cloned()
        .expect("app window icon is required for the tray");

    // 创建托盘弹出窗（形态 B：富内容小窗）—— 无边框、透明、置顶、初始隐藏
    if app.get_webview_window("popover").is_none() {
        let popover =
            WebviewWindowBuilder::new(app, "popover", WebviewUrl::App("popover.html".into()))
                .title("")
                .decorations(false)
                .transparent(true)
                .always_on_top(true)
                .skip_taskbar(true)
                .resizable(false)
                .shadow(true)
                .visible(false)
                .inner_size(340.0, 480.0)
                .build()?;

        // 失焦自动收起（点别处即关）
        let pc = popover.clone();
        popover.on_window_event(move |ev| {
            if let tauri::WindowEvent::Focused(false) = ev {
                let _ = pc.hide();
            }
        });
    }

    let menu = build_menu(app)?;

    TrayIconBuilder::with_id(TRAY_ID)
        .icon(icon)
        .tooltip("CodexManager")
        .icon_as_template(true) // macOS: 当作 template 图标；其余平台忽略
        .show_menu_on_left_click(false) // 左键交给弹出窗
        .menu(&menu)
        .on_menu_event(|app, event| handle_menu_event(app, event.id.as_ref()))
        .on_tray_icon_event(|tray, event| {
            // 只处理一次点击完成事件。Move/Enter 事件频率很高，不能在其中扫描进程。
            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    position,
                    ..
                } => {
                    let _ = rebuild_menu(tray.app_handle());
                    toggle_popover(tray.app_handle(), position);
                }
                TrayIconEvent::Click {
                    button_state: MouseButtonState::Up,
                    ..
                } => {
                    let _ = rebuild_menu(tray.app_handle());
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}

/// 切换弹出窗显隐，并定位到托盘图标附近（macOS 在下方，Win/Linux 在上方）
fn toggle_popover(app: &AppHandle, icon_pos: PhysicalPosition<f64>) {
    let Some(popover) = app.get_webview_window("popover") else {
        return;
    };
    if popover.is_visible().unwrap_or(false) {
        let _ = popover.hide();
        return;
    }

    let size = popover.outer_size().unwrap_or_default();
    let (mw, mh) = popover
        .primary_monitor()
        .ok()
        .flatten()
        .map(|m| {
            let s = m.size();
            (s.width as f64, s.height as f64)
        })
        .unwrap_or((1440.0, 900.0));

    let w = size.width as f64;
    let h = size.height as f64;
    let mut x = icon_pos.x - w / 2.0;
    #[cfg(target_os = "macos")]
    let mut y = icon_pos.y + 4.0;
    #[cfg(not(target_os = "macos"))]
    let mut y = icon_pos.y - h - 4.0;

    if x < 8.0 {
        x = 8.0;
    }
    if x > mw - w - 8.0 {
        x = (mw - w - 8.0).max(8.0);
    }
    if y < 8.0 {
        y = 8.0;
    }
    if y > mh - h - 8.0 {
        y = (mh - h - 8.0).max(8.0);
    }

    let _ = popover.set_position(PhysicalPosition {
        x: x as i32,
        y: y as i32,
    });
    let _ = popover.show();
    let _ = popover.set_focus();
}

/// 依据当前 profile 列表 + 运行状态重建原生菜单。
pub fn rebuild_menu(app: &AppHandle) -> tauri::Result<()> {
    let menu = build_menu(app)?;
    if let Some(tray) = app.tray_by_id(TRAY_ID) {
        tray.set_menu(Some(menu))?;
    }
    Ok(())
}

fn build_menu(app: &AppHandle) -> tauri::Result<tauri::menu::Menu<tauri::Wry>> {
    let profiles = app
        .try_state::<AppState>()
        .map(|s| s.profiles.lock().unwrap().clone())
        .unwrap_or_default();
    let running = LauncherEngine::get_running_profiles();

    let mut builder = MenuBuilder::new(app);

    if profiles.is_empty() {
        builder = builder.item(&MenuItem::with_id(
            app,
            "noop",
            "No profiles",
            false,
            None::<&str>,
        )?);
    } else {
        for p in &profiles {
            let (id, label) = if let Some(pid) = running.get(&p.id) {
                (
                    format!("stop:{}", p.id),
                    format!("● {}  (pid {})", p.name, pid),
                )
            } else {
                (format!("launch:{}", p.id), format!("▶  {}", p.name))
            };
            builder = builder.item(&MenuItem::with_id(app, id, label, true, None::<&str>)?);
        }
    }

    builder = builder
        .separator()
        .item(&MenuItem::with_id(
            app,
            "stopall",
            "■  Stop All",
            !running.is_empty(),
            None::<&str>,
        )?)
        .separator()
        .item(&MenuItem::with_id(
            app,
            "toggle",
            "Show / Hide Window",
            true,
            None::<&str>,
        )?)
        .item(&PredefinedMenuItem::quit(app, Some("Quit"))?);

    builder.build()
}

fn handle_menu_event(app: &AppHandle, id: &str) {
    if let Some(pid) = id.strip_prefix("launch:") {
        let project = app.try_state::<AppState>().and_then(|s| {
            s.profiles
                .lock()
                .unwrap()
                .iter()
                .find(|p| p.id == pid)
                .and_then(|p| p.default_project_path.clone())
        });
        LauncherEngine::launch(pid, project);
        let _ = rebuild_menu(app);
    } else if let Some(pid) = id.strip_prefix("stop:") {
        LauncherEngine::stop(pid);
        let _ = rebuild_menu(app);
    } else if id == "stopall" {
        LauncherEngine::stop_all();
        let _ = rebuild_menu(app);
    } else if id == "toggle" {
        if let Some(win) = app.get_webview_window("main") {
            if win.is_visible().unwrap_or(false) {
                let _ = win.hide();
            } else {
                let _ = win.show();
                let _ = win.set_focus();
            }
        }
    }
    // "noop" 与预定义 Quit 无需处理
}
