#!/usr/bin/env bash
#
# g15.044 — GPUI offscreen capture feasibility: complete reproduction + verifier.
#
# Retained instead of a proof fixture. This file is the whole recipe: harness
# source, both manifests, the deterministic migration patch, and every command.
# It is dependency-isolated by construction — it lives under docs/ log assets,
# nothing imports it, it declares no package, and it writes only into a
# throwaway directory. It never modifies the Poodle repository it reads.
#
# It is also the verifier for the documented claims. Every durable structural
# claim in the research note and batch log is asserted here against the EXPECT
# block below. Drift in any one of them fails the run. Timings are observational
# and are recorded but not asserted, because they are machine-dependent.
#
# Usage:
#   bash reproduce.sh <path-to-poodle-repo-root> [workdir]
#
# Default workdir is `mktemp -d`. Needs macOS with a Metal device, network
# access for the Cargo git fetch, and rustc 1.97.1 (upstream's pinned channel).
#
# Emits a receipt to $WORK/receipt.txt. Exits non-zero if any claim is false.

set -uo pipefail

REPO="${1:?usage: reproduce.sh <path-to-poodle-repo-root> [workdir]}"
REPO="$(cd "$REPO" && pwd)"
WORK="${2:-$(mktemp -d)}"
mkdir -p "$WORK"
WORK="$(cd "$WORK" && pwd)"

ZED_URL="https://github.com/zed-industries/zed"
ZED_REV="1ea16c1ab9dd6d36649e002dc60995634da04daf"

# ---------------------------------------------------------------------------
# EXPECT — the documented claims. Every one of these is asserted below.
# ---------------------------------------------------------------------------
EXPECT_NB_ERRORS=8            # poodle-gpui-node-backend, before its patch
EXPECT_PV_ERRORS=6            # poodle-gpui-preview bin, before its patch
EXPECT_TS_ERRORS=3            # preview headless tests, before their patch
EXPECT_TOTAL_ERRORS=17
EXPECT_LOCK_PACKAGES=702
EXPECT_ZED_PACKAGES=23
EXPECT_REGRESSIONS=56         # migrated headless_regressions, all passing.
                              # Tracks the base commit: sibling lanes that add a
                              # native regression move it, and this assertion is
                              # what forces the docs to be updated on rebase.
EXPECT_EQUAL_INPUT=10         # equal-input captures
EXPECT_WIDTH=480              # 240 logical x 2.0 hardcoded scale
EXPECT_HEIGHT=160             # 80 logical x 2.0
EXPECT_SHA="be94eaceb6c310c4e067c012b579c53d2c6d4147fc63160673316538c9997c6d"
COMMITTED_PNG="$REPO/docs/logs/2026-08/assets/g15-044/button-offscreen.png"

RECEIPT="$WORK/receipt.txt"
: > "$RECEIPT"
say()  { echo "$*" | tee -a "$RECEIPT"; }
FAILURES=0

# check <label> <actual> <expected> — records drift, never aborts early, so one
# run reports every mismatch instead of only the first.
check() {
  if [ "$2" = "$3" ]; then
    say "  PASS  $1: $2"
  else
    say "  FAIL  $1: got '$2', documented '$3'"
    FAILURES=$((FAILURES + 1))
  fi
}
check_ne() {
  if [ "$2" != "$3" ]; then
    say "  PASS  $1"
  else
    say "  FAIL  $1: values are equal ('$2') but must differ"
    FAILURES=$((FAILURES + 1))
  fi
}
die() { say "FATAL: $*"; say "receipt: $RECEIPT"; exit 1; }

say "# g15.044 reproduction receipt"
say "repo:          $REPO"
say "repo commit:   $(git -C "$REPO" rev-parse HEAD 2>/dev/null || echo 'not a git checkout')"
say "repo describe: $(git -C "$REPO" log -1 --format='%h %s' 2>/dev/null || echo n/a)"
say "repo dirty:    $([ -n "$(git -C "$REPO" status --porcelain 2>/dev/null)" ] && echo yes || echo no)"
say "workdir:       $WORK"
say "upstream:      $ZED_URL @ $ZED_REV"
say "rustc:         $(rustc --version)"
say "uname:         $(uname -srm)"
say "date:          $(date -u '+%Y-%m-%dT%H:%M:%SZ')"
say ""

