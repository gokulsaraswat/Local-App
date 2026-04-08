#!/usr/bin/env bash
set -euo pipefail

TARGET_DIR="${1:-.}"
node "$(cd "$(dirname "$0")" && pwd)/apply_patch.mjs" "$TARGET_DIR" --install
