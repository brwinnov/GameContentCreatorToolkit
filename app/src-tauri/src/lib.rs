use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tauri::{AppHandle, Emitter, Manager};
use tokio::io::{AsyncBufReadExt, BufReader};

const HISTORY_FILE: &str = "steam-history.json";
const SETTINGS_FILE: &str = "settings.json";
const MAX_FFMPEG_ARCHIVE_BYTES: u64 = 250 * 1024 * 1024;
const MAX_FFMPEG_TOOL_BYTES: u64 = 200 * 1024 * 1024;

#[cfg(target_os = "windows")]
const FFMPEG_ARCHIVE_NAME: &str = "ffmpeg-master-latest-win64-lgpl.zip";
#[cfg(target_os = "windows")]
const FFMPEG_ARCHIVE_URL: &str = "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-lgpl.zip";
#[cfg(target_os = "windows")]
const FFMPEG_CHECKSUMS_URL: &str =
    "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/checksums.sha256";

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

fn configure_hidden_process(command: &mut tokio::process::Command) {
    #[cfg(windows)]
    {
        command.creation_flags(CREATE_NO_WINDOW);
    }
}

fn configure_hidden_std_process(command: &mut std::process::Command) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(CREATE_NO_WINDOW);
    }
}

// ── Data shapes shared with the frontend ────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrailerInfo {
    pub name: String,
    pub dash_url: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SteamTrailerResponse {
    pub game_name: String,
    pub trailers: Vec<TrailerInfo>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadSummary {
    pub success: u32,
    pub skipped: u32,
    pub failed: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SteamSearchResult {
    pub app_id: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppMetadata {
    pub version: &'static str,
    pub build: &'static str,
    pub creator: &'static str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FfmpegInfo {
    path: String,
    version: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppSettings {
    ffmpeg_path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProgressEvent {
    tag: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<u8>,
}

fn emit_progress(app: &AppHandle, tag: &str, message: impl Into<String>) {
    emit_progress_pct(app, tag, message, None);
}

fn emit_progress_pct(app: &AppHandle, tag: &str, message: impl Into<String>, percent: Option<u8>) {
    let _ = app.emit(
        "download-progress",
        ProgressEvent {
            tag: tag.to_string(),
            message: message.into(),
            percent,
        },
    );
}

#[tauri::command]
fn get_app_metadata() -> AppMetadata {
    AppMetadata {
        version: env!("CARGO_PKG_VERSION"),
        build: option_env!("GCC_BUILD_NUMBER")
            .or(option_env!("GITHUB_RUN_NUMBER"))
            .unwrap_or("local"),
        creator: "Barry Reilly / AckrosGaming",
    }
}

fn app_data_file(app: &AppHandle, name: &str) -> Result<PathBuf, String> {
    app.path()
        .app_local_data_dir()
        .map(|directory| directory.join(name))
        .map_err(|error| format!("Could not resolve the app data folder: {error}"))
}

fn write_json_atomic<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| "App data file has no parent folder.".to_string())?;
    std::fs::create_dir_all(parent)
        .map_err(|error| format!("Could not create the app data folder: {error}"))?;
    let temporary = path.with_extension("tmp");
    let data = serde_json::to_vec_pretty(value)
        .map_err(|error| format!("Could not serialize app data: {error}"))?;
    std::fs::write(&temporary, data)
        .map_err(|error| format!("Could not write app data: {error}"))?;
    if path.exists() {
        std::fs::remove_file(path)
            .map_err(|error| format!("Could not replace app data: {error}"))?;
    }
    std::fs::rename(&temporary, path).map_err(|error| format!("Could not save app data: {error}"))
}

fn copy_directory(source: &Path, destination: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(destination)?;
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let destination_path = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_directory(&entry.path(), &destination_path)?;
        } else {
            std::fs::copy(entry.path(), destination_path)?;
        }
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn migrate_legacy_webview_data() {
    let Some(local_data) = dirs::data_local_dir() else {
        return;
    };
    let current_root = local_data.join("com.ackrosgaming.gcc");
    let marker = current_root.join("legacy-webview-migration-v1.complete");
    let history_file = current_root.join(HISTORY_FILE);
    let durable_history_has_entries = std::fs::read(&history_file)
        .ok()
        .and_then(|data| serde_json::from_slice::<Vec<serde_json::Value>>(&data).ok())
        .map(|history| !history.is_empty())
        .unwrap_or(false);
    let legacy_storage = local_data
        .join("com.brwinnov.ggt")
        .join("EBWebView")
        .join("Default")
        .join("Local Storage");
    let current_storage = current_root
        .join("EBWebView")
        .join("Default")
        .join("Local Storage");

    if marker.exists() || durable_history_has_entries || !legacy_storage.is_dir() {
        return;
    }

    if current_storage.exists() {
        let backup = current_storage.with_file_name("Local Storage.before-legacy-migration");
        if backup.exists() {
            let _ = std::fs::remove_dir_all(&backup);
        }
        if std::fs::rename(&current_storage, &backup).is_err() {
            return;
        }
    }

    if copy_directory(&legacy_storage, &current_storage).is_ok() {
        let _ = std::fs::write(
            marker,
            b"Legacy localStorage copied before WebView startup.\n",
        );
    }
}

#[cfg(not(target_os = "windows"))]
fn migrate_legacy_webview_data() {}

fn load_settings(app: &AppHandle) -> AppSettings {
    app_data_file(app, SETTINGS_FILE)
        .ok()
        .and_then(|path| std::fs::read(path).ok())
        .and_then(|data| serde_json::from_slice(&data).ok())
        .unwrap_or_default()
}

#[tauri::command]
fn load_history(app: AppHandle) -> Result<Vec<serde_json::Value>, String> {
    let path = app_data_file(&app, HISTORY_FILE)?;
    match std::fs::read(path) {
        Ok(data) => serde_json::from_slice(&data)
            .map_err(|error| format!("Could not read download history: {error}")),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(error) => Err(format!("Could not read download history: {error}")),
    }
}

#[tauri::command]
fn save_history(app: AppHandle, history: Vec<serde_json::Value>) -> Result<(), String> {
    write_json_atomic(&app_data_file(&app, HISTORY_FILE)?, &history)
}

// ── Steam appdetails lookup ──────────────────────────────────────────────
// Same endpoint and age-gate cookies as scripts/pwsh/download_steam_trailers.ps1.
// These cookie values are fixed, publicly-known constants (not credentials) —
// see SECURITYAUDIT.md at the repo root.

#[tauri::command]
async fn fetch_steam_trailers(app_id: String) -> Result<SteamTrailerResponse, String> {
    let url = format!(
        "https://store.steampowered.com/api/appdetails?appids={}&cc=us&l=english",
        app_id
    );

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header(
            "Cookie",
            "birthtime=757382401; lastagecheckage=1-0-1994; mature_content=1",
        )
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .send()
        .await
        .map_err(|_| {
            "Could not reach the Steam API. Check your internet connection.".to_string()
        })?;

    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Steam API returned an unexpected response: {e}"))?;

    let app_data = body
        .get(&app_id)
        .ok_or_else(|| format!("Steam returned no data for App ID {app_id}."))?;

    let success = app_data
        .get("success")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if !success {
        return Err(format!(
            "Steam returned no data for App ID {app_id}. The app may not exist, be region-locked, or not yet released."
        ));
    }

    let movies = app_data
        .get("data")
        .and_then(|d| d.get("movies"))
        .and_then(|m| m.as_array())
        .cloned()
        .unwrap_or_default();

    let game_name = app_data
        .get("data")
        .and_then(|d| d.get("name"))
        .and_then(|name| name.as_str())
        .filter(|name| !name.trim().is_empty())
        .ok_or_else(|| format!("Steam returned no game name for App ID {app_id}."))?
        .to_string();

    let trailers = movies
        .iter()
        .filter_map(|m| {
            let name = m.get("name")?.as_str()?.to_string();
            let dash_url = m.get("dash_h264")?.as_str()?.to_string();
            Some(TrailerInfo { name, dash_url })
        })
        .collect();

    Ok(SteamTrailerResponse {
        game_name,
        trailers,
    })
}

// ── Steam game-name lookup ──────────────────────────────────────────────

#[tauri::command]
async fn search_steam_games_by_name(query: String) -> Result<Vec<SteamSearchResult>, String> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let escaped = trimmed.replace(' ', "+");
    let url = format!(
        "https://store.steampowered.com/api/storesearch/?term={escaped}&l=english&cc=us&category1=998&snr=1"
    );

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .send()
        .await
        .map_err(|_| {
            "Could not reach the Steam search API. Check your internet connection.".to_string()
        })?;

    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Steam search returned an unexpected response: {e}"))?;

    let items = body
        .get("items")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "Steam returned no search results for that game name.".to_string())?;

    let mut results = Vec::new();
    for item in items {
        let name = item
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        if name.is_empty() {
            continue;
        }

        let app_id = item
            .get("id")
            .or_else(|| item.get("steam_appid"))
            .and_then(|v| v.as_u64())
            .map(|v| v.to_string());

        let Some(app_id) = app_id else {
            continue;
        };

        results.push(SteamSearchResult {
            app_id,
            name: name.to_string(),
        });

        if results.len() >= 8 {
            break;
        }
    }

    if results.is_empty() {
        return Err("Steam returned no search results for that game name.".to_string());
    }

    Ok(results)
}

// ── ffmpeg detection ──────────────────────────────────────────────────────

fn ffmpeg_candidates() -> Vec<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        vec![
            PathBuf::from(r"F:\ffmpeg\bin\ffmpeg.exe"),
            PathBuf::from(r"C:\ffmpeg\bin\ffmpeg.exe"),
            PathBuf::from(r"D:\ffmpeg\bin\ffmpeg.exe"),
            PathBuf::from(r"E:\ffmpeg\bin\ffmpeg.exe"),
            PathBuf::from(r"C:\Program Files\ffmpeg\bin\ffmpeg.exe"),
            PathBuf::from(r"C:\tools\ffmpeg\bin\ffmpeg.exe"),
        ]
    }
    #[cfg(target_os = "macos")]
    {
        vec![
            PathBuf::from("/opt/homebrew/bin/ffmpeg"),
            PathBuf::from("/usr/local/bin/ffmpeg"),
        ]
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        vec![
            PathBuf::from("/usr/bin/ffmpeg"),
            PathBuf::from("/usr/local/bin/ffmpeg"),
            PathBuf::from("/snap/bin/ffmpeg"),
        ]
    }
}

