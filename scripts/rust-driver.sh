#!/bin/sh
# Rust driver: `oxide` locally, `cargo` under GitHub Actions.
#
# Every Rust build, check, test, fetch or run in this repository goes through
# here: the Effigy selectors in tasks/effigy.tasks.toml, and the Bun and sh
# scripts that spawn Cargo themselves. Pass a Cargo subcommand and its
# arguments, for example `sh scripts/rust-driver.sh test --quiet --manifest-path ...`.
# Locally it execs `oxide`, which takes the subcommand and leases the build.
# Under `GITHUB_ACTIONS=true` the CI runners do not have Oxide, so it execs
# `cargo` with the same arguments; workflow files do not change.
#
# Escapes to plain Cargo, for when `oxide` is broken on a developer machine:
#   - a Rust selector's `:cargo` sibling;
#   - `POODLE_RUST_DRIVER=cargo` in the environment, for scripts and their selectors.
# A missing `oxide` fails loudly and names both. It never falls back silently.
#
# POSIX shell on purpose, like check-rustfmt.sh: ci:rust runs on ubuntu, which
# ships `sh` and no Bun.
set -eu

if [ "${GITHUB_ACTIONS:-}" = "true" ] || [ "${POODLE_RUST_DRIVER:-}" = "cargo" ]; then
  exec cargo "$@"
fi

if ! command -v oxide >/dev/null 2>&1; then
  echo "rust-driver: oxide is not on PATH. Install Oxide, or run the ':cargo' sibling of this selector, or set POODLE_RUST_DRIVER=cargo (plain Cargo)." >&2
  exit 1
fi

exec oxide "$@"
