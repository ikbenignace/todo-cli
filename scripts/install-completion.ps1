# Todo CLI Completions Installer for PowerShell

param(
    [string]$InstallDir = "$env:USERPROFILE\Documents\PowerShell\Completions"
)

$ErrorActionPreference = "Stop"

Write-Host "🚀 Installing PowerShell completions for todo..." -ForegroundColor Green

# Create directory
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# Generate completion
$BinaryPath = Join-Path $PSScriptRoot "..\target\release\todo.exe"
& $BinaryPath completion powershell > (Join-Path $InstallDir "todo.ps1")

Write-Host "✅ Completions installed for PowerShell" -ForegroundColor Green
Write-Host ""
Write-Host "Completions will load automatically in new PowerShell sessions." -ForegroundColor Yellow
Write-Host "To load in current session, run:" -ForegroundColor Yellow
Write-Host "  . $env:USERPROFILE\Documents\PowerShell\Completions\todo.ps1" -ForegroundColor White