fn valid_ffmpeg_path(path: &Path) -> bool {
    path.is_file()
        && path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| {
                name.eq_ignore_ascii_case(if cfg!(windows) {
                    "ffmpeg.exe"
                } else {
                    "ffmpeg"
                })
            })
            .unwrap_or(false)
}

fn managed_ffmpeg_path(app: &AppHandle) -> Option<PathBuf> {
    app.path()
        .app_local_data_dir()
        .ok()
        .map(|directory| {
            directory
                .join("tools")
                .join("ffmpeg")
                .join("bin")
                .join(if cfg!(windows) {
                    "ffmpeg.exe"
                } else {
                    "ffmpeg"
                })
        })
        .filter(|path| valid_ffmpeg_path(path))
}

fn find_ffmpeg_path(app: &AppHandle) -> Option<String> {
    if let Some(path) = load_settings(app)
        .ffmpeg_path
        .map(PathBuf::from)
        .filter(|path| valid_ffmpeg_path(path))
    {
        return Some(path.display().to_string());
    }

    if let Some(path) = managed_ffmpeg_path(app) {
        return Some(path.display().to_string());
    }

    // 1. Is it on PATH?
    let on_path = std::process::Command::new(if cfg!(windows) { "where" } else { "which" })
        .arg("ffmpeg")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.lines().next().unwrap_or("").trim().to_string())
        .filter(|s| !s.is_empty());

    if let Some(p) = on_path {
        return Some(p);
    }

    // 2. Common install locations not always on PATH.
    ffmpeg_candidates()
        .into_iter()
        .find(|p| p.exists())
        .map(|p| p.display().to_string())
}

