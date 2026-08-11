# 002 Pilot Fixture And Metrics Freeze

Status: planned — not dispatchable
Milestone: `g13.001`
Owner: Poodle core
Depends on: `g13-b001-authority-inventory` merged and reviewed
Governing refs: `docs/specs/063-rust-authored-component-and-scene-ir.md`
(`IR-05`–`IR-11`), `docs/contracts/components/button.md`,
`docs/contracts/components/range-slider.md`,
`docs/contracts/components/text-input.md`

## Goal

Turn the reviewed authority inventory into stable, executable pilot fixtures
and a quantitative before-state. This card becomes worker-ready only after the
orchestrator rules on every conflict or dead path found by batch `001`.

## Planned Scope

- Freeze stable identifiers for Button, RangeSlider, TextInput, preview shell,
  theme selection, and size/density specimen-axis scenarios.
- Bind each fixture to current contract sections, implementation paths, and
  existing interaction/accessibility/visual evidence.
- Capture current authored LOC, generated LOC, duplicated definition count,
  runtime extension count, clean build time, diagnostic quality, and drift
  failures using the measurement method fixed by the reviewed inventory.
- Add or normalize fixtures only where the existing evidence surface can own
  them without changing component behavior.
- Produce one baseline manifest consumed by the Button/RangeSlider/TextInput
  pilot cards.

## Planning Gates

- Exact manifest path and schema come from batch `001`'s authority map.
- Existing visual baselines are referenced, not refreshed, unless a separate
  visual-change decision is approved.
- No fixture may bless a current implementation contradiction; the
  orchestrator must rule first.
- Worker rules, writable paths, exact commands, and stop conditions must be
  filled before status can move to `ready`.

## Exit

- Every pilot acceptance claim has a stable fixture/evidence identifier.
- The quantitative baseline is reproducible from a clean worktree.
- Remaining unknowns are placement/schema decisions owned by batch `003`.