# ---------------------------------------------------------------------------
# 1. Vendor disposable copies of the real Poodle crates.
#    packages/tokens/artifacts is required: poodle-tokens includes generated
#    code from outside its own crate directory (see PAPERCUTS.md).
# ---------------------------------------------------------------------------
say "## 1. vendor"
mkdir -p "$WORK/vendor/packages/gpui" "$WORK/vendor/packages/tokens"
cp -R "$REPO/packages/contracts"         "$WORK/vendor/packages/contracts"
cp -R "$REPO/packages/render"            "$WORK/vendor/packages/render"
cp -R "$REPO/packages/tokens/artifacts"  "$WORK/vendor/packages/tokens/artifacts"
cp -R "$REPO/packages/gpui/adapter"      "$WORK/vendor/packages/gpui/adapter"
cp -R "$REPO/packages/gpui/node-backend" "$WORK/vendor/packages/gpui/node-backend"
cp -R "$REPO/packages/gpui/preview"      "$WORK/vendor/packages/gpui/preview"
find "$WORK/vendor" -name target -type d -prune -exec rm -rf {} + 2>/dev/null || true
rm -f "$WORK/vendor/packages/gpui/preview/Cargo.lock"

# poodle-render's only dev-dependency is poodle-jetstream, which is out of the
# proof's scope. Dropping it keeps the vendored graph resolvable.
python3 - "$WORK/vendor/packages/render/Cargo.toml" <<'PY' || die "vendor rewrite failed"
import sys
p = sys.argv[1]; s = open(p).read()
old = 'poodle-jetstream = { version = "0.1.0", path = "../jetstream/adapter" }\n'
assert old in s, "poodle-render dev-dependency text has moved"
open(p, 'w').write(s.replace(old, ''))
PY
say "vendored $(find "$WORK/vendor" -name Cargo.toml | wc -l | tr -d ' ') manifests"
say ""

# ---------------------------------------------------------------------------
# 2. Workspace manifest.
# ---------------------------------------------------------------------------
cat > "$WORK/Cargo.toml" <<'EOF'
[workspace]
resolver = "2"
members = [
  "harness",
  "vendor/packages/contracts/adapter",
  "vendor/packages/contracts/components",
  "vendor/packages/contracts/events",
  "vendor/packages/contracts/headless",
  "vendor/packages/contracts/ir",
  "vendor/packages/contracts/layout",
  "vendor/packages/contracts/markdown",
  "vendor/packages/contracts/node",
  "vendor/packages/contracts/style",
  "vendor/packages/contracts/tokens",
  "vendor/packages/render",
  "vendor/packages/gpui/adapter",
  "vendor/packages/gpui/node-backend",
  "vendor/packages/gpui/preview",
]
EOF

# ---------------------------------------------------------------------------
# 3. Harness manifest and source, in full.
# ---------------------------------------------------------------------------
mkdir -p "$WORK/harness/src"
cat > "$WORK/harness/Cargo.toml" <<EOF
[package]
name = "offscreen-proof"
version = "0.0.0"
edition = "2021"
publish = false

[dependencies]
poodle-render = { path = "../vendor/packages/render" }
poodle-specs = { path = "../vendor/packages/contracts/components" }
poodle-gpui = { path = "../vendor/packages/gpui/adapter" }
poodle-gpui-node-backend = { path = "../vendor/packages/gpui/node-backend" }
anyhow = "1"
image = "0.25"
gpui = { git = "$ZED_URL", rev = "$ZED_REV", features = ["test-support"] }
gpui_platform = { git = "$ZED_URL", rev = "$ZED_REV", features = ["test-support", "font-kit"] }
EOF

