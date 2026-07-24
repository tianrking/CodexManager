use crate::profile::ProfileStore;
use std::collections::HashMap;
use std::process::Command;

#[cfg(target_os = "windows")]
use std::collections::HashSet;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
#[cfg(target_os = "windows")]
use std::path::{Path, PathBuf};
#[cfg(target_os = "windows")]
use std::sync::{Mutex, OnceLock};

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

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
        #[cfg(any(target_os = "macos", target_os = "windows"))]
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
                    cmd.arg(format!(
                        "--disk-cache-dir={}",
                        cache_dir.join("disk").display()
                    ));
                    cmd.arg(format!(
                        "--crash-dumps-dir={}",
                        cache_dir.join("crashes").display()
                    ));
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
            cmd.arg(format!(
                "--disk-cache-dir={}",
                cache_dir.join("disk").display()
            ));
            cmd.arg(format!(
                "--crash-dumps-dir={}",
                cache_dir.join("crashes").display()
            ));
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
            let Some(executable) = locate_windows_codex_executable() else {
                eprintln!(
                    "Codex Desktop was not found. Install the Microsoft Store app or set CODEX_DESKTOP_PATH."
                );
                return false;
            };

            let roaming_dir = profile_dir.join("AppData/Roaming");
            let local_dir = profile_dir.join("AppData/Local");
            let codex_home = profile_dir.join(".codex");
            let config_dir = profile_dir.join(".config");
            let disk_cache_dir = cache_dir.join("disk");
            let crash_dir = cache_dir.join("crashes");

            for dir in [
                &roaming_dir,
                &local_dir,
                &codex_home,
                &config_dir,
                &userdata_dir,
                &tmp_dir,
                &disk_cache_dir,
                &crash_dir,
            ] {
                if let Err(error) = std::fs::create_dir_all(dir) {
                    eprintln!(
                        "Failed to prepare isolated Windows profile directory {}: {}",
                        dir.display(),
                        error
                    );
                    return false;
                }
            }

            // 必须直接启动桌面 GUI 可执行文件，不能使用 `cmd /C start Codex`：
            // 用户的 PATH 中常常先命中 npm/VS Code 附带的 codex CLI。
            let mut launch_arguments = vec![
                format!("--user-data-dir={}", userdata_dir.display()),
                format!("--disk-cache-dir={}", disk_cache_dir.display()),
                format!("--crash-dumps-dir={}", crash_dir.display()),
                "--password-store=basic".to_string(),
            ];
            if let Some(path) = project_path {
                if !path.trim().is_empty() {
                    launch_arguments.push(path);
                }
            }

            let mut cmd = Command::new(&executable);
            cmd.env("HOME", &profile_dir);
            cmd.env("USERPROFILE", &profile_dir);
            cmd.env("CODEX_HOME", &codex_home);
            cmd.env("XDG_CONFIG_HOME", &config_dir);
            cmd.env("APPDATA", &roaming_dir);
            cmd.env("LOCALAPPDATA", &local_dir);
            cmd.env("TMP", &tmp_dir);
            cmd.env("TEMP", &tmp_dir);
            cmd.env("NODE_KEYRING_DISABLE", "1");
            cmd.args(&launch_arguments);

            match cmd.spawn() {
                Ok(_) => true,
                Err(error) => {
                    eprintln!(
                        "Failed to launch isolated Windows Codex Desktop at {}: {}",
                        executable.display(),
                        error
                    );
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
            let processes = windows_profile_processes();
            terminate_windows_process_trees(&processes, |process| {
                profile_id_from_windows_command_line(&process.command_line)
                    .is_some_and(|id| id.eq_ignore_ascii_case(profile_id))
            });
        }
    }

    /// 一键关闭所有 Profile
    pub fn stop_all() {
        #[cfg(target_os = "windows")]
        {
            let processes = windows_profile_processes();
            terminate_windows_process_trees(&processes, |_| true);
        }

        #[cfg(not(target_os = "windows"))]
        {
            let running = Self::get_running_profiles();
            for (profile_id, _) in running {
                Self::stop(&profile_id);
            }

            let _ = Command::new("pkill")
                .arg("-9")
                .arg("-f")
                .arg(".codex_manager/profiles/")
                .status();
        }
    }
}

