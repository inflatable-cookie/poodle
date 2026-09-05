# g16.105 — Window-Capture Icon-Geometry Fixture Kind

Status: complete — merged in PR #210 at `85609d941`; the lab icon-geometry GPUI leg ran on it (`g16.051` admission)
Type: capture-binary extension — no public API, no contract change
Opened: 2026-09-04
Depends on: merged `g16.050`
Blocks: the GPUI leg of poodle-lab `g01.002` and therefore `g16.051`
Governing refs: `050-icon-geometry-internal-runtime-substrate.md`,
`051-icon-geometry-native-visual-admission.md`,
`packages/gpui/preview/src/bin/window_capture/` (`fixture_capture.rs`,
`inventory.rs`), `packages/core/src/icons/morph-pairs.json`,
`../../architecture/012-semantic-motion-policy.md`
Dispatch manifest: `../dispatch.md`

## Goal

Let the non-activating window-capture binary render an icon-geometry
fixture: one candidate pair, one direction, one named state, through the
production GPUI geometry path from `g16.050`, so the lab's GPUI leg can
capture exact pixels for `g16.051`.

## Fixed Boundary

- Add a second fixture kind beside the Button inventory: `icon-geometry`
  with `pair` (one of the six `candidate` ids in `morph-pairs.json`),
  `direction` (`forward` | `reverse`), and `state` (`endpoint-from`,
  `endpoint-to`, `midpoint`, `reverse-midpoint`, `frozen`, `interruption`,
  `teardown`) as defined by g16.050's laws. Rejected pair ids fail before a
  window opens.
- The scene is the canvas-coloured padded surface at a fixed logical size
  recorded in the receipt; the icon is drawn through the resolved geometry
  node and the production GPUI paint path, never a static SVG.
- State realisation is deterministic: the binary advances the plan to the
  exact sample (0.5 for `midpoint`), issues the reversal or second target at
  0.5 for `reverse-midpoint`/`interruption`, applies the frozen policy for
  `frozen`, and for `teardown` removes the host and captures the following
  frame. No wall-clock timers decide a frame.
- Receipt schema `poodle.icon-geometry-visual-capture.v1` carrying pair,
  direction, state, policy, sample, plus the existing transport, focus,
  foreground, and permission fields. Same non-activating window path; same
  `macos-window-server-nonactivating` proof.
- No change to Button fixtures, the public `Icon`, pair status, or any
  contract. Headless tests only in this repository (`regressions:native`
  style); no windowed selector runs here.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Registry is closed | a rejected pair id | typed refusal before window creation (headless test) |
| Sample is exact | request `midpoint` | receipt records sample 0.5 and the plan's frame hash equals the headless geometry test's frame at 0.5 |
| Reverse rebases | `reverse-midpoint` | receipt frame differs from `midpoint` and matches the headless reversal trace |
| Teardown is clean | `teardown` | captured frame hash equals the empty scene |
| Button path untouched | run the Button fixture suite | identical receipts |

## Validation

`effigy regressions:native`, `cargo test` for the window-capture binary's
headless tests, `effigy check:gpui`, `effigy docs:check`, `git diff --check
origin/main...HEAD`. Never run `*-windowed` selectors.

## Owned Paths

`packages/gpui/preview/src/bin/window_capture/` (new fixture kind module,
receipt schema, registry), its headless tests, execution log under
`docs/logs/2026-09/`, root `PAPERCUTS.md` (append only).

Reserved for the coordinator at merge: `docs/roadmaps/g16/README.md`,
`docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`.

## Stop Conditions

Stop if realising a state needs a change to the g16.050 runtime rather than
to the binary, or if the non-activating window path must change. Escalation
owner: Chatterbox.