cat > "$WORK/harness/src/main.rs" <<'EOF'
//! g15.044 offscreen capture proof.
//!
//! Renders a real Poodle Button (poodle-render -> poodle-node ->
//! poodle-gpui-node-backend) to an RGBA PNG through upstream GPUI's
//! HeadlessAppContext + MetalHeadlessRenderer. No NSWindow is created, no
//! desktop capture is invoked, no focus is taken, no subprocess is spawned.
//!
//! Args: [output_path] [repeat_count].  Env: PROOF_W, PROOF_H (logical size).

use std::sync::Arc;

use anyhow::Result;
use gpui::{
    AnyElement, App, Context, HeadlessAppContext, IntoElement, ParentElement, Render, Styled,
    Window, div, px, size,
};
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ButtonSpec, ButtonVariant};

struct ProofRoot;

impl Render for ProofRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = GpuiThemeProvider::new();
        let spec = ButtonSpec::new()
            .with_label("Save")
            .with_variant(ButtonVariant::Primary);
        let node = poodle_render::button(&spec, &theme, None);
        poodle_gpui_node_backend::reset_element_ids();
        let element: AnyElement = poodle_gpui_node_backend::to_gpui(&node);
        div()
            .size_full()
            .p(px(16.0))
            .bg(gpui::rgb(0xffffff))
            .child(element)
    }
}

fn main() -> Result<()> {
    let out = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/tmp/poodle-button-offscreen.png".to_string());
    let repeats: usize = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    let t0 = std::time::Instant::now();
    let platform = gpui_platform::current_platform(true);
    let text_system = platform.text_system();
    let mut cx = HeadlessAppContext::with_platform(text_system, Arc::new(()), || {
        gpui_platform::current_headless_renderer()
    });
    eprintln!("context ready in {:?}", t0.elapsed());

    let w: f32 = std::env::var("PROOF_W")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(240.0);
    let h: f32 = std::env::var("PROOF_H")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(80.0);

    let window = cx.open_window(size(px(w), px(h)), |_, cx: &mut App| {
        <App as gpui::AppContext>::new(cx, |_| ProofRoot)
    })?;
    cx.run_until_parked();

    for i in 0..repeats {
        let t = std::time::Instant::now();
        let image = cx.capture_screenshot(window.into())?;
        let path = if repeats == 1 {
            out.clone()
        } else {
            format!("{}.{}.png", out.trim_end_matches(".png"), i)
        };
        image.save(&path)?;
        eprintln!(
            "capture {} -> {} ({}x{}) in {:?}",
            i,
            path,
            image.width(),
            image.height(),
            t.elapsed()
        );
    }
    Ok(())
}
EOF

# ---------------------------------------------------------------------------
# 4. Baseline: the vendored copy must build against the production pin (0.2.2)
#    before anything is repointed. This proves the copy is faithful.
# ---------------------------------------------------------------------------
say "## 4. baseline at the production pin (gpui 0.2.2)"
cd "$WORK" || die "cannot enter workdir"
if cargo build -p poodle-gpui-node-backend >/dev/null 2>&1; then
  say "  PASS  poodle-gpui-node-backend @ gpui 0.2.2 builds"
else
  die "poodle-gpui-node-backend does not build at gpui 0.2.2 — vendored copy is not faithful"
fi
say ""

# ---------------------------------------------------------------------------
# 5. Repoint the vendored crates at the immutable upstream revision.
#    Only the `gpui` dependency lines change here; no code is touched yet, so
#    the error counts below are the honest unmigrated cost.
# ---------------------------------------------------------------------------
say "## 5. repoint at $ZED_REV"
python3 - "$WORK" "$ZED_URL" "$ZED_REV" <<'PY' || die "repoint failed — dependency text has moved"
import sys, pathlib
work, url, rev = sys.argv[1], sys.argv[2], sys.argv[3]
git = 'git = "%s", rev = "%s"' % (url, rev)

