use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncBufReadExt, BufReader};

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

fn configure_hidden_process(command: &mut tokio::process::Command) {
    #[cfg(windows)]
    {
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

    let trailers = movies
        .iter()
        .filter_map(|m| {
            let name = m.get("name")?.as_str()?.to_string();
            let dash_url = m.get("dash_h264")?.as_str()?.to_string();
            Some(TrailerInfo { name, dash_url })
        })
        .collect();

    Ok(SteamTrailerResponse { trailers })
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

fn find_ffmpeg_path() -> Option<String> {
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

#[tauri::command]
fn find_ffmpeg() -> Option<String> {
    find_ffmpeg_path()
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

// ── Download ──────────────────────────────────────────────────────────────

#[tauri::command]
async fn download_trailers(
    app: AppHandle,
    output_dir: String,
    trailers: Vec<TrailerInfo>,
) -> Result<DownloadSummary, String> {
    let ffmpeg = find_ffmpeg_path().ok_or_else(|| {
        "ffmpeg not found. Install it (winget/scoop/choco/brew) or check Settings.".to_string()
    })?;

    let out_dir = Path::new(&output_dir);
    tokio::fs::create_dir_all(out_dir)
        .await
        .map_err(|e| format!("Could not create output folder: {e}"))?;

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
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_app_metadata,
            fetch_steam_trailers,
            search_steam_games_by_name,
            find_ffmpeg,
            get_default_download_dir,
            download_trailers,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
