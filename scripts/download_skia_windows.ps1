# download_skia_windows.ps1 — Download pre-compiled Skia for Windows
# MIT License — Copyright (c) 2025 Webcien and U contributors

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Skia Download Script for Windows" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$SKIA_VERSION = "m116"
$SKIA_DIR = "C:\skia"
$DOWNLOAD_URL = "https://github.com/JetBrains/skia-build/releases/download/$SKIA_VERSION/Skia-$SKIA_VERSION-windows-Release-x64.zip"

Write-Host "This script will download and install Skia for Windows." -ForegroundColor Yellow
Write-Host "Installation directory: $SKIA_DIR" -ForegroundColor White
Write-Host ""

# Check if already installed
if (Test-Path $SKIA_DIR) {
    Write-Host "Skia is already installed at $SKIA_DIR" -ForegroundColor Green
    $response = Read-Host "Do you want to reinstall? (y/N)"
    if ($response -ne "y") {
        Write-Host "Installation cancelled." -ForegroundColor Yellow
        exit 0
    }
    Remove-Item -Recurse -Force $SKIA_DIR
}

# Create directory
Write-Host "[1/4] Creating directory..." -ForegroundColor Yellow
New-Item -ItemType Directory -Force -Path $SKIA_DIR | Out-Null

# Download Skia
Write-Host "[2/4] Downloading Skia..." -ForegroundColor Yellow
Write-Host "URL: $DOWNLOAD_URL" -ForegroundColor Gray
$TEMP_ZIP = "$env:TEMP\skia.zip"

try {
    Invoke-WebRequest -Uri $DOWNLOAD_URL -OutFile $TEMP_ZIP -UseBasicParsing
    Write-Host "Download completed." -ForegroundColor Green
} catch {
    Write-Host "ERROR: Failed to download Skia" -ForegroundColor Red
    Write-Host "Please download manually from:" -ForegroundColor Yellow
    Write-Host "  https://github.com/JetBrains/skia-build/releases" -ForegroundColor White
    exit 1
}

# Extract
Write-Host "[3/4] Extracting..." -ForegroundColor Yellow
Expand-Archive -Path $TEMP_ZIP -DestinationPath $SKIA_DIR -Force
Remove-Item $TEMP_ZIP

# Set environment variables
Write-Host "[4/4] Setting environment variables..." -ForegroundColor Yellow
[System.Environment]::SetEnvironmentVariable("SKIA_DIR", $SKIA_DIR, [System.EnvironmentVariableTarget]::User)
[System.Environment]::SetEnvironmentVariable("SKIA_LIB_DIR", "$SKIA_DIR\lib", [System.EnvironmentVariableTarget]::User)
[System.Environment]::SetEnvironmentVariable("SKIA_INCLUDE_DIR", "$SKIA_DIR\include", [System.EnvironmentVariableTarget]::User)

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "Skia installed successfully!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Installation directory: $SKIA_DIR" -ForegroundColor White
Write-Host "Environment variables set:" -ForegroundColor White
Write-Host "  SKIA_DIR = $SKIA_DIR" -ForegroundColor Gray
Write-Host "  SKIA_LIB_DIR = $SKIA_DIR\lib" -ForegroundColor Gray
Write-Host "  SKIA_INCLUDE_DIR = $SKIA_DIR\include" -ForegroundColor Gray
Write-Host ""
Write-Host "Please restart your terminal for changes to take effect." -ForegroundColor Yellow
Write-Host ""