/// Windows 上优先解析 Microsoft Store 的 OpenAI.Codex 包，也支持便携版/自定义路径。
/// `CODEX_DESKTOP_PATH` 可指向 exe、安装根目录或包含 app 子目录的包目录。
#[cfg(target_os = "windows")]
fn locate_windows_codex_executable() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("CODEX_DESKTOP_PATH") {
        if let Some(executable) = resolve_windows_codex_path(Path::new(&path)) {
            return Some(executable);
        }
    }

    if let Some(install_location) = query_windows_store_codex_location() {
        if let Some(executable) = materialize_windows_store_runtime(&install_location) {
            return Some(executable);
        }
    }

    let mut candidates = Vec::new();
    if let Some(local_app_data) = std::env::var_os("LOCALAPPDATA") {
        let local = PathBuf::from(local_app_data);
        candidates.extend([
            local.join("Programs/Codex"),
            local.join("Programs/OpenAI Codex"),
            local.join("Codex"),
        ]);
    }
    if let Some(program_files) = std::env::var_os("ProgramFiles") {
        let programs = PathBuf::from(program_files);
        candidates.extend([programs.join("Codex"), programs.join("OpenAI Codex")]);
    }

    candidates
        .iter()
        .find_map(|candidate| resolve_windows_codex_path(candidate))
}

#[cfg(target_os = "windows")]
fn resolve_windows_codex_path(path: &Path) -> Option<PathBuf> {
    if path.is_file() {
        return Some(path.to_path_buf());
    }

    windows_codex_candidates(path)
        .into_iter()
        .find(|candidate| candidate.is_file())
}

#[cfg(target_os = "windows")]
fn windows_codex_candidates(base: &Path) -> Vec<PathBuf> {
    // Store 包的 Manifest 主程序名仍是 ChatGPT.exe，但其产品元数据是 Codex。
    // ChatGPT.exe 是真正的 Chromium GUI 主进程；Codex.exe 是较小的启动桩。
    [
        base.join("app/ChatGPT.exe"),
        base.join("app/Codex.exe"),
        base.join("ChatGPT.exe"),
        base.join("Codex.exe"),
    ]
    .into()
}

#[cfg(target_os = "windows")]
fn query_windows_store_codex_location() -> Option<PathBuf> {
    let script = concat!(
        "$ErrorActionPreference='SilentlyContinue';",
        "[Console]::OutputEncoding=[System.Text.Encoding]::UTF8;",
        "Get-AppxPackage -Name OpenAI.Codex | ",
        "Sort-Object Version -Descending | ",
        "Select-Object -First 1 -ExpandProperty InstallLocation"
    );

    let mut command = Command::new("powershell.exe");
    command
        .args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            script,
        ])
        .creation_flags(CREATE_NO_WINDOW);
    let output = command.output().ok()?;
    if !output.status.success() {
        return None;
    }

    let path = String::from_utf8_lossy(&output.stdout)
        .trim_start_matches('\u{feff}')
        .trim()
        .to_string();
    (!path.is_empty()).then(|| PathBuf::from(path))
}

