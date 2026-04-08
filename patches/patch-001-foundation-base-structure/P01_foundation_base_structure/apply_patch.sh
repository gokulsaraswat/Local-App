#!/usr/bin/env sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
TARGET_DIR="${1:-$PWD}"

node "$SCRIPT_DIR/apply_patch.mjs" "$TARGET_DIR"
