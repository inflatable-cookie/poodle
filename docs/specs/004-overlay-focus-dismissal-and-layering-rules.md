# 004 Overlay Focus, Dismissal, And Layering Rules

Status: active
Updated: 2026-03-11
Depends on: `001-token-source-and-artifact-contract.md`, `002-component-contract-template-and-parity-rules.md`, `003-accessibility-and-assistive-technology-baseline.md`

## Purpose

Freeze the cross-runtime rules for overlay behavior before Svelte and GPUI
implementations start diverging on focus, dismissal, or stacking details.

## Core Rule

Overlay behavior is contract-governed surface behavior, not implementation
flavor.

That includes:

- what owns open state,
- where focus goes when the overlay opens,
- whether background content remains interactive,
- how dismissal works,
- where focus returns after close,
- and how the overlay is layered and announced.

## Overlay Classes

The initial overlay family splits into three behavioral classes:

- descriptive overlays: `Tooltip`
- anchored interactive overlays: `Popover`, `Menu`, `ContextMenu`, and
  select/listbox-style surfaces
- blocking overlays: `Dialog` and modal `Drawer`

Each class may vary visually.
Each class may not blur its focus and dismissal semantics.

## Invocation And Ownership Rule

Every overlay contract must define:

- the invoking control or invocation target,
- whether open state is controlled, uncontrolled, or fully internal,
- the semantic relationship between invoker and overlay content,
- and the fallback restoration target if the invoker disappears while the
  overlay is open.

Keyboard-origin invocation is Tier 1 parity, not a web convenience.

That is especially important for:

- menus,
- context menus,
- selects,
- dialogs opened from command surfaces,
- and drawers opened from shell controls.

## Focus Entry Rule

Opening an overlay must place focus according to overlay class:

- `Tooltip`: no focus moves into the overlay
- `Menu` and `ContextMenu`: active-item focus or active-descendant semantics
  move into the menu
- `Popover`: focus may move into the overlay based on documented initial-focus
  policy, but the overlay does not trap focus
- `Dialog` and modal `Drawer`: focus moves inside and becomes trapped until the
  overlay closes

If no meaningful focus target exists, the overlay surface itself must become
the temporary focus anchor where platform semantics allow it.

## Focus Trap Rule

Focus trapping is allowed only when the overlay is modal.

That means:

- required for `Dialog`
- required for `Drawer` when `isModal=true`
- disallowed for `Tooltip`
- disallowed for ordinary `Popover`
- disallowed for `Menu` and `ContextMenu` beyond their internal roving-focus or
  active-item model

Blocking the rest of the UI without a documented modal contract is a bug.

## Dismissal Rule

Every overlay contract must define:

- whether `Escape` dismisses,
- whether outside interaction dismisses,
- whether focus loss dismisses,
- whether selecting an item dismisses,
- and whether dismissal can be vetoed or delayed by the host.

Dismissal semantics must not depend only on pointer interaction.

If pointer and keyboard dismissal differ, the contract must say so explicitly
and explain why.

## Focus Restoration Rule

Closing an overlay must restore focus to:

- the invoking control,
- the invocation target,
- or a documented fallback when the original invoker no longer exists.

Failure to restore focus is a contract bug for:

- `Menu`
- `ContextMenu`
- `Popover` when focus entered the content
- `Dialog`
- `Drawer`
- and select/listbox-style controls

## Layering Rule

Overlay stacking must be driven by semantic token families rather than
hard-coded per-runtime magic numbers.

The current token contract already reserves:

- overlay roles,
- elevation roles,
- and motion roles

for this purpose.

Until the overlay token namespace is more deeply expanded, contracts must still
state the relative layer intent, such as:

- descriptive overlays above base content,
- anchored interactive overlays above their invokers,
- blocking overlays above anchored overlays,
- and urgent transient explanatory surfaces such as tooltips not hiding modal
  controls or stealing focus.

Svelte may realize this with portal order and z-index.
GPUI may realize it with render order or native window/view stacking.
The semantic layer order must stay consistent.

## Positioning And Collision Rule

Anchored overlays must define:

- anchor source,
- preferred placement,
- viewport or shell-boundary collision handling,
- and behavior when the anchor becomes partially or fully unreachable

Allowed collision strategies include:

- flip,
- shift,
- resize,
- or hide

The runtime may choose different internal algorithms.
It may not leave the overlay orphaned or unreachable.

## Announcement Rule

Overlay announcement must match overlay type:

- `Tooltip` uses descriptive relationships, not live-region urgency
- `Menu` and `ContextMenu` expose menu semantics and current-item semantics
- `Popover` uses its content semantics and label, not fake alert behavior
- `Dialog` and modal `Drawer` must announce title, description, and modal
  context

GPUI must implement native announcement or accessible-tree equivalents instead
of assuming visual layering communicates enough.

## GPUI-Specific Rule

GPUI implementations must explicitly own:

- overlay stack bookkeeping,
- background inertness for modal overlays,
- focus handoff and restoration,
- roving-focus or active-item state where the pattern requires it,
- native role/name/state/value exposure for overlay content,
- and announcement of modal or urgent state transitions

Missing GPUI overlay accessibility behavior is not an allowed platform delta.

## Contract Author Checklist

Overlay and navigation contracts must define:

- invoker and overlay relationship
- open-state ownership model
- focus entry rule
- whether focus traps
- dismissal triggers
- focus restoration target
- layer intent and token families used
- collision and reposition behavior
- GPUI-native accessible-tree and announcement expectations

## Seed Evidence

The first contracts that explicitly exercise this baseline are:

- `docs/contracts/components/select.md`
- `docs/contracts/components/tabs.md`
- `docs/contracts/components/tab-strip.md`
- `docs/contracts/components/menu.md`
- `docs/contracts/components/context-menu.md`
- `docs/contracts/components/tooltip.md`
- `docs/contracts/components/popover.md`
- `docs/contracts/components/dialog.md`
- `docs/contracts/components/drawer.md`
