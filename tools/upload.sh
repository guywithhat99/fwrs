#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$REPO_ROOT"

cargo build

OBJ_COPY="$REPO_ROOT/tools/compiler/arm-gnu-toolchain/bin/arm-none-eabi-objcopy"
INPUT="$REPO_ROOT/target/thumbv7em-none-eabihf/debug/fwrs"
OUTPUT="$REPO_ROOT/fwrs.hex"

"$OBJ_COPY" -O ihex "$INPUT" "$OUTPUT"

tycmd upload "$OUTPUT"
