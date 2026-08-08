# 002 - Anchored Overlays

Status: active
Owner: Poodle core
Applies to: every component whose contract has an anchored surface

## Why This Rule Exists

An overlay anchored to an inline trigger — a picker panel, a menu, a tooltip —
used to be a positioned child of that trigger. That is correct only while no
ancestor interferes, and in a real application one always does:

- an ancestor with `overflow: hidden`, `auto` or `scroll` clips the surface, and
  no `z-index` can lift it out
- an ancestor with `transform`, `filter`, `contain` or `backdrop-filter`
  establishes a containing block, which traps even `position: fixed`
- an ancestor that opens a stacking context caps the surface's paint order
  against its siblings, whatever the surface's own `z-index`

All three fail the same way — the overlay is cut off or painted underneath —
and all three are invisible from inside the component. So the library does not
try: an anchored surface leaves its trigger's subtree entirely.

## The Rule

Every anchored overlay surface **must**:

1. **Portal to the theme root.** The mount point is the explicit
   `[data-poodle-theme-root]` when the host declares one, otherwise the nearest
   `[data-theme]` ancestor, never `<html>`, falling back to `<body>`. Mounting
   inside the theme scope keeps token inheritance intact.
2. **Be positioned in viewport coordinates**, by the shared collision-aware
   resolver — never by offsets relative to the trigger. The surface stylesheet
   describes how the surface *looks*; it declares no `top` / `left` / `bottom` /
   `right` and no `position`.
3. **Track its anchor.** Reposition on scroll of any scrollable ancestor and on
   resize of the window, the anchor or the surface.
4. **Hide when its anchor is not visible.** When the anchor scrolls out of its
   own clipping ancestor the surface takes `data-anchor-hidden="true"`, so it
   never floats detached over unrelated content.
5. **Widen its dismiss layer.** The surface is no longer a descendant of the
   trigger, so `contains` must ask both (`layerContains`). Missing this makes
   the first click inside the surface read as an outside interaction.
6. **Carry its own presentation attributes.** A descendant selector that crosses
   the trigger/surface boundary (`.poodle-x[data-density="compact"]
   .poodle-x__surface`) stops matching once the surface is portalled. Put
   `data-size` / `data-density` / `data-variant` on the surface itself and scope
   the rule to it.

## Implementation

| Layer | Surface |
|-------|---------|
| `@inflatable-cookie/poodle-core` | `resolveOverlayPosition`, `resolveClipRect`, `isAnchorClipped`, `observeAnchorMovement`, `resolvePortalTarget`, `pointAnchor`, `layerContains` |
| `@inflatable-cookie/poodle-svelte` | `anchored` action (`use:anchored={{ anchor, placement, offset }}`) |
| `@inflatable-cookie/poodle-react` | `<AnchoredSurface anchor={…} placement={…} offset={…}>` |
| `@inflatable-cookie/poodle-core/styles` | `anchored-surface.css` — the shared `position: fixed` shell and the `data-anchor-hidden` rule |

Both framework primitives take the same options, so a contract describes the
behaviour once:

- `anchor` — the trigger element, or a **virtual anchor** (`pointAnchor(x, y)`)
  for pointer-positioned overlays such as context menus
- `placement` — the *requested* placement; the resolver may flip or shift it
- `offset` — gap between anchor and surface, in px
- `matchWidth` / `minWidth` — size the surface to the anchor, for listbox-style
  pickers that used to rely on an absolute inset
- `onPlacement` — reports the placement that survived collision resolution.
  Supplying it hands the consumer ownership of `data-placement`, for components
  that publish a coarser value there (`top` / `above`) than the resolver's

## Public Geometry Observation

Built-in surface elements remain private. A consumer that coordinates an
independent viewport may opt into immutable geometry changes through the
owning component. It must not query Poodle classes, roles, ids, portal targets,
or elements.

