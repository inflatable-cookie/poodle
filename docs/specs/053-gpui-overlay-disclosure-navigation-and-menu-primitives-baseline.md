# 053 GPUI Overlay, Disclosure, Navigation, And Menu Primitives Baseline

Status: active
Updated: 2026-03-12
Depends on: `052-gpui-selection-feedback-and-date-time-primitives-baseline.md`

## Purpose

Freeze the next GPUI primitive tranche that closes the remaining foundation
gap before composite parity work. This baseline adds overlay, disclosure,
navigation, and menu semantics to `poodle-gpui-primitives` so later GPUI
composites and workstation shells inherit one explicit native posture instead
of inventing their own dismissal, selection, or layered-surface models.

## Package Rule

The `g04.006` tranche extends `poodle-gpui-primitives` with:

- `AccordionSpec`
- `CollapsibleSpec`
- `DialogSpec`
- `DrawerSpec`
- `PopoverSpec`
- `TooltipSpec`
- `MenuSpec`
- `ContextMenuSpec`
- `TabsSpec`
- `NavigationMenuSpec`
- `MenubarSpec`
- `TabStripSpec`

These exports remain part of the same preview-channel public-intent Rust crate
as the earlier structural, form-foundation, and selection/date tranches.

## Contract Coverage Rule

The crate must stay aligned to the existing foundation contracts for:

- `accordion`
- `collapsible`
- `dialog`
- `drawer`
- `popover`
- `tooltip`
- `menu`
- `context-menu`
- `tabs`
- `navigation-menu`
- `menubar`
- `tab-strip`

## Overlay And Disclosure Rule

This baseline freezes the shared posture that later GPUI composites must reuse:

- controlled and uncontrolled open state
- explicit modal versus non-modal layer ownership
- explicit outside-interaction and escape dismissal posture
- explicit alert-dialog versus general-dialog semantics
- disclosure models for single or multiple expanded sections

Later composites should consume these semantics rather than introducing
runtime-local overlay and disclosure meaning.

## Navigation And Menu Rule

This tranche also freezes the shared posture for navigational selection and
menu-like command surfaces:

- active tab or tab-strip value selection
- automatic versus manual tab activation
- persistent menu-bar ownership of top-level command groups
- navigation-menu ownership of active top-level disclosure content
- menu and context-menu entry classification using action, checkbox, radio,
  and separator item kinds

This keeps the GPUI surface aligned with the Svelte contract layer even before
full mounted native widgets exist for every family.

## Runtime Honesty Rule

This tranche remains spec-first and honest about current depth:

- open-state, dismissal, item classification, and token-backed layer roles are
  explicit
- mounted focus-scope, hover timing, outside-interaction wiring, submenu
  choreography, and complete keyboard proof still belong to later `g04`
  milestones

The repo may expose these primitives as contract-backed GPUI specs before each
one is rendered by a fully mounted native control implementation.

## Token Rule

Overlay, disclosure, navigation, and menu primitives must continue resolving
from `poodle-gpui-tokens` for at least:

- elevated and overlay background roles
- dialog and overlay elevation roles
- accent and focus appearance semantics
- panel, rule, and subtle border roles
- spacing and radius roles shared with earlier primitive tranches

## Native Delta Rule

GPUI implementations should imitate the same UI as Svelte where the UI belongs
to Poodle, but this baseline keeps genuine native deltas explicit:

- hover-triggered patterns must not assume browser hover behavior maps
  one-to-one to native pointer systems
- outside-interaction handling must stay explicit rather than inferred from DOM
  event bubbling
- focus-scope and menu or submenu traversal must be documented as native
  runtime work, not silently assumed from spec-level state

## Seed Evidence

- `packages/gpui/overlay-navigation-menu-baseline.json`
- `packages/gpui/primitives/README.md`
- `packages/gpui/primitives/src/lib.rs`
- `packages/gpui/primitives/src/accordion.rs`
- `packages/gpui/primitives/src/collapsible.rs`
- `packages/gpui/primitives/src/dialog.rs`
- `packages/gpui/primitives/src/drawer.rs`
- `packages/gpui/primitives/src/popover.rs`
- `packages/gpui/primitives/src/tooltip.rs`
- `packages/gpui/primitives/src/menu.rs`
- `packages/gpui/primitives/src/context_menu.rs`
- `packages/gpui/primitives/src/tabs.rs`
- `packages/gpui/primitives/src/navigation_menu.rs`
- `packages/gpui/primitives/src/menubar.rs`
- `packages/gpui/primitives/src/tab_strip.rs`