fn parse_ffmpeg_version(output: &str) -> Option<String> {
    output.lines().find_map(|line| {
        line.trim()
            .strip_prefix("ffmpeg version ")
            .and_then(|remainder| remainder.split_whitespace().next())
            .filter(|version| !version.is_empty())
            .map(str::to_string)
    })
}

fn ffmpeg_info(path: String) -> FfmpegInfo {
    let mut command = std::process::Command::new(&path);
    configure_hidden_std_process(&mut command);
    let version = command
        .arg("-version")
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| {
            parse_ffmpeg_version(&String::from_utf8_lossy(&output.stdout))
                .or_else(|| parse_ffmpeg_version(&String::from_utf8_lossy(&output.stderr)))
        })
        .unwrap_or_else(|| "unknown".to_string());
    FfmpegInfo { path, version }
}

#[tauri::command]
fn find_ffmpeg(app: AppHandle) -> Option<FfmpegInfo> {
    find_ffmpeg_path(&app).map(ffmpeg_info)
}

#[tauri::command]
fn set_ffmpeg_path(app: AppHandle, path: Option<String>) -> Result<Option<FfmpegInfo>, String> {
    let normalized = match path {
        Some(value) => {
            let candidate = PathBuf::from(value);
            if !valid_ffmpeg_path(&candidate) {
                return Err("Choose the ffmpeg executable, not its folder.".to_string());
            }
            let ffprobe = candidate.with_file_name(if cfg!(windows) {
                "ffprobe.exe"
            } else {
                "ffprobe"
            });
            if !ffprobe.is_file() {
                return Err("ffprobe must be in the same folder as ffmpeg.".to_string());
            }
            Some(candidate.display().to_string())
        }
        None => None,
    };
    write_json_atomic(
        &app_data_file(&app, SETTINGS_FILE)?,
        &AppSettings {
            ffmpeg_path: normalized.clone(),
        },
    )?;
    Ok(normalized.map(ffmpeg_info))
}

