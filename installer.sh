OS=$(uname -s)
ARCH=$(uname -m)


if [ "$OS" = "Linux" ]; then
    TARGET="https://github.com/lade-package-manager/installer/releases/download/0.2/lade-installer"
    FILE="lade-installer-linux"
elif [ "$OS" = "Darwin" ]; then
    TARGET="https://github.com/lade-package-manager/installer/releases/download/0.2/lade-installer-macos"
    FILE="lade-installer-macos"
elif [ "$OS" = "CYGWIN" ] || [ "$OS" = "MINGW" ] || [ "$OS" = "MSYS" ]; then
    TARGET="https://github.com/lade-package-manager/installer/releases/download/0.2/lade-installer.exe"
    FILE="lade-installer.exe"
else
    echo "Unsupported OS"
    exit 1
fi


printf "\x1b[32;1m>>>\x1b[0m\x1b[1m Downloading installer\x1b[0m\n"
curl -fsSL "$TARGET" -o $FILE

chmod +x ./$FILE
./$FILE

rm ./$FILE
