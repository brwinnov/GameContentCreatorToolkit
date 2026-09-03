use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::io::{AsyncBufReadExt, BufReader};

const HISTORY_FILE: &str = "steam-history.json";
const SETTINGS_FILE: &str = "settings.json";
const MAX_FFMPEG_ARCHIVE_BYTES: u64 = 250 * 1024 * 1024;
const MAX_FFMPEG_TOOL_BYTES: u64 = 200 * 1024 * 1024;
const UPDATE_CHECK_TTL: Duration = Duration::from_secs(24 * 60 * 60);
#[cfg_attr(not(target_os = "windows"), allow(dead_code))]
const UPDATE_CHECK_TIMEOUT: Duration = Duration::from_secs(5);

// BtbN publishes every build under the rolling `latest` release tag together
// with one `checksums.sha256` manifest. The `n8.1-latest` asset tracks the 8.1
// release branch, so `ffmpeg -version` reports a plain `8.1` instead of the
// git-describe string (`N-126390-g…`) the `master-latest` nightly produces.
// LGPL is deliberate: this project is MIT-licensed and redistributes nothing,
// but the LGPL build avoids pulling GPL-only components onto users' machines.
#[cfg(target_os = "windows")]
const FFMPEG_ARCHIVE_NAME: &str = "ffmpeg-n8.1-latest-win64-lgpl-8.1.zip";
#[cfg(target_os = "windows")]
const FFMPEG_ARCHIVE_URL: &str = "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-n8.1-latest-win64-lgpl-8.1.zip";
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
    pub release_type: &'static str,
    pub version: &'static str,
    pub build: &'static str,
    pub creator: &'static str,
}

/// Where the active ffmpeg executable comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FfmpegSource {
    /// Installed by the app under `<app data>/tools/ffmpeg/bin`.
    Managed,
    /// Found on PATH or in one of the well-known install locations.
    System,
    /// Chosen by the user in Settings and persisted in `settings.json`.
    Custom,
}

impl FfmpegSource {
    fn as_str(self) -> &'static str {
        match self {
            FfmpegSource::Managed => "managed",
            FfmpegSource::System => "system",
            FfmpegSource::Custom => "custom",
        }
    }
}

/// Full media-engine picture for the Settings row and the Steam banner.
///
/// `status` is `notFound`, `ready`, or `broken`. `broken` means an executable
/// was resolved but cannot run (or a persisted custom path no longer exists);
/// it is deliberately distinct from `notFound` so the UI can offer Repair.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaEngineStatus {
    status: &'static str,
    source: Option<&'static str>,
    path: Option<String>,
    version: Option<String>,
    version_display: Option<String>,
    error: Option<String>,
    update_available: Option<bool>,
}

impl MediaEngineStatus {
    fn not_found() -> Self {
        MediaEngineStatus {
            status: "notFound",
            source: None,
            path: None,
            version: None,
            version_display: None,
            error: None,
            update_available: None,
        }
    }
}

#[cfg_attr(not(target_os = "windows"), allow(dead_code))]
/// Progress of a managed ffmpeg install, emitted as `media-engine-progress`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct MediaEngineProgress {
    phase: &'static str,
    bytes_done: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    bytes_total: Option<u64>,
}

/// Cross-command state for the managed ffmpeg installer.
#[derive(Default)]
struct MediaEngineState {
    installing: AtomicBool,
    cancel_requested: AtomicBool,
    /// Cached result of the last update check: (when, newer build available?).
    update_check: Mutex<Option<(Instant, Option<bool>)>>,
}

impl MediaEngineState {
    fn cached_update(&self) -> Option<bool> {
        self.update_check
            .lock()
            .ok()
            .and_then(|guard| *guard)
            .filter(|(checked_at, _)| checked_at.elapsed() < UPDATE_CHECK_TTL)
            .and_then(|(_, available)| available)
    }

    #[cfg_attr(not(target_os = "windows"), allow(dead_code))]
    fn remember_update(&self, available: Option<bool>) {
        if let Ok(mut guard) = self.update_check.lock() {
            *guard = Some((Instant::now(), available));
        }
    }
}

