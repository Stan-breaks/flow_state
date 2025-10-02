#!/usr/bin/env bash
set -e

# Linux (native)
cargo build --release

# Windows
cross build --release --target x86_64-pc-windows-gnu

# macOS (via cross)
cross build --release --target x86_64-apple-darwin
cross build --release --target aarch64-apple-darwin
