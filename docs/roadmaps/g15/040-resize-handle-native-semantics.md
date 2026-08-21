# g15.040 — ResizeHandle Native Keyboard and Value Semantics

Status: **complete** — PR #56, merge `8954658a`
Parent: `027-screen-clear-human-review.md`
Found by: `030-review-foundation-layout.md`
Unblocks: `g15.031`–`g15.033`; `g15.012` remains behind the completed
human-centred audit
Governing refs: `../../contracts/components/resize-handle.md`,
`../../contracts/003-native-accessibility.md`,
`../../contracts/001-working-rules.md`, `specimen-catalogue-audit.md`

## Problem

The shared native ResizeHandle render path only wires pointer drag. It declares
the splitter role and label, but does not make the enabled handle focusable,
handle Arrow/Home/End keys, or project `ResizeHandleSpec.aria_value_*` into the
renderer-neutral node. The GPUI specimen added by `g15.030` can now demonstrate
drag, but it cannot teach the same keyboard and value semantics as Svelte and
React.

GPUI 0.2.2 still has the accepted platform accessibility-projection gap in
contract 003. This card closes the component and renderer-neutral declaration
gap; it must not claim that upstream platform AT projection is fixed.

## Goal

Make the active native ResizeHandle keyboard-operable and semantically complete
through the existing shared Rust composition and GPUI input path, without
changing the web implementations or adding a compatibility surface.

## Scope

- Project current, minimum, and maximum numeric values from
  `ResizeHandleSpec` into `NodeA11y`. Add the smallest general numeric-range
  fields to `NodeA11y` if the vocabulary cannot currently carry them.
- Make enabled handles focusable and disabled handles unreachable. Preserve the
  documented splitter role, label, orientation, disabled state, and focus
  treatment.
- Route axis-appropriate Arrow keys as `±8` deltas and Home/End as `±9999`
  through the existing native resize callback. Do not add a second public
  keyboard-only callback when the current delta seam is sufficient.
- Keep pointer drag behavior and per-frame delta semantics unchanged.
- Update the GPUI specimen so keyboard changes the pane and current value just
  as drag does.
- Add focused renderer and mounted GPUI evidence for focus entry, axis key
  filtering, Arrow/Home/End deltas, value/range declaration, disabled
  suppression, and retained drag behavior.
- Return the ResizeHandle audit row to the grade/disposition supported by the
  repaired evidence and mechanically recount totals.

## Acceptance

- Enabled ResizeHandle nodes are focusable; disabled nodes are not.
- Horizontal handles respond to Left/Right, vertical handles to Up/Down, and
  both respond to Home/End with the contract's exact deltas. Cross-axis arrows
  do nothing.
- `aria_value_now`, `aria_value_min`, and `aria_value_max` survive the spec to
  renderer-neutral node path with orientation and label intact.
- Mounted headless GPUI input proves a focused handle changes the specimen pane
  and renderer-neutral current value through the real key route.
- Pointer drag regression evidence remains green.
- The audit no longer carries a ResizeHandle contract/runtime blocker and its
  totals are recounted from all 175 rows.
- No Svelte, React, Jetstream, public web API, or platform accessibility claim
  changes.

## Stop Conditions

- Keyboard support requires a new public callback or a component-specific
  backend route rather than the existing node key vocabulary.
- Numeric range semantics require a broader accessibility redesign instead of
  a small general `NodeA11y` extension.
- A visible focus treatment cannot be expressed through the existing node
  style/focus vocabulary without changing ResizeHandle geometry.
- Validation exposes the same semantic gap as a family-wide primitive defect
  whose repair would materially expand this card.

## Writable Scope

- ResizeHandle spec/render composition, focused tests, and GPUI specimen
- the smallest general `NodeA11y` numeric-range extension and focused node
  evidence required by the contract
- ResizeHandle contract wording only where renderer-neutral/native behavior
  needs clarification
- `specimen-catalogue-audit.md`, one August batch log, and `PAPERCUTS.md`

## Validation

- focused `poodle-node`, `poodle-render`, and GPUI preview/backend tests
- `effigy ci:rust`
- `effigy check:gpui`
- `effigy regressions:native`
- `effigy probe:gpui-specimens`
- `effigy docs:check`
- `effigy qa`
- `git diff --check origin/main...HEAD`

Headless only. Never run a `*-windowed`, `test:native-visual`, Jetstream, or
release selector.