#[cfg(target_os = "windows")]
fn materialize_windows_store_runtime(install_location: &Path) -> Option<PathBuf> {
    static RUNTIME_COPY_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    let _copy_guard = RUNTIME_COPY_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .ok()?;

    let package_name = install_location.file_name()?.to_string_lossy();
    let runtime_root = ProfileStore::get_base_dir()
        .join("runtime")
        .join(package_name.as_ref());
    let target_app_dir = runtime_root.join("app");
    let target_executable = target_app_dir.join("ChatGPT.exe");
    let ready_marker = runtime_root.join(".ready");

    if windows_store_runtime_is_ready(&target_app_dir, &ready_marker) {
        return Some(target_executable);
    }

    let source_app_dir = install_location.join("app");
    if !source_app_dir.join("ChatGPT.exe").is_file() {
        eprintln!(
            "The Codex Store package at {} does not contain app\\ChatGPT.exe.",
            install_location.display()
        );
        return None;
    }
    if let Err(error) = std::fs::create_dir_all(&target_app_dir) {
        eprintln!(
            "Failed to create the shared Codex runtime directory {}: {}",
            target_app_dir.display(),
            error
        );
        return None;
    }

    // WindowsApps 中的 GUI 主程序禁止普通 CreateProcess，且 AppX 激活器不会继承
    // 每个 Profile 的 CODEX_HOME。首次使用时将已安装、已签名的 Store 运行时复制一份，
    // 后续所有 Profile 共用这份只读来源副本，各自的数据仍完全隔离。
    let mut copy = Command::new("robocopy.exe");
    copy.args([
        source_app_dir.as_os_str(),
        target_app_dir.as_os_str(),
        std::ffi::OsStr::new("/E"),
        std::ffi::OsStr::new("/COPY:DAT"),
        std::ffi::OsStr::new("/DCOPY:DAT"),
        std::ffi::OsStr::new("/XJ"),
        std::ffi::OsStr::new("/R:2"),
        std::ffi::OsStr::new("/W:1"),
        std::ffi::OsStr::new("/MT:8"),
        std::ffi::OsStr::new("/NFL"),
        std::ffi::OsStr::new("/NDL"),
        std::ffi::OsStr::new("/NJH"),
        std::ffi::OsStr::new("/NJS"),
        std::ffi::OsStr::new("/NP"),
    ])
    .creation_flags(CREATE_NO_WINDOW);
    let status = match copy.status() {
        Ok(status) => status,
        Err(error) => {
            eprintln!("Failed to start robocopy for the Codex runtime: {}", error);
            return None;
        }
    };
    // Robocopy defines 0..=7 as success (including files copied or minor differences).
    if status.code().is_none_or(|code| code >= 8) {
        eprintln!(
            "Failed to prepare the shared Codex runtime at {} (robocopy exit code {:?}).",
            target_app_dir.display(),
            status.code()
        );
        return None;
    }

    let marker = format!("source={}\n", install_location.display());
    if let Err(error) = std::fs::write(&ready_marker, marker) {
        eprintln!(
            "Failed to finalize the shared Codex runtime marker {}: {}",
            ready_marker.display(),
            error
        );
        return None;
    }
    windows_store_runtime_is_ready(&target_app_dir, &ready_marker).then_some(target_executable)
}

#[cfg(target_os = "windows")]
fn windows_store_runtime_is_ready(app_dir: &Path, ready_marker: &Path) -> bool {
    ready_marker.is_file()
        && [
            app_dir.join("ChatGPT.exe"),
            app_dir.join("chrome.dll"),
            app_dir.join("resources/app.asar"),
            app_dir.join("resources/codex.exe"),
        ]
        .iter()
        .all(|path| path.is_file())
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
    #[cfg(target_os = "windows")]
    {
        running_windows_profiles(&windows_profile_processes())
    }

    #[cfg(not(target_os = "windows"))]
    {
        let mut result = HashMap::new();
        let output = Command::new("/bin/ps")
            .args(["-ax", "-o", "pid,command"])
            .output();

        if let Ok(out) = output {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                if (line.contains("Codex") || line.contains("codex"))
                    && line.contains(".codex_manager/profiles/")
                {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if let Some(pid_str) = parts.first() {
                        if let Ok(pid) = pid_str.parse::<u32>() {
                            if let Some(pos) = line.find(".codex_manager/profiles/") {
                                let sub = &line[pos + ".codex_manager/profiles/".len()..];
                                let profile_id =
                                    sub.split(&['/', '\\', ' '][..]).next().unwrap_or("");
                                if !profile_id.is_empty() {
                                    result.insert(profile_id.to_string(), pid);
                                }
                            }
                        }
                    }
                }
            }
        }
        result
    }
}

