use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub note: String,
    pub color: String,
    pub default_project_path: Option<String>,
}

pub struct ProfileStore;

impl ProfileStore {
    pub fn get_base_dir() -> PathBuf {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home.join(".codex_manager")
    }

    pub fn get_profiles_dir() -> PathBuf {
        Self::get_base_dir().join("profiles")
    }

    pub fn get_config_file() -> PathBuf {
        Self::get_base_dir().join("config.json")
    }

    pub fn get_profile_dir(profile_id: &str) -> PathBuf {
        let path = Self::get_profiles_dir().join(profile_id);
        let _ = fs::create_dir_all(path.join("Library/Application Support"));
        let _ = fs::create_dir_all(path.join("userdata"));
        let _ = fs::create_dir_all(path.join("tmp"));
        let _ = fs::create_dir_all(path.join("cache"));

        // 细节优化：自动继承/软链接宿主机的 ~/.gitconfig 与 ~/.ssh，保全 Agent 原生 Git/SSH 能力
        if let Some(home) = dirs::home_dir() {
            let host_gitconfig = home.join(".gitconfig");
            let profile_gitconfig = path.join(".gitconfig");
            if host_gitconfig.exists() && !profile_gitconfig.exists() {
                #[cfg(not(target_os = "windows"))]
                let _ = std::os::unix::fs::symlink(&host_gitconfig, &profile_gitconfig);
                #[cfg(target_os = "windows")]
                let _ = fs::copy(&host_gitconfig, &profile_gitconfig);
            }

            let host_ssh = home.join(".ssh");
            let profile_ssh = path.join(".ssh");
            if host_ssh.exists() && !profile_ssh.exists() {
                #[cfg(not(target_os = "windows"))]
                let _ = std::os::unix::fs::symlink(&host_ssh, &profile_ssh);
            }
        }

        path
    }

    pub fn load_profiles() -> Vec<Profile> {
        let file = Self::get_config_file();
        if file.exists() {
            if let Ok(content) = fs::read_to_string(file) {
                if let Ok(profiles) = serde_json::from_str::<Vec<Profile>>(&content) {
                    return profiles;
                }
            }
        }
        
        let default_profiles = vec![
            Profile {
                id: "work_account".to_string(),
                name: "Work Account".to_string(),
                note: "工作/公司项目 Profile (凭据+环境彻底隔离)".to_string(),
                color: "#007AFF".to_string(),
                default_project_path: None,
            },
            Profile {
                id: "personal_account".to_string(),
                name: "Personal Account".to_string(),
                note: "个人开源与私有项目 (独立 Session)".to_string(),
                color: "#34C759".to_string(),
                default_project_path: None,
            },
        ];
        Self::save_profiles(&default_profiles);
        default_profiles
    }

    pub fn save_profiles(profiles: &[Profile]) {
        let file = Self::get_config_file();
        if let Some(parent) = file.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(profiles) {
            let _ = fs::write(file, json);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_directory_isolation() {
        let dir = ProfileStore::get_profile_dir("test_unit_account");
        assert!(dir.join("userdata").exists());
        assert!(dir.join("tmp").exists());
        assert!(dir.join("cache").exists());
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_profile_store_persistence() {
        let profiles = ProfileStore::load_profiles();
        assert!(!profiles.is_empty());
    }
}