#[cfg_attr(not(target_os = "windows"), allow(dead_code))]
/// Clears the `installing` flag when an install finishes by any path.
struct InstallGuard<'a>(&'a MediaEngineState);

impl Drop for InstallGuard<'_> {
    fn drop(&mut self) {
        self.0.installing.store(false, Ordering::SeqCst);
        self.0.cancel_requested.store(false, Ordering::SeqCst);
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppSettings {
    /// User-selected ffmpeg. `None` means "use the managed copy, else search".
    ffmpeg_path: Option<String>,
    /// SHA-256 of the archive the managed copy was extracted from, compared
    /// against the upstream manifest to decide whether a newer build exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    managed_archive_sha256: Option<String>,
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
        release_type: "ALPHA",
        version: env!("CARGO_PKG_VERSION"),
        build: option_env!("GCC_BUILD_NUMBER")
            .or(option_env!("GITHUB_RUN_NUMBER"))
            .unwrap_or("local"),
        creator: "AckrosGaming",
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
        .map_err(|error| format!("Could not serialise app data: {error}"))?;
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

fn save_settings(app: &AppHandle, settings: &AppSettings) -> Result<(), String> {
    write_json_atomic(&app_data_file(app, SETTINGS_FILE)?, settings)
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
// See docs/SECURITYAUDIT.md.

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

fn ffmpeg_exe_name() -> &'static str {
    if cfg!(windows) {
        "ffmpeg.exe"
    } else {
        "ffmpeg"
    }
}

fn ffprobe_exe_name() -> &'static str {
    if cfg!(windows) {
        "ffprobe.exe"
    } else {
        "ffprobe"
    }
}

/// `<app data>/tools/ffmpeg` — the managed install root.
fn managed_tools_dir(app: &AppHandle) -> Option<PathBuf> {
    app.path()
        .app_local_data_dir()
        .ok()
        .map(|directory| directory.join("tools").join("ffmpeg"))
}

fn managed_ffmpeg_path(app: &AppHandle) -> Option<PathBuf> {
    managed_tools_dir(app)
        .map(|tools| tools.join("bin").join(ffmpeg_exe_name()))
        .filter(|path| valid_ffmpeg_path(path))
}

/// Classifies a resolved executable by location: anything under the managed
/// `bin` folder is `Managed`, regardless of how it was resolved.
fn classify_source(path: &Path, managed_bin: Option<&Path>, from_settings: bool) -> FfmpegSource {
    if managed_bin.is_some_and(|bin| path.starts_with(bin)) {
        FfmpegSource::Managed
    } else if from_settings {
        FfmpegSource::Custom
    } else {
        FfmpegSource::System
    }
}

fn ffmpeg_on_path() -> Option<PathBuf> {
    let mut command = std::process::Command::new(if cfg!(windows) { "where" } else { "which" });
    configure_hidden_std_process(&mut command);
    command
        .arg("ffmpeg")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.lines().next().unwrap_or("").trim().to_string())
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
}

/// Outcome of locating an ffmpeg executable, before it has been run.
enum Resolved {
    Found {
        path: PathBuf,
        source: FfmpegSource,
    },
    /// A persisted custom path that no longer points at a file.
    Missing {
        path: String,
    },
    NotFound,
}

fn resolve_ffmpeg(app: &AppHandle) -> Resolved {
    let managed_bin = managed_tools_dir(app).map(|tools| tools.join("bin"));

    if let Some(configured) = load_settings(app).ffmpeg_path {
        let candidate = PathBuf::from(&configured);
        if valid_ffmpeg_path(&candidate) {
            return Resolved::Found {
                source: classify_source(&candidate, managed_bin.as_deref(), true),
                path: candidate,
            };
        }
        return Resolved::Missing { path: configured };
    }

    if let Some(path) = managed_ffmpeg_path(app) {
        return Resolved::Found {
            path,
            source: FfmpegSource::Managed,
        };
    }

    if let Some(path) = ffmpeg_on_path() {
        return Resolved::Found {
            source: classify_source(&path, managed_bin.as_deref(), false),
            path,
        };
    }

    match ffmpeg_candidates().into_iter().find(|p| p.is_file()) {
        Some(path) => Resolved::Found {
            source: classify_source(&path, managed_bin.as_deref(), false),
            path,
        },
        None => Resolved::NotFound,
    }
}

