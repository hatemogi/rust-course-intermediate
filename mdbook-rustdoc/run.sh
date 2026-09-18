#!/bin/sh
set -eu
TOOL_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
exec cargo run --quiet --locked --manifest-path "$TOOL_DIR/Cargo.toml" -- "$@"
