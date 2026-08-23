# g16.005 GPUI crates.io Recovery

Date: 2026-08-23
Card: `../../roadmaps/g16/005-gpui-cratesio-recovery.md`
Generation: `../../roadmaps/g16/README.md`
Policy: `../../specs/022-packaging-versioning-and-release-channel-rules.md`
Decision prototype: `../../research/gpui-cratesio-nonactivating-capture.md`
Superseded proof: `../../research/gpui-offscreen-capture-feasibility.md` (g15.044, still valid for the forked graph)
Handoff: `../../handoffs/20260823-225904-g16-005-gpui-cratesio-recovery.md`
Worker branch: `t3code/recover-gpui-cratesio`

## Outcome

`poodle-gpui-node-backend` and `poodle-gpui-preview` resolve `gpui = "0.2.2"`
from crates.io again. The `inflatable-cookie/zed` fork and `gpui_platform` are
gone from every active manifest and lockfile, both graphs resolve zero Git
sources, and a downstream crate that declares crates.io `gpui = "0.2.2"` for
itself now compiles against Poodle's node backend and passes GPUI values
across the boundary in both directions — the exact thing published `v0.2.1`
made impossible.

The fork existed to reach `HeadlessAppContext` / `render_to_image`, which are
not published APIs. That capability is genuinely lost. Rather than keep the
word, the capture target was replaced: `poodle-window-capture` opens ONE real
GPUI window with `focus: false`, never activates the application, captures
that window by its own window id, and refuses to publish anything if the
frontmost application changed during the run.

No version, release-note, candidate, tag, workflow, or publication change is
in this branch. `g16.006` and `g16.007` own those.

## Dependency And Source Boundary

| Surface | Before | After |
| --- | --- | --- |
| `packages/gpui/node-backend/Cargo.toml` | `gpui = { git = "…/inflatable-cookie/zed", rev = "87d9afbe…" }` | `gpui = "0.2.2"` |
| `packages/gpui/preview/Cargo.toml` (normal) | same fork pin + `gpui_platform` (`font-kit`) | `gpui = "0.2.2"`, no platform crate |
| `packages/gpui/preview/Cargo.toml` (dev) | fork pin + `test-support` | `gpui = { version = "0.2.2", features = ["test-support"] }` |
| Git sources in either lock | 23 packages from the fork + 3 zed-industries repos | **zero** |

Lockfiles were regenerated, then reviewed package by package. Every version
delta is GPUI-graph transitive — mostly downgrades, because the published
0.2.2 graph is older than the adopted fork revision (`taffy` 0.13→0.9,
`metal` 0.33→0.29, `cosmic-text` 0.19→0.14, the `objc2` 0.5/0.6 split
collapsing to 0.6, and so on). `cargo update` on the node backend also bumped
`log` 0.4.33→0.4.34 and `crc32fast` 1.5.0→1.5.1; both were pinned back, so no
crate outside the GPUI graph moved. Package counts: node backend 462→709
(the monolithic published crate declares every platform's dependencies, where
the workspace-split fork did not), preview 691→733.

`bzip2` and `libbz2-rs-sys` left both graphs with the fork. See **Follow-ups**.

## Mechanical API Reversal

`g15.044` measured the forward migration at 17 errors across nine files. The
reversal is eight distinct API deltas across ten files. It was driven by the
compiler, not by reverting files: later work (g15.046–g15.052) touched most of
them, and all of that is preserved.

| API | Fork | crates.io 0.2.2 | Sites |
| --- | --- | --- | --- |
| `ShapedLine::paint` | 6 args (`TextAlign`, `Option<_>`) | 4 args | `node-backend/src/input_text.rs` |
| `FocusHandle::focus` | `(window, cx)` | `(window)` | `node-backend/src/interaction.rs` ×2, `preview/src/headless_driver.rs` ×2 |
| `Window::focus_next` | `(cx)` | `()` | `preview/src/headless_driver.rs` |
| `Styled::flex_grow_1` | `flex_grow_1()` | `flex_grow()` | `node-backend/src/style.rs` ×2, two specimens ×3 |
| `ScrollHandle::max_offset` | `.y` | `.height` | `node-backend/src/tracked_scroll.rs` |
| `KeyDownEvent` | `prefer_character_input` field | absent | `preview/src/headless_driver.rs` |
| `BoxShadow` | `inset` field | absent | `node-backend/src/style.rs`, `preview/src/specimens/mod.rs` ×2 |
| application constructor | `gpui_platform::application()` | `Application::new()` | `preview/src/main.rs` |

