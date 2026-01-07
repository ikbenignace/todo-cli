# Todo CLI Installer for Windows

param(
    [string]$Version = "latest",
    [string]$InstallDir = "$env:USERPROFILE\bin"
)

$ErrorActionPreference = "Stop"

Write-Host "🚀 Installing Todo CLI..." -ForegroundColor Green

# Detect architecture
if ([Environment]::Is64BitProcess) {
    $Filename = "todo-windows-x86_64.exe"
} else {
    Write-Host "❌ Only 64-bit Windows is supported" -ForegroundColor Red
    exit 1
}

# Determine download URL
if ($Version -eq "latest") {
    Write-Host "📥 Fetching latest release..." -ForegroundColor Yellow
    $LatestRelease = Invoke-RestMethod -Uri "https://api.github.com/repos/ikbenignace/todo-cli/releases/latest"
    $DownloadUrl = ($LatestRelease.assets | Where-Object { $_.name -eq $Filename }).browser_download_url
} else {
    $DownloadUrl = "https://github.com/ikbenignace/todo-cli/releases/download/$Version/$Filename"
}

if (-not $DownloadUrl) {
    Write-Host "❌ Could not find release for $Filename" -ForegroundColor Red
    exit 1
}

# Create temporary directory
$TempDir = Join-Path $env:TEMP "todo-cli-install"
New-Item -ItemType Directory -Path $TempDir -Force | Out-Null

# Download binary
Write-Host "📥 Downloading $Filename..." -ForegroundColor Yellow
$DownloadPath = Join-Path $TempDir $Filename
Invoke-WebRequest -Uri $DownloadUrl -OutFile $DownloadPath -UseBasicParsing

# Create install directory
Write-Host "📦 Installing to $InstallDir..." -ForegroundColor Yellow
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# Copy binary
$Destination = Join-Path $InstallDir "todo.exe"
Copy-Item -Path $DownloadPath -Destination $Destination -Force

# Cleanup
Remove-Item -Path $TempDir -Recurse -Force

Write-Host "✅ Todo CLI installed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "To use it, add $InstallDir to your PATH:" -ForegroundColor Yellow
Write-Host "  1. Press Win+R, type 'sysdm.cpl', and press Enter" -ForegroundColor White
Write-Host "  2. Go to Advanced > Environment Variables" -ForegroundColor White
Write-Host "  3. Edit 'Path' in User variables" -ForegroundColor White
Write-Host "  4. Add $InstallDir" -ForegroundColor White
Write-Host ""
Write-Host "Or run this command to add to PATH (current session only):" -ForegroundColor Yellow
Write-Host "  `$env:Path += \";$InstallDir\"" -ForegroundColor White
Write-Host ""
Write-Host "Then you can use:" -ForegroundColor Yellow
Write-Host "  todo \"your task description\"" -ForegroundColor White
