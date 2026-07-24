use std::collections::HashMap;
use std::process::Command;
use crate::profile::ProfileStore;

pub struct LauncherEngine;

impl LauncherEngine {
    /// 扫描当前运行的 Codex 进程并返回 map: profile_id -> PID
    pub fn get_running_profiles() -> HashMap<String, u32> {
        var_fetch_running_profiles()
    }

    /// 跨平台多账号隔离启动引擎 (深度细节与环境死锁全面防御)
    pub fn launch(profile_id: &str, project_path: Option<String>) -> bool {
        let profile_dir = ProfileStore::get_profile_dir(profile_id);
        let userdata_dir = profile_dir.join("userdata");
        let tmp_dir = profile_dir.join("tmp");
        let cache_dir = profile_dir.join("cache");

        #[cfg(target_os = "macos")]
        {
            let bundle = locate_app_bundle();

            // 首选：直接启动 bundle 内的二进制 —— 唯一能保证 HOME / CODEX_HOME / TMPDIR
            // 等环境变量被 Codex 进程真正继承的方式（open 命令经 LaunchServices 会丢失环境变量）。
            if let Some(bundle_path) = bundle.as_ref() {
                if let Some(exec) = resolve_macos_executable(bundle_path) {
                    let mut cmd = Command::new(&exec);
                    cmd.env("HOME", &profile_dir);
                    cmd.env("CODEX_HOME", profile_dir.join(".codex"));
                    cmd.env("XDG_CONFIG_HOME", profile_dir.join(".config"));
                    cmd.env("TMPDIR", &tmp_dir);
                    cmd.env("NODE_KEYRING_DISABLE", "1");

                    cmd.arg(format!("--user-data-dir={}", userdata_dir.display()));
                    cmd.arg(format!("--disk-cache-dir={}", cache_dir.join("disk").display()));
                    cmd.arg(format!("--crash-dumps-dir={}", cache_dir.join("crashes").display()));
                    cmd.arg("--password-store=basic");

                    if let Some(path) = project_path.as_ref() {
                        if !path.trim().is_empty() {
                            cmd.arg(path);
                        }
                    }

                    match cmd.spawn() {
                        Ok(_) => return true,
                        Err(e) => eprintln!(
                            "Direct binary launch failed ({}, env isolation degraded -> fallback to open): {}",
                            exec.display(),
                            e
                        ),
                    }
                }
            }

            // 兜底：open -n。环境变量隔离可能不生效，但保证至少能拉起应用。
            let mut cmd = Command::new("open");
            cmd.arg("-n");
            let app_arg = bundle
                .as_ref()
                .map(|b| b.to_string_lossy().to_string())
                .unwrap_or_else(|| "Codex".to_string());
            cmd.arg("-a").arg(app_arg);
            cmd.env("HOME", &profile_dir);
            cmd.env("CODEX_HOME", profile_dir.join(".codex"));
            cmd.env("XDG_CONFIG_HOME", profile_dir.join(".config"));
            cmd.env("TMPDIR", &tmp_dir);
            cmd.env("NODE_KEYRING_DISABLE", "1");
            cmd.arg("--args");
            cmd.arg(format!("--user-data-dir={}", userdata_dir.display()));
            cmd.arg(format!("--disk-cache-dir={}", cache_dir.join("disk").display()));
            cmd.arg(format!("--crash-dumps-dir={}", cache_dir.join("crashes").display()));
            cmd.arg("--password-store=basic");

            if let Some(path) = project_path.as_ref() {
                if !path.trim().is_empty() {
                    cmd.arg(path);
                }
            }

            match cmd.spawn() {
                Ok(_) => true,
                Err(e) => {
                    eprintln!("Failed to launch via open: {}", e);
                    false
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            let mut cmd = Command::new("cmd");
            cmd.arg("/C").arg("start").arg("Codex");
            cmd.env("USERPROFILE", &profile_dir);
            cmd.env("APPDATA", profile_dir.join("AppData/Roaming"));
            cmd.env("LOCALAPPDATA", profile_dir.join("AppData/Local"));
            cmd.env("TMP", &tmp_dir);
            cmd.env("TEMP", &tmp_dir);
            
            cmd.arg(format!("--user-data-dir={}", userdata_dir.display()));
            cmd.arg(format!("--disk-cache-dir={}", cache_dir.join("disk").display()));
            cmd.arg("--password-store=basic");

            if let Some(path) = project_path {
                if !path.trim().is_empty() {
                    cmd.arg(path);
                }
            }

            match cmd.spawn() {
                Ok(_) => true,
                Err(e) => {
                    eprintln!("Failed to launch Windows process: {}", e);
                    false
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            let mut cmd = Command::new("codex-desktop");
            cmd.env("HOME", &profile_dir);
            cmd.env("CODEX_HOME", profile_dir.join(".codex"));
            cmd.env("TMPDIR", &tmp_dir);
            cmd.arg(format!("--user-data-dir={}", userdata_dir.display()));

            if let Some(path) = project_path {
                if !path.trim().is_empty() {
                    cmd.arg(path);
                }
            }

            match cmd.spawn() {
                Ok(_) => true,
                Err(e) => {
                    eprintln!("Failed to launch Linux process: {}", e);
                    false
                }
            }
        }
    }

    /// 在 Finder / 资源管理器中打开指定 Profile 的物理数据目录
    pub fn open_profile_dir(profile_id: &str) {
        let path = ProfileStore::get_profile_dir(profile_id);
        
        #[cfg(target_os = "macos")]
        {
            let _ = Command::new("open").arg(path).status();
        }
        #[cfg(target_os = "windows")]
        {
            let _ = Command::new("explorer").arg(path).status();
        }
        #[cfg(target_os = "linux")]
        {
            let _ = Command::new("xdg-open").arg(path).status();
        }
    }

    /// 100% 精准关闭指定 Profile 的所有相关进程
    pub fn stop(profile_id: &str) {
        if profile_id.trim().is_empty() {
            return;
        }

        #[cfg(not(target_os = "windows"))]
        {
            // 通过 pkill -9 -f 根据 Profile 专属路径匹配并精准杀死主进程与所有 Helper/Renderer 进程
            let pattern = format!(".codex_manager/profiles/{}/", profile_id);
            let _ = Command::new("pkill")
                .arg("-9")
                .arg("-f")
                .arg(&pattern)
                .status();
        }

        #[cfg(target_os = "windows")]
        {
            let pattern = format!("*.codex_manager\\profiles\\{}*", profile_id);
            let _ = Command::new("powershell")
                .arg("-Command")
                .arg(format!("Get-CimInstance Win32_Process | Where-Object {{ $_.CommandLine -like '{}' }} | Remove-CimInstance", pattern))
                .status();
        }
    }

    /// 一键关闭所有 Profile
    pub fn stop_all() {
        let running = Self::get_running_profiles();
        for (profile_id, _) in running {
            Self::stop(&profile_id);
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            let _ = Command::new("pkill")
                .arg("-9")
                .arg("-f")
                .arg(".codex_manager/profiles/")
                .status();
        }
    }
}

/// 在标准位置查找已安装的 Codex 应用 bundle（结构化为独立函数，便于未来扩展到多 Agent）。
#[cfg(target_os = "macos")]
fn locate_app_bundle() -> Option<std::path::PathBuf> {
    let candidates = [
        std::path::PathBuf::from("/Applications/Codex.app"),
        std::path::PathBuf::from("/Applications/OpenAI Codex.app"),
    ];
    for c in &candidates {
        if c.exists() {
            return Some(c.clone());
        }
    }
    if let Some(home) = dirs::home_dir() {
        let home_candidates = [
            home.join("Applications/Codex.app"),
            home.join("Applications/OpenAI Codex.app"),
        ];
        for c in &home_candidates {
            if c.exists() {
                return Some(c.clone());
            }
        }
    }
    None
}

/// 解析 .app bundle 内的可执行文件路径：优先读 Info.plist 的 CFBundleExecutable，
/// 兜底取 Contents/MacOS 下第一个文件。直接启动该二进制才能让环境变量真正被子进程继承。
#[cfg(target_os = "macos")]
fn resolve_macos_executable(bundle: &std::path::Path) -> Option<std::path::PathBuf> {
    // 1. 读取 Info.plist 的 CFBundleExecutable
    let info_plist = bundle.join("Contents/Info.plist");
    if info_plist.exists() {
        if let Ok(out) = Command::new("/usr/bin/defaults")
            .arg("read")
            .arg(info_plist.to_string_lossy().to_string())
            .arg("CFBundleExecutable")
            .output()
        {
            if out.status.success() {
                let name = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !name.is_empty() {
                    let exec = bundle.join("Contents/MacOS").join(&name);
                    if exec.exists() {
                        return Some(exec);
                    }
                }
            }
        }
    }
    // 2. 兜底：Contents/MacOS 下第一个文件
    let macos_dir = bundle.join("Contents/MacOS");
    if let Ok(entries) = std::fs::read_dir(&macos_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() {
                return Some(p);
            }
        }
    }
    None
}

/// 解析全量进程命令行的强力函数 (解决 macOS ps 命令行截断问题)
fn var_fetch_running_profiles() -> HashMap<String, u32> {
    let mut result = HashMap::new();

    #[cfg(not(target_os = "windows"))]
    {
        let output = Command::new("/bin/ps")
            .args(&["-ax", "-o", "pid,command"])
            .output();

        if let Ok(out) = output {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                if (line.contains("Codex") || line.contains("codex")) && line.contains(".codex_manager/profiles/") {
                    let parts: Vec<&str> = line.trim().split_whitespace().collect();
                    if let Some(pid_str) = parts.first() {
                        if let Ok(pid) = pid_str.parse::<u32>() {
                            if let Some(pos) = line.find(".codex_manager/profiles/") {
                                let sub = &line[pos + ".codex_manager/profiles/".len()..];
                                let profile_id = sub.split(&['/', '\\', ' '][..]).next().unwrap_or("");
                                if !profile_id.is_empty() {
                                    result.insert(profile_id.to_string(), pid);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("powershell")
            .args(&["-Command", "Get-CimInstance Win32_Process | Select-Object ProcessId, CommandLine | Format-Table -HideTableHeaders"])
            .output();

        if let Ok(out) = output {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                if line.contains(".codex_manager") && line.contains("profiles") {
                    let parts: Vec<&str> = line.trim().split_whitespace().collect();
                    if let Some(pid_str) = parts.first() {
                        if let Ok(pid) = pid_str.parse::<u32>() {
                            if let Some(pos) = line.find("profiles\\") {
                                let sub = &line[pos + "profiles\\".len()..];
                                let profile_id = sub.split(&['/', '\\', ' '][..]).next().unwrap_or("");
                                if !profile_id.is_empty() {
                                    result.insert(profile_id.to_string(), pid);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_command_execution() {
        LauncherEngine::stop("");
        LauncherEngine::stop("non_existent_profile_test");
        LauncherEngine::stop_all();
    }
}
