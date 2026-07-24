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

    /// 彻底删除指定 Profile 的物理隔离数据目录（含凭据 / userdata / cache / tmp）
    /// 注意：刻意不走 get_profile_dir()，避免触发"创建目录 + 建符号链接"的副作用。
    pub fn remove_profile_dir(profile_id: &str) {
        let dir = Self::get_profiles_dir().join(profile_id);
        if dir.exists() {
            let _ = fs::remove_dir_all(&dir);
        }
    }

    pub fn get_profile_dir(profile_id: &str) -> PathBuf {
        let path = Self::get_profiles_dir().join(profile_id);
        let _ = fs::create_dir_all(path.join("Library/Application Support"));
        let _ = fs::create_dir_all(path.join("userdata"));
        let _ = fs::create_dir_all(path.join("tmp"));
        let _ = fs::create_dir_all(path.join("cache"));

        // 细节优化：自动继承/软链接宿主机的 ~/.gitconfig 与 ~/.ssh，保全 Agent 原生 Git/SSH 能力
        if let Some(home) = dirs::home_dir() {
            let gitconfig = home.join(".gitconfig");
            if gitconfig.exists() {
                let target = path.join(".gitconfig");
                if !target.exists() {
                    #[cfg(unix)]
                    let _ = std::os::unix::fs::symlink(&gitconfig, &target);
                    #[cfg(windows)]
                    {
                        if std::os::windows::fs::symlink_file(&gitconfig, &target).is_err() {
                            let _ = std::fs::copy(&gitconfig, &target);
                        }
                    }
                }
            }

            let ssh = home.join(".ssh");
            if ssh.exists() {
                let target = path.join(".ssh");
                if !target.exists() {
                    #[cfg(unix)]
                    let _ = std::os::unix::fs::symlink(&ssh, &target);
                    #[cfg(windows)]
                    {
                        if std::os::windows::fs::symlink_dir(&ssh, &target).is_err() {
                            let _ = copy_dir_all(&ssh, &target);
                        }
                    }
                }
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

#[cfg(windows)]
fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
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