`Window::focus_next(cx)` was **not** in g15.044's inventory: g15.052 added
that call site later, against the fork's signature. It is the one delta the
historical measurement did not predict.

### The one behavioural loss

`BoxShadow` has no `inset` flag on crates.io 0.2.2, so the node backend
returns to the pre-`g15.045` approximation: inset (highlight) shadow layers
are dropped, drop layers project exactly, and an all-inset stack projects no
shadow refinement at all. This is a real loss and it is asserted rather than
quietly reinstated — `g15.045`'s two projection tests were rewritten to pin
the drop (`inset_shadow_layers_are_dropped_and_drop_layers_map_exactly`,
`the_fallback_descriptor_shadow_projects_its_single_drop_layer`) and a third
case was added for the all-inset stack. A future upstream `inset` field is
then a visible test change, not a silent one.

Everything else is preserved: component rendering, the presentation cascade,
focus rings, the 18-case fixture inventory, receipt integrity, the comparison
policy, the specimens, and the headless interaction evidence.

## Capture: What Replaced The Fork-Only Transport

`poodle-offscreen-capture` is gone with no alias. It depended on
`HeadlessAppContext::capture_screenshot`, `PlatformWindow::render_to_image`,
and `gpui_platform::current_headless_renderer()` — none published.

`poodle-window-capture` (`src/bin/window_capture.rs` +
`window_capture/{transport,fixture_capture,focus_evidence,forbidden,inventory}.rs`)
keeps all three modes (`--fixture`, `--focus-evidence`, and the single-Button
smoke) and all three CLI contracts, on a new transport:

- one GPUI window, `focus: false`, `show: true`, `titlebar: None`;
- no activation call anywhere — see **Focus boundary** below;
- `screencapture -x -o -l <own-window-id>`; the id comes from
  `CGWindowListCopyWindowInfo` filtered to this process's pid;
- the captured PNG is published **verbatim**. It is never decoded and
  re-encoded, so what lands on disk is what the window server produced. Only
  the IHDR header is read, which also removed the `image` crate dependency
  entirely.

`titlebar: None` is a contract, not decoration. GPUI maps it to a titled,
full-size-content window with a transparent, title-less bar and no traffic
lights, so the window **frame equals the requested logical content rect** and
the capture can assert `device == logical × 2` rather than guessing where
content sits inside a frame. A mismatch fails loudly, naming both sizes; there
is no resample and no crop.

The capture target no longer needs `gpui/test-support` in the shipping graph.
The fork-era coupling between "capture" and "test-only APIs" is gone; the
`window-capture` feature now enables only `sha2` and two `objc2-app-kit`
feature flags.

### Freezing the loading spinner

The fork path called `App::set_reduce_motion(true)`, which 0.2.2 does not
have. `fixture_capture::freeze_node_animations` clears the node tree's own
animation declarations instead, so `to_gpui` builds an un-animated element
that paints its declared initial frame and schedules nothing. Same end state,
through Poodle's own vocabulary rather than a GPUI capability that no longer
exists.

### Receipt schemas — renamed, not reused

| Schema | Before | After |
| --- | --- | --- |
| smoke | `poodle.gpui-offscreen-capture.v1` | `poodle.gpui-window-capture.v1` |
| fixture | `poodle.button-visual-capture.v1` | `poodle.button-visual-capture.**v2**` |
| focus evidence | `poodle.gpui-focus-evidence.v1` | `poodle.gpui-focus-evidence.**v2**` |

`renderer: "metal-headless"` became `transport:
"macos-window-server-nonactivating"`. `gpuiRevision: <40-hex>` became
`gpuiSource: "crates.io"` + `gpuiVersion: "0.2.2"`, and the TypeScript
verifier now **asserts** `gpuiSource === "crates.io"` — a receipt produced
against a forked GPUI is not evidence about what a consumer gets. Every
receipt additionally carries the run's own frontmost-application samples.

## Focus Boundary: How It Is Proved Without Running It

The card's central claim cannot be exercised by a worker, because exercising
it needs a window server and Screen Recording permission. It is proved two
ways instead.

**Structurally.** `window_capture/forbidden.rs` reads all five capture sources
back through `include_str!` and fails if any **code** line names
`cx.activate(`, `.activate_window(`, `makeKeyAndOrderFront`,
`orderFrontRegardless`, `activateIgnoringOtherApps`, `osascript`, or
`screencapture`'s `-R`/`-D`/`-C` flags. It also fails on any code line
claiming `offscreen`, `metal-headless`, `render_to_image`,
`HeadlessAppContext`, or `gpui_platform` — the receipt cannot lie about what
produced it. Comment lines are skipped so the sources can say what they
refuse to do. The check was negative-tested:
`the_boundary_check_detects_planted_violations` feeds it four planted lines
and asserts it catches exactly the three code ones, at the right line number,
and ignores the comment. Two of the planted fragments were caught for real
during development (an error string and a test temp-directory name still said
"offscreen"), which is how I know it is not decorative.

