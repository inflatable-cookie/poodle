# g11.004 Headless Core Full Component Sweep

Status: active — wave 1 complete; wave 2 dialog-family batch complete (2026-07-10)
Owner: Poodle core
Depends on: `g11.003`
Updated: 2026-07-10

## Purpose

Move every behavioral component onto core machines, wave by wave, writing the
machine spec into each component's contract as it goes. End state: all
interaction logic lives in `@poodle/headless`; `@poodle/svelte` components are
adapter + recipe shells.

## What Gets A Machine

Behavioral components only. Structural/presentational primitives (`Box`,
`Stack`, `Grid`, `Surface`, `Separator`, `Skeleton`, `Badge`, `Pill`, ...)
have no machine; they stay styled-layer-only. Record the classification per
component in its contract so the boundary is explicit, not vibes.

## Wave Order

Reuse the g11.001 wave families, ordered by machinery reuse:

1. **Selection and value**: `Checkbox` (done in pilot), `Switch`,
   `TriStateSwitch`, `RadioGroup`, `SegmentedControl`, `ToggleGroup`,
   `Slider`, `RangeSlider`
2. **Overlay and menus** (heaviest shared-machinery reuse from the popover
   pilot): `Dialog`, `AlertDialog`, `Drawer`, `Tooltip`, `Menu`,
   `ContextMenu`, `HoverCard`, `Toast`
3. **Disclosure and navigation**: `Tabs` (pilot), `Collapsible`, `Accordion`,
   `NavigationMenu`, `Menubar`, `Pagination`
4. **Text entry and pickers**: `TextInput`, `NumberEntry`, `Select`,
   `CommandPalette`, `Calendar` + the date/time picker cluster, `ColorPicker`
5. **Long tail**: remaining behavioral components; classify-or-machine
   decision recorded for everything left in `src/index.ts`

Adjust membership against the real export list at wave start; the list above
is a seed, not an inventory.

## Per-Wave Process

1. write/complete machine specs in the component contracts (g11.002 format)
2. implement machines + tests in core
3. swap Svelte internals; public surface unchanged (spec `062` rule 1)
4. run the consumer validation matrix; record results in this file
5. if a surface must change: g11.001 wave process, Underlay first, consumers
   in the same wave

## Wave 1 Status (complete, 2026-07-10)

Machine-backed and swapped: `Switch`, `TriStateSwitch`, `RadioGroup`,
`SegmentedControl`, `ToggleGroup`, `Slider`, `RangeSlider`
(+ `Checkbox` from the g11.003 pilot).

- Core modules: `switch.ts`, `single-select.ts` (shared by RadioGroup /
  SegmentedControl / TriStateSwitch), `toggle-group.ts`, `slider.ts`
  (single + range). 59 core tests green.
- Machine specs written into all seven contracts (compact form referencing
  the shared machines; the full-table pilot form remains the template).
- Public surfaces unchanged. These components lean on native inputs for
  keyboard/focus, so machines own value semantics, guards, and the
  change/commit callback split only — recorded per contract under
  Machinery Dependencies.
- Runtime-verified in the preview: switch toggle + readOnly revert, radio
  and segmented selection, tri-state cycle, toggle-group single/multiple
  membership, slider snap + percent var, range-slider thumb-crossing clamp.
- Consumer typecheck matrix clean: underlay, acme-admin, compli-me/admin,
  cp-admin, composer-admin (greenhouse still blocked by pre-existing
  songsprout/stem errors unrelated to Poodle).

`ToggleGroup` single-mode reselect (without `allowDeactivation`) re-emits
`onValueChange` with the same value — preserved from pre-machine behavior,
recorded in the contract.

## Wave 2 Status (dialog-family batch complete, 2026-07-10)

Machine-backed and swapped: `Dialog`, `AlertDialog` (composes Dialog),
`Drawer` — shared `modalTransition` machine plus `trapFocusKeydown` focus
machinery. Escape now routes through the dismissable-layer stack (innermost
first; a modal with `dismissOnEscape=false` swallows escape — modal
semantics, recorded in the contracts). 64 core tests green.

Runtime-verified: dialog open (focus entry, scroll lock), Tab trap wrap,
escape and backdrop close with scroll unlock, persistent
(`dismissOnEscape/Backdrop=false`) dialog correctly refusing both; drawer
scroll-lock cycle across escape and backdrop paths.

Hover batch complete (2026-07-10): `Tooltip` and `HoverCard` share
`hoverTransition` (closed/opening/open/closing with adapter-owned timers).
Runtime-verified: tooltip hidden through the open delay then shown,
immediate close on leave; hover-card open delay, close-delay window,
re-enter cancelling a pending close. Recorded delta: hover-card `LEAVE`
while closed is now inert instead of firing a redundant close callback.
69 core tests green.

Menu batch complete (2026-07-10): `Menu` and `ContextMenu` share
`menuTransition` (open/close + action-then-close), dismissal via the layer
stack. Runtime-verified: trigger open with focus-first-item, item action
closing and firing, escape and outside dismiss, contextmenu open at pointer.
Recorded deltas in context-menu.md (overlay-only containment; re-invoke
repositions without re-emitting open).

