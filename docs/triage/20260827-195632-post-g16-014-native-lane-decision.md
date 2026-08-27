# Post-g16.014 Native Lane Decision

Status: resolved — merged in g16.015 / PR #90
Captured: 2026-08-27
Resolved: 2026-08-27
Source: orchestrator evidence checkpoint after PR #88

## Checkpoint

The active-cohort ledger is at 43 mounted / 131 missing. The next lane should
repair a reusable behavior seam through the real GPUI tree without opening a
new value-model, overlay, visual-comparison, accessibility, or Jetstream
programme.

## Selected Lane — CollapseToggle

CollapseToggle is the strongest bounded candidate. Svelte, React, and the
detailed contract agree on one externally controlled disclosure button, while
the shared Rust renderer drops several observable parts of that contract.

- Both web runtimes label the expanded control `Collapse`, label the collapsed
  control `Expand`, expose `aria-expanded = !collapsed`, report the next
  collapsed value, flip the directional chevron, and make disabled controls
  inert and absent from focus traversal.
- `CollapseToggleSpec` already owns the same value, direction, default/custom
  label, callback payload, focus-ring tokens, and disabled state. No public API
  decision or compatibility migration is required.
- `poodle_render::collapse_toggle` hardcodes `Toggle section` unless an explicit
  label is supplied, never projects expanded state, leaves disabled controls
  focusable, and declares no structured focus ring or enabled tab index.
- The GPUI specimen already rebuilds host-owned collapsed state and has stable
  fixture ids. It needs proof, not a specimen redesign.
- A DockRegion mounted test happens to exercise a separate dock-owned collapse
  button. It does not prove the standalone CollapseToggle renderer.

The repair is one foundation-control lane: project the spec's effective label
and expanded state, give enabled controls the contracted focus/tab behavior,
remove disabled controls from focus and activation, preserve the existing next-
state callback and icon mapping, then prove pointer and keyboard host rebuilds
through the production node backend.

Expected ledger movement is 43 → 44 mounted and 131 → 130 missing. Known-delta
totals stay 115 present / 60 not-applicable. Visual and broad accessibility
cells do not move.

## Deferred Candidates

- **Select:** high leverage, but its native mode resolution, search/query
  ownership, freeform commit, option identity, open state, and overlay keyboard
  behavior form a larger lane. Its current Rust comment also contradicts the
  promoted ghost-chevron contract. It needs a separate selection checkpoint.
- **EditableLabel:** still couples activation mode, draft ownership,
  commit/cancel payloads, select-on-focus, and focus restoration. The existing
  Tab regression proves generic routing, not full component parity.
- **NumberInput:** remains blocked on the raw-draft / committed-number decision
  in `20260826-213343-number-input-native-value-model.md`.
- **Rating:** retains the whole-step versus fractional-value semantic split and
  legacy Rust value shape noted at the previous checkpoint.
- **Dialog/menu overlays:** important, but dismissal, focus containment,
  placement, and stacked overlay ownership are not one small primitive repair.

## Decision

Compile CollapseToggle as `g16.015`. Keep the public web surface, Rust spec API,
generic node/backend vocabulary, composite collapse implementations, visual
comparison, broad native accessibility, and Jetstream admission outside the
card.