p = pathlib.Path(work, 'vendor/packages/gpui/node-backend/Cargo.toml')
s = p.read_text()
assert 'gpui = "0.2.2"' in s, 'node-backend gpui pin text has moved'
p.write_text(s.replace('gpui = "0.2.2"', 'gpui = { %s }' % git))

p = pathlib.Path(work, 'vendor/packages/gpui/preview/Cargo.toml')
s = p.read_text()
assert 'gpui = "0.2.2"' in s, 'preview gpui pin text has moved'
dev = 'gpui = { version = "0.2.2", features = ["test-support"] }'
assert dev in s, 'preview gpui dev-dependency text has moved'
s = s.replace('gpui = "0.2.2"', 'gpui = { %s }' % git)
s = s.replace(dev, 'gpui = { %s, features = ["test-support"] }' % git)
prof = '[profile.dev]\nopt-level = 3\n'
assert prof in s, 'preview profile.dev block has moved'
s = s.replace(prof, '')   # profile belongs to the workspace root, not a member
p.write_text(s)
PY
say "  PASS  dependency repointing applied"
say ""

# errors_for <cargo args...> — prints the error lines for vendored sources.
errors_for() { "$@" --message-format short 2>&1 | grep -E '^vendor/.*error\[' || true; }
count_lines() { [ -z "$1" ] && echo 0 || printf '%s\n' "$1" | wc -l | tr -d ' '; }

# ---------------------------------------------------------------------------
# 6. Migration cost, surface by surface. Each surface is measured BEFORE its
#    own patch is applied, and patched before the next is measured (they
#    depend on each other).
# ---------------------------------------------------------------------------
say "## 6. migration cost"

say "### poodle-gpui (adapter)"
if grep -q '^gpui' "$WORK/vendor/packages/gpui/adapter/Cargo.toml"; then
  say "  FAIL  adapter declares a gpui dependency; documented as having none"
  FAILURES=$((FAILURES + 1))
else
  say "  PASS  adapter has no gpui dependency: 0 errors"
fi
say ""

say "### poodle-gpui-node-backend"
NB_ERRORS="$(errors_for cargo build -p poodle-gpui-node-backend)"
NB_N="$(count_lines "$NB_ERRORS")"
[ -n "$NB_ERRORS" ] && say "$NB_ERRORS"
check "node-backend error count" "$NB_N" "$EXPECT_NB_ERRORS"

python3 - "$WORK" <<'PY' || die "node-backend migration patch did not apply cleanly"
import sys, pathlib
nb = pathlib.Path(sys.argv[1], 'vendor/packages/gpui/node-backend/src')

def sub(path, old, new, least=1):
    p = nb/path; s = p.read_text()
    n = s.count(old)
    assert n >= least, '%s: expected >=%d occurrence(s) of %r, found %d' % (path, least, old[:60], n)
    p.write_text(s.replace(old, new))
    return n

# FocusHandle::focus(window) -> focus(window, cx)
sub('interaction.rs', 'handle.focus(window);', 'handle.focus(window, cx);', least=2)
# Styled::flex_grow() now takes an f32
sub('style.rs', 'el = el.flex_grow(),', 'el = el.flex_grow_1(),')
sub('style.rs', 'el = el.flex_grow();', 'el = el.flex_grow_1();')
# BoxShadow gains a required `inset` field
sub('style.rs', '                spread_radius: px(l.spread),\n',
                '                spread_radius: px(l.spread),\n                inset: false,\n')
sub('style.rs', '            spread_radius: px(0.0),\n',
                '            spread_radius: px(0.0),\n            inset: false,\n')
# ScrollHandle::max_offset() returns Point<Pixels>, not Size<Pixels>
sub('tracked_scroll.rs', 'self.handle.max_offset().height.into()',
                         'self.handle.max_offset().y.into()')
# Line::paint gains align + align_width
sub('input_text.rs',
    'line.paint(point(origin_x, text_top), window.line_height(), window, cx)',
    'line.paint(point(origin_x, text_top), window.line_height(), '
    'gpui::TextAlign::Left, None, window, cx)')
