# g15.040 — ResizeHandle native keyboard and value semantics

Date: 2026-08-20
Card: `docs/roadmaps/g15/040-resize-handle-native-semantics.md`
Handoff: `docs/handoffs/20260820-230943-g15-040-resize-handle-native-semantics.md`
Parent: `docs/roadmaps/g15/027-screen-clear-human-review.md`
PR: #56

## Outcome

The shared native ResizeHandle is no longer drag-only. An enabled handle is a
real focus stop with a visible focus treatment, answers the contract's exact
Arrow/Home/End deltas through the existing resize callback, and declares its
axis, name, and current/minimum/maximum value on the renderer-neutral node. A
disabled handle takes no focus, answers no key, and starts no gesture.

Mounted headless GPUI input proves it through the real event tree: the focused
separator moves the host's pane and the value the next node declares. No web
implementation, public callback, or platform accessibility claim changed.

## Change class

This is a **breaking, pre-1.0, operator-approved** public Rust API migration,
taken after review round 1 rejected a derived focus key. No alias, optional
twin, `Default`, or silent fallback remains.

- **Packages changed:** `poodle-specs`, `poodle-node` (additive),
  `poodle-render`, GPUI preview specimens and tests, GPUI adapter demo app,
  Jetstream preview specimens and adapter compile callers
- **Public-intent entry points:** `ResizeHandleSpec::new(instance_id)` and
  `SplitViewSpec::new(instance_id, orientation)` now require a caller-owned
  native instance scope; `ResizeHandleSpec` no longer implements `Default`;
  new `SplitViewSpec::divider_instance_id()`; new
  `poodle_render::resize_handle_focus_id`; additive `NodeA11y.value_min` /
  `NodeA11y.value_max`
- **Compatibility:** clean break on both constructors, pre-1.0; operator
  approved 2026-08-21. The `NodeA11y` fields are additive.
- **Downstream re-check:** every out-of-repo `ResizeHandleSpec::new()` and
  `SplitViewSpec::new(orientation)` call must supply a lifetime-stable scope.
  A mounted ResizeHandle now takes focus and consumes axis Arrow/Home/End while
  focused, and its root carries `runtime_id` (not `id`), so a host reading the
  semantic `id` finds none. No Svelte, React, or other web surface changed:
  the browser owns identity there.

## Implementation

- `NodeA11y` gains the smallest general numeric range: `value_min` and
  `value_max` beside the existing `value`. Both default to absent — an
  invented `0..100` on every node reads exactly like a declared one.
- `ResizeHandleSpec.aria_value_now/min/max`, orientation, and the contract's
  `"Resize"` default name now reach the node. The range is declared even when
  the host states no current value; a current value alone announces a number
  with no span.
- Keyboard resize reuses the resize callback. One keystroke is one whole
  gesture — `Start(0)`, `Move(delta)`, `End(0)` — so a host that commits on
  release commits once per key. Horizontal answers Left/Right, vertical
  answers Up/Down, both answer Home/End, cross-axis arrows are not consumed.
  Deltas are the contract's `±8` and `±9999`.
- The composition collapses the affordance line onto the root. §7 already puts
  the line at `inset: 0` — the two are the same pixels — and in the node
  vocabulary only the node that holds focus can carry a focus state, so a
  separate line child would put the paint out of the focus channel's reach.
  Idle, hover, active, and disabled geometry and colour are unchanged.
- focus-visible recolors that hairline to `color.accent.focusRing`. GPUI has
  no outline that costs no layout, and the handle's whole footprint is the
  `0.125rem` line, so a border would move the split it describes. Recorded in
  the contract's GPUI notes rather than left as an undocumented deviation.
- Identity is caller-supplied and carried on `Node.runtime_id`, the
  vocabulary's backend-state key. Orientation, name, and value are semantics —
  two handles may legitimately share all three, and a name that changes with a
  translation would move the key of a control that never moved. `SplitView`
  states its own scope and derives the divider's (`{scope}:divider`), so two
  ordinary splits no longer resolve one focus handle.