fn expected_checksum(manifest: &str, archive_name: &str) -> Option<String> {
    manifest.lines().find_map(|line| {
        let mut fields = line.split_whitespace();
        let checksum = fields.next()?;
        let filename = fields.next()?.trim_start_matches('*');
        (filename == archive_name
            && checksum.len() == 64
            && checksum
                .chars()
                .all(|character| character.is_ascii_hexdigit()))
        .then(|| checksum.to_ascii_lowercase())
    })
}

#[cfg(target_os = "windows")]
fn extract_ffmpeg_tools(archive_path: &Path, destination: &Path) -> Result<(), String> {
    let file = File::open(archive_path)
        .map_err(|error| format!("Could not open the ffmpeg archive: {error}"))?;
    let mut archive =
        zip::ZipArchive::new(file).map_err(|error| format!("Invalid ffmpeg ZIP: {error}"))?;
    std::fs::create_dir_all(destination)
        .map_err(|error| format!("Could not create the ffmpeg folder: {error}"))?;

    for tool in ["ffmpeg.exe", "ffprobe.exe"] {
        let index = (0..archive.len())
            .find(|index| {
                archive
                    .by_index(*index)
                    .ok()
                    .and_then(|entry| entry.enclosed_name())
                    .and_then(|path| path.file_name().map(|name| name.to_owned()))
                    .and_then(|name| name.to_str().map(str::to_owned))
                    .map(|name| name.eq_ignore_ascii_case(tool))
                    .unwrap_or(false)
            })
            .ok_or_else(|| format!("The ffmpeg archive does not contain {tool}."))?;
        let mut entry = archive
            .by_index(index)
            .map_err(|error| format!("Could not read {tool} from the archive: {error}"))?;
        if entry.size() > MAX_FFMPEG_TOOL_BYTES {
            return Err(format!("{tool} exceeds the safe extraction size limit."));
        }
        let mut output = File::create(destination.join(tool))
            .map_err(|error| format!("Could not create {tool}: {error}"))?;
        std::io::copy(&mut entry, &mut output)
            .map_err(|error| format!("Could not extract {tool}: {error}"))?;
    }
    Ok(())
}

