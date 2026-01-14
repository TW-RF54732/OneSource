@echo off
setlocal EnableDelayedExpansion
color 0B
title OneSource Installer

:: =========================================================
::  VERSION HOLDER (Updated by release.py)
:: =========================================================
set "VERSION=v1.2.0"

:: =========================================================
::  BANNER
:: =========================================================
cls
echo.
echo ==========================================================
echo   ____  _   _ _____   ____   ___  _   _ ____   ____ _____ 
echo  / __ ^| \ ^| ^| ____^| / ___^| / _ \^| ^| ^| ^|  _ \ / ___^| ____^|
echo ^| ^|  ^| ^|  \^| ^|  _^|   \___ \^| ^| ^| ^| ^| ^| ^| ^|_) ^| ^|   ^|  _^|  
echo ^| ^|__^| ^| ^|\  ^| ^|___   ___) ^| ^|_^| ^| ^|_^| ^|  _ ^<^| ^|___^| ^|___ 
echo  \____/^|_^| \_^|_____^| ^|____/ \___/ \___/^|_^| \_\\____^|_____^|
echo.                          
echo  ^>^> OneSource %VERSION% ^| Local Installer ^<^<
echo ==========================================================
echo.

:: =========================================================
::  CONFIGURATION
:: =========================================================
set "INSTALL_DIR=%LOCALAPPDATA%\Programs\OneSource"
set "EXE_NAME=OneSource.exe"

:: =========================================================
::  INSTALLATION LOGIC
:: =========================================================

echo [1/3] Creating installation directory...
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"
echo       Target: %INSTALL_DIR%

echo.
echo [2/3] Copying executable...
:: Copy file from the current directory (%~dp0) to target
copy /Y "%~dp0%EXE_NAME%" "%INSTALL_DIR%\%EXE_NAME%" >nul
if %errorlevel% neq 0 (
    echo.
    echo [ERROR] Failed to find %EXE_NAME%.
    echo Please make sure you extracted the ENTIRE zip file.
    pause
    exit /b
)
echo       Done.

echo.
echo [3/3] Updating System PATH...
:: Use PowerShell to cleanly append to PATH without truncation risks
powershell -Command "$p=[Environment]::GetEnvironmentVariable('Path', 'User'); if ($p -notlike '*%INSTALL_DIR%*') { [Environment]::SetEnvironmentVariable('Path', $p + ';%INSTALL_DIR%', 'User'); Write-Host '      [SUCCESS] Added to PATH.' -ForegroundColor Green } else { Write-Host '      [SKIP] Already in PATH.' -ForegroundColor Gray }"

echo.
echo ==========================================================
echo   INSTALLATION COMPLETE!
echo   You can now delete this installer folder.
echo   Please RESTART your terminal to use 'OneSource'.
echo ==========================================================
echo.
pause