PY
if cargo build -p poodle-gpui-node-backend >/dev/null 2>&1; then
  say "  PASS  node-backend builds after patch"
else
  die "node-backend still fails after the documented patch"
fi
say ""

say "### poodle-gpui-preview (bin)"
PV_ERRORS="$(errors_for cargo build -p poodle-gpui-preview)"
PV_N="$(count_lines "$PV_ERRORS")"
[ -n "$PV_ERRORS" ] && say "$PV_ERRORS"
check "preview error count" "$PV_N" "$EXPECT_PV_ERRORS"

python3 - "$WORK" "$ZED_URL" "$ZED_REV" <<'PY' || die "preview migration patch did not apply cleanly"
import sys, pathlib, re
work, url, rev = sys.argv[1], sys.argv[2], sys.argv[3]
base = pathlib.Path(work, 'vendor/packages/gpui/preview')

total = 0
for rel in ['src/specimens/embed_input_specimen.rs', 'src/specimens/region.rs']:
    p = base/rel; s = p.read_text()
    n = s.count('.flex_grow()')
    assert n >= 1, '%s: no .flex_grow() call sites found' % rel
    total += n
    p.write_text(s.replace('.flex_grow()', '.flex_grow_1()'))
assert total == 3, 'expected 3 preview flex_grow sites, found %d' % total

p = base/'src/specimens/mod.rs'; s = p.read_text()
n = len(re.findall(r' *spread_radius: px\([^)]*\),\n', s))
assert n == 2, 'expected 2 preview BoxShadow sites, found %d' % n
p.write_text(re.sub(r'( *)(spread_radius: px\([^)]*\),\n)',
                    lambda m: m.group(0) + m.group(1) + 'inset: false,\n', s))

# Application::new() is gone; the platform crate owns construction now.
p = base/'src/main.rs'; s = p.read_text()
old = 'Application::new().with_assets(assets)'
assert s.count(old) == 1, 'expected exactly 1 Application::new() site'
p.write_text(s.replace(old, 'gpui_platform::application().with_assets(assets)'))

# ...which makes gpui_platform a new direct production dependency.
p = base/'Cargo.toml'; s = p.read_text()
anchor = 'poodle-gpui = { path = "../adapter" }'
assert anchor in s, 'preview manifest anchor has moved'
p.write_text(s.replace(anchor,
    'gpui_platform = { git = "%s", rev = "%s" }\n%s' % (url, rev, anchor)))
PY
if cargo build -p poodle-gpui-preview >/dev/null 2>&1; then
  say "  PASS  preview builds after patch"
else
  die "preview still fails after the documented patch"
fi
say ""

say "### poodle-gpui-preview headless tests"
TS_ERRORS="$(errors_for cargo test -p poodle-gpui-preview --no-run)"
TS_N="$(count_lines "$TS_ERRORS")"
[ -n "$TS_ERRORS" ] && say "$TS_ERRORS"
check "headless-test error count" "$TS_N" "$EXPECT_TS_ERRORS"

python3 - "$WORK" <<'PY' || die "headless-driver migration patch did not apply cleanly"
import sys, pathlib
p = pathlib.Path(sys.argv[1], 'vendor/packages/gpui/preview/src/headless_driver.rs')
s = p.read_text()

def sub(old, new):
    global s
    assert old in s, 'headless_driver.rs: text has moved: %r' % old[:70]
    s = s.replace(old, new)

sub('self.cx.update(|window, _cx| {\n'
    '            if let Some(handle) = poodle_gpui_node_backend::focus_handle_for(element_id) {\n'
    '                handle.focus(window);\n            }\n        });',
    'self.cx.update(|window, cx| {\n'
    '            if let Some(handle) = poodle_gpui_node_backend::focus_handle_for(element_id) {\n'
    '                handle.focus(window, cx);\n            }\n        });')