/// Path of a runnable ffmpeg for the download pipeline, if one is configured.
fn find_ffmpeg_path(app: &AppHandle) -> Option<String> {
    match resolve_ffmpeg(app) {
        Resolved::Found { path, .. } => Some(path.display().to_string()),
        _ => None,
    }
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

/// Runs `ffmpeg -version` and returns the version token, or why it failed.
fn probe_ffmpeg_version(path: &Path) -> Result<String, String> {
    let mut command = std::process::Command::new(path);
    configure_hidden_std_process(&mut command);
    let output = command
        .arg("-version")
        .output()
        .map_err(|error| format!("ffmpeg could not be started: {error}"))?;
    if !output.status.success() {
        return Err(format!("ffmpeg exited with status {}.", output.status));
    }
    parse_ffmpeg_version(&String::from_utf8_lossy(&output.stdout))
        .or_else(|| parse_ffmpeg_version(&String::from_utf8_lossy(&output.stderr)))
        .ok_or_else(|| "ffmpeg ran but did not report a version.".to_string())
}

const MONTH_ABBREVIATIONS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// Turns the raw `ffmpeg -version` token into something readable.
///
/// * `8.1`, `8.1.1-full_build`, `7.1-essentials_build` → `8.1`, `8.1.1`, `7.1`
/// * `n8.1.2-50-g1a748fe2cd-20260902` (release branch) → `8.1.2`
/// * `N-126390-g9fc8c785e2-20260902` (git nightly) → `nightly build · 2 Sep 2026`
/// * anything else is returned unchanged.
fn display_version(raw: &str) -> String {
    let trimmed = raw.trim();

    let mut nightly = trimmed.strip_prefix("N-").map(|rest| rest.split('-'));
    if let Some(parts) = nightly.as_mut() {
        let commit_count = parts.next().unwrap_or_default();
        let hash = parts.next().unwrap_or_default();
        let looks_like_git = !commit_count.is_empty()
            && commit_count.chars().all(|c| c.is_ascii_digit())
            && hash.len() > 1
            && hash.starts_with('g')
            && hash[1..].chars().all(|c| c.is_ascii_hexdigit());
        if looks_like_git {
            let date = parts
                .next()
                .filter(|date| date.len() == 8 && date.chars().all(|c| c.is_ascii_digit()));
            return match date.and_then(format_yyyymmdd) {
                Some(date) => format!("nightly build · {date}"),
                None => "nightly build".to_string(),
            };
        }
    }

    // Release-branch builds report a git-describe token such as `n8.1.2-50-g<hash>`.
    let versioned = trimmed
        .strip_prefix('n')
        .filter(|rest| rest.starts_with(|c: char| c.is_ascii_digit()))
        .unwrap_or(trimmed);

    let numeric: String = versioned
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    let numeric = numeric.trim_matches('.');
    if !numeric.is_empty() && numeric.chars().any(|c| c.is_ascii_digit()) {
        return numeric.to_string();
    }

    trimmed.to_string()
}

fn format_yyyymmdd(date: &str) -> Option<String> {
    let year = &date[..4];
    let month: usize = date[4..6].parse().ok()?;
    let day: u32 = date[6..8].parse().ok()?;
    let month_name = MONTH_ABBREVIATIONS.get(month.checked_sub(1)?)?;
    (1..=31)
        .contains(&day)
        .then(|| format!("{day} {month_name} {year}"))
}

fn media_engine_status(app: &AppHandle, state: &MediaEngineState) -> MediaEngineStatus {
    let mut status = MediaEngineStatus::not_found();
    match resolve_ffmpeg(app) {
        Resolved::NotFound => {}
        Resolved::Missing { path } => {
            status.status = "broken";
            status.source = Some(FfmpegSource::Custom.as_str());
            status.path = Some(path.clone());
            status.error = Some(format!("The ffmpeg at {path} is no longer available."));
        }
        Resolved::Found { path, source } => {
            status.source = Some(source.as_str());
            status.path = Some(path.display().to_string());
            match probe_ffmpeg_version(&path) {
                Ok(version) => {
                    status.status = "ready";
                    status.version_display = Some(display_version(&version));
                    status.version = Some(version);
                    if source == FfmpegSource::Managed {
                        status.update_available = state.cached_update();
                    }
                }
                Err(error) => {
                    status.status = "broken";
                    status.error = Some(error);
                }
            }
        }
    }
    status
}

#[tauri::command]
fn find_ffmpeg(app: AppHandle, state: State<'_, MediaEngineState>) -> MediaEngineStatus {
    media_engine_status(&app, &state)
}

#[tauri::command]
fn set_ffmpeg_path(
    app: AppHandle,
    state: State<'_, MediaEngineState>,
    path: Option<String>,
) -> Result<MediaEngineStatus, String> {
    if state.installing.load(Ordering::SeqCst) {
        return Err("Wait for the current ffmpeg installation to finish.".to_string());
    }
    let normalized = match path {
        Some(value) => {
            let candidate = PathBuf::from(value);
            if !valid_ffmpeg_path(&candidate) {
                return Err("Choose the ffmpeg executable, not its folder.".to_string());
            }
            if !candidate.with_file_name(ffprobe_exe_name()).is_file() {
                return Err("ffprobe must be in the same folder as ffmpeg.".to_string());
            }
            Some(candidate.display().to_string())
        }
        None => None,
    };
    let mut settings = load_settings(&app);
    settings.ffmpeg_path = normalized;
    save_settings(&app, &settings)?;
    Ok(media_engine_status(&app, &state))
}

/// Opens the folder containing the active ffmpeg in the system file manager.
/// Only the resolved path is ever revealed, never a caller-supplied one.
#[tauri::command]
fn reveal_ffmpeg_folder(app: AppHandle) -> Result<(), String> {
    let path = match resolve_ffmpeg(&app) {
        Resolved::Found { path, .. } => path,
        _ => return Err("No ffmpeg folder to show.".to_string()),
    };
    let folder = path
        .parent()
        .ok_or_else(|| "ffmpeg has no parent folder.".to_string())?;

    #[cfg(target_os = "windows")]
    let mut command = {
        let mut command = std::process::Command::new("explorer");
        command.arg(format!("/select,{}", path.display()));
        command
    };
    #[cfg(target_os = "macos")]
    let mut command = {
        let mut command = std::process::Command::new("open");
        command.arg("-R").arg(&path);
        command
    };
    #[cfg(all(unix, not(target_os = "macos")))]
    let mut command = {
        let mut command = std::process::Command::new("xdg-open");
        command.arg(folder);
        command
    };

    let _ = folder;
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("Could not open the ffmpeg folder: {error}"))
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

/// `Some(true)` when the upstream manifest lists a different archive hash than
/// the one the managed copy was built from; `None` when either side is unknown.
fn update_available(installed_sha256: Option<&str>, latest_sha256: Option<&str>) -> Option<bool> {
    let installed = installed_sha256?.trim().to_ascii_lowercase();
    let latest = latest_sha256?.trim().to_ascii_lowercase();
    if installed.is_empty() || latest.is_empty() {
        return None;
    }
    Some(installed != latest)
}

#[tauri::command]
async fn check_ffmpeg_update(
    app: AppHandle,
    state: State<'_, MediaEngineState>,
) -> Result<Option<bool>, String> {
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (&app, &state);
        Ok(None)
    }

    #[cfg(target_os = "windows")]
    {
        let is_managed = matches!(
            resolve_ffmpeg(&app),
            Resolved::Found {
                source: FfmpegSource::Managed,
                ..
            }
        );
        let installed = load_settings(&app).managed_archive_sha256;
        if !is_managed || installed.is_none() {
            return Ok(None);
        }
        if let Ok(guard) = state.update_check.lock() {
            if let Some((checked_at, cached)) = *guard {
                if checked_at.elapsed() < UPDATE_CHECK_TTL {
                    return Ok(cached);
                }
            }
        }

        let latest = async {
            let client = reqwest::Client::builder()
                .user_agent("GCCtoolkit ffmpeg installer")
                .timeout(UPDATE_CHECK_TIMEOUT)
                .build()
                .ok()?;
            let manifest = client
                .get(FFMPEG_CHECKSUMS_URL)
                .send()
                .await
                .ok()?
                .error_for_status()
                .ok()?
                .text()
                .await
                .ok()?;
            expected_checksum(&manifest, FFMPEG_ARCHIVE_NAME)
        }
        .await;

        let result = update_available(installed.as_deref(), latest.as_deref());
        state.remember_update(result);
        Ok(result)
    }
}

