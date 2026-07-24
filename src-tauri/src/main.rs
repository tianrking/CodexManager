// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod launcher;
mod profile;
mod tray;

use launcher::LauncherEngine;
use profile::{Profile, ProfileStore};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::Manager;

pub(crate) struct AppState {
    pub(crate) profiles: Mutex<Vec<Profile>>,
}

#[tauri::command]
fn get_profiles(state: tauri::State<AppState>) -> Vec<Profile> {
    let profiles = state.profiles.lock().unwrap();
    profiles.clone()
}

#[tauri::command]
fn add_profile(state: tauri::State<AppState>, profile: Profile) -> Vec<Profile> {
    let mut profiles = state.profiles.lock().unwrap();
    profiles.push(profile);
    ProfileStore::save_profiles(&profiles);
    profiles.clone()
}

#[tauri::command]
fn update_profile(state: tauri::State<AppState>, profile: Profile) -> Vec<Profile> {
    let mut profiles = state.profiles.lock().unwrap();
    if let Some(idx) = profiles.iter().position(|p| p.id == profile.id) {
        profiles[idx] = profile;
        ProfileStore::save_profiles(&profiles);
    }
    profiles.clone()
}

#[tauri::command]
fn delete_profile(state: tauri::State<AppState>, profile_id: String) -> Vec<Profile> {
    LauncherEngine::stop(&profile_id);
    // 彻底清除磁盘上的隔离凭据目录，避免登录 token 残留与孤儿目录堆积
    ProfileStore::remove_profile_dir(&profile_id);
    let mut profiles = state.profiles.lock().unwrap();
    profiles.retain(|p| p.id != profile_id);
    ProfileStore::save_profiles(&profiles);
    profiles.clone()
}

#[tauri::command]
fn get_running_status() -> HashMap<String, u32> {
    LauncherEngine::get_running_profiles()
}

#[tauri::command]
fn launch_profile(profile_id: String, project_path: Option<String>) -> bool {
    LauncherEngine::launch(&profile_id, project_path)
}

#[tauri::command]
fn open_profile_dir(profile_id: String) {
    LauncherEngine::open_profile_dir(&profile_id);
}

#[tauri::command]
fn stop_profile(profile_id: String) {
    LauncherEngine::stop(&profile_id);
}

#[tauri::command]
fn stop_all_profiles() {
    LauncherEngine::stop_all();
}

/// 应用元信息（供设置页展示版本 / 平台 / 仓库地址）
#[derive(serde::Serialize)]
struct AppInfo {
    name: String,
    version: String,
    platform: String,
    arch: String,
    repo: String,
}

#[tauri::command]
fn get_app_info(app: tauri::AppHandle) -> AppInfo {
    let pkg = app.package_info();
    AppInfo {
        name: pkg.name.clone(),
        version: pkg.version.to_string(),
        platform: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        repo: "https://github.com/tianrking/CodexManager".to_string(),
    }
}

/// 显示并聚焦主窗口（从托盘弹出窗调用）
#[tauri::command]
fn show_main_window(app: tauri::AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.show();
        let _ = win.unminimize();
        let _ = win.set_focus();
    }
}

/// 退出应用
#[tauri::command]
fn quit_app(app: tauri::AppHandle) {
    app.exit(0);
}

fn main() {
    let loaded = ProfileStore::load_profiles();
    let state = AppState {
        profiles: Mutex::new(loaded),
    };

    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            tray::build_tray(app.handle())?;

            // 有托盘时，关闭主窗口应隐藏而不是销毁；否则托盘的“Show / Hide”
            // 将无法重新显示窗口。托盘菜单中的 Quit 仍会真正退出应用。
            if let Some(window) = app.get_webview_window("main") {
                let window_to_hide = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = window_to_hide.hide();
                    }
                });
            }

            // 后台定期重建菜单，及时反映运行状态变化（含外部进程退出）
            let handle = app.handle().clone();
            std::thread::spawn(move || loop {
                std::thread::sleep(std::time::Duration::from_secs(2));
                let _ = tray::rebuild_menu(&handle);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_profiles,
            add_profile,
            update_profile,
            delete_profile,
            get_running_status,
            launch_profile,
            open_profile_dir,
            stop_profile,
            stop_all_profiles,
            get_app_info,
            show_main_window,
            quit_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
