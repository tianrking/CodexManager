// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod profile;
mod launcher;

use profile::{Profile, ProfileStore};
use launcher::LauncherEngine;
use std::collections::HashMap;
use std::sync::Mutex;

struct AppState {
    profiles: Mutex<Vec<Profile>>,
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

fn main() {
    let loaded = ProfileStore::load_profiles();
    let state = AppState {
        profiles: Mutex::new(loaded),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_profiles,
            add_profile,
            update_profile,
            delete_profile,
            get_running_status,
            launch_profile,
            open_profile_dir,
            stop_profile,
            stop_all_profiles
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
