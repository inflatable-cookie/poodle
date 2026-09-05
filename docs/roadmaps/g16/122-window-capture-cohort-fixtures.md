# g16.122 — Window-Capture Cohort Fixture Kind

Status: complete — merged in PR #227 at `cab7faf10`; the lab `g01.006` GPUI leg consumes this kind
Type: capture-binary extension — renders any Nucleus cohort row from its
shared scenario file in a live non-activating window; no public API
Opened: 2026-09-05
Depends on: merged `g16.105` (icon-geometry fixture kind precedent), merged
`g16.111` (shared scenario files)
Blocks: the lab's cohort fixture adapter (poodle-lab `g01.006`) GPUI leg and
Poodle `g17.001` (formerly `g16.123`)
Governing refs: `packages/gpui/preview/src/bin/window_capture/` (`fixture_capture.rs`,
the `icon-geometry` kind), `test/nucleus-a11y/scenarios/*.json` (29 files,
`poodle.g16-nucleus-a11y-scenario.v1`), `packages/gpui/preview/src/headless_driver.rs`
(scenario replay), `nucleus-gpui-parity-programme.md` (V1 definition)
Dispatch manifest: `../dispatch.md`

## Goal

Let the non-activating window-capture binary render one cohort row in one
scenario state and return exact pixels, so the lab can capture V1 evidence
for all 29 rows through the same production GPUI path the M1 driver mounts.

## Fixed Boundary

- Add fixture kind `cohort` with `scenario_id` (one of the 29 scenario
  files, read from the pinned Poodle source) and `state` (`initial`, or
  `after-actions`: the scenario's actions replayed through production
  dispatch before capture, the same replay the A1 receipt uses).
- The scene is the canvas-coloured surface at a fixed logical size per row
  declared in the scenario file (add a `capture` block with width and height
  to the scenario schema; the A1 extractors ignore it). Theme, density, and
  control size come from the scenario props.
- Receipt schema `poodle.cohort-visual-capture.v1`: scenario id, state,
  scenario file hash, Poodle source id, and the existing transport, focus,
  foreground, and permission fields. Same non-activating proof.
- Unknown scenario ids or states fail before a window opens. No Nucleus
  data enters; these are Poodle fixtures.
- Headless tests only in this repository.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Registry is closed | unknown scenario id | typed refusal before window creation |
| Replay is the real one | `after-actions` differs from the A1 driver's end state | node inventory equals the A1 snapshot's tree |
| Scenario schema is backward compatible | A1 extractors fail on the `capture` block | `test:nucleus-a11y` green |
| Icon and Button kinds untouched | their suites | identical receipts |

## Validation

Headless capture-binary tests, `effigy regressions:native`, `effigy
test:nucleus-a11y`, `effigy check:gpui`, `effigy docs:check`, `git diff
--check origin/main...HEAD`. Never run `*-windowed` selectors.

## Owned Paths

`packages/gpui/preview/src/bin/window_capture/` (new kind, schema,
registry) and tests, `test/nucleus-a11y/scenarios/*.json` (`capture` block
only) and the scenario schema, execution log, `PAPERCUTS.md` (append).

## Stop Conditions

Stop if a scenario cannot be replayed without a driver change beyond
reusing the A1 replay. Escalation owner: Chatterbox.