#[tauri::command]
async fn install_ffmpeg(app: AppHandle) -> Result<FfmpegInfo, String> {
    #[cfg(not(target_os = "windows"))]
    return Err(
        "Automatic ffmpeg installation is currently available on Windows only.".to_string(),
    );

    #[cfg(target_os = "windows")]
    {
        let app_data = app
            .path()
            .app_local_data_dir()
            .map_err(|error| format!("Could not resolve the app data folder: {error}"))?;
        let tools_dir = app_data.join("tools").join("ffmpeg");
        let archive_path = tools_dir.join(FFMPEG_ARCHIVE_NAME);
        let bin_dir = tools_dir.join("bin");
        tokio::fs::create_dir_all(&tools_dir)
            .await
            .map_err(|error| format!("Could not create the ffmpeg folder: {error}"))?;

        let client = reqwest::Client::builder()
            .user_agent("GCCtoolkit ffmpeg installer")
            .build()
            .map_err(|error| format!("Could not initialize the downloader: {error}"))?;
        let manifest = client
            .get(FFMPEG_CHECKSUMS_URL)
            .send()
            .await
            .and_then(reqwest::Response::error_for_status)
            .map_err(|error| format!("Could not download ffmpeg checksums: {error}"))?
            .text()
            .await
            .map_err(|error| format!("Could not read ffmpeg checksums: {error}"))?;
        let expected = expected_checksum(&manifest, FFMPEG_ARCHIVE_NAME).ok_or_else(|| {
            "The checksum manifest does not list the Windows ffmpeg ZIP.".to_string()
        })?;

        let mut response = client
            .get(FFMPEG_ARCHIVE_URL)
            .send()
            .await
            .and_then(reqwest::Response::error_for_status)
            .map_err(|error| format!("Could not download ffmpeg: {error}"))?;
        if response
            .content_length()
            .is_some_and(|length| length > MAX_FFMPEG_ARCHIVE_BYTES)
        {
            return Err("The ffmpeg archive exceeds the safe download size limit.".to_string());
        }
        let mut archive_file = tokio::fs::File::create(&archive_path)
            .await
            .map_err(|error| format!("Could not create the ffmpeg archive: {error}"))?;
        let mut hasher = Sha256::new();
        let mut downloaded_bytes = 0u64;
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|error| format!("ffmpeg download failed: {error}"))?
        {
            downloaded_bytes += chunk.len() as u64;
            if downloaded_bytes > MAX_FFMPEG_ARCHIVE_BYTES {
                drop(archive_file);
                let _ = tokio::fs::remove_file(&archive_path).await;
                return Err("The ffmpeg archive exceeds the safe download size limit.".to_string());
            }
            hasher.update(&chunk);
            tokio::io::AsyncWriteExt::write_all(&mut archive_file, &chunk)
                .await
                .map_err(|error| format!("Could not save ffmpeg: {error}"))?;
        }
        tokio::io::AsyncWriteExt::flush(&mut archive_file)
            .await
            .map_err(|error| format!("Could not finish saving ffmpeg: {error}"))?;
        drop(archive_file);

        let actual = format!("{:x}", hasher.finalize());
        if actual != expected {
            let _ = tokio::fs::remove_file(&archive_path).await;
            return Err("The ffmpeg download failed SHA-256 verification.".to_string());
        }

        let archive_for_task = archive_path.clone();
        let bin_for_task = bin_dir.clone();
        tokio::task::spawn_blocking(move || extract_ffmpeg_tools(&archive_for_task, &bin_for_task))
            .await
            .map_err(|error| format!("ffmpeg extraction task failed: {error}"))??;
        let _ = tokio::fs::remove_file(&archive_path).await;

        set_ffmpeg_path(app, Some(bin_dir.join("ffmpeg.exe").display().to_string()))?
            .ok_or_else(|| "ffmpeg installation did not produce an executable.".to_string())
    }
}

