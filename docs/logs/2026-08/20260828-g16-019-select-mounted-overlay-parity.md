# g16.019 — Select Mounted Overlay Parity

Date: 2026-08-28
Status: complete — PR #94, review follow-up applied
Branch: `t3code/select-overlay-worker-handoff`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-4f7abc53`
Card: `docs/roadmaps/g16/019-select-mounted-overlay-parity.md`
Handoff: `docs/handoffs/20260828-181625-g16-019-select-overlay-worker.md`
Source triage: `docs/triage/20260828-085200-post-g16-017-native-lane-decision.md`

## Outcome

Native Select now edits query through existing Node input/caret/edit channels,
routes pointer/keyboard/focus/dismissal through the landed
`select_transition` machine, and proves deferred overlay option rows with real
GPUI press/release after host rebuilds. The host still owns `SelectSpec`; each
event emits one complete `SelectTransitionResult`.

The generated ledger moves only Select: 46 → 47 mounted and 128 → 127 missing.
Known-delta totals stay 115 present / 60 not-applicable.

## Editable search

Open searchable Select renders a compact editor, not a nested TextInput shell.
Replacement text, edit-key, insert, caret, submit, cancel, and blur ride the
existing Node channels. Query reports `SelectEvent::Query`. Enter is
`CommitHighlighted`. Escape is `Close`. Searchable focus stays on the editor
while highlight moves; non-searchable focus stays on the trigger. Option rows
are pointer targets (`tab_index` unset, not focusable), not sequential tab
stops.

## Overlay repair

The deferred-row miss was reproduced with a generic fixture (no Select
identifier): trigger rebuilds the tree open, then pointer on a runtime_id
option must fire `on_activate` and must not first dismiss. The fixture also
covers `overflow:hidden` on the overlay and an in-flow sibling under the
panel.

The bounded backend seam:

- Record painted bounds from resolved identity (`runtime_id` or `id`), not
  only `Node.id`.
- Inherit dismiss-layer membership onto descendants so option rows spare the
  layer.
- Overlay surfaces `occlude()` so deferred paint does not leak clicks to
  widgets they cover.
- Skip Taffy `overflow:hidden` on overlay nodes. Auto-height absolute/deferred
  boxes otherwise collapse to padding and clip option hitboxes.
- Skip the `size_full` bounds canvas on overlay surfaces themselves. That
  child is what collapsed the panel; trigger and option canvases still record
  containment.
- Non-focusable layer members `prevent_default` on press so the window overlay
  host cannot steal focus, blur the trigger, and unmount the row before click.

No Select-specific coordinates. Nested popover deferred-draw still holds.

## Pagination workaround

The choose path no longer stamps `pagination-limit-25` or a test-only ring.
It pointer-opens the production trigger and pointer-activates
`select:pagination-limit:option:25`. The numbered/simple/loading paths stay
green.

## Mounted proof

`select_two_instances_search_pointer_and_dismiss_through_mounted_rebuilds`
drives two independently scoped Selects through real GPUI dispatch:

- pointer open and deferred option commit on the non-searchable instance;
- disabled option inertia;
- outside pointer dismiss without a value change;
- searchable typing reports query only, Enter commits highlighted banana;
- clear and Escape;
- no test-only option id or ring.

The curated GPUI specimen host applies the complete next context, then
requests editor or trigger focus. Examples stay curated.

## Explicit non-claims

- no new Select machine, web public API, or composed-component behavioral
  closure
- no generic input vocabulary
- no menu/popover/dialog migration, visual comparison, or broad native
  accessibility claim
- no Jetstream admission
- no NumberInput, EditableLabel, audio, motion, Longhorn, release, version,
  workflow, or downstream change

## Validation

Ran in `/Users/tom/.t3/worktrees/poodle/t3code-4f7abc53` on
`t3code/select-overlay-worker-handoff`:

- focused TypeScript core Select tests (14)
- focused `poodle-headless` `select_conformance` / `single_select_conformance`
- focused `poodle-specs` and `poodle-render` Select tests
- focused Svelte/React Select tests (15 + 15) via `ci:web`
- generic overlay pointer regression, named Select mounted regression, and
  Pagination mounted regression
- `effigy probe:gpui-specimens` (8)
- `effigy regressions:native` (94/94)
- `effigy drift:handlers`, `effigy drift:events`, `effigy docs:spec-drift`,
  `effigy docs:contract-drift`
- `effigy test:parity-evidence-ledger` and
  `effigy check:parity-evidence-ledger` (47 mounted / 127 missing; 115 / 60)
- `effigy ci:rust`, `effigy ci:native`, `effigy ci:web`, `effigy docs:check`
- `effigy qa`
- `git diff --check`

Not run / blocked:

- `effigy drift:roles` and Jetstream preview — deferred Jetstream sibling
  (`PAPERCUTS.md`)
- `*-windowed` / native visual / release / workflow mutation — out of scope

## Review follow-up

PR review required four blockers; they are now in the same PR:

1. Control blur emits one `SelectTransitionResult` (`CommitFreeform` then
   `Close` only if still open). Renderer and mounted tests assert callback
   count plus final value/query/open.
2. Search caret/selection is host-authored on `SelectSpec` through the existing
   edit/select-range channels. Mid-string insert, keyboard caret, pointer
   placement, and Tab blur are in the mounted regression.
3. Overlay overflow is applied again. `a_capped_deferred_overlay_clips_overflowing_rows`
   covers Hidden + `max_height`. Select menus stay content-sized (Visible)
   because Hidden on an auto-height overlay zeroes content unless max binds.
4. Overlay surfaces record containment with inset-0 observers, not `size_full`.
   A group-header click stays inside the layer.

Follow-up validation:

- `cargo test --lib -- select::` in `packages/render` (25 passed, including
  one-result blur and mid-string search insert)
- `a_deferred_overlay_row_receives_pointer_after_host_rebuild`
- `a_capped_deferred_overlay_clips_overflowing_rows`
- `select_two_instances_search_pointer_and_dismiss_through_mounted_rebuilds`
- `pagination_navigation_limit_and_loading_through_mounted_pointer_and_keyboard`
- `git diff --check origin/main...HEAD`

Second review follow-up:

1. Search selection keeps `(anchor, head)` order. `search_selection_range()`
   is paint/range only. Mounted Shift+Arrow after a backward rebuild shrinks
   the head, not a swapped anchor.
2. Home/End on the search row highlight first/last option. Mounted End →
   cherry, Home → apple. Edit keys skip home/end so they do not move the
   caret.
3. Short Select menus stay Visible. Long menus set Scroll when estimated
   content exceeds `size.menu.maxHeight`. GPUI now resolves that overlay
   size token (it was 0). `a_long_select_menu_clips_overflowing_option_rows`
   is a production Select: last row does not activate; wheel on the listbox
   is the scroll path.

Validation: render Select 28, GPUI overlay + Select + long clip + Pagination,
GPUI overlay token mapping, `git diff --check`.
