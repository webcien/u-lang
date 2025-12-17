@echo off
REM link_gui_windows.bat — Automatic linking script for GUI apps on Windows
REM MIT License — Copyright (c) 2025 Webcien and U contributors

setlocal enabledelayedexpansion

if "%~1"=="" (
    echo Usage: link_gui_windows.bat ^<app_name^>
    echo Example: link_gui_windows.bat todo_app
    exit /b 1
)

set APP_NAME=%~1
set C_FILE=%APP_NAME%.c
set EXE_FILE=%APP_NAME%.exe
set RUNTIME_DIR=%~dp0..\runtime

echo ========================================
echo U Language GUI Linking Script
echo ========================================
echo.
echo App: %APP_NAME%
echo C file: %C_FILE%
echo Output: %EXE_FILE%
echo.

REM Check if C file exists
if not exist "%C_FILE%" (
    echo ERROR: %C_FILE% not found
    echo Please compile your .ul file first:
    echo   ul build %APP_NAME%.ul --no-link
    exit /b 1
)

REM Check for Skia
if not defined SKIA_DIR (
    echo WARNING: SKIA_DIR not set
    echo Using mock Skia implementation
    set USE_MOCK_SKIA=1
) else (
    echo Using Skia from: %SKIA_DIR%
    set USE_MOCK_SKIA=0
)

echo.
echo [1/2] Compiling...

REM Try MSVC first
where cl >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo Using MSVC compiler
    if !USE_MOCK_SKIA! EQU 1 (
        cl /nologo %C_FILE% %RUNTIME_DIR%\skia_windows.c /Fe:%EXE_FILE% /link user32.lib gdi32.lib
    ) else (
        cl /nologo %C_FILE% %RUNTIME_DIR%\skia_windows.c /I%SKIA_INCLUDE_DIR% /Fe:%EXE_FILE% /link %SKIA_LIB_DIR%\skia.lib user32.lib gdi32.lib
    )
    goto check_result
)

REM Try MinGW
where gcc >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo Using MinGW compiler
    if !USE_MOCK_SKIA! EQU 1 (
        gcc %C_FILE% %RUNTIME_DIR%\skia_windows.c -lgdi32 -luser32 -o %EXE_FILE%
    ) else (
        gcc %C_FILE% %RUNTIME_DIR%\skia_windows.c -I%SKIA_INCLUDE_DIR% -L%SKIA_LIB_DIR% -lskia -lgdi32 -luser32 -o %EXE_FILE%
    )
    goto check_result
)

echo ERROR: No C compiler found
echo Please install:
echo   - Visual Studio (MSVC)
echo   - Or MinGW-w64
exit /b 1

:check_result
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ERROR: Compilation failed
    exit /b 1
)

echo [2/2] Linking completed

echo.
echo ========================================
echo Build successful!
echo ========================================
echo.
echo Executable: %EXE_FILE%
echo.
echo To run:
echo   %EXE_FILE%
echo.

endlocal