sub('self.cx.update(|window, _cx| {\n'
    '            let handle = self.root_focus.clone();\n'
    '            handle.focus(window);\n        });',
    'self.cx.update(|window, cx| {\n'
    '            let handle = self.root_focus.clone();\n'
    '            handle.focus(window, cx);\n        });')
# KeyDownEvent gains prefer_character_input
sub('keystroke: keystroke.clone(),\n            is_held: false,\n        });',
    'keystroke: keystroke.clone(),\n            is_held: false,\n'
    '            prefer_character_input: false,\n        });')
p.write_text(s)
PY
if cargo test -p poodle-gpui-preview --no-run >/dev/null 2>&1; then
  say "  PASS  headless tests build after patch"
else
  die "headless tests still fail after the documented patch"
fi
say ""

TOTAL_N=$((NB_N + PV_N + TS_N))
check "total migration errors" "$TOTAL_N" "$EXPECT_TOTAL_ERRORS"
say ""

say "### lock delta"
LOCK_N="$(grep -c '^\[\[package\]\]' "$WORK/Cargo.lock")"
ZED_N="$(grep -c "source = \"git+$ZED_URL" "$WORK/Cargo.lock")"
check "packages in lock" "$LOCK_N" "$EXPECT_LOCK_PACKAGES"
check "packages from the zed git source" "$ZED_N" "$EXPECT_ZED_PACKAGES"
say ""

# ---------------------------------------------------------------------------
# 7. Behaviour: the retained headless regression suite on the migrated copy.
# ---------------------------------------------------------------------------
say "## 7. headless regressions on the migrated copy"
REG_LINE="$(cargo test -p poodle-gpui-preview --test headless_regressions 2>&1 \
            | grep 'test result:' | head -1)"
say "$REG_LINE"
REG_PASSED="$(printf '%s\n' "$REG_LINE" | sed -n 's/.*ok\. \([0-9][0-9]*\) passed.*/\1/p')"
REG_FAILED="$(printf '%s\n' "$REG_LINE" | sed -n 's/.*; \([0-9][0-9]*\) failed.*/\1/p')"
check "regressions passed" "${REG_PASSED:-none}" "$EXPECT_REGRESSIONS"
check "regressions failed"  "${REG_FAILED:-none}"  "0"
say ""

# ---------------------------------------------------------------------------
# 8. The capture proof.
# ---------------------------------------------------------------------------
say "## 8. offscreen capture"
mkdir -p "$WORK/out"
cargo build -p offscreen-proof >/dev/null 2>&1 || die "proof harness does not build"
BIN="$WORK/target/debug/offscreen-proof"

# Equal-input set A: the canonical single capture.
"$BIN" "$WORK/out/button.png" 1 2>&1 | tee -a "$RECEIPT"
# Equal-input set B: five successive captures inside one process.
"$BIN" "$WORK/out/rep.png" 5 2>&1 | tee -a "$RECEIPT"
# Equal-input set C: three separate process invocations.
for i in 1 2 3; do "$BIN" "$WORK/out/proc$i.png" 1 >/dev/null 2>&1; done
# Equal-input set D: one capture after a full clean rebuild.
cargo clean >/dev/null 2>&1
CLEAN_START=$(date +%s)
cargo build -p offscreen-proof >/dev/null 2>&1 || die "proof harness does not rebuild after clean"
CLEAN_SECS=$(( $(date +%s) - CLEAN_START ))
"$BIN" "$WORK/out/after-clean.png" 1 >/dev/null 2>&1
say "clean rebuild of the proof binary: ${CLEAN_SECS}s  (observational, not asserted)"
say ""

