# g15.044 GPUI Offscreen Capture Feasibility

Date: 2026-08-21
Card: `../../roadmaps/g15/044-gpui-offscreen-capture-feasibility.md`
Parent: `../../roadmaps/g15/012-visual-conformance-lane.md`
Research note: `../../research/gpui-offscreen-capture-feasibility.md`
Reproduction: `assets/g15-044/reproduce.sh` — the complete recipe
Receipt: `assets/g15-044/receipt.txt` — verbatim output of a full clean run
Posture: strict-ready research/proof lane; production dependency graph untouched

## Verdict

**Go.** A real Poodle Button renders offscreen to a deterministic RGBA PNG at
`zed-industries/zed@1ea16c1ab9dd6d36649e002dc60995634da04daf`, with no visible
window, no desktop capture, no focus theft, and no permission prompt. The
production migration is 17 mechanical compile errors across 9 files plus one
added dependency, and the retained headless regression suite passes 56/56 after
it.

The pin is unchanged in this branch. `g15.045` is the only adoption authority
and requires operator review of this verdict.

## Isolated Proof Command

```sh
bash docs/logs/2026-08/assets/g15-044/reproduce.sh . "$(mktemp -d)"
```

That is the exact command, and it is exact because the script is the whole
recipe rather than a summary of one. It contains the complete harness source,
the workspace manifest, the harness manifest, the vendoring steps, the
dependency repointing, the deterministic migration patch, and every measurement.
It reads the Poodle repository and writes only into the throwaway directory.

### It is also the verifier

The script's `EXPECT` block holds every durable structural claim in this log and
the research note, and asserts all of them — **25 checks**:

| Asserted | Value |
| --- | --- |
| vendored copy builds at the production pin | must build |
| adapter has no `gpui` dependency | 0 errors |
| node-backend / preview / headless-test error counts | 8 / 6 / 3 |
| total migration errors | 17 |
| packages in lock / from the zed git source | 702 / 23 |
| regressions passed / failed | 56 / 0 |
| equal-input capture count | 10 |
| distinct hashes across that set | 1 |
| canonical capture SHA-256 | `be94eace…` |
| canonical capture size (IHDR) | 480×160 |
| committed evidence PNG hash and size | same as above |
| viewport captures, size and difference | 640×240, 320×96, all distinct |
| every literal migration replacement | must match before it is applied |

It reports *all* drift in one run rather than stopping at the first mismatch,
and exits non-zero. Timings are recorded but **not** asserted — they are
machine- and load-dependent, and failing on them would be noise.

**Negative-tested.** Seeding two deliberately wrong `EXPECT` values
(`EXPECT_REGRESSIONS=53`, `EXPECT_NB_ERRORS=9`) produced exit 1 and named both
mismatches. The gate is real, not decorative.

The previous revision of this script asserted only two dependency strings and
one image hash while the docs claimed it asserted every claim. It printed
`RESULT: all claims reproduced` while the regression count had drifted from 53
to 54. That is the gap this section closes.

An earlier version of this log replaced the workspace, manifests, and patching
with prose comments, which made the "exact proof command" unreproducible. That
is fixed: nothing needed to recreate the result is described rather than
recorded.

### Why a script instead of a fixture

The card allows a retained proof fixture only if it is dependency-isolated and
cannot enter package, workspace, QA, or release graphs. A Rust harness would
need a crate and manifests and would enter Cargo and QA discovery. This script
cannot: it lives under `docs/` log assets, declares no package, is imported by
nothing, and is executed only when a human runs it. It therefore satisfies the
retention requirement without the discovery risk.

### Verified run

Run from a fresh `mktemp -d` against the clean pre-receipt tree recorded by
the retained receipt:

```text
# g15.044 reproduction receipt
repo commit:   a1f6c2f2b590bc3084e5b620aae611b5f9990a7c
upstream:      https://github.com/zed-industries/zed @ 1ea16c1ab9dd6d36649e002dc60995634da04daf
rustc:         rustc 1.97.1 (8bab26f4f 2026-07-14)
uname:         Darwin 25.5.0 arm64

  PASS  poodle-gpui-node-backend @ gpui 0.2.2 builds
  PASS  node-backend error count: 8
  PASS  preview error count: 6
  PASS  headless-test error count: 3
  PASS  total migration errors: 17
  PASS  packages in lock: 702
  PASS  packages from the zed git source: 23
  PASS  regressions passed: 56
  PASS  regressions failed: 0
  PASS  equal-input capture count: 10
  PASS  distinct hashes across the equal-input set: 1
  PASS  canonical capture SHA-256: be94eace…
  PASS  canonical capture size: 480x160
  PASS  committed evidence PNG SHA-256: be94eace…
  PASS  320x120 logical -> device pixels: 640x240
  PASS  160x48 logical -> device pixels: 320x96
## RESULT: all documented claims reproduced (0 failures)
```

