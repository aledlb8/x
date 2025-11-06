$ErrorActionPreference = "Stop"

function Get-DefaultTarget {
    $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture
    switch ($arch) {
        "X64" { return "x86_64-pc-windows-msvc" }
        default {
            throw "Unsupported Windows architecture: $arch. Set X_TARGET to a supported Rust target triple."
        }
    }
}

$Repo = "aledlb8/x"
$BinaryBaseName = "x"
$BinaryFileName = "x.exe"
$InstallRoot = if ($env:X_INSTALL_ROOT) { $env:X_INSTALL_ROOT } else { Join-Path $env:LOCALAPPDATA "x-cli" }
$BinDir = if ($env:X_INSTALL_BIN_DIR) { $env:X_INSTALL_BIN_DIR } else { $InstallRoot }
$InstallPath = Join-Path $BinDir $BinaryFileName
$VersionSpec = if ($env:X_VERSION) { $env:X_VERSION } else { "latest" }
$TargetTriple = if ($env:X_TARGET) { $env:X_TARGET } else { Get-DefaultTarget }

function Resolve-ReleaseMetadata([string] $Version) {
    if ($Version -eq "latest") {
        $uri = "https://api.github.com/repos/$Repo/releases/latest"
    }
    else {
        $tag = if ($Version.StartsWith("v")) { $Version } else { "v$Version" }
        $uri = "https://api.github.com/repos/$Repo/releases/tags/$tag"
    }

    return Invoke-RestMethod -Uri $uri -Headers @{ "User-Agent" = "x-installer" }
}

function Find-AssetUrl([object] $Release, [string] $Pattern) {
    foreach ($asset in $Release.assets) {
        if ($asset.browser_download_url -like "*$Pattern*") {
            return $asset.browser_download_url
        }
    }

    return $null
}

function Ensure-Directory([string] $Path) {
    if (-not (Test-Path $Path)) {
        New-Item -ItemType Directory -Force -Path $Path | Out-Null
    }
}

function Update-UserPath([string] $Directory) {
    $envTarget = [System.EnvironmentVariableTarget]::User
    $currentPath = [System.Environment]::GetEnvironmentVariable("Path", $envTarget)
    if ($currentPath -notlike "*$Directory*") {
        $newPath = if ($currentPath) { "$currentPath;$Directory" } else { $Directory }
        [System.Environment]::SetEnvironmentVariable("Path", $newPath, $envTarget)
        Write-Host "Updated PATH to include $Directory" -ForegroundColor Green
    }
    else {
        Write-Host "$Directory is already in your PATH." -ForegroundColor Yellow
    }
}

function Install-From-Source {
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        throw "No prebuilt binary available for $TargetTriple and Rust is not installed. Install Rust from https://rustup.rs/ or provide X_TARGET."
    }

    Write-Host "Building from source with cargo install..." -ForegroundColor Yellow
    Ensure-Directory $InstallRoot
    & cargo install --git "https://github.com/$Repo.git" --locked --force --root $InstallRoot

    $buildBinDir = Join-Path $InstallRoot "bin"
    if ($BinDir -ne $buildBinDir) {
        Ensure-Directory $BinDir
        $builtBinary = Join-Path $buildBinDir $BinaryFileName
        Copy-Item -Path $builtBinary -Destination $InstallPath -Force
    }
    else {
        $script:InstallPath = Join-Path $buildBinDir $BinaryFileName
    }

    Write-Host "Installed CLI to $InstallPath" -ForegroundColor Green
}

function Install-Binary {
    param(
        [string] $BinarySource
    )

    Ensure-Directory $BinDir
    Copy-Item -Path $BinarySource -Destination $InstallPath -Force
    Write-Host "Installed CLI to $InstallPath" -ForegroundColor Green
}

try {
    Write-Host "Fetching release metadata..." -ForegroundColor Cyan
    $release = Resolve-ReleaseMetadata -Version $VersionSpec
    $assetPattern = "$BinaryBaseName-$TargetTriple"
    $assetUrl = Find-AssetUrl -Release $release -Pattern $assetPattern

    if (-not $assetUrl) {
        Write-Warning "No prebuilt artifact was found for $TargetTriple. Attempting to build from source."
        Install-From-Source
    }
    else {
        $tempDir = Join-Path ([IO.Path]::GetTempPath()) ([guid]::NewGuid().ToString())
        Ensure-Directory $tempDir
        $archivePath = Join-Path $tempDir "package.zip"

        try {
            Write-Host "Downloading $assetUrl ..." -ForegroundColor Cyan
            Invoke-WebRequest -Uri $assetUrl -OutFile $archivePath -Headers @{ "User-Agent" = "x-installer" }

            $extractDir = Join-Path $tempDir "extract"
            Ensure-Directory $extractDir
            Expand-Archive -Path $archivePath -DestinationPath $extractDir -Force

            $binary = Get-ChildItem -Path $extractDir -Filter $BinaryFileName -Recurse -File | Select-Object -First 1
            if (-not $binary) {
                throw "Extracted archive did not contain $BinaryFileName."
            }

            Install-Binary -BinarySource $binary.FullName
        }
        finally {
            if (Test-Path $tempDir) {
                Remove-Item -Recurse -Force -Path $tempDir
            }
        }
    }

    Update-UserPath $BinDir
    Write-Host "Installation complete. Please restart your terminal and run 'x --help'." -ForegroundColor Green
}
catch {
    Write-Host "Installation failed: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}
