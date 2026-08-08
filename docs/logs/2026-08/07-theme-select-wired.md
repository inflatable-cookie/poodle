---
title: ThemeSelect wired, and both native preview headers moved onto it
status: complete
owner: Poodle core
updated: 2026-08-07
tags: [log, g12.019, theme-select, gpui, jetstream]
---

## What Was Wrong

Operator report: the GPUI preview header listed every theme as a button, where
the Svelte preview uses ThemeSelect — and the ThemeSelect component itself was
inert.

It was half-built, not broken. `poodle_render::theme_select` already took
`on_change` and attached `on_activate` to each swatch tile, so *selecting* a
theme worked. The **trigger** had no activation, no pointer cursor, and there
was no open/close parameter at all. Since `ThemeSelectSpec::is_open` is
controlled, the host had no way to learn the trigger was pressed, so the panel
could never be opened — and nothing wired `on_change` either.

That is a gap against the contract, not a design choice. Contract §States:
"open | click trigger | popover grid of swatch tiles". Svelte: `toggleOpen()` on
the trigger, `select(value)` on the tiles.

Neither gate could have caught it. `drift:handlers` flags handlers *accepted and
unused*; it says nothing about a contract event with no handler at all. The rule
that did cover that case was `drift:clicks`' rule 2 — retired with the Jetstream
component tier it scanned.

## Changes

- **`ThemeSelectHandlers { on_change, on_open_change }`** plus an additive
  `theme_select_with_handlers`, keeping the existing `theme_select` signature —
  the `EditableLabelHandlers` pattern. The trigger reports the open state it is
  moving to and gains a pointer cursor, focusability, and a stable id.
- **Stable per-tile ids** (`theme-select-tile-{value}`). Jetstream dispatches by
  `token_key` from the node id and cannot reach an unnamed tile at all; GPUI
  needs identity that survives a rebuild between a click's press and release.
- **Anchored the panel.** The recipe laid the open surface out as a flow sibling
  *beside* the trigger — the old GPUI tier's placement, matched deliberately in
  Wave 23. Svelte anchors it `bottom-start` with a 0.5rem offset and portals it,
  and the contract root is the `position: relative` anchor. It is now
  `NodePosition::Absolute` below the trigger, so opening the picker no longer
  reflows the trigger or collides with the neighbouring controls.
- **Both native headers** now use the component. GPUI wraps it in gpui's
  `deferred()` so the panel paints above later siblings — the backend's half of
  "portalled" — and the controls bar lost its `overflow_hidden`, which had been
  slicing the grid in half. Jetstream routes the trigger and tiles through its
  existing `token_key` dispatch, with a new `ToggleThemeSelect` action.
- **Swatches resolve from each preset's own tokens**, not hardcoded hex, built
  once per app rather than per frame. A token change cannot leave the picker
  previewing a colour the theme no longer uses. Jetstream gained
  `ThemePreset::theme_definition()` so its swatch builder and `rebuild_shell`
  read one mapping instead of two copies.

## Verification

GPUI, driven end to end: click the trigger → the panel opens with all twelve
swatches and the current theme ringed; click `nord` → the theme applies, the
header re-themes, the trigger label updates, the panel closes.

Jetstream: 138 specimens render offscreen with no failures.

Green: `poodle-render` 109, `poodle-jetstream` 161, both preview builds,
`effigy drift:handlers`, `git diff --check`.

## Baselines Deliberately Not Refreshed

The header change touches every GPUI capture — `theme-select`, `button` and
`avatar` all differ by ~0.656%, which is the header delta alone. Anchoring the
panel also changes the `theme-select` specimen, whose baseline shows an open
picker.

Left stale on purpose: more is expected to change in this round, and a blanket
`--update` would bake the 37 named residuals into their baselines and destroy
the "every residual stays explainable" property. When the round settles, refresh
the currently-exact slugs and record that the residual ones carry the header
delta on top of their own.

## Follow-Up: The Specimens Assumed The Old Inline Layout

Anchoring the panel broke both ThemeSelect specimens, which the operator caught
on the GPUI page: the popover looked permanently open, ignored clicks, and
rendered behind everything below it.

All three symptoms were the same root cause. The specimens were written against
the old layout, where the open surface was an ordinary flow sibling that took up
space. Made absolute, it overlays whatever follows. On top of that the GPUI
specimen hardcoded `with_open(true)`, passed no handlers at all, and did not
defer — so it could not be closed, could not respond, and painted under its
siblings.

- **GPUI specimen** is now interactive, following the Popover and Menu pattern:
  open state is a specimen toggle, the trigger drives it through the node event
  queue, and choosing a swatch records the value and closes. Wrapped in
  `deferred()` so the panel paints above the examples below it. Verified with
  the click driver: `theme-select-open=true` after the trigger, then
  `theme-select-open=false theme-select-value="eclipse"` after picking a tile.
- **Jetstream specimen** cannot do that — its specimens are pure functions of
  the theme with no host state — so the open example simply moved to the end,
  where nothing follows it to be overlaid.

Worth noting for the rest of this round: any specimen that hardcodes an open
overlay has the same latent collision. `with_open(true)` appears in the
Jetstream AlertDialog, CommandPalette, TimeZoneSelect and DateRangePicker
specimens; those components were not re-anchored here, so they are unaffected
today, but the same fix applies if they are.

