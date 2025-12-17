# U Language - Windows Build Script (PowerShell)
# MIT License — Copyright (c) 2025 Webcien and U contributors

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "U Language Windows Build Script" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check if Rust is installed
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "ERROR: Cargo not found. Please install Rust from https://rustup.rs/" -ForegroundColor Red
    exit 1
}

# Check if Zig is installed
if (-not (Get-Command zig -ErrorAction SilentlyContinue)) {
    Write-Host "ERROR: Zig not found. Please install Zig from https://ziglang.org/download/" -ForegroundColor Red
    exit 1
}

Write-Host "[1/3] Building U compiler..." -ForegroundColor Yellow
Set-Location compiler
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Failed to build compiler" -ForegroundColor Red
    exit 1
}
Set-Location ..

Write-Host "[2/3] Building LSP server..." -ForegroundColor Yellow
Set-Location lsp
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Failed to build LSP server" -ForegroundColor Red
    exit 1
}
Set-Location ..

Write-Host "[3/3] Setting up environment..." -ForegroundColor Yellow
$UL_COMPILER = Join-Path $PWD "compiler\target\release\ul.exe"
$UL_LSP = Join-Path $PWD "lsp\target\release\u-lsp.exe"

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "Build completed successfully!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Compiler: $UL_COMPILER" -ForegroundColor White
Write-Host "LSP Server: $UL_LSP" -ForegroundColor White
Write-Host ""
Write-Host "To use U Language, add to your PATH:" -ForegroundColor Yellow
Write-Host "  $PWD\compiler\target\release" -ForegroundColor White
Write-Host ""
Write-Host "Or run directly:" -ForegroundColor Yellow
Write-Host "  & '$UL_COMPILER' build your_file.ul" -ForegroundColor White
Write-Host ""

# Optionally add to PATH for current session
$env:Path += ";$PWD\compiler\target\release"
Write-Host "Added to PATH for current session." -ForegroundColor Green