**At runtime, on every capture.** `ForegroundMonitor` samples
`NSWorkspace.frontmostApplication` every 50 ms — baseline taken **before** any
window exists — for the whole run. `run_capture` evaluates it *before*
publishing: a run whose frontmost application changed fails with both the
baseline and the observed set, and writes nothing. `foreground_changed` is a
pure function with its own tests, including the case that matters most: an
absent baseline (locked screen, login window) with any observation counts as a
change, because no evidence is not the same as proof.

## Source Policy

Two independent, fail-closed layers.

`scripts/repository-security-policy.ts`:

- `approvedGitRevisions` is now **empty**. The five entries existed only to
  carry the GPUI fork; with an empty allowlist every Git dependency in every
  Cargo manifest and lockfile is rejected. The allowlist became a parameter so
  the revision-mismatch and mutable-reference rules stay under test.
- `registryOnlyCrates = ["gpui", "gpui_platform"]` rejects those two from a
  Git source **even if a repository is ever re-admitted**. Optional tooling
  does not get to choose the crate identity a public runtime package exposes.
  The lockfile side tracks the enclosing `[[package]]` name so it blames the
  right crate.
- The scan is Cargo-manifest and Cargo-lock only, so historical logs,
  handoffs, and research notes that quote the old fork pin stay readable. A
  test pins that.
- `Cargo.toml.template` is now scanned too — the dual-dependency proof's
  manifest declares a real `gpui` requirement and must obey the same rule.

`deny.toml`: `allow-git = []` with `unknown-git = "deny"` retained. Before
this change `cargo deny` exited 0 while emitting five `unmatched-source`
warnings; the policy is now actually closed rather than merely unused.

## Consumer Compatibility Proof

`test/consumer-dual-dependency/` — `effigy drift:gpui-consumer-identity`, in
`ci:native`.

`consumer/` is written the way a real consumer writes it: it declares
`gpui = "0.2.2"` for itself, depends on `poodle-gpui-node-backend` by path,
and threads GPUI values both ways — `to_gpui` → `gpui::AnyElement`, `color` →
`gpui::Hsla`, `focus_handle_for` → `gpui::FocusHandle`, `bounds_for` →
`gpui::Bounds<Pixels>`, and a Poodle element composed into a tree the consumer
builds with its own `gpui` (`ParentElement::child` rejects a divergent
identity even if every annotation were removed). No `[patch]`, no `[replace]`,
no override: the proof is that none is needed.

`run.ts` stages it in a temporary directory with `path` dependencies rewritten
to absolute paths, so no lockfile and no target directory enters the checkout,
then asserts the manifest carries no override, the crate compiles, the
resolved lock holds exactly one `gpui` with every `gpui*` crate from the
registry, and — the negative control — that the same crate with one wrong GPUI
type annotation **fails** with a type mismatch. Compilation reuses a stable
target directory under the system temp dir: ~35 s cold, ~3.6 s warm.

```
resolved: gpui 0.2.2 from registry+https://github.com/rust-lang/crates.io-index
```

## Selectors

| Selector | Windowed? | Change |
| --- | --- | --- |
| `smoke:gpui-offscreen-capture` | — | **removed**; the transport it drove cannot exist on stock 0.2.2 |
| `smoke:gpui-window-capture` | no | new; headless. Builds the target, runs its unit tests (activation boundary, device-size policy, foreground rule, publish atomicity), and proves 12 negative invocations are rejected during argument validation. In `ci:native`. |
| `capture:gpui-windowed` | **yes** | new; the capture itself. Repeat byte-identity, receipt verification, foreground evidence. Operator-approved only. |
| `drift:gpui-consumer-identity` | no | new; the proof above. In `ci:native`. |
| `test:visual-comparator` | no | new; the comparator's own 26 unit tests, split out so they stay headless |
| `test:visual-button-comparison` | — | **renamed** to `test:visual-button-comparison-windowed`; its GPUI leg now opens windows |

The `-windowed` suffix is deliberate: `AGENTS.md` already forbids running a
`*-windowed` selector locally without operator approval, so naming carries the
guard. `qa`, `ci`, `ci:web`, `docs:check`, and every release gate open no
window and need no Screen Recording permission.

