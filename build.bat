@echo off
REM U Language - Windows Build Script
REM MIT License — Copyright (c) 2025 Webcien and U contributors

echo ========================================
echo U Language Windows Build Script
echo ========================================
echo.

REM Check if Rust is installed
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Cargo not found. Please install Rust from https://rustup.rs/
    exit /b 1
)

REM Check if Zig is installed
where zig >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Zig not found. Please install Zig from https://ziglang.org/download/
    exit /b 1
)

echo [1/3] Building U compiler...
cd compiler
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Failed to build compiler
    exit /b 1
)
cd ..

echo [2/3] Building LSP server...
cd lsp
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Failed to build LSP server
    exit /b 1
)
cd ..

echo [3/3] Setting up environment...
set UL_COMPILER=%CD%\compiler\target\release\ul.exe
set UL_LSP=%CD%\lsp\target\release\u-lsp.exe

echo.
echo ========================================
echo Build completed successfully!
echo ========================================
echo.
echo Compiler: %UL_COMPILER%
echo LSP Server: %UL_LSP%
echo.
echo To use U Language, add to your PATH:
echo   %CD%\compiler\target\release
echo.
echo Or run directly:
echo   %UL_COMPILER% build your_file.ul
echo.

pause