# ---------------------------------------------------------------------------
# 9. Repeatability — equal-input captures only.
#    The viewport captures in step 10 use different inputs and are excluded by
#    construction: this list is built explicitly, never by a glob.
# ---------------------------------------------------------------------------
say "## 9. repeatability — equal-input captures (240x80 logical, default args)"
EQUAL_INPUT=(
  "$WORK/out/button.png"
  "$WORK/out/rep.0.png" "$WORK/out/rep.1.png" "$WORK/out/rep.2.png"
  "$WORK/out/rep.3.png" "$WORK/out/rep.4.png"
  "$WORK/out/proc1.png" "$WORK/out/proc2.png" "$WORK/out/proc3.png"
  "$WORK/out/after-clean.png"
)
check "equal-input capture count" "${#EQUAL_INPUT[@]}" "$EXPECT_EQUAL_INPUT"
shasum -a 256 "${EQUAL_INPUT[@]}" | tee -a "$RECEIPT"
DISTINCT="$(shasum -a 256 "${EQUAL_INPUT[@]}" | awk '{print $1}' | sort -u | wc -l | tr -d ' ')"
check "distinct hashes across the equal-input set" "$DISTINCT" "1"

CANON_SHA="$(shasum -a 256 "$WORK/out/button.png" | awk '{print $1}')"
check "canonical capture SHA-256" "$CANON_SHA" "$EXPECT_SHA"

# PNG dimensions, read from the IHDR chunk — proves the 2.0 scale claim.
read_png_size() {
  python3 - "$1" <<'PY'
import struct, sys
d = open(sys.argv[1], 'rb').read(33)
w, h = struct.unpack('>II', d[16:24])
print('%dx%d' % (w, h))
PY
}
check "canonical capture size" "$(read_png_size "$WORK/out/button.png")" "${EXPECT_WIDTH}x${EXPECT_HEIGHT}"

# The committed evidence PNG must match what this run produced.
if [ -f "$COMMITTED_PNG" ]; then
  COMMITTED_SHA="$(shasum -a 256 "$COMMITTED_PNG" | awk '{print $1}')"
  check "committed evidence PNG SHA-256" "$COMMITTED_SHA" "$EXPECT_SHA"
  check "committed evidence PNG size" "$(read_png_size "$COMMITTED_PNG")" "${EXPECT_WIDTH}x${EXPECT_HEIGHT}"
else
  say "  FAIL  committed evidence PNG not found at $COMMITTED_PNG"
  FAILURES=$((FAILURES + 1))
fi
say ""

# ---------------------------------------------------------------------------
# 10. Viewport control — different inputs, which must produce different output.
# ---------------------------------------------------------------------------
say "## 10. viewport control (different inputs — these must differ)"
PROOF_W=320 PROOF_H=120 "$BIN" "$WORK/out/v320.png" 1 2>&1 | grep capture | tee -a "$RECEIPT"
PROOF_W=160 PROOF_H=48  "$BIN" "$WORK/out/v160.png" 1 2>&1 | grep capture | tee -a "$RECEIPT"
shasum -a 256 "$WORK/out/v320.png" "$WORK/out/v160.png" | tee -a "$RECEIPT"
check "320x120 logical -> device pixels" "$(read_png_size "$WORK/out/v320.png")" "640x240"
check "160x48 logical -> device pixels"  "$(read_png_size "$WORK/out/v160.png")" "320x96"
V320_SHA="$(shasum -a 256 "$WORK/out/v320.png" | awk '{print $1}')"
V160_SHA="$(shasum -a 256 "$WORK/out/v160.png" | awk '{print $1}')"
check_ne "320x120 capture differs from canonical" "$V320_SHA" "$CANON_SHA"
check_ne "160x48 capture differs from canonical"  "$V160_SHA" "$CANON_SHA"
check_ne "the two viewport captures differ"       "$V320_SHA" "$V160_SHA"
say ""

# ---------------------------------------------------------------------------
# 11. Gate.
# ---------------------------------------------------------------------------
if [ "$FAILURES" -eq 0 ]; then
  say "## RESULT: all documented claims reproduced (0 failures)"
  say "receipt: $RECEIPT"
  exit 0
else
  say "## RESULT: FAILED — $FAILURES documented claim(s) did not reproduce"
  say "The EXPECT block at the top of this script is the documented state."
  say "Either the evidence has drifted or the docs are stale; fix one of them."
  say "receipt: $RECEIPT"
  exit 1
fi
