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
# A tool that runs Cargo itself, or that needs its own target directory, goes
# through the same driver as `exec`, which is Oxide's `oxide exec`:
#   sh scripts/rust-driver.sh exec --manifest-path <Cargo.toml> -- <tool> <args>
# Oxide leases the slot and supplies the intermediates; the tool keeps its own
# `CARGO_TARGET_DIR` for its final outputs, so its result is unchanged.
#
# One escape to plain Cargo, for when `oxide` is broken on a developer machine:
#   EFFIGY_OXIDE_ESCAPE=cargo sh scripts/rust-driver.sh <args>
# (or the whole selector with `EFFIGY_OXIDE_ESCAPE=cargo effigy <selector>`).
# It is the same escape Effigy's own selectors use, and the only one: there are
# no `:cargo` sibling selectors. On an `exec` route it runs the tool itself,
# which has no Oxide verb to escape from, so the tool's own Cargo stays plain.
# A missing `oxide` fails loudly and names the escape. It never falls back
# silently.
#
# POSIX shell on purpose, like check-rustfmt.sh: ci:rust runs on ubuntu, which
# ships `sh` and no Bun.
set -eu

tool=${EFFIGY_OXIDE_ESCAPE:-oxide}
case $tool in
  oxide | cargo) ;;
  *)
    echo "rust-driver: EFFIGY_OXIDE_ESCAPE must be oxide or cargo, not '$tool'" >&2
    exit 2
    ;;
esac

if [ "${GITHUB_ACTIONS:-}" = "true" ]; then
  tool=cargo
fi

if [ "$tool" = cargo ]; then
  # Plain Cargo has no `exec` verb, so an `exec` route drops Oxide's own options
  # and runs the tool itself; whatever the tool spawns is plain Cargo already.
  if [ "${1:-}" = exec ]; then
    shift
    while [ "$#" -gt 0 ] && [ "$1" != "--" ]; do
      shift
    done
    if [ "$#" -gt 0 ]; then
      shift
    fi
    if [ "$#" -eq 0 ]; then
      echo "rust-driver: exec needs a tool after --" >&2
      exit 2
    fi
    exec "$@"
  fi
  exec cargo "$@"
fi

if ! command -v oxide >/dev/null 2>&1; then
  echo "rust-driver: oxide is not on PATH. Install Oxide, or set EFFIGY_OXIDE_ESCAPE=cargo for the plain-Cargo escape." >&2
  exit 1
fi

# Oxide resolves its default pool from $HOME, so a harness that runs a selector
# with a temporary HOME would create a cold pool under that HOME on the
# internal SSD (3.3-3.8 GB per run, measured 2026-10-24). Resolve the pool the
# way a normal run does - `oxide status` under the account's real home - and
# never hard-code a path. Oxide shares one pool between concurrent runs, so
# passing the real pool through does not weaken HOME isolation. The same block
# covers the `exec` route, which reaches Oxide through this script too.
if [ -z "${OXIDE_POOL:-}" ]; then
  real_home=$( (eval echo "~$(id -un)") 2>/dev/null ) || real_home=
  case $real_home in
    /?*) ;;
    *) real_home= ;;
  esac
  if [ -z "$real_home" ]; then
    echo "rust-driver: could not find the account's real home; set OXIDE_POOL to the shared Oxide pool so this run does not create a cold pool under ${HOME:-an unset HOME}" >&2
  elif [ "${HOME:-}" != "$real_home" ]; then
    real_pool=$(HOME="$real_home" oxide status 2>/dev/null |
      awk '/^pool:/ { sub(/^pool:[ \t]*/, ""); sub(/[ \t]*\(.*$/, ""); print; exit }') || real_pool=
    case ${real_pool:-} in
      /?*) export OXIDE_POOL="$real_pool" ;;
      *)
        echo "rust-driver: no pool reported by 'oxide status' under $real_home; set OXIDE_POOL to the shared Oxide pool so this private-HOME run (${HOME:-unset HOME}) does not create a cold pool on the internal SSD" >&2
        ;;
    esac
  fi
fi

exec oxide "$@"
