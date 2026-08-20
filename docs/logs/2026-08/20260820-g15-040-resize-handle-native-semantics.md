# g15.040 — ResizeHandle native keyboard and value semantics

Date: 2026-08-20
Card: `docs/roadmaps/g15/040-resize-handle-native-semantics.md`
Handoff: `docs/handoffs/20260820-230943-g15-040-resize-handle-native-semantics.md`
Parent: `docs/roadmaps/g15/027-screen-clear-human-review.md`
PR: pending

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

- **Packages changed:** `poodle-node` (additive), `poodle-render`, GPUI
  preview specimen and its tests
- **Public-intent entry points:** additive `NodeA11y.value_min` /
  `NodeA11y.value_max`; new `poodle_render::resize_handle_focus_id`. The
  `resize_handle` signature and `ResizeHandleSpec` are unchanged.
- **Compatibility:** additive only. No alias, no second callback, no dual API.
- **Downstream re-check:** a mounted ResizeHandle now takes focus and consumes
  axis Arrow/Home/End while focused, and its root node carries an id derived
  from orientation and accessible name. A host that assigned its own id to the
  handle root must assign it after the render call, as before.

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
- The root's element id is derived from orientation and accessible name, so
  the page's two live sections cannot share a backend focus handle.
- Drag repair found on the way: the horizontal handle's grab overlay carried
  no drag handler (the vertical one did), so grabbing anywhere but the 2px
  line did nothing. Drags do not bubble; both hit targets now carry it in both
  orientations.
- GPUI specimen: the live horizontal and vertical sections declare the pane
  they actually draw (`48–280` and `40–120`, current value included), matching
  what the Svelte and React specimens already pass.

## Evidence

- render (9 focused tests): focus stop only when enabled; the focus patch
  repaints the visible hairline; per-axis key filtering with exact deltas;
  one keystroke = Start/Move/End; role, name, axis and full range on the node;
  contract default name and default range on a bare spec; distinct ids;
  per-frame axis drag delta from both hit targets in both orientations
- node: a node declares no numeric range until a component states one
- mounted GPUI (`headless_regressions`, 2 new): a focused separator steps the
  host pane and the declared current value through the real key route
  (`right` → 128, `up` → unchanged, `left` → 120, `home` → 48, `end` → 280,
  range intact across every rebuild); a disabled separator never becomes a
  focus target and answers no key
- GPUI specimen (4 tests): the page's own handler moves the pane from the
  keyboard and clamps at the page's own minimum; the declared range is the
  pane drawn; the two live sections do not share an id; the disabled section
  stays inert

## Audit

`specimen-catalogue-audit.md` revision 11: ResizeHandle returns to `A / A / A`
with disposition `keep`. Totals recounted mechanically from all 175 rows:
GPUI 103 A / 65 B / 6 C / 1 n/a; worst-of-three 66 A / 48 B / 52 C / 9 D;
`keep` 56; `contract/runtime-blocker` 0.

## Validation

Headless only. No windowed, `test:native-visual`, browser, Jetstream, or
release selector ran.

- `cargo test --manifest-path packages/contracts/node/Cargo.toml` — 3 passed
- `cargo test --manifest-path packages/render/Cargo.toml` — 362 passed
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --bin
  poodle-preview resize_handle` — 6 passed
- `effigy ci:rust` — passed
- `effigy check:gpui` — passed
- `effigy regressions:native` — 52 passed
- `effigy probe:gpui-specimens` — 7 passed
- `effigy docs:check` — passed
- `effigy qa` — passed
- `git diff --check origin/main...HEAD` — clean

## Unresolved

- **Anonymous composed handles share one focus identity.** The id is derived
  from orientation and accessible name, which is enough for a page that names
  its handles. `SplitView` passes neither, so every divider it composes
  derives the same id and therefore the same backend focus handle. Before this
  card those dividers were not focusable at all, so nothing collided; now a
  page with several SplitViews has several dividers resolving one handle. The
  honest fix is composer-supplied, lifetime-stable identity — the same
  conclusion `g15.038` reached for SegmentedControl — which is a breaking
  `SplitViewSpec` change outside this card's scope. Routed to the
  orchestrator, not fixed here.
- **No platform AT projection.** GPUI 0.2.2 exposes no accessibility
  attributes (`docs/contracts/003-native-accessibility.md`). The role, name,
  axis, and range reach the renderer-neutral node and stop there. Nothing in
  this card claims otherwise.
- Card and roadmap status, the dispatch ledger, merge, and the `g15.031`
  promotion stay with the orchestrator.