#[cfg(target_os = "windows")]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct WindowsProcess {
    process_id: u32,
    parent_process_id: u32,
    command_line: String,
}

#[cfg(target_os = "windows")]
fn windows_profile_processes() -> Vec<WindowsProcess> {
    let script = concat!(
        "$ErrorActionPreference='SilentlyContinue';",
        "[Console]::OutputEncoding=[System.Text.Encoding]::UTF8;",
        "@(Get-CimInstance Win32_Process | ",
        "Where-Object { $_.CommandLine -and ",
        "$_.CommandLine.IndexOf('.codex_manager\\profiles\\', ",
        "[System.StringComparison]::OrdinalIgnoreCase) -ge 0 } | ",
        "Select-Object ProcessId,ParentProcessId,CommandLine) | ",
        "ConvertTo-Json -Compress"
    );

    let mut command = Command::new("powershell.exe");
    command
        .args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            script,
        ])
        .creation_flags(CREATE_NO_WINDOW);

    let Ok(output) = command.output() else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }

    parse_windows_process_json(&String::from_utf8_lossy(&output.stdout))
}

#[cfg(target_os = "windows")]
fn parse_windows_process_json(text: &str) -> Vec<WindowsProcess> {
    let text = text.trim_start_matches('\u{feff}').trim();
    if text.is_empty() {
        return Vec::new();
    }

    let Ok(value) = serde_json::from_str::<serde_json::Value>(text) else {
        return Vec::new();
    };

    match value {
        serde_json::Value::Array(items) => items
            .into_iter()
            .filter_map(|item| serde_json::from_value(item).ok())
            .collect(),
        serde_json::Value::Object(_) => serde_json::from_value(value)
            .map(|process| vec![process])
            .unwrap_or_default(),
        _ => Vec::new(),
    }
}

#[cfg(target_os = "windows")]
fn profile_id_from_windows_command_line(command_line: &str) -> Option<String> {
    const MARKER: &str = ".codex_manager\\profiles\\";

    let normalized = command_line.replace('/', "\\");
    let lower = normalized.to_ascii_lowercase();
    let start = lower.find(MARKER)? + MARKER.len();
    let remainder = &normalized[start..];
    let profile_id = remainder
        .split(['\\', '/', ' ', '"', '\''])
        .next()
        .unwrap_or_default()
        .trim();

    (!profile_id.is_empty()).then(|| profile_id.to_string())
}

#[cfg(target_os = "windows")]
fn running_windows_profiles(processes: &[WindowsProcess]) -> HashMap<String, u32> {
    let matches: Vec<(String, &WindowsProcess)> = processes
        .iter()
        .filter_map(|process| {
            profile_id_from_windows_command_line(&process.command_line)
                .map(|profile_id| (profile_id, process))
        })
        .collect();

    let mut running = HashMap::new();
    for (profile_id, process) in &matches {
        let parent_is_same_profile = matches.iter().any(|(other_id, other)| {
            other_id.eq_ignore_ascii_case(profile_id)
                && other.process_id == process.parent_process_id
        });
        if !parent_is_same_profile {
            running
                .entry(profile_id.clone())
                .or_insert(process.process_id);
        }
    }

    running
}

#[cfg(target_os = "windows")]
fn terminate_windows_process_trees(
    processes: &[WindowsProcess],
    matches: impl Fn(&WindowsProcess) -> bool,
) {
    let selected: Vec<&WindowsProcess> = processes
        .iter()
        .filter(|process| matches(process))
        .collect();
    let selected_ids: HashSet<u32> = selected.iter().map(|process| process.process_id).collect();

    // 只对每棵树的根调用 taskkill /T，避免先杀父进程后遗漏 Helper/Renderer。
    for process in selected
        .into_iter()
        .filter(|process| !selected_ids.contains(&process.parent_process_id))
    {
        let mut command = Command::new("taskkill.exe");
        command
            .args(["/PID", &process.process_id.to_string(), "/T", "/F"])
            .creation_flags(CREATE_NO_WINDOW);
        let _ = command.status();
    }
}

