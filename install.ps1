$URL = "https://github.com/aledlb8/x/releases/download/2.0/x-win.exe"
$INSTALL_PATH = "$env:ProgramFiles\x.exe"

Write-Host "Downloading CLI..."
Invoke-WebRequest -Uri $URL -OutFile $INSTALL_PATH

Write-Host "Adding to PATH..."
[System.Environment]::SetEnvironmentVariable("Path", "$env:Path;$INSTALL_PATH", [System.EnvironmentVariableTarget]::Machine)

Write-Host "✅ Installed! Run 'x --help' to get started."