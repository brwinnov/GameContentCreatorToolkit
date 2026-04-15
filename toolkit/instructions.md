# Game Content Creator Toolkit - Instructions

Welcome to the toolkit! This package includes standalone PowerShell scripts designed for content creators (streamers, YouTubers, etc.) to quickly grab assets from Steam.

## 📦 What's Included
- `Steam-Trailer-Downloader.ps1`: The main script for downloading high-quality game trailers.

## 🛠️ Prerequisites
Before running the script, you must have the following installed on your system:

### 1. PowerShell 7+ (Recommended)
While older versions of PowerShell might work, **PowerShell 7 (Core)** is highly recommended for performance and compatibility.
- [Download PowerShell 7](https://aka.ms/Powershell-Release?tag=stable)

### 2. ffmpeg
This toolkit uses `ffmpeg` to process video streams.
- **Link:** [Download ffmpeg @ gyan.dev](https://www.gyan.dev/ffmpeg/builds/ffmpeg-git-full.7z)
- **Installation:** Extract the contents. For best results, add the `bin` folder to your system **PATH** variable.
- **Auto-Detect:** If not on your PATH, the script will check common locations like `C:\ffmpeg\bin\ffmpeg.exe`.

## 🚀 How to Run
1. **Unzip** this toolkit to a folder of your choice (e.g., `C:\GGT\`).
2. **Open PowerShell** in that folder (Right-click folder -> "Open in Terminal" or "Open PowerShell window here").
3. **Set Execution Policy** (if this is your first time running scripts):
   ```powershell
   Set-ExecutionPolicy -Scope CurrentUser -ExecutionPolicy RemoteSigned
   ```
4. **Run the script**:
   ```powershell
   .\Steam-Trailer-Downloader.ps1 -AppId <STEAM_APP_ID>
   ```
   *Example:* For "No Man's Sky" (AppID: 275850):
   ```powershell
   .\Steam-Trailer-Downloader.ps1 -AppId 275850
   ```

## ❓ Finding the Steam App ID
The App ID is the number in the Steam store URL:
`https://store.steampowered.com/app/275850/No_Mans_Sky/`
The App ID here is **275850**.

---
*Game Content Creator Toolkit BETA*
[GitHub Repository](https://github.com/brwinnov/GameContentToolkit)