#[tauri::command]
fn cancel_ffmpeg_install(state: State<'_, MediaEngineState>) -> Result<(), String> {
    if !state.installing.load(Ordering::SeqCst) {
        return Err("No ffmpeg installation is running.".to_string());
    }
    state.cancel_requested.store(true, Ordering::SeqCst);
    Ok(())
}

#[cfg(target_os = "windows")]
fn emit_media_engine_progress(
    app: &AppHandle,
    phase: &'static str,
    bytes_done: u64,
    bytes_total: Option<u64>,
) {
    let _ = app.emit(
        "media-engine-progress",
        MediaEngineProgress {
            phase,
            bytes_done,
            bytes_total,
        },
    );
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

/// Downloads, verifies, extracts and tests the managed build inside
/// `staging_dir`, then swaps it into `bin_dir`. Returns the archive SHA-256.
#[cfg(target_os = "windows")]
async fn install_ffmpeg_staged(
    app: &AppHandle,
    state: &MediaEngineState,
    staging_dir: &Path,
    bin_dir: &Path,
) -> Result<String, String> {
    let cancelled = || state.cancel_requested.load(Ordering::SeqCst);
    let archive_path = staging_dir.join(FFMPEG_ARCHIVE_NAME);
    let staged_bin = staging_dir.join("bin");

    tokio::fs::create_dir_all(staging_dir)
        .await
        .map_err(|error| format!("Could not create the ffmpeg folder: {error}"))?;

    emit_media_engine_progress(app, "checksums", 0, None);
    let client = reqwest::Client::builder()
        .user_agent("GCCtoolkit ffmpeg installer")
        .build()
        .map_err(|error| format!("Could not initialise the downloader: {error}"))?;
    let manifest = client
        .get(FFMPEG_CHECKSUMS_URL)
        .send()
        .await
        .and_then(reqwest::Response::error_for_status)
        .map_err(|error| format!("Could not download ffmpeg checksums: {error}"))?
        .text()
        .await
        .map_err(|error| format!("Could not read ffmpeg checksums: {error}"))?;
    let expected = expected_checksum(&manifest, FFMPEG_ARCHIVE_NAME)
        .ok_or_else(|| "The checksum manifest does not list the Windows ffmpeg ZIP.".to_string())?;
    if cancelled() {
        return Err("Installation cancelled.".to_string());
    }

    let mut response = client
        .get(FFMPEG_ARCHIVE_URL)
        .send()
        .await
        .and_then(reqwest::Response::error_for_status)
        .map_err(|error| format!("Could not download ffmpeg: {error}"))?;
    let total = response.content_length();
    if total.is_some_and(|length| length > MAX_FFMPEG_ARCHIVE_BYTES) {
        return Err("The ffmpeg archive exceeds the safe download size limit.".to_string());
    }
    emit_media_engine_progress(app, "downloading", 0, total);

    let mut archive_file = tokio::fs::File::create(&archive_path)
        .await
        .map_err(|error| format!("Could not create the ffmpeg archive: {error}"))?;
    let mut hasher = Sha256::new();
    let mut downloaded_bytes = 0u64;
    let mut last_emit = Instant::now();
    let mut last_emitted_bytes = 0u64;
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|error| format!("ffmpeg download failed: {error}"))?
    {
        if cancelled() {
            return Err("Installation cancelled.".to_string());
        }
        downloaded_bytes += chunk.len() as u64;
        if downloaded_bytes > MAX_FFMPEG_ARCHIVE_BYTES {
            return Err("The ffmpeg archive exceeds the safe download size limit.".to_string());
        }
        hasher.update(&chunk);
        tokio::io::AsyncWriteExt::write_all(&mut archive_file, &chunk)
            .await
            .map_err(|error| format!("Could not save ffmpeg: {error}"))?;
        if last_emit.elapsed() >= Duration::from_millis(250)
            || downloaded_bytes - last_emitted_bytes >= 512 * 1024
        {
            emit_media_engine_progress(app, "downloading", downloaded_bytes, total);
            last_emit = Instant::now();
            last_emitted_bytes = downloaded_bytes;
        }
    }
    tokio::io::AsyncWriteExt::flush(&mut archive_file)
        .await
        .map_err(|error| format!("Could not finish saving ffmpeg: {error}"))?;
    drop(archive_file);
    emit_media_engine_progress(app, "downloading", downloaded_bytes, total);

    emit_media_engine_progress(app, "verifying", downloaded_bytes, total);
    let actual = format!("{:x}", hasher.finalize());
    if actual != expected {
        return Err("The ffmpeg download failed SHA-256 verification.".to_string());
    }

    emit_media_engine_progress(app, "extracting", downloaded_bytes, total);
    let archive_for_task = archive_path.clone();
    let staged_bin_for_task = staged_bin.clone();
    tokio::task::spawn_blocking(move || {
        extract_ffmpeg_tools(&archive_for_task, &staged_bin_for_task)
    })
    .await
    .map_err(|error| format!("ffmpeg extraction task failed: {error}"))??;
    let _ = tokio::fs::remove_file(&archive_path).await;
    if cancelled() {
        return Err("Installation cancelled.".to_string());
    }

    emit_media_engine_progress(app, "testing", downloaded_bytes, total);
    let staged_exe = staged_bin.join("ffmpeg.exe");
    tokio::task::spawn_blocking(move || probe_ffmpeg_version(&staged_exe))
        .await
        .map_err(|error| format!("ffmpeg test task failed: {error}"))?
        .map_err(|error| format!("The downloaded ffmpeg does not run: {error}"))?;

    // Swap: the previous bin/ is only renamed aside once the new one is proven.
    let retired = bin_dir.with_file_name("bin.previous");
    if retired.exists() {
        let _ = tokio::fs::remove_dir_all(&retired).await;
    }
    let had_previous = bin_dir.exists();
    if had_previous {
        rename_with_retry(bin_dir, &retired)
            .await
            .map_err(|error| {
                format!("Could not replace the existing ffmpeg (is it still running?): {error}")
            })?;
    }
    if let Err(error) = rename_with_retry(&staged_bin, bin_dir).await {
        if had_previous {
            let _ = tokio::fs::rename(&retired, bin_dir).await;
        }
        return Err(format!("Could not move ffmpeg into place: {error}"));
    }
    let _ = tokio::fs::remove_dir_all(&retired).await;

    Ok(actual)
}

