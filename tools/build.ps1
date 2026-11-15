# SPDX-License-Identifier: GPL-3.0-only
#
# Build the Phantom Camera Rust GDExtension on Windows and install the DLL where
# the addon's .gdextension expects it:
#
#   bin/debug/phantom_camera_rust.dll
#   bin/release/phantom_camera_rust.dll
#
# Usage: tools/build.ps1 [-Profile debug|release|all]
param(
    [ValidateSet('debug', 'release', 'all')]
    [string]$Profile = 'debug'
)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$profiles = if ($Profile -eq 'all') { @('debug', 'release') } else { @($Profile) }
foreach ($p in $profiles) {
    $flags = if ($p -eq 'release') { @('--release') } else { @() }
    Write-Host "==> building $p"
    cargo build --manifest-path (Join-Path $root 'Cargo.toml') @flags
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    $dest = Join-Path $root "bin/$p"
    New-Item -ItemType Directory -Force -Path $dest | Out-Null
    Copy-Item (Join-Path $root "target/$p/phantom_camera_rust.dll") $dest -Force
    Write-Host "    installed $dest/phantom_camera_rust.dll"
}
