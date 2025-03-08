$URL = "https://github.com/YOUR_USERNAME/YOUR_REPO/releases/latest/download/x-cli.zip"
$INSTALL_DIR = "$env:ProgramFiles\x-cli"

Write-Host "Downloading CLI..."
Invoke-WebRequest -Uri $URL -OutFile "$env:TEMP\x-cli.zip"

Write-Host "Extracting..."
Expand-Archive -Path "$env:TEMP\x-cli.zip" -DestinationPath $INSTALL_DIR -Force

Write-Host "Adding to PATH..."
[System.Environment]::SetEnvironmentVariable("Path", "$env:Path;$INSTALL_DIR", [System.EnvironmentVariableTarget]::Machine)

Write-Host "✅ Installed! Run 'x --help' to get started."