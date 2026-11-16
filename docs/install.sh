#!/bin/bash
set -e

# ==============================================================================
# aroe — Universal Installer for macOS and Linux
# Installs the standalone native binary to ~/.local/bin/aroe
# ==============================================================================

BOLD="\033[1m"
GOLD="\033[38;5;220m"
DIM="\033[38;5;244m"
RESET="\033[0m"

echo -e "${GOLD}"
echo "    ▄▀▀▀▄ █▀▀▀▄ ▄▀▀▀▄ █▀▀▀▀"
echo "    █   █ █▄▄▄▀ █   █ █▄▄▄ "
echo "    █▀▀▀█ █  ▀▄ █   █ █    "
echo "    █   █ █   █ ▀▄▄▄▀ █▄▄▄▄"
echo -e "${RESET}"
echo -e "${BOLD}the anime recommender that lives in your terminal.${RESET}"
echo -e "${DIM}https://github.com/he3vn/Aroe${RESET}"
echo ""

# 1. Detect OS & Architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Darwin)
    TARGET="macos"
    BINARY_NAME="aroe-macos"
    ;;
  Linux)
    TARGET="linux"
    BINARY_NAME="aroe-linux"
    ;;
  *)
    echo "❌ Unsupported operating system: $OS"
    echo "For Windows, download 'aroe-windows.exe' directly from:"
    echo "https://github.com/he3vn/Aroe/releases/latest"
    exit 1
    ;;
esac

echo "→ Detected OS: $OS ($ARCH)"

# 2. Determine installation directories
INSTALL_DIR="$HOME/.local/bin"
DATA_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/aroe"

mkdir -p "$INSTALL_DIR"
mkdir -p "$DATA_DIR"

# 3. Download the executable binary from latest GitHub release
DOWNLOAD_URL="https://github.com/he3vn/Aroe/releases/latest/download/${BINARY_NAME}"
echo "→ Downloading latest aroe binary for $TARGET..."

if command -v curl >/dev/null 2>&1; then
  curl -sSL -f "$DOWNLOAD_URL" -o "$INSTALL_DIR/aroe"
elif command -v wget >/dev/null 2>&1; then
  wget -q "$DOWNLOAD_URL" -O "$INSTALL_DIR/aroe"
else
  echo "❌ Neither curl nor wget was found. Please install either to continue."
  exit 1
fi

chmod +x "$INSTALL_DIR/aroe"

# 4. Download default anime catalog data so it works 100% offline
CATALOG_URL="https://raw.githubusercontent.com/he3vn/Aroe/main/data.json"
if [ ! -f "$DATA_DIR/data.json" ]; then
  echo "→ Downloading offline anime catalog database (~60MB)..."
  if command -v curl >/dev/null 2>&1; then
    curl -sSL -f "$CATALOG_URL" -o "$DATA_DIR/data.json" || true
  elif command -v wget >/dev/null 2>&1; then
    wget -q "$CATALOG_URL" -O "$DATA_DIR/data.json" || true
  fi
fi

# 5. Check PATH
echo ""
echo -e "${GOLD}⚡ aroe was successfully installed to: $INSTALL_DIR/aroe${RESET}"

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
  SHELL_NAME="$(basename "$SHELL")"
  case "$SHELL_NAME" in
    zsh)  PROFILE_FILE="$HOME/.zshrc" ;;
    bash) PROFILE_FILE="$HOME/.bashrc" ;;
    fish) PROFILE_FILE="$HOME/.config/fish/config.fish" ;;
    *)    PROFILE_FILE="$HOME/.profile" ;;
  esac

  echo ""
  echo -e "${DIM}Notice: $INSTALL_DIR is not yet in your PATH.${RESET}"
  echo -e "Add it by running:"
  echo -e "  ${BOLD}export PATH=\"\$HOME/.local/bin:\$PATH\"${RESET}"
  echo -e "or add that line to your ${BOLD}$PROFILE_FILE${RESET}"
  echo ""
  echo "To run aroe right now:"
  echo -e "  ${GOLD}$INSTALL_DIR/aroe${RESET}"
else
  echo ""
  echo "You can now run it from any terminal window:"
  echo -e "  ${GOLD}${BOLD}aroe${RESET}"
fi
echo ""
