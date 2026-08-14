# g14.003 — RangeSlider Controlled-control Proof

Date: 2026-08-14
Card: `docs/roadmaps/g14/003-range-slider-controlled-control-proof.md`
Depends on: g14.002 / PR #11
Status: complete — ready for PR (do not flip roadmap status)

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
- GPUI scrub via mouse-down/move/up (on_drag swallowed synthetic AppKit path)

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
| `effigy conformance:test-gpui` | pass (20 button + 10 range-slider) |
| `effigy conformance:compare` | pass (30 cases × 3 runtimes) |
| `cargo test … range_slider::tests` | pass |
| planted role divergence | fails then restores |

## Papercut

Fresh worktrees need `bun install` before `conformance:test-web`.
