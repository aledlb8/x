#!/usr/bin/env bash
set -euo pipefail

REPO="aledlb8/x"
BINARY_NAME="x"
INSTALL_ROOT="${X_INSTALL_ROOT:-$HOME/.local}"
BIN_DIR="${X_INSTALL_BIN_DIR:-$INSTALL_ROOT/bin}"
VERSION="${X_VERSION:-latest}"
TARGET_TRIPLE="${X_TARGET:-}"

DOWNLOADER=""

have_command() {
    command -v "$1" >/dev/null 2>&1
}

setup_downloader() {
    if have_command curl; then
        DOWNLOADER="curl"
    elif have_command wget; then
        DOWNLOADER="wget"
    else
        echo "Error: Please install curl or wget to continue." >&2
        exit 1
    fi
}

detect_target_triple() {
    local os arch
    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Linux*) os="linux" ;;
        Darwin*) os="macos" ;;
        *)
            echo "Unsupported operating system: $os" >&2
            exit 1
            ;;
    esac

    case "$os" in
        linux)
            case "$arch" in
                x86_64|amd64) echo "x86_64-unknown-linux-gnu" ;;
                arm64|aarch64) echo "aarch64-unknown-linux-gnu" ;;
                *)
                    echo "Unsupported Linux architecture: $arch" >&2
                    exit 1
                    ;;
            esac
            ;;
        macos)
            case "$arch" in
                x86_64|amd64) echo "x86_64-apple-darwin" ;;
                arm64|aarch64) echo "aarch64-apple-darwin" ;;
                *)
                    echo "Unsupported macOS architecture: $arch" >&2
                    exit 1
                    ;;
            esac
            ;;
    esac
}

fetch_release_json() {
    local api_url cmd

    if [[ "$VERSION" == "latest" ]]; then
        api_url="https://api.github.com/repos/${REPO}/releases/latest"
    else
        api_url="https://api.github.com/repos/${REPO}/releases/tags/${VERSION}"
    fi

    case "$DOWNLOADER" in
        curl)
            if ! release_json=$(curl -fsSL "$api_url"); then
                echo "Failed to fetch release metadata from GitHub." >&2
                exit 1
            fi
            ;;
        wget)
            if ! release_json=$(wget -qO- "$api_url"); then
                echo "Failed to fetch release metadata from GitHub." >&2
                exit 1
            fi
            ;;
    esac

    echo "$release_json"
}

extract_release_tag() {
    printf '%s\n' "$1" |
        grep -m1 '"tag_name"' |
        sed -E 's/.*"tag_name": *"([^"]+)".*/\1/'
}

extract_asset_url() {
    local release_json pattern
    release_json="$1"
    pattern="$2"

    printf '%s\n' "$release_json" |
        grep -oE "\"browser_download_url\": \"[^\"]*${pattern}[^\"]*\"" |
        head -n1 |
        sed -E 's/^.*"([^"]+)".*$/\1/'
}

download_asset() {
    local url output
    url="$1"
    output="$2"

    case "$DOWNLOADER" in
        curl)
            if ! curl -fsSL -o "$output" "$url"; then
                return 1
            fi
            ;;
        wget)
            if ! wget -qO "$output" "$url"; then
                return 1
            fi
            ;;
    esac

    return 0
}

install_binary() {
    local src_path="$1"

    mkdir -p "$BIN_DIR"
    install -m 755 "$src_path" "$BIN_DIR/$BINARY_NAME"
}

extract_archive() {
    local archive_path extract_dir
    archive_path="$1"
    extract_dir="$2"

    case "$archive_path" in
        *.tar.gz|*.tgz)
            tar -xzf "$archive_path" -C "$extract_dir"
            ;;
        *.tar.xz)
            tar -xJf "$archive_path" -C "$extract_dir"
            ;;
        *.zip)
            if ! have_command unzip; then
                echo "Error: unzip is required to extract $archive_path" >&2
                exit 1
            fi
            unzip -q "$archive_path" -d "$extract_dir"
            ;;
        *.gz)
            gunzip -c "$archive_path" >"$extract_dir/$BINARY_NAME"
            ;;
        *)
            cp "$archive_path" "$extract_dir/"
            ;;
    esac
}

post_install_message() {
    if ! printf '%s\n' "$PATH" | tr ':' '\n' | grep -Fx "$BIN_DIR" >/dev/null; then
        cat <<EOF
Installed to $BIN_DIR but that directory is not on your PATH.
Add the following to your shell configuration:
    export PATH="\$PATH:$BIN_DIR"
EOF
    fi

    echo "Installation complete. Run 'x --help' to get started."
}

build_from_source() {
    if ! have_command cargo; then
        cat <<'EOF' >&2
No prebuilt binary available for this platform and Rust is not installed.
Please install Rust from https://rustup.rs/ and re-run this script.
EOF
        exit 1
    fi

    echo "Building from source with cargo install..." >&2
    mkdir -p "$INSTALL_ROOT"
    cargo install --git "https://github.com/${REPO}.git" --locked --force --root "$INSTALL_ROOT"

    if [[ "$BIN_DIR" != "$INSTALL_ROOT/bin" ]]; then
        install -m 755 "$INSTALL_ROOT/bin/$BINARY_NAME" "$BIN_DIR/$BINARY_NAME"
    fi

    post_install_message
}

main() {
    local target release_json release_tag asset_pattern asset_url tmp_dir archive_path binary_path

    setup_downloader
    if [[ -z "$TARGET_TRIPLE" ]]; then
        TARGET_TRIPLE="$(detect_target_triple)"
    fi

    release_json="$(fetch_release_json)"
    release_tag="$(extract_release_tag "$release_json")"
    if [[ -z "$release_tag" || "$release_tag" == "null" ]]; then
        if [[ "$VERSION" != "latest" ]]; then
            release_tag="$VERSION"
        else
            release_tag=""
        fi
    fi

    if [[ -n "$release_tag" ]]; then
        asset_pattern="${BINARY_NAME}-${release_tag}-${TARGET_TRIPLE}"
    else
        asset_pattern="${BINARY_NAME}-${TARGET_TRIPLE}"
    fi
    asset_url="$(extract_asset_url "$release_json" "$asset_pattern")"

    if [[ -z "$asset_url" ]]; then
        echo "No matching prebuilt binary found for target ${TARGET_TRIPLE}. Falling back to source build." >&2
        build_from_source
        return
    fi

    tmp_dir="$(mktemp -d)"
    archive_path="$tmp_dir/${asset_url##*/}"

    trap 'rm -rf "$tmp_dir"' EXIT

    echo "Downloading $asset_url..." >&2
    if ! download_asset "$asset_url" "$archive_path"; then
        echo "Download failed. Attempting to build from source." >&2
        build_from_source
        return
    fi

    extract_archive "$archive_path" "$tmp_dir"
    binary_path="$(find "$tmp_dir" -type f -name "$BINARY_NAME" -perm -111 -print -quit)"

    if [[ -z "$binary_path" ]]; then
        echo "Failed to locate extracted binary. Falling back to source build." >&2
        build_from_source
        return
    fi

    install_binary "$binary_path"
    post_install_message
}

main "$@"
