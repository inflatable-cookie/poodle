#!/bin/sh
# The contracts, shared renderer and specimens must stay usable without a backend,
# including in tests and on platform-specific dependency paths.
set -eu

# The contracts glob includes poodle-conformance and its own test graph.
for manifest in packages/contracts/*/Cargo.toml packages/render/Cargo.toml packages/specimens/Cargo.toml; do
  tree=$(sh scripts/rust-driver.sh tree --manifest-path "$manifest" \
    --all-features --target all --edges normal,build,dev --prefix none)
  if matches=$(printf '%s\n' "$tree" |
    grep -E '^(poodle-)?(gpui|jetstream)([[:space:]-]|$)|/packages/(gpui|jetstream)/'); then
    printf '%s depends on a native backend:\n%s\n' "$manifest" "$matches" >&2
    exit 1
  fi
  printf '%s has no GPUI or Jetstream dependencies\n' "$manifest"
done
