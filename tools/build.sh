#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only

set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

profiles=(debug)
cargo_args=()
while (($#)); do
  case "$1" in
    --debug) profiles=(debug); shift ;;
    --release) profiles=(release); shift ;;
    --all) profiles=(debug release); shift ;;
    -h|--help)
      sed -n '2,18p' "$0" | sed 's/^# \{0,1\}//'
      exit 0
      ;;
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
