$URL = "https://github.com/aledlb8/x/releases/download/0.1.3/x.exe"

$INSTALL_PATH = "$env:LOCALAPPDATA\x-cli"
$EXE_PATH = "$INSTALL_PATH\x.exe"

Write-Host "Downloading CLI..." -ForegroundColor Cyan

New-Item -ItemType Directory -Force -Path $INSTALL_PATH | Out-Null

Invoke-WebRequest -Uri $URL -OutFile $EXE_PATH

Write-Host "Downloaded CLI to $EXE_PATH" -ForegroundColor Green

Write-Host "Adding installation folder to your User PATH..." -ForegroundColor Cyan
$UserPath = [System.Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::User)
if ($UserPath -notlike "*$INSTALL_PATH*") {
    [System.Environment]::SetEnvironmentVariable("Path", "$UserPath;$INSTALL_PATH", [System.EnvironmentVariableTarget]::User)
    Write-Host "Updated PATH to include $INSTALL_PATH" -ForegroundColor Green
} else {
    Write-Host "$INSTALL_PATH is already in your PATH." -ForegroundColor Yellow
}

Write-Host "✅ Installation complete! Please close and reopen your terminal, then run 'x --help'." -ForegroundColor Green