MenuSurface batch complete (2026-07-10): item navigation moved to core
machinery (`menuListNavigate` / `menuNavigableItems` /
`menuListCanActivate`); `internal.ts` re-exports `menuNavigableItems` so
ListCard/Menubar/SplitButton pick up the core version unchanged.
Runtime-verified: focus-first on open, arrow navigation with disabled skip
and wrap, Home/End enabled boundaries, Enter activate-and-close. Typeahead
is not present in the Svelte implementation and therefore not modeled
(parity authority). 77 core tests green.

Toast batch complete (2026-07-10): `ToastHost` runs on core machinery —
tone resolution, normalization, stickiness, and the auto-dismiss timer
reconciliation plan (`reconcileToastTimers` returns clear/start ids; the
adapter owns real timers and the store). `ToastStack` classified
styled-only. Runtime-verified: toast render and manual dismiss; timer
semantics covered by core unit tests (browser wait for the 6s auto-dismiss
not exercised). 82 core tests green.

Positioning batch complete (2026-07-10) — **Floating UI rejected** (see
spec 062): the in-house collision-aware resolver was promoted into core as
`position.ts` (pure, viewport-parameterized, Rust-portable).
Tooltip/Menu/IconButton unchanged via the window-bound wrapper; HoverCard
swapped its bespoke math onto the shared resolver (edge-flip improvement
delta recorded); Popover keeps CSS anchoring by deliberate contract delta;
ContextMenu pointer anchoring stays adapter-side. 87 core tests green,
including flip/clamp/scoring vectors. Verification note: the preview
browser pane reports a 0×0 JS viewport (harness artifact, pre-existing —
the old code read the same values), so pixel placement could not be
runtime-verified there; confidence rests on the line-identical port plus
core unit tests with real viewport sizes.

Wave 2 is complete.

## Wave 3 Status (complete, 2026-07-10)

- `Collapsible` on new `disclosureTransition`; `CollapseToggle` classified
  styled-only (stateless callback button)
- `Accordion` reuses `toggleGroupTransition` (`collapsible` maps to
  `allowDeactivation`)
- `Pagination` on core machinery: `buildVisiblePages` window math and
  `canRequestPage` guard; controller/scroll stay adapter-side
- `Menubar` and `NavigationMenu` composed onto shared machinery: menu-list
  navigation, roving helpers, dismissable-layer stack replacing their
  document listeners
- Runtime-verified: collapsible toggle/restore, accordion open +
  collapsible-close, pagination ellipsis window + page activation, menubar
  and navigation-menu escape/outside dismiss. 91 core tests green.

## Wave 4 Status (in progress)

Batch A complete (2026-07-10): `TextInput` (slugify / slug validation /
validation-state mapping), `NumberInput` (numeric coercion, step parsing,
nullable-bounds clamping, validation-state mapping), `CommandPalette`
(focus trap onto shared `trapFocusKeydown`) — all on core `input.ts` and
focus machinery. Debounce/async-validation/commit plumbing stays
adapter-side by design. Runtime-verified: slug input normalizing
"Héllo Wörld Test" → "hello-world-test"; number input clamping 999999→100
and -999999→0 on blur. 97 core tests green.

Select batch complete (2026-07-10): option flatten/filter/disabled
helpers, open placement (`selectMenuPlacement`), and open-highlight
resolution moved to core; dismissal onto the layer stack. Lazy loading,
query state, freeform, and native-mode delegation stay adapter-side.
Runtime-verified: open, escape and outside dismiss, ArrowDown+Enter
selection committing and closing. 104 core tests green.

Date/time batch complete (2026-07-10): the Svelte `date.ts` module (452
lines of pure ISO/date-time/zone math and calendar-grid construction) was
promoted wholesale into core, with the six value types now defined in core
for the Rust mirror; the Svelte module re-exports, so Calendar and all
five pickers plus TimeZoneSelect are unchanged. Test-writing surfaced two
intentional semantics worth recording: `addMonths` anchors to the 1st
(month paging), and `dayDeltaForWeekBoundary` takes an ISO string.
Runtime-verified: calendar renders full-week grids and click selection.
114 core tests green.

ColorPicker + DurationInput batch complete (2026-07-10): the color-utils
module promoted wholesale into core (`color.ts`, Svelte re-exports —
zero component changes); DurationInput segment semantics (carry/borrow
adjustment, clamped direct entry, total seconds, padding) moved to
`duration.ts` and the component swapped onto them. Runtime-verified:
duration seconds-carry (01:30:59 → 01:31:00 on ArrowUp) and color picker
open. 121 core tests green.

**Wave 4 is complete.** Remaining for this milestone: the long-tail
classification sweep — walk `src/index.ts`, mark every remaining export
machine-backed or styled-only in its contract, and extract anything that
still owns interaction logic.

## Exit Criteria

- every export classified: machine-backed or styled-only
- all machine-backed components run on core
- contracts carry machine specs for all of them
- consumer matrix green per wave, evidence recorded here
- promotion: machine model and adapter boundary promoted to
  `docs/architecture/` per spec `062`

## Validation

Per wave: core unit tests, `effigy svelte:surface-audit`, targeted
`effigy svelte:build`, consumer typecheck matrix, preview state coverage.

## Next Task

`g11.005` recipe productization can start after wave 2 (does not need the
full sweep); `g11.006` Rust mirror waits for this milestone's machine shape
to stabilize.