Negative invocations exit **2** (rejected while parsing) versus **1** (failed
after the window opened), so the smoke's `status === 2` assertion is itself
the proof that no window was involved.

## Validation

All headless, on the worker worktree. No windowed, native-visual, Jetstream
preview/QA, release, workflow, tag, or publication command was run.

| Check | Result |
| --- | --- |
| `cargo test -p poodle-gpui-node-backend` | 28/28 |
| `regressions:native` (headless GPUI test platform) | 64/64 |
| `probe:gpui-specimens` | 8/8 |
| `catalogue` / `visual_fixture_inventory` | 7/7, 15/15 |
| `cargo test --bin poodle-window-capture --features window-capture` | 32/32 |
| `effigy smoke:gpui-window-capture` | 17/17 checks pass |
| `effigy drift:gpui-consumer-identity` | 8/8 checks pass, negative control fails as required |
| `bun test scripts/audit-repository-security.test.ts` | 12/12 |
| `bun test test/visual/button-comparison/compare.test.ts` | 26/26 |
| `effigy ci:native` | pass (exit 0) |
| `effigy qa` | pass (exit 0) |
| `effigy docs:check` | pass (exit 0) |
| `effigy audit:licenses` / `audit:security` | pass; no `unmatched-source` warnings remain |
| `git diff --check origin/main...HEAD` | clean |

A fresh worktree needs `bun install --frozen-lockfile` before the JS boards
run, as g15.051 also recorded.

## Operator-Owned Windowed Review

The card reserves one visual run for the orchestrator, after code review and
with explicit operator approval. Exact commands, in order:

```sh
# 1. The transport itself: 3 captures of identical input, one hash, receipts
#    verified, frontmost application unchanged across all three.
effigy capture:gpui-windowed

# 2. The retained 18-fixture Button runner and the cross-runtime comparison.
effigy test:visual-button-comparison-windowed
```

Both need a macOS window server and Screen Recording permission for the
terminal's parent process. Neither writes into the repository: `(1)` works in
a temp directory, `(2)` writes to the disposable
`test/visual/button-comparison/out`. `(2)` exits non-zero by design on
blocking findings — that was already true at `g15.047` and `g15.052`.

Watch for, in `(1)`'s output: `transport:
macos-window-server-nonactivating`, `device: 480x160`, and `foreground:
baseline=<your app> observed=["<your app>"] changed=false`.

## What The Worker Could Not Verify

Stated plainly, because the review run is the thing that settles them:

- **Captured frame geometry.** The reasoning that `titlebar: None` makes frame
  == content is read from GPUI 0.2.2's macOS window source, not measured. If
  the window server includes a rounded-corner mask or any frame inset, the
  capture will **fail loudly** on the device-size assertion rather than
  publish a wrong image — but it will fail. The decision prototype's own
  numbers (a 420×120 window captured at 976×442) include the drop shadow that
  `-o` now excludes, so they do not answer this.
- **Repeat byte-identity through the window server.** Deterministic in
  principle for static content on one display; not measured here.
- **`NSWorkspace.frontmostApplication` off the main thread.** The objc2
  bindings do not mark it main-thread-only and the accepted prototype sampled
  the same way, but this run did not execute it.
- **Corner/shadow effects on the Svelte↔GPUI comparison.** If `(2)` shows new
  corner-region deltas that `g15.047` did not have, that is a finding for a
  follow-up card, not something this branch could have measured.

## Follow-ups (not in scope here)

- **The capture target was already broken on `main`.** It still called
  `poodle_render::button(&spec, &theme, None)`, the pre-`g15.043` signature.
  `required-features` kept it out of `--all-targets`, so no gate ever built
  it and nothing noticed. It is fixed here (`RenderContext::new`), and
  `smoke:gpui-window-capture` in `ci:native` now builds and tests the target
  on every native board so it cannot rot invisibly again.
- **Stale bzip2 notices.** `bzip2`/`libbz2-rs-sys` left both GPUI graphs with
  the fork, so `THIRD_PARTY_NOTICES.md` at the repository root and in the
  public-intent node-backend package, the `bzip2-1.0.6` entry in `deny.toml`,
  and spec 022's bzip2 paragraph now describe a dependency Poodle does not
  have. `audit:licenses` still passes (the markers are present and
  `unused-allowed-license = "allow"`), and over-attribution harms nobody, so
  this was left alone rather than widened into a licence change. It should be
  re-derived from the final graph in `g16.006`, where notices are already part
  of candidate preparation.
