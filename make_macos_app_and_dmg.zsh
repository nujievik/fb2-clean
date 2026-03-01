#!/usr/bin/env zsh

set -e

if [[ -z "$1" ]]; then
  echo "Usage: $0 /path/to/binary"
  exit 1
fi

BIN_PATH="$1"
if [[ ! -f "$BIN_PATH" ]]; then
  echo "Binary not found: $BIN_PATH"
  exit 1
fi

LOGO="assets/logo.png"
if [[ ! -f "$LOGO" ]]; then
  echo "Logo not found: $LOGO"
  exit 1
fi

CARGO_TOML="Cargo.toml"
[[ ! -f "$CARGO_TOML" ]] && { echo "Cargo.toml not found"; exit 1; }

APP_VERSION=$(awk -F\" '/^version *=/ {print $2; exit}' "$CARGO_TOML")
[[ -z "$APP_VERSION" ]] && { echo "Failed to read version from Cargo.toml"; exit 1; }

APP_NAME="$(basename "$BIN_PATH")"
APP_DIR="${APP_NAME}.app"
CONTENTS="$APP_DIR/Contents"
MACOS="$CONTENTS/MacOS"
RES="$CONTENTS/Resources"
ICONSET="$RES/icon.iconset"
ICNS="$RES/icon.icns"

echo "Creating bundle: $APP_DIR"

mkdir -p "$MACOS" "$RES" "$ICONSET"

# copy binary
cp "$BIN_PATH" "$MACOS/$APP_NAME"
chmod +x "$MACOS/$APP_NAME"

# generate iconset sizes
sizes=(16 32 128 256 512)
for s in $sizes; do
  sips -z $s $s "$LOGO" --out "$ICONSET/icon_${s}x${s}.png" >/dev/null
  sips -z $((s*2)) $((s*2)) "$LOGO" --out "$ICONSET/icon_${s}x${s}@2x.png" >/dev/null
done
sips -z 1024 1024 "$LOGO" --out "$ICONSET/icon_1024x1024.png" >/dev/null

# convert to icns
iconutil -c icns "$ICONSET" -o "$ICNS"
rm -rf "$ICONSET"

# Info.plist
cat > "$CONTENTS/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
 "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>
  <string>$APP_NAME</string>
  <key>CFBundleIdentifier</key>
  <string>com.example.$APP_NAME</string>
  <key>CFBundleName</key>
  <string>$APP_NAME</string>
  <key>CFBundleVersion</key>
  <string>$APP_VERSION</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleIconFile</key>
  <string>icon</string>
</dict>
</plist>
EOF

echo "Done app: $APP_DIR"

create-dmg $APP_DIR --overwrite --no-version-in-filename --no-code-sign
