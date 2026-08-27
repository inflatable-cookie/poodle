# Post-g16.008 Native Lane Decision

Status: promoted — operator approved the clean migration for `g16.009`
Captured: 2026-08-27
Source: orchestrator evidence checkpoint after PR #82

## Finding

The next useful mounted-evidence card should improve a primitive's real
contract alignment, not only add another named test. Three adjacent candidates
were inspected.

### DurationInput — recommended next

The web contract is clear, but the shared Rust spec still has two competing
value sources and the renderer reads the wrong one:

- the contract exposes `hours`, `minutes`, and `seconds`; Rust also exposes a
  formatted `value` field that is not in the public contract;
- the renderer formats and edits from `value`, while the new segment fields
  are used only when the host manually keeps both representations aligned;
- the contract default for `showSeconds` is `true`; Rust defaults it to
  `false`;
- `minTotalSeconds` and `maxTotalSeconds` are stored but do not derive the
  renderer's invalid state; a separate `validation_state` input controls the
  border instead.

This means a mounted regression can currently prove only a carefully duplicated
fixture, not the public DurationInput contract. Recommended repair: make the
three segments the sole Rust authority, derive display text, totals, and bounds
validation from them, align the default, then prove carry, borrow, direct digit
entry, bounds, disabled behavior, and real segment traversal in a mounted GPUI
host. That is a clean pre-1.0 break to the Rust spec; do not retain aliases or
silent synchronization.

### IconButton — later primitive lane

IconButton is high leverage and still lacks mounted GPUI evidence. Earlier
focus-ring and disclosure-projection papercuts are already fixed in the shared
renderer, although redundant composite restatements and stale papercut entries
remain. The larger parity gaps are `onPressedChange` and built-in tooltip
lifecycle. Closing those honestly may require a handler API change and a
separate native tooltip decision, so it is not the smallest post-routing card.

### EditableLabel — later editing lane

EditableLabel now proves Enter, Escape, and blur-time Tab routing, but its Rust
surface still conflates committed value and draft. The renderer cannot produce
the contract's `{ value, previousValue }` commit payload from current inputs,
and activation-mode, select-on-focus, and focus restoration remain unproved.
It needs a dedicated draft/committed-value and focus-effect decision rather
than being bundled into DurationInput.

## Operator Decision

Approve or reject the clean DurationInput Rust-spec migration:

1. remove the formatted `value` authority from `DurationInputSpec`;
2. use `hours` / `minutes` / `seconds` as the sole controlled value;
3. default `show_seconds` to `true`;
4. derive formatted text and min/max invalid state from the segments;
5. update all in-repo Rust callers in one card, with no compatibility shim.

If approved, promote this decision into `g16.009` and dispatch one serial
worker. If rejected, return to the evidence checkpoint rather than certifying
the duplicated current model.

Approved by the operator on 2026-08-27. Promoted into
`../roadmaps/g16/009-duration-input-single-source-and-mounted-behaviour.md`.
Keep this note until the worker closes the card, then mark it resolved with the
execution log reference.
