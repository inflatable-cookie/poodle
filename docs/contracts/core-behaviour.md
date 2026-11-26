# Supported core behaviour for custom controls

Import the behaviours below from `@inflatable-cookie/poodle-core`. The named
transition types and the types in the listed function signatures are part of
this subset. The [drag-and-drop integration](#drag-and-drop) and
[preview theme resolution](#preview-theme-resolution) are also listed here.
Other root exports are internal to Poodle components. The package subpaths
for tokens, styles and icons have their own contracts.

## Portable functions

These functions need no DOM or cleanup. Transitions take the current state or
context and an event, and return the next state or context and ordered effect
intents. The caller supplies the initial state and context, dispatches events
and performs any effects.

| Export | Inputs and defaults | Result and effects |
| --- | --- | --- |
| `firstEnabledIndex` | Ordered array whose items may have `disabled` or `isDisabled`. Omitted flags are false. | First enabled index, or `-1` when none exists. |
| `findNextEnabledIndex` | Same array, a start index and direction (`1` or `-1`). No default. Use `-1` to start forward before the first item, or `0` to start backward from the end. | Wraps and skips either disabled flag. Returns the start index when all items are disabled and `-1` for an empty array. Check the returned item before using it as an active option. |
| `singleSelectTransition` | `SingleSelectContext` takes `value` (`string \| null`), `options` (`SelectOption[]`, optional `disabled` per option) and `disabled` for the control. `SingleSelectEvent` is `SELECT(value)` or `SET_VALUE(value)`. No context defaults. | `SELECT` changes the value and emits `emitValueChange` for a different enabled option in an enabled control. `SET_VALUE` synchronizes any supplied value without effects. |
| `mergeTokens` | Current and incoming string arrays and a required `dedupe` flag. | New array in input order. With dedupe, keeps the first occurrence of each exact string. |
| `splitTokenInput` | Raw string, split regex or `null`, and a string of terminating separator characters. No default. | `null` when splitting is off or no token is complete. Otherwise returns `TokenSplit` with `committed` parts and an unfinished `remainder`. A trailing separator clears the remainder. Caller trims and validates parts. |
| `tokenBackspaceRemoves` | Input value and token count. No default. | True when the input is empty and the count is positive. Caller handles the key, chooses a token and removes it. |
| `disclosureTransition` | `DisclosureContext` takes `open` and `disabled`. `DisclosureEvent` is `TOGGLE` or `SET_OPEN(open)`. No defaults. | An enabled toggle flips `open` and emits `emitOpenChange`. `SET_OPEN` synchronizes the boolean silently, including when disabled. |
| `hoverTransition` | `HoverState` is `closed`, `opening`, `open` or `closing`. `HoverContext` requires `openDelayMs` and `closeDelayMs`. `HoverEvent` is `ENTER`, `LEAVE`, `TIMER_FIRE`, `DISMISS` or `SET_OPEN(open)`. No defaults. | Returns timer intents (`clearTimer`, `startTimer(ms)`) and `emitOpenChange`. Run timer effects in order and dispatch `TIMER_FIRE` when the current timer fires. A nonpositive close delay closes immediately. `SET_OPEN` clears pending timing without announcing a change. Cancel the caller's timer on teardown. |
| `popoverTransition` | `PopoverState` is `closed` or `open`. `PopoverContext` requires `disabled`, `dismissOnOutsideInteract` and `initialFocus` (`first-focusable`, `content` or `none`). `PopoverEvent` is `TOGGLE`, `OPEN`, `CLOSE`, `ESCAPE` or `OUTSIDE_INTERACT`. No defaults. | Disabled controls stay put. An open emits `emitOpenChange(true)` then `focusOnOpen` with the supplied strategy. A close emits `emitOpenChange(false)` then `restoreTriggerFocus`. Outside interaction closes only when enabled by the context. The caller performs focus and dismissal wiring. |
| `resolveOverlayPosition` | Anchor rect, overlay rect, preferred `OverlayPlacement`, a `ViewportSize` (`{ width, height }`), and a gap in pixels (default `6`). | Returns `top`, `left` and the actual `placement` in viewport coordinates after collision resolution. Caller measures and applies the result. |

## Preview option and selection policies

`moveActiveOption(items, activeValue, move, wrap = true)` takes ordered
`ActiveOption` items with stable, unique string values and an optional
`disabled` flag that defaults to false. `ActiveOptionMove` is `next`,
`previous`, `first` or `last`. From a null, missing or disabled value,
next starts at the first enabled option and previous starts at the last.
First and last go straight to the corresponding enabled end. Pass
`wrap = false` to stop at the ends.

`reconcileActiveOption(previousItems, items, activeValue)` takes both lists
when results change. It keeps the current value if it is still enabled.
Otherwise it picks the enabled option nearest the old index in the new list,
preferring the later index on ties. An unknown old value falls back to the
first enabled option. A null value stays null. Both functions return null
when no option is enabled. The caller supplies the order, decides when to
reset or reconcile, and wires keyboard intents, focus and ARIA.

`selectionTransition(selected, event, options?)` takes an ordered array
of string IDs or records. `SelectionOptions` defaults to `mode: "single"`
and an unlimited `max` in multiple mode. Supply `key: (item) => string` to
identify records by a stable key. Without a key, strings compare by value and
records by reference. Use a nonnegative whole number for `max`. Zero
prevents additions.

`SelectionEvent` supports `SELECT` and `TOGGLE` with an item and an optional
`disabled` flag (defaults to false), `REMOVE` with an item, `CLEAR`, and
`REPLACE` with an array of items. Selecting appends in multiple mode or
replaces the current choice in single mode. Selecting an existing key keeps
the saved item. Toggling an existing key removes it. Disabled candidates
ignore select and toggle. Remove matches by key, including for items absent
from the current results. Replace keeps the first item per key in input order
up to the mode and count limit. Start with an empty array or seed with
replace. Select, toggle, remove and clear return the same array when the
selection is unchanged. The caller stores the result, supplies candidate
eligibility, and projects which options to show. Both policies are pure and
emit no effects.

## Preview theme resolution

`resolveTheme(settings?)` returns 44 RGBA roles (integer channels from 0
to 255) and a report of clamped inputs, adjustments, warnings and notes.
It needs no DOM. Settings default to dark, mono base at 0.25 blend, mono
accent at 0 blend, contrast 1 and even tint on. Colours accept three- or
six-digit sRGB hex or normalized RGB triples.

`themePresets` is preview data. Each tuple contains the id, name, scheme,
base, base blend, accent, accent blend, contrast and an optional
`[base, baseBlend]` override for even tint. See the
[theme resolution architecture](../knowledge/architecture/015-theme-resolution.md)
for fit targets and the Rust mirror.

## Browser helpers

| Export | Inputs and defaults | Effect and cleanup |
| --- | --- | --- |
| `registerDismissLayer` | `DismissLayer` with `contains(target)`, `onDismiss(reason)`, required `dismissOnOutsideInteract` and optional `hostElement`. Without a host, parentage defaults to the current top layer. | Registers an open layer and installs shared document listeners as needed. Escape requests dismissal of the innermost layer. Outside mouse down requests dismissal of opted-in layers outside the hit and its ancestors. The caller closes the surface and invokes the returned unregister function on close or unmount. |
| `layerContains` | Target `Node` and any number of elements. Null or undefined elements are ignored. | Tests DOM containment across disjoint roots, such as a trigger and a portalled surface. No cleanup. |
| `getFocusableElements` | Root `HTMLElement` or `null`. No default. | Returns matching focusable descendants in DOM order. Excludes elements with a `hidden` attribute or a null `offsetParent`. A null root returns an empty array. No cleanup. |
| `trapFocusKeydown` | Modal surface element or `null`, and the originating keyboard event. No defaults. | Handles Tab at the boundaries using `getFocusableElements`. Wraps focus or focuses the surface if there are no candidates. The surface needs to be programmatically focusable for that fallback. Call from the surface's keydown handler. No helper cleanup. |
| `pointAnchor` | Viewport `x` and `y`, optional DOM context element (default `null`). | Constructs a zero-size `VirtualAnchor`. No DOM reads or cleanup. |
| `observeAnchorMovement` | `AnchorTarget \| null` (element or virtual anchor), surface element or `null`, and `onUpdate` callback. No defaults. | Calls back on relevant ancestor or window scroll, window resize, and anchor or surface resize where `ResizeObserver` is available. Call the returned teardown when an element changes or unmounts. It is safe to call twice and is a no-op when there is no window or anchor. |

On Svelte, the public [`anchored` action](002-anchored-overlays.md) handles
portal placement, measurement and teardown. Use `layerContains` when
registering a dismissable surface that the action has portalled.

## Drag and drop

The [drag-and-drop substrate contract](../knowledge/specs/069-dependable-drag-and-drop-substrate.md)
owns the drag API, including host and file bridges. For a custom web surface,
`createDragDropController(options?)` takes a `DragDropControllerOptions` object
(default `{}`). The DOM controller binds pointer, keyboard and drag events
when connected to a root. It routes commit and announcement events through the
host callbacks. Unregister retired handles, disconnect when releasing the root,
and destroy the controller when disposing it. The substrate contract also
covers the inputs, defaults and lifecycle for the web and native transports.

## Example

Build an [async tag combobox](../guides/015-async-tag-combobox.md) using
active-option navigation, persistent selection, token removal, dismissal and
the public Svelte anchor action.