- Drag repair found on the way: the horizontal handle's grab overlay carried
  no drag handler (the vertical one did), so grabbing anywhere but the 2px
  line did nothing. Drags do not bubble; both hit targets now carry it in both
  orientations.
- GPUI specimen: the live horizontal and vertical sections declare the pane
  they actually draw (`48–280` and `40–120`, current value included), matching
  what the Svelte and React specimens already pass.

## Evidence

- render (11 focused tests): focus stop only when enabled; the focus patch
  repaints the visible hairline; per-axis key filtering with exact deltas;
  one keystroke = Start/Move/End; role, name, axis and full range on the node;
  contract default name and default range on a bare spec; per-frame axis drag
  delta from both hit targets in both orientations
- identity (3 of those): two handles that agree on axis, name and value still
  keep distinct backend identities; one instance keeps its identity across a
  rebuild that changes orientation, label and value; a disabled handle is
  identified too, so re-enabling it finds the same node
- node: a node declares no numeric range until a component states one
- mounted GPUI (`headless_regressions`, 3 new): a focused separator steps the
  host pane and the declared current value through the real key route
  (`right` → 128, `up` → unchanged, `left` → 120, `home` → 48, `end` → 280,
  range intact across every rebuild); a disabled separator never becomes a
  focus target and answers no key; **two composed `SplitView`s with identical
  orientation, label and ratio do not share a divider focus handle** —
  focusing one leaves the other blurred, both ways round
- GPUI specimen (4 tests): the page's own handler moves the pane from the
  keyboard and clamps at the page's own minimum; the declared range is the
  pane drawn; all four sections carry their own backend identity, scoped by
  the same key the page stores its pane under; the disabled section stays
  inert

## Audit

`specimen-catalogue-audit.md` revision 11: ResizeHandle returns to `A / A / A`
with disposition `keep`. Totals recounted mechanically from all 175 rows:
GPUI 103 A / 65 B / 6 C / 1 n/a; worst-of-three 66 A / 48 B / 52 C / 9 D;
`keep` 56; `contract/runtime-blocker` 0.

## Validation

Headless only. No windowed, `test:native-visual`, browser, Jetstream, or
release selector ran.

- `cargo test --manifest-path packages/contracts/node/Cargo.toml` — 3 passed
- `cargo test --manifest-path packages/render/Cargo.toml` — 364 passed
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --bin
  poodle-preview resize_handle` — 6 passed
- `effigy ci:rust` — passed
- `effigy check:gpui` — passed (364 render, 22 backend, preview check)
- `effigy regressions:native` — 53 passed
- `effigy probe:gpui-specimens` — 7 passed
- `effigy docs:check` — passed
- `effigy qa` — passed
- `git diff --check origin/main...HEAD` — clean

`packages/jetstream/preview` cannot compile in this worktree: its
`jetstream-input` path dependency resolves to a sibling checkout that does not
exist here. Its two migrated specimen files parse and are type-trivial (a
`&str` scope into `impl Into<String>`); `poodle-jetstream` itself, which
`poodle-render` builds against, compiles clean.

## Review round 1 (PR #56)

One blocker: the first implementation derived `Node.id` from orientation plus
accessible name. Those are semantics, not identity — two same-axis handles with
the same label collided, a relabelling moved the key, and `SplitView` composed
every divider with the default `"Resize"` name, so ordinary splits resolved one
focus handle. The batch log recorded that as an accepted residual while the
contract claimed the opposite. Both are gone: identity is caller-supplied,
carried on `runtime_id`, proven by the three tests above, and the contract now
states what is true. Operator approved the breaking constructor change.

## Unresolved

- **No platform AT projection.** GPUI 0.2.2 exposes no accessibility
  attributes (`docs/contracts/003-native-accessibility.md`). The role, name,
  axis, and range reach the renderer-neutral node and stop there. Nothing in
  this card claims otherwise.
- Card and roadmap status, the dispatch ledger, merge, and the `g15.031`
  promotion stay with the orchestrator.
