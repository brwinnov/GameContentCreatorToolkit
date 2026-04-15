# GGT (Grab Game Trailer) Development Plan

## Overview
GGT is a PowerShell 7-based tool for downloading Steam game trailers as local MP4 files using the Steam API. This plan outlines enhancements, cross-platform publishing, and implementation steps to make it accessible for game content creators on Windows, Linux, and macOS.

## Current State
- **Core Functionality**: Downloads trailers via Steam API (`dash_h264` URLs) using `ffmpeg` for DASH-to-MP4 conversion. Supports filtering (latest/oldest), interactive selection, and safe re-runs.
- **Scripts**: Primary `download_steam_trailers.ps1` (PowerShell 7). Reference `download_steam_trailers.sh` (bash, deprecated due to Windows PATH issues).
- **Dependencies**: `ffmpeg` (required). `yt-dlp` mentioned but unused.
- **Testing**: Manual testing on Windrose and No Man's Sky. No automated tests or CI/CD.
- **Documentation**: Comprehensive README and background docs.

## Future Features Roadmap

### Phase 1: Core Enhancements (High Priority)
- **Batch Processing**: Support multiple App IDs via CSV or command-line (e.g., `-AppIds 123,456,789`).
- **Quality Options**: Add `-Quality high|medium|low` to select DASH variants from manifest parsing.
- **Metadata Embedding**: Embed trailer info (name, date) into MP4s using `ffmpeg` tags. Generate JSON sidecar files.
- **Progress & Logging**: Add progress bars, verbose logging, and retry logic with exponential backoff.
- **Subtitles Support**: Download and mux subtitles if available in API response.

### Phase 2: User Experience (Medium Priority)
- **GUI Wrapper**: Simple WPF-based GUI for Windows; web-based (Node.js) for cross-platform.
- **Dry-Run Mode**: Extend `-ListOnly` with sizes, durations, and ETAs.
- **Output Customization**: Filename templates and auto-folder organization by game name.
- **yt-dlp Integration**: Optional fallback for robustness (detect installation, use if DASH fails).

### Phase 3: Advanced Features (Low Priority)
- **Playlist Generation**: Create M3U playlists or HTML galleries.
- **Steam Workshop Support**: Extend to Workshop items (research API).
- **Caching**: Local API response caching with TTL.
- **Analytics**: JSON/CSV reports of downloads.
- **Multi-Threading**: Parallel downloads (with rate limiting).

### Phase 4: Maintenance & Ecosystem
- **Automated Testing**: Pester tests for PS script; mock API responses.
- **CI/CD**: GitHub Actions for testing on Windows/Linux/macOS.
- **Version Checking**: Flag to check for updates from repo.
- **Error Telemetry**: Optional anonymous reporting.

## Publishing and Distribution Plan

### GitHub Repository Setup
- **Repo Name**: `steam-trailer-downloader` or `ggt-steam-trailers`.
- **Structure**:
  - `/scripts/`: Core PS and deprecated bash scripts.
  - `/docs/`: README, STEAM_TRAILER_DOWNLOAD.md, and expanded wiki.
  - `/setup/`: Platform-specific installation scripts.
  - `/tests/`: Pester tests and CI configs.
  - `/.github/`: Issue/PR templates, workflows.
- **Licensing**: MIT (compatible with ffmpeg/yt-dlp GPL/LGPL dependencies).
- **Releases**: Tagged versions (e.g., v1.0.0) with changelogs. Attach scripts and optional binaries.
- **Community**: Enable Issues, Discussions, PRs. Add badges for PS version, platforms, and license.

### Cross-Platform Strategy
- **Primary Script**: Promote `download_steam_trailers.ps1` as cross-platform (PowerShell 7 runs on all OSes).
- **Deprecate Bash**: Remove `download_steam_trailers.sh` to avoid confusion.
- **Unified Runner**: Create `run.ps1` wrapper that detects OS and invokes the main script.
- **Platform Wrappers**: Simple shell scripts (`run-linux.sh`, `run-mac.sh`) for dependency installation and execution.

## Dependencies and Installation

### Core Dependencies
- **PowerShell 7**: Cross-platform runtime.
- **ffmpeg**: For DASH processing (LGPL/GPL licensed).
- **yt-dlp** (Optional): For fallback downloading (GPL licensed).

### Platform-Specific Prerequisites and Installation

#### Windows 11
- **Pre-reqs**: PowerShell 7 (built-in or via winget), ffmpeg.
- **Install Command**: `winget install Microsoft.PowerShell; winget install ffmpeg`.
- **Notes**: Script auto-detects ffmpeg in common paths or PATH.

#### Linux (Ubuntu/Debian/etc.)
- **Pre-reqs**: `pwsh`, `ffmpeg`.
- **Install Command**:
  ```bash
  wget -q https://packages.microsoft.com/config/ubuntu/$(lsb_release -rs)/packages-microsoft-prod.deb
  sudo dpkg -i packages-microsoft-prod.deb
  sudo apt update
  sudo apt install -y powershell ffmpeg
  ```
- **Notes**: Use PS script directly. Provide `setup-linux.sh` for automation.

#### macOS
- **Pre-reqs**: `pwsh`, `ffmpeg`.
- **Install Command**: `brew install powershell/tap/powershell ffmpeg`.
- **Notes**: PS script runs natively. Fallback to direct Microsoft install if Homebrew unavailable.

### Unified Installation Strategy
- **Automated Setup**: `setup.ps1` script detects OS (`$PSVersionTable.Platform`) and installs dependencies via package managers (winget/apt/brew).
- **Container Option**: Dockerfile for isolated environments.
- **Binary Handling**: Do not bundle ffmpeg/yt-dlp in repo (size/licensing issues). Instead:
  - Provide `install-deps.ps1` to download latest binaries from official sources.
  - Attach binaries to GitHub Releases for convenience.
  - Users verify hashes for security.

## Implementation Steps

### Step 1: Repository Preparation (1-2 weeks)
- Create GitHub repo with initial structure.
- Migrate files, update README with installation guides.
- Add LICENSE, CONTRIBUTING.md, and issue templates.

### Step 2: Core Feature Development (4-6 weeks)
- Implement Phase 1 features in `download_steam_trailers.ps1`.
- Add Pester tests and basic CI/CD.
- Test on all platforms.

### Step 3: Cross-Platform Polish (2-4 weeks)
- Create setup scripts and wrappers.
- Update docs with platform-specific guides.
- Release v1.0.0.

### Step 4: Advanced Features & Maintenance (Ongoing)
- Roll out Phase 2-4 features based on feedback.
- Monitor issues, add telemetry if opted-in.
- Quarterly releases with updates.

## Risks and Mitigations
- **Dependency Licensing**: Stick to MIT; link to official ffmpeg/yt-dlp sources.
- **API Changes**: Monitor Steam API; add yt-dlp fallback.
- **Platform Compatibility**: Test on VMs for Linux/macOS.
- **Security**: No binaries in repo; encourage hash verification.

## Success Metrics
- 100+ GitHub stars.
- Successful downloads reported in issues.
- Cross-platform usage confirmed via feedback.

This plan positions GGT as a robust, user-friendly tool for game content creators. Contributions welcome!</content>
<filePath>c:\Users\barry\project\GGT\PLAN.md