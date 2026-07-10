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

Remaining wave 2: `Menu`, `ContextMenu` (list navigation + typeahead —
the last real machinery gap), `Toast` (queue/timeout), plus the Floating UI
anchor-positioning swap for Popover/Tooltip/HoverCard/Menu.

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
