#!/bin/sh
# poodle#072 — the rustfmt gate over every Poodle Rust crate.
#
# `cargo fmt` cannot be the gate. There is deliberately no root Cargo
# workspace, and stable rustfmt has no per-path exclusion (`ignore` in
# rustfmt.toml is nightly-only), so the only stable way to leave generated
# Rust alone is to enumerate the authored `.rs` files and drive rustfmt over
# them directly with `--config skip_children=true`. That flag matters:
# rustfmt recurses into child modules from every explicitly passed file
# (`#[path = "generated/…"]` mod declarations resolve and are formatted
# too), which would reformat generated Rust that must stay byte-identical to
# its generator — an identity owned by `ir:check`, `catalogue:check`,
# `audit:tokens` and `audit:icons`, not here. Nothing authored is missed by
# skipping children: the enumeration below passes every authored `.rs` file
# under each crate explicitly.
#
# Excluded, and the generator that owns each:
#   - any `generated/` directory segment:
#       packages/contracts/tokens/src/generated/    build-tokens.ts
#       packages/contracts/headless/src/generated/  poodle-codegen `machine-rust`
#       packages/gpui/preview/src/generated/        poodle-codegen `shell-rust`, `specimen-rust`, `catalogue-rust`
#       packages/jetstream/preview/src/generated/   same targets
#   - `*.generated.rs`:
#       packages/contracts/components/src/icon_geometry.generated.rs  build-default-icons.ts
#
# This is a POSIX shell script on purpose: the ubuntu `ci:rust` job ships a
# Rust toolchain, git and `sh` and no Bun (a `bun:` selector there fails with
# exit 127), and workflow edits are out of scope. That job's 1.95 toolchain
# also ships without the rustfmt component, so the gate probes `rustfmt
# --version` (the rustup shim is on PATH even when the component is missing
# and fails with exit 1), repairs through `rustup component add rustfmt` when
# rustup can, and classifies output before any verdict: only real `Diff in `
# output may produce "not rustfmt-clean" — a diff-less exit 1 is an
# environment error, which is exactly how CI's missing component first
# presented. `--self-test` needs only sh, git and rustfmt, so the gate
# carries its own proof on that job.
#
#   sh scripts/check-rustfmt.sh             # check mode (the gate)
#   sh scripts/check-rustfmt.sh --write     # format in place, before committing
#   sh scripts/check-rustfmt.sh --list      # print the per-crate file census
#   sh scripts/check-rustfmt.sh --self-test # exclusion + recursion proofs
#
# Run with `effigy check:rustfmt` (a `ci:rust` member).
set -u

# Word-split only on newlines: multi-file rustfmt invocations below expand
# unquoted lists, and this keeps them correct per path.
IFS='
'

MODE="${1:---check}"
case $MODE in
--check | --write | --list | --self-test) ;;
*)
  echo "usage: sh scripts/check-rustfmt.sh [--check|--write|--list|--self-test]" >&2
  exit 1
  ;;
esac

ROOT=$(git rev-parse --show-toplevel 2>/dev/null) || {
  echo "check-rustfmt: not inside a git checkout" >&2
  exit 1
}
cd "$ROOT" || exit 1

command -v rustfmt >/dev/null 2>&1 || {
  echo "check-rustfmt: rustfmt not found; is the Rust toolchain installed?" >&2
  exit 1
}

# The rustup shim answers `command -v rustfmt` even when the component is
# missing and then fails with exit 1 — the same status real diffs produce.
# Probe with --version, repair through rustup when it can, and refuse to
# continue on a toolchain that cannot format at all.
rustfmt_usable() {
  rustfmt --version >/dev/null 2>&1
}

ensure_rustfmt() {
  if rustfmt_usable; then
    return 0
  fi
  if command -v rustup >/dev/null 2>&1; then
    echo "check-rustfmt: rustfmt is missing from the active toolchain; running \`rustup component add rustfmt\`" >&2
    rustup component add rustfmt >&2 || true
  fi
  if rustfmt_usable; then
    return 0
  fi
  echo "check-rustfmt: rustfmt is unavailable and could not be installed; install the rustfmt component for the active toolchain" >&2
  return 1
}