The receipt records the source Poodle commit it was generated from, so a stale
receipt is detectable rather than merely suspected. It necessarily names the
commit it ran against, whose tree is this one minus the receipt update — a
receipt cannot contain its own hash. Re-running the script is what confirms it,
and that run is one command.

The canonical PNG from that independent run is byte-identical to the committed
`assets/g15-044/button-offscreen.png` — SHA-256 `be94eace…`. Full output:
`assets/g15-044/receipt.txt`.

## Every Command Actually Run

Source reconnaissance:

```sh
grep -n -B2 -A6 'name = "gpui"' packages/gpui/preview/Cargo.lock
grep -rn -E 'fn (draw_to_image|render_to_image|capture|read_pixels|to_image|screenshot|offscreen|snapshot)' \
  ~/.cargo/registry/src/*/gpui-0.2.2/src
sed -n '/pub(crate) trait PlatformWindow/,/^}/p' ~/.cargo/registry/src/*/gpui-0.2.2/src/platform.rs
grep -rn "screencapture" --exclude-dir=target --exclude-dir=node_modules .
curl -sS -H "User-Agent: poodle-g15-044-research (…)" https://index.crates.io/gp/ui/gpui
gh api repos/zed-industries/zed/commits/main --jq '.sha + "  " + .commit.committer.date'
gh api -X GET "search/code?q=repo:zed-industries/zed+offscreen+path:crates/gpui"
gh api -X GET "search/code?q=repo:zed-industries/zed+render_to_image"
gh api "repos/zed-industries/zed/contents/crates/gpui/src/app/headless_app_context.rs?ref=1ea16c1a…"
gh api "repos/zed-industries/zed/contents/rust-toolchain.toml?ref=1ea16c1a…"
```

Proof and migration costing — every one of these is a step inside
`reproduce.sh`, and all of them run inside the throwaway workspace:

```sh
cargo build -p poodle-gpui-node-backend           # baseline at 0.2.2: BUILDS
# repoint gpui at the rev, then measure each surface before patching it:
cargo build -p poodle-gpui-node-backend           # 8 errors  -> patch -> BUILDS
cargo build -p poodle-gpui-preview                # 6 errors  -> patch -> BUILDS
cargo test  -p poodle-gpui-preview --no-run       # 3 errors  -> patch -> BUILDS
cargo test  -p poodle-gpui-preview --test headless_regressions   # 56 passed, 0 failed

# equal-input captures (240x80 logical, default args)
"$BIN" out/button.png 1                           # canonical
"$BIN" out/rep.png 5                              # 5 in one process
for i in 1 2 3; do "$BIN" "out/proc$i.png" 1; done  # 3 separate processes
cargo clean && cargo build -p offscreen-proof
"$BIN" out/after-clean.png 1                      # 1 after a clean rebuild

# hashed over the explicitly enumerated equal-input list, never a glob
shasum -a 256 out/button.png out/rep.{0,1,2,3,4}.png out/proc{1,2,3}.png out/after-clean.png

# viewport control: DIFFERENT inputs, expected to differ, hashed separately
PROOF_W=320 PROOF_H=120 "$BIN" out/v320.png 1
PROOF_W=160 PROOF_H=48  "$BIN" out/v160.png 1
shasum -a 256 out/v320.png out/v160.png
```

The earlier version of this log recorded `cargo clean` and a rebuild but no
capture after it, and hashed `out/*.png` — a glob that also swept in the
deliberately different viewport images, so it could not have shown one hash.
Both are corrected above: the post-clean capture is explicit, and the hash set
is an enumerated list.

Longhorn contract 022 was read with `sed -n`/`grep` only. The Longhorn checkout
was not modified.

## Measured Results

Repeatability — **10** captures of identical input, one SHA-256
(`be94eace…`), distinct hashes **1**:

| Run set | Captures | Files |
| --- | --- | --- |
| canonical single capture | 1 | `button.png` |
| successive, one process | 5 | `rep.0.png` … `rep.4.png` |
| separate processes | 3 | `proc1.png` … `proc3.png` |
| after `cargo clean` + rebuild | 1 | `after-clean.png` |
| **total** | **10** | — |

The previous revision of this log said "8" and then listed 5 + 3 + 1. The
enumeration was right and the total was wrong; the canonical capture also
belongs to the set, so the correct figure is 10. `reproduce.sh` now builds this
list explicitly and asserts the distinct-hash count is exactly 1.