// ── ffprobe (for progress %) ────────────────────────────────────────────
// ffprobe ships alongside ffmpeg in the same bin folder, so try that first
// before falling back to a PATH lookup.

fn find_ffprobe_path(ffmpeg_path: &str) -> Option<String> {
    let sibling = Path::new(ffmpeg_path).with_file_name(if cfg!(windows) {
        "ffprobe.exe"
    } else {
        "ffprobe"
    });
    if sibling.exists() {
        return Some(sibling.display().to_string());
    }

    std::process::Command::new(if cfg!(windows) { "where" } else { "which" })
        .arg("ffprobe")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.lines().next().unwrap_or("").trim().to_string())
        .filter(|s| !s.is_empty())
}

async fn probe_duration_seconds(ffprobe: &str, url: &str) -> Option<f64> {
    let mut command = tokio::process::Command::new(ffprobe);
    configure_hidden_process(&mut command);
    let output = command
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            url,
        ])
        .stdin(Stdio::null())
        .output()
        .await
        .ok()?;

    if !output.status.success() {
        return None;
    }

    String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<f64>()
        .ok()
        .filter(|d| *d > 0.0)
}

// ── Download location ────────────────────────────────────────────────────

#[tauri::command]
fn get_default_download_dir() -> String {
    dirs::download_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| ".".to_string())
}

// ── Filename sanitizing (mirrors the PowerShell script's safe-name logic) ─

fn safe_filename(name: &str) -> String {
    let kept: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c.is_whitespace() || c == '-' || c == '_' {
                c
            } else {
                ' '
            }
        })
        .collect();

    let mut out = String::new();
    let mut last_was_sep = false;
    for c in kept.trim().chars() {
        if c.is_whitespace() || c == '_' {
            if !last_was_sep {
                out.push('_');
                last_was_sep = true;
            }
        } else {
            out.push(c);
            last_was_sep = false;
        }
    }

    out.trim_matches('_').to_lowercase()
}

fn safe_game_folder_name(name: &str) -> String {
    const MAX_GAME_NAME_LENGTH: usize = 100;
    let mut out = String::new();
    let mut last_was_separator = false;

    for character in name.chars() {
        if out.len() >= MAX_GAME_NAME_LENGTH {
            break;
        }
        if character.is_ascii_alphanumeric() {
            out.push(character);
            last_was_separator = false;
        } else if !out.is_empty() && !last_was_separator {
            out.push('_');
            last_was_separator = true;
        }
    }

    let sanitized = out.trim_matches('_');
    if sanitized.is_empty() {
        "Steam_Game".to_string()
    } else {
        sanitized.to_string()
    }
}

fn existing_game_dir(output_dir: &Path, app_id: &str) -> Option<PathBuf> {
    let prefix = format!("{app_id} ");
    std::fs::read_dir(output_dir)
        .ok()?
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map(|kind| kind.is_dir()).unwrap_or(false))
        .find(|entry| entry.file_name().to_string_lossy().starts_with(&prefix))
        .map(|entry| entry.path())
}

fn game_download_dir(output_dir: &Path, app_id: &str, game_name: &str) -> Result<PathBuf, String> {
    if app_id.is_empty() || !app_id.chars().all(|character| character.is_ascii_digit()) {
        return Err("Steam App ID must contain digits only.".to_string());
    }

    if let Some(existing) = existing_game_dir(output_dir, app_id) {
        return Ok(existing);
    }

    Ok(output_dir.join(format!("{app_id} {}", safe_game_folder_name(game_name))))
}

#[cfg(test)]
mod tests {
    use super::{
        expected_checksum, game_download_dir, parse_ffmpeg_version, safe_game_folder_name,
    };
    use std::path::Path;

    #[test]
    fn sanitizes_game_name_for_cross_platform_folder() {
        assert_eq!(
            safe_game_folder_name("Call of Duty: Modern Warfare 4™"),
            "Call_of_Duty_Modern_Warfare_4"
        );
        assert_eq!(safe_game_folder_name("???"), "Steam_Game");
        assert_eq!(safe_game_folder_name(&"A".repeat(150)).len(), 100);
    }