# Only real formatting output may produce the unformatted verdict. A
# diff-less exit 1 (for example the rustup shim's component-missing error,
# which also exits 1) is an environment failure, not a formatting finding.
has_diff_output() {
  printf '%s\n' "$1" | grep -q 'Diff in '
}

# Generated Rust stays byte-identical to its generator and is none of this
# gate's business (see the header for the exact generator mapping). A
# `generated` prefix or suffix inside a file name is not a generated
# directory: `generated_tokens.rs` is authored.
is_generated() {
  case $1 in
  */generated/* | *.generated.rs) return 0 ;;
  *) return 1 ;;
  esac
}

# Keep the authored side of a `git ls-files` stream.
filter_generated() {
  while IFS= read -r file; do
    is_generated "$file" || printf '%s\n' "$file"
  done
}

# The rustfmt invocation for one crate. `--config skip_children=true` is the
# recursion guard described in the header; --self-test proves it end to end
# because dropping it silently puts generated files back in rustfmt's reach.
run_rustfmt() {
  edition=$1
  check=$2
  shift 2
  if [ "$check" = check ]; then
    rustfmt --edition "$edition" --config skip_children=true --check "$@"
  else
    rustfmt --edition "$edition" --config skip_children=true "$@"
  fi
}

# The exclusion boundary and the recursion guard, proven on real files: a
# scratch crate whose generated child module is mis-formatted must be
# neither flagged in check mode nor touched in write mode, while its
# authored root is both flagged and formatted.
self_test() {
  tmp=$(mktemp -d) || exit 1
  cleanup() { rm -rf "$tmp"; }
  trap cleanup EXIT

  fails=0
  expect_generated() {
    if is_generated "$1"; then
      echo "  ok   generated: $1"
    else
      echo "  FAIL not excluded: $1"
      fails=$((fails + 1))
    fi
  }
  expect_authored() {
    if is_generated "$1"; then
      echo "  FAIL excluded: $1"
      fails=$((fails + 1))
    else
      echo "  ok   authored:  $1"
    fi
  }

  # 1. Every generator output shape in this repository.
  expect_generated "packages/contracts/tokens/src/generated/primitives.rs"
  expect_generated "packages/contracts/headless/src/generated/machines/modal.rs"
  expect_generated "packages/gpui/preview/src/generated/preview-shell.rs"
  expect_generated "packages/jetstream/preview/src/generated/specimens/specimens.rs"
  expect_generated "packages/contracts/components/src/icon_geometry.generated.rs"
  # 2. Authored files survive, including generated-adjacent names.
  expect_authored "packages/render/src/action_discovery_panel.rs"
  expect_authored "packages/contracts/tokens/src/lib.rs"
  expect_authored "packages/x/src/generated_tokens.rs"
  expect_authored "packages/x/src/generated_tests/mod.rs"

  # 3. The recursion guard, end to end.
  mkdir -p "$tmp/src/generated" || exit 1
  printf 'pub mod generated;\nfn    badly_authored(  ) {}\n' >"$tmp/src/lib.rs"
  printf 'pub mod inner;\n' >"$tmp/src/generated/mod.rs"
  printf 'fn    generated_child(  ) {}\n' >"$tmp/src/generated/inner.rs"
  cp "$tmp/src/generated/inner.rs" "$tmp/inner.before"

  out=$(cd "$tmp" && run_rustfmt 2021 check src/lib.rs 2>&1)
  status=$?
  if [ "$status" -eq 1 ] &&
    printf '%s\n' "$out" | grep -q 'src/lib\.rs' &&
    ! printf '%s\n' "$out" | grep -q 'generated/inner\.rs'; then
    echo "  ok   check flags the authored file, never the generated child"
  else
    echo "  FAIL check mode on scratch crate: status=$status"
    fails=$((fails + 1))
  fi

  if run_rustfmt 2021 write "$tmp/src/lib.rs" >/dev/null 2>&1 &&
    grep -q 'fn badly_authored()' "$tmp/src/lib.rs" &&
    cmp -s "$tmp/inner.before" "$tmp/src/generated/inner.rs"; then
    echo "  ok   write formats the authored file, leaves the generated child byte-identical"
  else
    echo "  FAIL write mode on scratch crate"
    fails=$((fails + 1))
  fi

  # 4. Exit-status classification: the unformatted verdict requires real
  #    `Diff in` output. The rustup shim's component-missing error also
  #    exits 1 — CI hit exactly that (rustfmt absent from the 1.95
  #    toolchain) and it must never read as "not rustfmt-clean".
  if has_diff_output 'Diff in src/lib.rs at line 1:
-fn a(){}
+fn a() {}'; then
    echo "  ok   diff output classified as a formatting finding"
  else
    echo "  FAIL real diff output not classified as a formatting finding"
    fails=$((fails + 1))
  fi
  for sample in \
    "error: 'rustfmt' is not installed for the toolchain 'fake-1.95'" \
    "" ; do
    if has_diff_output "$sample"; then
      echo "  FAIL non-diff output classified as a formatting finding: $sample"
      fails=$((fails + 1))
    else
      echo "  ok   non-diff output kept out of the unformatted verdict"
    fi
  done

  # 5. The gate under a broken toolchain, end to end in a scratch checkout:
  #    a one-crate fake repository plus fake rustfmt/rustup binaries drive
  #    the real script through each classification branch.
  init_scratch_repo() {
    git init -q "$1"
    mkdir -p "$1/packages/scratch/src"
    printf '[package]\nname = "scratch"\nversion = "0.0.0"\nedition = "2021"\n' >"$1/packages/scratch/Cargo.toml"
    printf 'fn    badly_authored(  ) {}\n' >"$1/packages/scratch/src/lib.rs"
    git -C "$1" add -A
  }
  init_scratch_repo "$tmp/repo"
  fake_bin="$tmp/fakebin"
  mkdir -p "$fake_bin"

  # 5a. CI's exact failure: shim present, component missing.
  cat >"$fake_bin/rustfmt" <<'EOF'
#!/bin/sh
echo "error: 'rustfmt' is not installed for the toolchain 'fake-1.95'" >&2
exit 1
EOF
  cat >"$fake_bin/rustup" <<'EOF'
#!/bin/sh
echo "fake rustup cannot install components" >&2
exit 1
EOF
  chmod +x "$fake_bin/rustfmt" "$fake_bin/rustup"
  out=$(cd "$tmp/repo" && PATH="$fake_bin:$PATH" sh "$ROOT/scripts/check-rustfmt.sh" 2>&1)
  status=$?
  if [ "$status" -eq 1 ] &&
    printf '%s\n' "$out" | grep -q 'rustfmt is unavailable' &&
    ! printf '%s\n' "$out" | grep -q 'is not rustfmt-clean'; then
    echo "  ok   missing component: clear environment error, never misreported as unformatted"
  else
    echo "  FAIL missing component misclassified: status=$status"
    fails=$((fails + 1))
  fi

  # 5b. Exit 1 with real diff output: the unformatted verdict, end to end.
  cat >"$fake_bin/rustfmt" <<'EOF'
#!/bin/sh
if [ "${1:-}" = "--version" ]; then
  echo "rustfmt fake (classification fixture)"
  exit 0
fi
printf 'Diff in packages/scratch/src/lib.rs at line 1:\n-fn a(){}\n+fn a() {}\n'
exit 1
EOF
  chmod +x "$fake_bin/rustfmt"
  out=$(cd "$tmp/repo" && PATH="$fake_bin:$PATH" sh "$ROOT/scripts/check-rustfmt.sh" 2>&1)
  status=$?
  if [ "$status" -eq 1 ] &&
    printf '%s\n' "$out" | grep -q 'is not rustfmt-clean' &&
    printf '%s\n' "$out" | grep -q 'Diff in ' &&
    ! printf '%s\n' "$out" | grep -q 'rustfmt failed on'; then
    echo "  ok   diff-bearing exit 1: reported as not rustfmt-clean"
  else
    echo "  FAIL diff-bearing exit 1 misclassified: status=$status"
    fails=$((fails + 1))
  fi

  # 5c. A non-0/1 status: hard environment error, never a verdict.
  cat >"$fake_bin/rustfmt" <<'EOF'
#!/bin/sh
if [ "${1:-}" = "--version" ]; then
  echo "rustfmt fake (classification fixture)"
  exit 0
fi
echo "boom" >&2
exit 7
EOF
  chmod +x "$fake_bin/rustfmt"
  out=$(cd "$tmp/repo" && PATH="$fake_bin:$PATH" sh "$ROOT/scripts/check-rustfmt.sh" 2>&1)
  status=$?
  if [ "$status" -eq 1 ] &&
    printf '%s\n' "$out" | grep -q 'rustfmt failed on' &&
    ! printf '%s\n' "$out" | grep -q 'is not rustfmt-clean'; then
    echo "  ok   non-diff exit status: hard environment error"
  else
    echo "  FAIL non-diff exit status misclassified: status=$status"
    fails=$((fails + 1))
  fi

  if [ "$fails" -ne 0 ]; then
    echo "check-rustfmt self-test: $fails failure(s)" >&2
    exit 1
  fi
  echo "check-rustfmt self-test: exclusion boundary and recursion guard hold"
}

if [ "$MODE" = --self-test ]; then
  self_test
  exit $?
fi

ensure_rustfmt || exit 1

manifests=$(git ls-files --cached -- 'packages/*/Cargo.toml' 'packages/*/*/Cargo.toml' | LC_ALL=C sort)
if [ -z "$manifests" ]; then
  echo "check-rustfmt: no crate manifests found under packages/; refusing to pass on an empty census" >&2
  exit 1
fi

crates=0
files=0
unformatted=""
for manifest in $manifests; do
  crate_dir=${manifest%/*}
  crate_files=$(
    git ls-files --cached --others --exclude-standard -- "$crate_dir" |
      grep '\.rs$' | filter_generated
  )
  if [ -z "$crate_files" ]; then
    continue
  fi
  count=$(printf '%s\n' $crate_files | wc -l | tr -d ' ')
  crates=$((crates + 1))
  files=$((files + count))

  if [ "$MODE" = --list ]; then
    echo "$crate_dir: $count file(s)"
    printf '%s\n' $crate_files | sed 's/^/  /'
    continue
  fi

  edition=$(sed -n 's/^[[:space:]]*edition[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$manifest" | head -n 1)
  if [ -z "$edition" ]; then
    echo "check-rustfmt: no edition in $manifest; refusing to guess" >&2
    exit 1
  fi

  if [ "$MODE" = --write ]; then
    out=$(run_rustfmt "$edition" write $crate_files 2>&1) || {
      printf 'check-rustfmt: rustfmt failed on %s\n%s\n' "$crate_dir" "$out" >&2
      exit 1
    }
    echo "check-rustfmt: $crate_dir formatted ($count file(s))"
    continue
  fi

  out=$(run_rustfmt "$edition" check $crate_files 2>&1)
  status=$?
  if [ "$status" -eq 0 ]; then
    echo "check-rustfmt: $crate_dir clean ($count file(s))"
  elif [ "$status" -eq 1 ] && has_diff_output "$out"; then
    unformatted="$unformatted $crate_dir"
    printf 'check-rustfmt: %s is not rustfmt-clean:\n%s\n' "$crate_dir" "$out"
  else
    printf 'check-rustfmt: rustfmt failed on %s\n%s\n' "$crate_dir" "$out" >&2
    exit 1
  fi
done

if [ "$MODE" = --list ]; then
  echo "check-rustfmt: $crates crate(s), $files authored file(s)"
  exit 0
fi
if [ "$MODE" = --write ]; then
  echo "check-rustfmt: formatted $files file(s) across $crates crate(s)"
  exit 0
fi
if [ -n "$unformatted" ]; then
  echo "check-rustfmt:$unformatted not rustfmt-clean" >&2
  echo "run \`sh scripts/check-rustfmt.sh --write\` and commit the result" >&2
  exit 1
fi
echo "check-rustfmt: $crates crate(s), $files file(s), all rustfmt-clean"