#[cfg(all(test, target_os = "windows"))]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[test]
    fn empty_profile_id_never_stops_processes() {
        LauncherEngine::stop("");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn windows_candidates_prefer_store_gui_executable() {
        let candidates = windows_codex_candidates(Path::new(r"C:\Store\OpenAI.Codex"));
        assert_eq!(
            candidates[0],
            PathBuf::from(r"C:\Store\OpenAI.Codex\app\ChatGPT.exe")
        );
        assert_eq!(
            candidates[1],
            PathBuf::from(r"C:\Store\OpenAI.Codex\app\Codex.exe")
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn extracts_profile_id_from_windows_command_lines() {
        let command = concat!(
            r#""C:\Program Files\Codex\ChatGPT.exe" "#,
            r#"--user-data-dir=C:\Users\Alice\.codex_manager\profiles\profile_123\userdata"#
        );
        assert_eq!(
            profile_id_from_windows_command_line(command).as_deref(),
            Some("profile_123")
        );

        let forward_slashes =
            "--user-data-dir=C:/Users/Alice/.codex_manager/profiles/工作账号/userdata";
        assert_eq!(
            profile_id_from_windows_command_line(forward_slashes).as_deref(),
            Some("工作账号")
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn parses_process_json_and_selects_profile_root() {
        let json = r#"[
            {
                "ProcessId": 100,
                "ParentProcessId": 50,
                "CommandLine": "ChatGPT.exe --user-data-dir=C:\\Users\\Alice\\.codex_manager\\profiles\\work\\userdata"
            },
            {
                "ProcessId": 101,
                "ParentProcessId": 100,
                "CommandLine": "ChatGPT.exe --type=crashpad --user-data-dir=C:\\Users\\Alice\\.codex_manager\\profiles\\work\\userdata"
            }
        ]"#;
        let processes = parse_windows_process_json(json);
        assert_eq!(processes.len(), 2);
        assert_eq!(running_windows_profiles(&processes).get("work"), Some(&100));
    }

    #[cfg(target_os = "windows")]
    #[test]
    #[ignore = "opens the installed Codex Desktop; run explicitly for Windows integration validation"]
    fn windows_live_launch_detect_stop_smoke_test() {
        const PROFILE_ID: &str = "codex_manager_windows_smoke_test";

        struct Cleanup;
        impl Drop for Cleanup {
            fn drop(&mut self) {
                LauncherEngine::stop(PROFILE_ID);
                std::thread::sleep(std::time::Duration::from_millis(500));
                ProfileStore::remove_profile_dir(PROFILE_ID);
            }
        }
        let _cleanup = Cleanup;

        assert!(
            locate_windows_codex_executable().is_some(),
            "Codex Desktop executable should be discoverable"
        );
        assert!(
            LauncherEngine::launch(PROFILE_ID, None),
            "Codex Desktop should launch with an isolated profile"
        );

        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
        let detected = loop {
            if let Some(pid) = LauncherEngine::get_running_profiles().get(PROFILE_ID) {
                break Some(*pid);
            }
            if std::time::Instant::now() >= deadline {
                break None;
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        };
        assert!(
            detected.is_some(),
            "launched Codex process should be detected by its isolated profile path"
        );

        let codex_home = ProfileStore::get_profile_dir(PROFILE_ID).join(".codex");
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(15);
        let codex_home_used = loop {
            if std::fs::read_dir(&codex_home)
                .map(|mut entries| entries.next().is_some())
                .unwrap_or(false)
            {
                break true;
            }
            if std::time::Instant::now() >= deadline {
                break false;
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        };
        assert!(
            codex_home_used,
            "the copied Store runtime should use the isolated CODEX_HOME environment"
        );

        LauncherEngine::stop(PROFILE_ID);
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(15);
        while std::time::Instant::now() < deadline {
            if !LauncherEngine::get_running_profiles().contains_key(PROFILE_ID) {
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        panic!("isolated Codex process tree should stop within 15 seconds");
    }
}
