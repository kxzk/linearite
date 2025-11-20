#!/bin/sh

set -e

VERSION="v0.1.1"
REPO="kxzk/linearite"
BINARY_NAME="linearite"

get_arch() {
    arch=$(uname -m)
    case $arch in
        x86_64) echo "x86_64" ;;
        aarch64|arm64) echo "aarch64" ;;
        *) echo "Unsupported architecture: $arch" >&2; exit 1 ;;
    esac
}

get_os() {
    os=$(uname -s)
    case $os in
        Darwin) echo "apple-darwin" ;;
        Linux) echo "unknown-linux-gnu" ;;
        MINGW*|MSYS*|CYGWIN*) echo "pc-windows-msvc" ;;
        *) echo "Unsupported OS: $os" >&2; exit 1 ;;
    esac
}

main() {
    arch=$(get_arch)
    os=$(get_os)

    target="${arch}-${os}"

    if [ "$os" = "pc-windows-msvc" ]; then
        filename="${BINARY_NAME}-${target}.exe"
    else
        filename="${BINARY_NAME}-${target}"
    fi

    url="https://github.com/${REPO}/releases/download/${VERSION}/${filename}"

    echo "🦀 Installing Linearite ${VERSION} for ${target}..."

    tmpdir=$(mktemp -d)
    trap 'rm -rf "$tmpdir"' EXIT

    cd "$tmpdir"

    if command -v curl > /dev/null 2>&1; then
        curl -fsSL "$url" -o "$BINARY_NAME"
    elif command -v wget > /dev/null 2>&1; then
        wget -q "$url" -O "$BINARY_NAME"
    else
        echo "Error: curl or wget required" >&2
        exit 1
    fi

    binary_file="$BINARY_NAME"

    install_dir="/usr/local/bin"
    if [ -w "$install_dir" ]; then
        mv "$binary_file" "$install_dir/$BINARY_NAME"
        echo "🦀 Installed to ${install_dir}/${BINARY_NAME}"
    else
        echo "🦀 Installing to ${install_dir} (requires sudo)..."
        sudo mv "$binary_file" "$install_dir/$BINARY_NAME"
    fi

    chmod +x "${install_dir}/${BINARY_NAME}"

    if command -v "$BINARY_NAME" > /dev/null 2>&1; then
        echo "🦀 Success! Run '${BINARY_NAME} help' to get started."
    else
        echo "🦀 Installed but ${install_dir} may not be in PATH"
        echo "Add to PATH or move binary to a directory in PATH"
    fi
}

main