Viewport control: 240×80 → 480×160 (`be94eace…`), 320×120 → 640×240
(`a4ffa571…`), 160×48 → 320×96 (`56f775b1…`). Different inputs, different
hashes, excluded from the equal-input set by construction.
Scale is fixed at 2.0 — `TestWindow::scale_factor` is hardcoded upstream.

Cost, Apple M5 Max / Darwin 25.5.0 / SDK 26.5 / rustc 1.97.1, debug profile:

| Measure | Value |
| --- | --- |
| headless context construction | 36–63 ms |
| first capture | 26–52 ms |
| subsequent captures in-process | ~15.5 ms |
| process total, cached build | ~117 ms |

An earlier ad-hoc clean-rebuild measurement read 74.6 s and did not reproduce.
Three consecutive measurements under `reproduce.sh` give 26–28 s; that is the
figure to trust. Both are recorded rather than quietly dropping the awkward one.
| clean rebuild of the proof binary | 26–28 s (3 measurements, not asserted) |

Migration surface:

| Surface | Errors | Files |
| --- | --- | --- |
| `poodle-gpui` (adapter) | 0 | — (no `gpui` dependency) |
| `poodle-gpui-node-backend` | 8 | 4 |
| `poodle-gpui-preview` (bin) | 6 | 4 |
| `poodle-gpui-preview` headless tests | 3 | 1 |
| Total | 17 | 9 |

Lock delta 704 → 702 packages; 23 packages move to the zed git source; one new
direct dependency (`gpui_platform`).

## Regression Count Is Base-Dependent

| Base | Count | What moved it |
| --- | --- | --- |
| `eb4bc165` (card start) | 53 | — |
| `dd00ab26` | 54 | `ae838e67` — g15.041 Popover interactive triggers |
| `5e72e2d5` (current) | 56 | `408577ab` — g15.042 Stepper native interaction parity |

Each rebase that picks up a new native regression moves the number. That is
sibling-lane coupling, not migration instability — the migrated copy has passed
100% at every base. `reproduce.sh` asserts the exact count, so drift fails the
run and forces this table to be corrected instead of leaving a stale figure in
the docs. It caught both moves; the second was caught after this branch was
rebuilt onto `5e72e2d5`.

## Stop Conditions — None Triggered

| Condition | Result |
| --- | --- |
| needs a visible `NSWindow` | no — `TestPlatform` builds a `TestWindow`; no `NSWindow` exists |
| needs `screencapture` | no — no subprocess of any kind |
| needs accessibility / Screen Recording permission | no — offscreen `MTLTexture` readback only |
| needs a private platform API | no — public `gpui` / `gpui_platform` surface |
| unbounded renderer or backend redesign | no — 17 mechanical signature fixes, 56/56 regressions pass |
| proof growing shared fixtures or comparison semantics | no — one Button, one capture, no fixture names |

## Boundaries Held

- Production `Cargo.toml`, `Cargo.lock`, packages, sources, baselines,
  workflows, and release artifacts: unchanged.
- Longhorn repository: read-only.
- `g15.045` not started, not marked.
- Stepper, Popover, Button production code, the specimen audit, and sibling
  worker closeout surfaces: untouched.
- No `*-windowed`, `test:native-visual`, Jetstream, release, or workflow
  selector was run. `effigy doctor` was not run.
- `headless_regressions` was run only against the disposable `$PROOF` copy, and
  it is the retained in-memory path — no window, no focus.

## Changed Files

- `docs/research/gpui-offscreen-capture-feasibility.md` (new)
- `docs/roadmaps/g15/044-gpui-offscreen-capture-feasibility.md`
- `docs/roadmaps/g15/012-visual-conformance-lane.md`
- `docs/roadmaps/g15/release-gap-register.md`
- `docs/logs/2026-08/assets/g15-044/button-offscreen.png` (new)
- `docs/logs/2026-08/assets/g15-044/reproduce.sh` (new)
- `docs/logs/2026-08/assets/g15-044/receipt.txt` (new)
- `PAPERCUTS.md`
- this log

## Validation

- `bash docs/logs/2026-08/assets/g15-044/reproduce.sh . "$(mktemp -d)"` — pass,
  25/25 assertions met, 0 failures, 10/10 equal-input captures share one hash
- the same script with two seeded-wrong `EXPECT` values — exit 1, both
  mismatches named (gate verified)
- `cargo test -p poodle-gpui-preview --test headless_regressions` (disposable
  copy, migrated) — 56 passed, 0 failed
- `effigy docs:check` — see PR
- `git diff --check origin/main...HEAD` — see PR
