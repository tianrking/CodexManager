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
    tray::TrayIconBuilder,
    AppHandle, Manager,
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

    let menu = build_menu(app)?;

    TrayIconBuilder::with_id(TRAY_ID)
        .icon(icon)
        .tooltip("CodexManager")
        .icon_as_template(true) // macOS: 当作 template 图标；其余平台忽略
        .show_menu_on_left_click(true)
        .menu(&menu)
        .on_menu_event(|app, event| handle_menu_event(app, event.id.as_ref()))
        .on_tray_icon_event(|tray, _event| {
            // 点击即刷新，保证运行状态最新
            let _ = rebuild_menu(tray.app_handle());
        })
        .build(app)?;

    Ok(())
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
        builder = builder.item(&MenuItem::with_id(app, "noop", "No profiles", false, None::<&str>)?);
    } else {
        for p in &profiles {
            let (id, label) = if let Some(pid) = running.get(&p.id) {
                (format!("stop:{}", p.id), format!("● {}  (pid {})", p.name, pid))
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
