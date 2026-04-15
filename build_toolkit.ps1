# Build Script for Game Content Creator Toolkit
# This script bundles the toolkit files into a zip archive for distribution.

$Version = "0.1.0-BETA"
$ToolkitFolder = "./toolkit"
$OutputDir = "./dist"
$ZipFile = "$OutputDir/GameContentToolkit-$Version.zip"

Write-Host "📦 Building Game Content Creator Toolkit v$Version..." -ForegroundColor Cyan

# Create output directory if it doesn't exist
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir | Out-Null
}

# Clean previous zip
if (Test-Path $ZipFile) {
    Remove-Item $ZipFile -Force
}

Write-Host "Creating zip package..." -Selection 1
Compress-Archive -Path "$ToolkitFolder/*" -DestinationPath $ZipFile -Force

Write-Host "✅ Build complete: $ZipFile" -ForegroundColor Green