    #[test]
    fn reads_exact_ffmpeg_checksum_entry() {
        let checksum = "a".repeat(64);
        let manifest = format!(
            "{}  other.zip\n{} *ffmpeg-master-latest-win64-lgpl.zip\n",
            "b".repeat(64),
            checksum
        );

        assert_eq!(
            expected_checksum(&manifest, "ffmpeg-master-latest-win64-lgpl.zip"),
            Some(checksum)
        );
        assert_eq!(expected_checksum("invalid file.zip", "file.zip"), None);
    }

    #[test]
    fn reads_ffmpeg_version_token() {
        let output = "notice\nffmpeg version N-120041-g64fce7202c-20250626 Copyright\n";
        assert_eq!(
            parse_ffmpeg_version(output),
            Some("N-120041-g64fce7202c-20250626".to_string())
        );
        assert_eq!(parse_ffmpeg_version("no version here"), None);
    }

    #[test]
    fn builds_expected_game_folder_name() {
        let result = game_download_dir(
            Path::new("downloads"),
            "4435490",
            "Call of Duty: Modern Warfare 4",
        )
        .expect("valid game folder");

        assert_eq!(
            result,
            Path::new("downloads").join("4435490 Call_of_Duty_Modern_Warfare_4")
        );
    }

    #[test]
    fn rejects_non_numeric_app_id() {
        assert!(game_download_dir(Path::new("downloads"), "../4435490", "Game").is_err());
    }

    #[test]
    fn reuses_existing_folder_for_same_app_id() {
        let root = std::env::temp_dir().join(format!(
            "gcc-toolkit-game-folder-test-{}",
            std::process::id()
        ));
        let existing = root.join("4435490 Existing_Name");
        std::fs::create_dir_all(&existing).expect("create test directory");

        let result = game_download_dir(&root, "4435490", "Changed Name")
            .expect("reuse existing game folder");

        assert_eq!(result, existing);
        std::fs::remove_dir_all(root).expect("remove test directory");
    }
}

// ── Download ──────────────────────────────────────────────────────────────

