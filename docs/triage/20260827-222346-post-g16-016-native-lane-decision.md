# Post-g16.016 Native Lane Decision

Status: open — operator approval required for the recommended Rust API migration
Captured: 2026-08-27
Source: merged `g16.016` / PR #91 and the 45 mounted / 129 missing ledger

## Checkpoint

`g16.016` closed Pagination without claiming the composed Select. The next lane
must keep the evidence standard: a mounted cell moves only when the component's
defining behavior is coherent through the production tree, not when one
convenient callback can be driven.

## Candidate Assessment

### Recommended: Rating

Rating has one bounded semantic migration before mounted proof:

- Svelte and React use `number | null`, default `step=0.5`, fractional pointer
  and keyboard input, whole-step roving radio behavior, slider semantics in
  fractional mode, and `number | null` callback payloads.
- `RatingSpec` stores concrete `f64`, defaults `step=1`, retains legacy
  `precision`, and the renderer reports `u32` whole-star changes.
- The existing node vocabulary already carries radio/slider roles, numeric
  values, focus rings, pointer geometry, keyboard input, and host rebuilds.

The clean pre-1.0 repair is intentionally breaking: make authored/default
values nullable, make the default step `0.5`, remove `precision`, and change the
native callback to `Option<f64>`. No aliases, shims, integer fallback callback,
or silent conversion should remain. This needs operator approval before a card
is compiled.

### Not Yet Bounded: Select

Pagination proved Select's trigger and option-change seam, but Select itself is
not ready for a one-cell closure:

- the Rust handler surface has toggle/change/clear but no query-change path;
- the rendered search row is display text rather than a real editable input;
- open highlight, Arrow/Home/End navigation, Escape/Tab lifecycle, freeform
  commit, and focus return are not represented coherently; and
- deferred option rows currently need a test-only focus ring, while pointer
  targeting through the deferred overlay is not yet a dependable proof path.

Select needs a separate contract/API and overlay-behavior planning lane. Do not
reduce it to the page-size subset exercised by Pagination.

### Still Blocked Or Lower Leverage

- NumberInput remains behind
  `20260826-213343-number-input-native-value-model.md`.
- EditableLabel still needs a default double-click/select-on-focus/focus-return
  decision and an input route that can distinguish its activation gesture.
- SplitButton and the menu family inherit the larger Select/menu overlay and
  focus questions.
- Broad visual comparison, native accessibility, and Jetstream admission remain
  programme decisions, not substitutes for the next bounded behavior card.

## Recommended Decision

Approve the clean Rating Rust migration, then compile `g16.017` to close
nullable/fractional behavior and one mounted GPUI cell. If the migration is not
approved, pause component execution and plan Select as a larger contract/API
lane rather than selecting an easier but lower-value row merely to increase the
ledger count.
