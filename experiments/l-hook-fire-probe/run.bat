@echo off
chcp 65001 >nul
setlocal

set "SCRIPT_DIR=%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%SCRIPT_DIR%run.ps1"
echo.
echo Press any key to close this window.
pause >nul

endlocal
