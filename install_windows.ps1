# install_windows.ps1 — U Language Installer for Windows
# MIT License — Copyright (c) 2025 Webcien and U contributors

param(
    [switch]$WithSkia = $false
)

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "U Language Installer for Windows" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check administrator privileges
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "WARNING: Running without administrator privileges" -ForegroundColor Yellow
    Write-Host "Some features may not work correctly" -ForegroundColor Yellow
    Write-Host ""
}

# Step 1: Check prerequisites
Write-Host "[1/6] Checking prerequisites..." -ForegroundColor Yellow

$prerequisites = @{
    "Rust (cargo)" = "cargo"
    "Zig" = "zig"
    "Git" = "git"
}

$missing = @()
foreach ($tool in $prerequisites.GetEnumerator()) {
    if (Get-Command $tool.Value -ErrorAction SilentlyContinue) {
        Write-Host "  ✓ $($tool.Key) found" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $($tool.Key) not found" -ForegroundColor Red
        $missing += $tool.Key
    }
}

if ($missing.Count -gt 0) {
    Write-Host ""
    Write-Host "ERROR: Missing prerequisites:" -ForegroundColor Red
    foreach ($tool in $missing) {
        Write-Host "  - $tool" -ForegroundColor Red
    }
    Write-Host ""
    Write-Host "Please install:" -ForegroundColor Yellow
    Write-Host "  Rust: https://rustup.rs/" -ForegroundColor White
    Write-Host "  Zig: https://ziglang.org/download/" -ForegroundColor White
    Write-Host "  Git: https://git-scm.com/download/win" -ForegroundColor White
    exit 1
}

# Step 2: Clone repository (if not already cloned)
Write-Host ""
Write-Host "[2/6] Setting up repository..." -ForegroundColor Yellow

if (Test-Path "u-lang") {
    Write-Host "  Repository already exists" -ForegroundColor Green
    Set-Location u-lang
    git pull origin master
} else {
    Write-Host "  Cloning repository..." -ForegroundColor Gray
    git clone https://github.com/webcien/u-lang.git
    Set-Location u-lang
}

# Step 3: Build compiler
Write-Host ""
Write-Host "[3/6] Building U compiler..." -ForegroundColor Yellow
Set-Location compiler
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Failed to build compiler" -ForegroundColor Red
    exit 1
}
Set-Location ..

# Step 4: Build LSP server
Write-Host ""
Write-Host "[4/6] Building LSP server..." -ForegroundColor Yellow
Set-Location lsp
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Failed to build LSP server" -ForegroundColor Red
    exit 1
}
Set-Location ..

# Step 5: Install Skia (optional)
Write-Host ""
Write-Host "[5/6] Setting up Skia..." -ForegroundColor Yellow
if ($WithSkia) {
    Write-Host "  Installing Skia..." -ForegroundColor Gray
    & .\scripts\download_skia_windows.ps1
} else {
    Write-Host "  Skipping Skia installation (use -WithSkia to install)" -ForegroundColor Yellow
}

# Step 6: Set up environment
Write-Host ""
Write-Host "[6/6] Setting up environment..." -ForegroundColor Yellow

$UL_DIR = (Get-Location).Path
$UL_BIN = Join-Path $UL_DIR "compiler\target\release"

# Add to user PATH
$currentPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::User)
if ($currentPath -notlike "*$UL_BIN*") {
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$currentPath;$UL_BIN",
        [EnvironmentVariableTarget]::User
    )
    Write-Host "  Added to PATH: $UL_BIN" -ForegroundColor Green
} else {
    Write-Host "  Already in PATH" -ForegroundColor Green
}

# Set U_HOME
[Environment]::SetEnvironmentVariable("U_HOME", $UL_DIR, [EnvironmentVariableTarget]::User)
Write-Host "  Set U_HOME=$UL_DIR" -ForegroundColor Green

# Create desktop shortcut (optional)
$WshShell = New-Object -comObject WScript.Shell
$Shortcut = $WshShell.CreateShortcut("$env:USERPROFILE\Desktop\U Language.lnk")
$Shortcut.TargetPath = "powershell.exe"
$Shortcut.Arguments = "-NoExit -Command `"cd '$UL_DIR'`""
$Shortcut.IconLocation = "powershell.exe"
$Shortcut.Save()
Write-Host "  Created desktop shortcut" -ForegroundColor Green

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "Installation completed successfully!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "U Language is installed at: $UL_DIR" -ForegroundColor White
Write-Host ""
Write-Host "Compiler: ul.exe" -ForegroundColor White
Write-Host "LSP Server: u-lsp.exe" -ForegroundColor White
Write-Host ""
Write-Host "To start using U Language:" -ForegroundColor Yellow
Write-Host "  1. Restart your terminal (to load new PATH)" -ForegroundColor White
Write-Host "  2. Run: ul --version" -ForegroundColor White
Write-Host "  3. Try examples: cd examples\windows" -ForegroundColor White
Write-Host ""
Write-Host "Documentation: https://github.com/webcien/u-lang" -ForegroundColor Cyan
Write-Host ""

if (-not $WithSkia) {
    Write-Host "NOTE: Skia was not installed. To install it later:" -ForegroundColor Yellow
    Write-Host "  .\scripts\download_skia_windows.ps1" -ForegroundColor White
    Write-Host ""
}

Write-Host "Happy coding with U Language! 🚀" -ForegroundColor Green
