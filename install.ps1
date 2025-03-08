$URL = "https://github.com/aledlb8/x/releases/download/2.0/x-win.exe"
$INSTALL_PATH = "$env:ProgramFiles\x-cli"
$EXE_PATH = "$INSTALL_PATH\x.exe"

Write-Host "Downloading CLI..."
New-Item -ItemType Directory -Force -Path $INSTALL_PATH | Out-Null
Invoke-WebRequest -Uri $URL -OutFile $EXE_PATH

Write-Host "Adding to PATH..."
$Path = [System.Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::Machine)
if ($Path -notlike "*$INSTALL_PATH*") {
    [System.Environment]::SetEnvironmentVariable("Path", "$Path;$INSTALL_PATH", [System.EnvironmentVariableTarget]::Machine)
}

Write-Host "✅ Installed! Close and reopen your terminal, then run 'x --help'."