```ts
interface OverlayViewportRect {
  x: number;
  y: number;
  width: number;
  height: number;
  top: number;
  right: number;
  bottom: number;
  left: number;
}

interface OverlaySurfaceGeometry {
  surfaceId: string;
  rect: OverlayViewportRect;
  placement: OverlayPlacement | null;
  visible: boolean;
}

type OverlaySurfaceGeometryChange =
  | { type: "upsert"; surface: OverlaySurfaceGeometry }
  | { type: "remove"; surfaceId: string };
```

- Bounds use viewport CSS pixels and copied numbers, never `DOMRect`.
- `surfaceId` is opaque and stable for one mounted surface lifetime only.
- `upsert` follows initial positioning and every bounds, placement, or
  visibility change. Placement deduplication must not swallow geometry moves.
- `visible` is false when the anchor is hidden or the surface has zero area.
- `remove` fires once before unmount or destruction.
- One owning-component callback relays every surface it owns. A Menu includes
  nested submenu surfaces under independent ids.
- Server rendering emits no changes.

`Popover` and `Menu` expose this as `onSurfaceGeometryChange`. Svelte and React
use the same payload. This is an explicit per-component subscription, not a
global registry: opening an unrelated Tooltip, Select, or picker must not
silently enter a consumer's coordination policy.

The callback is web-only but host-neutral. Native renderers do not expose CSS
viewport geometry; browser elements, selectors, Tauri types, and consumer
policy do not enter the contract.

## Data Attributes

| Attribute | On | Meaning |
|-----------|----|---------|
| `data-poodle-anchored="true"` | surface | portalled and viewport-positioned; set by the primitive |
| `data-placement` | surface | the placement in effect after collision resolution |
| `data-anchor-hidden="true"` | surface | the anchor has scrolled out of its clipping ancestor |

## Exemptions

- **Modal layers** — `Dialog`, `Drawer`, `AlertDialog`, `CommandPalette`,
  `ToastHost`. These are viewport-anchored, not trigger-anchored; they portal
  (or are already fixed to the viewport) but have no anchor to track.
- **Nested submenu flyouts.** A submenu is positioned against its own row inside
  an already-portalled parent surface, so it inherits correct stacking and stays
  in flow. Passing no `anchor` leaves it exactly where it is.
- **In-flow disclosure panels** — `NavigationMenu`'s viewport, `Accordion`,
  `Collapsible`. These take space in the layout and are not overlays.

## Testing

A component with an anchored surface asserts, in both frameworks:

- the surface is not reachable from the render container, and
  `surface.closest('.poodle-<component>')` is null
- `data-poodle-anchored` is `"true"`
- a pointer interaction inside the surface does not dismiss the layer
- an opted-in observer receives copied viewport geometry after positioning,
  movement, visibility changes, and teardown

Because the surface leaves the container, tests reach it through the trigger's
`aria-controls` rather than a container query — which also keeps concurrently
rendered instances apart.

## Layering: Escaping A Clipping Ancestor Costs You The Stacking Context

Portalling to the theme root is what stops a scrolling or transformed ancestor
clipping a surface. It also takes the surface *out of* whatever opened it, and
that has a price nobody paid until a popover was opened from inside a dialog:

- `overlay.z.menu` is **400**, `overlay.z.dialog` is **800**
- portalled to the same root, the popover is no longer a descendant of the
  dialog, so nothing lifts it — it renders *behind* the modal that opened it,
  visible through the scrim and impossible to click

Raising the menu token above the dialog token only moves the bug: a page-level
menu would then float over an unrelated modal.

**So the layer is read from the DOM when the surface opens.** The trigger is
still inside the dialog at that moment, so walking its ancestors finds the
dialog's stacking context; `resolveLayerZIndex` returns one above the highest it
finds, and the surface keeps its own token when nothing outranks it. That
composes to any depth — a popover in a dialog in a drawer — with no context
plumbing.

Measured in a browser, on the shipped stylesheet: dialog `800`, popover `801`.
With the lift removed, the popover reports `400`, which is the reported bug.

**Every anchored surface must go through the primitive for this to hold.** A
component that portals itself and sets its own `z-index` from a token will
reproduce the bug exactly.