/// Renames a directory, retrying briefly because Windows virus scanners hold a
/// transient lock on freshly extracted executables.
async fn rename_with_retry(from: &Path, to: &Path) -> std::io::Result<()> {
    let mut last_error = match tokio::fs::rename(from, to).await {
        Ok(()) => return Ok(()),
        Err(error) => error,
    };
    for attempt in 1..=5u32 {
        tokio::time::sleep(Duration::from_millis(200 * u64::from(attempt))).await;
        match tokio::fs::rename(from, to).await {
            Ok(()) => return Ok(()),
            Err(error) => last_error = error,
        }
    }
    Err(last_error)
}

#[tauri::command]
async fn install_ffmpeg(
    app: AppHandle,
    state: State<'_, MediaEngineState>,
) -> Result<MediaEngineStatus, String> {
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (&app, &state);
        Err("Automatic ffmpeg installation is currently available on Windows only.".to_string())
    }

    #[cfg(target_os = "windows")]
    {
        if state.installing.swap(true, Ordering::SeqCst) {
            return Err("An ffmpeg installation is already running.".to_string());
        }
        let _guard = InstallGuard(&state);
        state.cancel_requested.store(false, Ordering::SeqCst);

        let tools_dir = managed_tools_dir(&app)
            .ok_or_else(|| "Could not resolve the app data folder.".to_string())?;
        let bin_dir = tools_dir.join("bin");
        let staging_dir = tools_dir.join(format!(
            "staging-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis())
                .unwrap_or_default()
        ));

        let outcome = install_ffmpeg_staged(&app, &state, &staging_dir, &bin_dir).await;
        let _ = tokio::fs::remove_dir_all(&staging_dir).await;
        let archive_sha256 = outcome?;

        // The managed copy is found by location, so the custom override is
        // cleared rather than pointed at bin/ — `ffmpeg_path` now means custom.
        let mut settings = load_settings(&app);
        settings.ffmpeg_path = None;
        settings.managed_archive_sha256 = Some(archive_sha256);
        save_settings(&app, &settings)?;
        state.remember_update(Some(false));

        let status = media_engine_status(&app, &state);
        if status.status != "ready" {
            return Err(status.error.unwrap_or_else(|| {
                "ffmpeg installation did not produce a working executable.".to_string()
            }));
        }
        Ok(status)
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
        classify_source, display_version, expected_checksum, game_download_dir,
        parse_ffmpeg_version, safe_game_folder_name, update_available, FfmpegSource,
    };
    use std::path::Path;

    #[test]
    fn displays_release_versions_without_build_suffix() {
        assert_eq!(display_version("8.1"), "8.1");
        assert_eq!(display_version("8.1.1-full_build"), "8.1.1");
        assert_eq!(display_version("7.1-essentials_build-www.gyan.dev"), "7.1");
        assert_eq!(display_version("n8.1"), "8.1");
        assert_eq!(display_version("n8.1.2-50-g1a748fe2cd-20260902"), "8.1.2");
        assert_eq!(display_version("nightly"), "nightly");
    }

    #[test]
    fn displays_nightly_builds_with_date() {
        assert_eq!(
            display_version("N-126390-g9fc8c785e2-20260902"),
            "nightly build · 2 Sep 2026"
        );
        assert_eq!(display_version("N-126390-g9fc8c785e2"), "nightly build");
        assert_eq!(
            display_version("N-not-a-git-describe"),
            "N-not-a-git-describe"
        );
    }

    #[test]
    fn classifies_source_by_location() {
        let managed_bin = Path::new(r"C:\data\com.ackrosgaming.gcc\tools\ffmpeg\bin");
        let managed_exe = managed_bin.join("ffmpeg.exe");
        assert_eq!(
            classify_source(&managed_exe, Some(managed_bin), true),
            FfmpegSource::Managed
        );
        assert_eq!(
            classify_source(&managed_exe, Some(managed_bin), false),
            FfmpegSource::Managed
        );
        let elsewhere = Path::new(r"F:\ffmpeg\bin\ffmpeg.exe");
        assert_eq!(
            classify_source(elsewhere, Some(managed_bin), true),
            FfmpegSource::Custom
        );
        assert_eq!(
            classify_source(elsewhere, Some(managed_bin), false),
            FfmpegSource::System
        );
        assert_eq!(
            classify_source(elsewhere, None, false),
            FfmpegSource::System
        );
    }

    #[test]
    fn compares_archive_hashes_for_updates() {
        let a = "a".repeat(64);
        let b = "b".repeat(64);
        assert_eq!(update_available(Some(&a), Some(&a)), Some(false));
        assert_eq!(
            update_available(Some(&a), Some(&a.to_ascii_uppercase())),
            Some(false)
        );
        assert_eq!(update_available(Some(&a), Some(&b)), Some(true));
        assert_eq!(update_available(None, Some(&b)), None);
        assert_eq!(update_available(Some(&a), None), None);
        assert_eq!(update_available(Some(""), Some(&b)), None);
    }

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
            "{}  other.zip\n{} *ffmpeg-n8.1-latest-win64-lgpl-8.1.zip\n",
            "b".repeat(64),
            checksum
        );

        assert_eq!(
            expected_checksum(&manifest, "ffmpeg-n8.1-latest-win64-lgpl-8.1.zip"),
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
        .manage(MediaEngineState::default())
        .invoke_handler(tauri::generate_handler![
            get_app_metadata,
            load_history,
            save_history,
            fetch_steam_trailers,
            search_steam_games_by_name,
            find_ffmpeg,
            set_ffmpeg_path,
            install_ffmpeg,
            cancel_ffmpeg_install,
            check_ffmpeg_update,
            reveal_ffmpeg_folder,
            get_default_download_dir,
            download_trailers,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
