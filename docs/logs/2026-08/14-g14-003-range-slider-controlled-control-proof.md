# g14.003 — RangeSlider Controlled-control Proof

Date: 2026-08-14
Card: `docs/roadmaps/g14/003-range-slider-controlled-control-proof.md`
Depends on: g14.002 / PR #11
Status: complete — ready for PR (do not flip roadmap status)
Reviewed against: `668ab7b3`

## Outcome

```text
portable interface + typed case corpus (10 cases)
  -> Svelte / React / GPUI execution (green)
  -> multi-component compare (button + range-slider)
  -> specimen corpus projections on all three active runtimes
  -> Jetstream program-deferred
```

## What landed

### Generic vocabulary

- `number` / `numberPair` prop types; `key` / `scrub` actions; part `value`
- native `{ kind: "id" }` resolve; part-scoped focus/disabled observation
- `ScrubPhase::Release`; `NodeA11y.value`
- numeric-aware value equality in native assert (i64 vs f64)
- GPUI scrub via captured drag plus synthetic mouse-move fallback

### Authority + harnesses

- `range-slider.ts` / `range-slider-cases.ts` + serialized fixtures
- Web adapters/hosts; GPUI `conformance_range_slider.rs` in the same windowed bin
- Dual-thumb native render; React role=group + commit-on-mouseup fixes

### Specimens

- Svelte, React, GPUI pages project the corpus (hand-written fixtures replaced)

### Defects caught

- React missing `role="group"`; React commit used continuous `onChange`
- GPUI scrub never fired under `on_drag` + synthetic LeftMouseDragged
- Stale FOCUS_STATES after remount onto disabled thumbs
- Empty-string root name on web vs null on native (shape mismatch)

### Orchestrator review fixes

- Restored captured GPUI drag so a real scrub continues beyond the thin track;
  retained the synthetic-event fallback without double delivery.
- Made React standard-thumb commits dirty-aware and commit the pending raw
  value. Unrelated key-up/mouse-up no longer emits, and controlled parents
  cannot cause a stale commit.
- Made compare derive required case IDs from the canonical corpus. A runtime
  now fails for omitted or unexpected cases even when every report drifts the
  same way.
- Restored embedded bipolar specimen coverage and size/density axes without
  growing the ten-case corpus.
- Completed GPUI fixture projection for `centerValue`, value text, and
  `sizeRole`.
- Split normal headless conformance from the disruptive foreground AppKit
  proof. The binary requires explicit `--windowed`; isolated macOS CI retains
  the full active-cohort gate.

### Known boundary

`RangeSliderSpec.law` remains outside the portable interface. It is a
structured `AudioValueLaw`, while the pilot vocabulary currently admits
scalar and number-pair values. g14.010 must either admit a bounded structured
value shape or rule this profile incomplete; this card does not claim full
interface replacement.

### Thin slider vectors

- Two-thumb claims owned by RangeSlider cases; single-value `slider` vectors stay

### Planted failure

- Planted `lower.role = "button"` in `svelte-range-slider.json` → compare failed
  naming the role divergence; restored → compare green

## Validation

| Command | Result |
| --- | --- |
| `effigy conformance:typecheck` | pass |
| `effigy conformance:test-web` | pass |
| `effigy conformance:test-gpui-windowed` | pass (20 button + 10 range-slider) |
| `effigy conformance:compare` | pass (30 cases × 3 runtimes) |
| `cargo test … range_slider::tests` | pass |
| planted role divergence | fails then restores |
| `effigy ci:web` | pass (100 files / 1301 tests) |
| `effigy ci:native` | pass |
| `effigy ci:conformance` | pass (headless board) |

## Papercut

Fresh worktrees need `bun install` before `conformance:test-web`.

The GPUI proof takes foreground AppKit focus. It is now guarded by an explicit
`--windowed` flag and excluded from normal local conformance/QA selectors.
