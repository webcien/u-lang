# link_gui_windows.ps1 — Automatic linking script for GUI apps on Windows
# MIT License — Copyright (c) 2025 Webcien and U contributors

param(
    [Parameter(Mandatory=$true)]
    [string]$AppName
)

$CFile = "$AppName.c"
$ExeFile = "$AppName.exe"
$RuntimeDir = Join-Path $PSScriptRoot "..\runtime"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "U Language GUI Linking Script" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "App: $AppName" -ForegroundColor White
Write-Host "C file: $CFile" -ForegroundColor White
Write-Host "Output: $ExeFile" -ForegroundColor White
Write-Host ""

# Check if C file exists
if (-not (Test-Path $CFile)) {
    Write-Host "ERROR: $CFile not found" -ForegroundColor Red
    Write-Host "Please compile your .ul file first:" -ForegroundColor Yellow
    Write-Host "  ul build $AppName.ul --no-link" -ForegroundColor White
    exit 1
}

# Check for Skia
$UseMockSkia = $false
if (-not $env:SKIA_DIR) {
    Write-Host "WARNING: SKIA_DIR not set" -ForegroundColor Yellow
    Write-Host "Using mock Skia implementation" -ForegroundColor Yellow
    $UseMockSkia = $true
} else {
    Write-Host "Using Skia from: $env:SKIA_DIR" -ForegroundColor Green
}

Write-Host ""
Write-Host "[1/2] Compiling..." -ForegroundColor Yellow

# Try MSVC first
if (Get-Command cl -ErrorAction SilentlyContinue) {
    Write-Host "Using MSVC compiler" -ForegroundColor Green
    if ($UseMockSkia) {
        & cl /nologo $CFile "$RuntimeDir\skia_windows.c" /Fe:$ExeFile /link user32.lib gdi32.lib
    } else {
        & cl /nologo $CFile "$RuntimeDir\skia_windows.c" /I"$env:SKIA_INCLUDE_DIR" /Fe:$ExeFile /link "$env:SKIA_LIB_DIR\skia.lib" user32.lib gdi32.lib
    }
} elseif (Get-Command gcc -ErrorAction SilentlyContinue) {
    Write-Host "Using MinGW compiler" -ForegroundColor Green
    if ($UseMockSkia) {
        & gcc $CFile "$RuntimeDir\skia_windows.c" -lgdi32 -luser32 -o $ExeFile
    } else {
        & gcc $CFile "$RuntimeDir\skia_windows.c" -I"$env:SKIA_INCLUDE_DIR" -L"$env:SKIA_LIB_DIR" -lskia -lgdi32 -luser32 -o $ExeFile
    }
} else {
    Write-Host "ERROR: No C compiler found" -ForegroundColor Red
    Write-Host "Please install:" -ForegroundColor Yellow
    Write-Host "  - Visual Studio (MSVC)" -ForegroundColor White
    Write-Host "  - Or MinGW-w64" -ForegroundColor White
    exit 1
}

if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "ERROR: Compilation failed" -ForegroundColor Red
    exit 1
}

Write-Host "[2/2] Linking completed" -ForegroundColor Green

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "Build successful!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Executable: $ExeFile" -ForegroundColor White
Write-Host ""
Write-Host "To run:" -ForegroundColor Yellow
Write-Host "  .\$ExeFile" -ForegroundColor White
Write-Host ""