#[tauri::command]
async fn download_trailers(
    app: AppHandle,
    app_id: String,
    game_name: String,
    output_dir: String,
    trailers: Vec<TrailerInfo>,
) -> Result<DownloadSummary, String> {
    let ffmpeg = find_ffmpeg_path(&app).ok_or_else(|| {
        "ffmpeg not found. Install it from Getting Started or choose it in Settings.".to_string()
    })?;

    let output_root = Path::new(&output_dir);
    tokio::fs::create_dir_all(output_root)
        .await
        .map_err(|e| format!("Could not create output folder: {e}"))?;
    let out_dir = game_download_dir(output_root, &app_id, &game_name)?;
    tokio::fs::create_dir_all(&out_dir)
        .await
        .map_err(|e| format!("Could not create game folder: {e}"))?;
    emit_progress(
        &app,
        "INFO",
        format!("Game folder -> {}", out_dir.display()),
    );

    let mut success = 0u32;
    let mut skipped = 0u32;
    let mut failed = 0u32;

    for trailer in trailers {
        let file_name = format!("{}.mp4", safe_filename(&trailer.name));
        let out_file = out_dir.join(&file_name);

        if out_file.exists() {
            emit_progress(&app, "SKIP", format!("{} — already exists", trailer.name));
            skipped += 1;
            continue;
        }

        emit_progress_pct(&app, "DOWN", trailer.name.clone(), Some(0));

        // Probing the total duration first lets us turn ffmpeg's own decode
        // progress into a 0-100% figure for the UI's progress bar.
        let ffprobe = find_ffprobe_path(&ffmpeg);
        let total_duration = match &ffprobe {
            Some(fp) => probe_duration_seconds(fp, &trailer.dash_url).await,
            None => None,
        };

        // Stdio is captured rather than inherited: ffmpeg's DASH demuxer
        // sometimes logs a benign "Error when loading first fragment of
        // playlist" line on segment retries even when the download completes
        // fine, and this way it never reaches the app's console panel. On a
        // real failure the last stderr line is surfaced for diagnostics.
        // stdout carries `-progress pipe:1` key=value lines, read
        // concurrently with waiting on the process so its pipe never backs up.
        // Windows: CREATE_NO_WINDOW keeps the launch silent for end users while
        // still letting the app monitor progress and surface failures.
        let mut command = tokio::process::Command::new(&ffmpeg);
        configure_hidden_process(&mut command);
        let spawned = command
            .args([
                "-i",
                &trailer.dash_url,
                "-c",
                "copy",
                out_file.to_str().unwrap_or_default(),
                "-y",
                "-loglevel",
                "error",
                "-progress",
                "pipe:1",
                "-nostats",
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        let (status, stderr_text) = match spawned {
            Ok(mut child) => {
                let stdout_task = child.stdout.take().map(|out| {
                    let app = app.clone();
                    let name = trailer.name.clone();
                    tokio::spawn(async move {
                        let mut lines = BufReader::new(out).lines();
                        let mut last_pct: i64 = 0;
                        while let Ok(Some(line)) = lines.next_line().await {
                            let Some(rest) = line.strip_prefix("out_time_us=") else {
                                continue;
                            };
                            let (Ok(us), Some(total)) =
                                (rest.trim().parse::<i64>(), total_duration)
                            else {
                                continue;
                            };
                            // Cap at 99 while still decoding — 100% is
                            // reserved for the OK event, once the file is
                            // actually finalized on disk.
                            let pct =
                                ((us as f64 / 1_000_000.0 / total) * 100.0).clamp(0.0, 99.0) as i64;
                            if pct != last_pct {
                                last_pct = pct;
                                emit_progress_pct(&app, "PROGRESS", name.clone(), Some(pct as u8));
                            }
                        }
                    })
                });

                let stderr_task = child.stderr.take().map(|err| {
                    tokio::spawn(async move {
                        let mut lines = BufReader::new(err).lines();
                        let mut buf = String::new();
                        while let Ok(Some(line)) = lines.next_line().await {
                            buf.push_str(&line);
                            buf.push('\n');
                        }
                        buf
                    })
                });

                let status = child.wait().await;
                if let Some(t) = stdout_task {
                    let _ = t.await;
                }
                let stderr_text = match stderr_task {
                    Some(t) => t.await.unwrap_or_default(),
                    None => String::new(),
                };
                (status, stderr_text)
            }
            Err(e) => (Err(e), String::new()),
        };

        match status {
            Ok(s) if s.success() => {
                let size_mb = tokio::fs::metadata(&out_file)
                    .await
                    .map(|m| m.len() as f64 / 1_048_576.0)
                    .unwrap_or(0.0);
                emit_progress_pct(
                    &app,
                    "OK",
                    format!("Saved -> {}  ({:.1} MB)", out_file.display(), size_mb),
                    Some(100),
                );
                success += 1;
            }
            Ok(s) => {
                let _ = tokio::fs::remove_file(&out_file).await;
                let detail = stderr_text.lines().last().unwrap_or("").trim().to_string();
                let suffix = if detail.is_empty() {
                    String::new()
                } else {
                    format!(": {detail}")
                };
                emit_progress(
                    &app,
                    "FAIL",
                    format!("{} — ffmpeg exited with code {}{}", trailer.name, s, suffix),
                );
                failed += 1;
            }
            Err(e) => {
                emit_progress(&app, "FAIL", format!("{} — {}", trailer.name, e));
                failed += 1;
            }
        }
    }

    Ok(DownloadSummary {
        success,
        skipped,
        failed,
    })
}

// ── App entry point ───────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    migrate_legacy_webview_data();
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            get_app_metadata,
            load_history,
            save_history,
            fetch_steam_trailers,
            search_steam_games_by_name,
            find_ffmpeg,
            set_ffmpeg_path,
            install_ffmpeg,
            get_default_download_dir,
            download_trailers,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
