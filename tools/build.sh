#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only

set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

usage() {
  cat <<'USAGE'
Usage: tools/build.sh [--debug | --release | --all] [cargo args...]

Build the Phantom Camera Rust GDExtension and install the library where the
addon's .gdextension loads it from: bin/<profile>/.

  --debug    debug library, the one the editor loads (default)
  --release  release library, the one an exported game ships
  --all      both
  -h, --help show this help

Any other argument is passed to cargo build.
USAGE
}

profiles=(debug)
cargo_args=()
while (($#)); do
  case "$1" in
    --debug) profiles=(debug); shift ;;
    --release) profiles=(release); shift ;;
    --all) profiles=(debug release); shift ;;
    -h|--help) usage; exit 0 ;;
    *) cargo_args+=("$1"); shift ;;
  esac
done

case "$(uname -s)" in
  MINGW*|MSYS*|CYGWIN*) lib="phantom_camera_rust.dll" ;;
  Darwin) lib="libphantom_camera_rust.dylib" ;;
  *) lib="libphantom_camera_rust.so" ;;
esac

for profile in "${profiles[@]}"; do
  flags=()
  [[ $profile == release ]] && flags+=(--release)
  echo "==> building $profile"
  cargo build --manifest-path "$root/Cargo.toml" "${flags[@]}" "${cargo_args[@]}"
  dest="$root/bin/$profile"
  mkdir -p "$dest"
  cp "$root/target/$profile/$lib" "$dest/$lib"
  echo "    installed $dest/$lib"
done
