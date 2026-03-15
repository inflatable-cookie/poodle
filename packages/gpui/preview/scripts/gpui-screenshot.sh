#!/usr/bin/env bash
#
# gpui-screenshot.sh — Launch the GPUI preview app on a specific component,
# capture a screenshot, and exit.
#
# Usage:
#   ./scripts/gpui-screenshot.sh <slug> [output_path] [-- extra cargo run args]
#
# Examples:
#   ./scripts/gpui-screenshot.sh button /tmp/gpui-button.png
#   ./scripts/gpui-screenshot.sh button /tmp/gpui-button.png -- --theme loophole-studio --density compact
#   ./scripts/gpui-screenshot.sh data-table
#
# Defaults:
#   output_path = /tmp/gpui-<slug>.png

set -euo pipefail

SLUG="${1:?Usage: gpui-screenshot.sh <slug> [output_path] [-- extra args]}"
OUTPUT="${2:-/tmp/gpui-${SLUG}.png}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PREVIEW_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
CAPTURE_SCRIPT="$SCRIPT_DIR/capture-window.swift"

# Collect extra args after "--"
EXTRA_ARGS=()
shift 2 2>/dev/null || shift $# 2>/dev/null || true
if [[ "${1:-}" == "--" ]]; then
    shift
    EXTRA_ARGS=("$@")
fi

cd "$PREVIEW_DIR"

# Build (quietly)
cargo build --quiet 2>/dev/null

# Launch the app with the specified component and any extra args
cargo run --quiet -- --component "$SLUG" ${EXTRA_ARGS[@]+"${EXTRA_ARGS[@]}"} &
APP_PID=$!

# Wait for window to appear (poll up to 5 seconds)
for i in $(seq 1 50); do
    if swift "$CAPTURE_SCRIPT" pug-preview /dev/null 2>/dev/null; then
        break
    fi
    sleep 0.1
done

# Give the app a moment to fully render after window appears
sleep 0.5

# Capture
swift "$CAPTURE_SCRIPT" pug-preview "$OUTPUT" 2>&1

# Kill the app
kill "$APP_PID" 2>/dev/null
wait "$APP_PID" 2>/dev/null || true

echo "Screenshot: $OUTPUT"
