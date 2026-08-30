# Drag-And-Drop Public Migration Boundary

Status: resolved — clean migration approved and promoted into spec 069 and g16.026
Captured: 2026-08-28
Source: g16 post-audit runway compilation

## Finding

The existing bespoke drag machinery is not wholly private. The root
`@inflatable-cookie/poodle-core` export surface includes the Tabs reorder
helpers from `tabs-reorder.ts` and the complete DockRegion external-drag
controller/types from `dock-external-drag.ts`. Svelte and React also re-export
the DockRegion types.

Architecture 011/spec 069 require component-local controllers and global side
channels to disappear after their replacement passes. Keeping the old exports
as aliases or silent adapters would violate the pre-1.0 no-shim rule. Removing
or replacing them is a breaking public migration and cannot be inferred from
approval of the new substrate alone.

## Decision Options Considered

Before the Tabs/EditableList or DockRegion migration cards become ready,
choose one explicit pre-1.0 boundary:

1. remove the obsolete root exports and migrate in-repo callers in the same
   release tranche; or
2. retain any helper that remains a genuine first-class API, give it a defined
   role on top of the new substrate, and do not describe it as compatibility.

The recommendation is option 1 for the event-shaped Tabs helpers. Reassess the
DockRegion types against the new host bridge before deciding whether any of
their concepts remain first-class under new names.

## Current Export Assessment

The two surfaces are not equally valuable:

- `tabs-reorder.ts` exports `ReorderState` plus `handleDragStart`,
  `handleDragOver`, and `handleDrop`. These helpers accept DOM `DragEvent`,
  write index strings into `DataTransfer`, and let `text/plain` stand in for a
  Poodle protocol. They are leaked component implementation, not a durable
  custom-surface capability. `applyReorder` is separately exported by the
  semantic Tabs module and remains useful without this file.
- `dock-external-drag.ts` contains a real consumer need — asynchronous host
  preparation and guaranteed cancellation before a native drag begins — but
  expresses it entirely through `PointerEvent`, `DragEvent`, `DataTransfer`, a
  window-global `dockPanelDragSession`, and DockRegion-specific panel/edge
  types. Its lifecycle distinctions should inform the new host bridge; its
  public types and controller should not survive as the cross-runtime API.
- `PanelDragData.sourceZone` is optional only as an older-build fallback. That
  compatibility rule also conflicts with the pre-1.0 clean-migration posture.

## Approved Clean Migration

The operator approved one explicit break across the later migration cards on
2026-08-28:

1. In `g16.026`, migrate Tabs' reorder and host-preparation seams alongside
   DockRegion, then delete the public DOM-shaped Tabs reorder helpers and both
   framework re-export files. Retain `applyReorder` only as the existing pure
   semantic helper from `tabs.ts`; do not retain an alias for the old module.
2. In `g16.026`, delete every `DockExternalDrag*` / `DockExternalDrop*` export,
   `createDockExternalDragController`, and `dockPanelDragSession` after
   DockRegion passes on the new opaque host bridge.
3. Preserve the useful prepare/cancel/revalidate lifecycle as first-class new
   substrate/host-bridge concepts, not as compatibility names or DOM-shaped
   wrappers. Fix their exact public names only after the landed kernel and GPUI
   substrate make the paired boundary concrete.
4. Preserve `onPanelDrop`'s semantic callback purpose, but make
   `PanelDragData.sourceZone` required when its migration is compiled. Remove
   the older-build fallback rather than carrying it into the new protocol.

This removes implementation leakage without deleting the consumer capability
Longhorn and Loophole need. It also keeps the breaking change in the cards that
actually land the replacement, rather than removing exports ahead of a usable
host bridge.

## Guardrail

Do not add aliases, deprecation wrappers, dual controllers, or silent fallback
from the new substrate to the old global session. The semantic-kernel and
custom-surface foundation cards may proceed because they do not remove these
exports.

## Promotion Route

The decision is authoritative in spec 069 and `g16.026`. `g16.023` now proves
the simple-reorder substrate through EditableList without touching Tabs or its
DockRegion consumers. The current component contracts and exports remain
truthful until the replacement card lands. The migration card removes its old
public surface only after the mounted replacement passes. This note is
historical context, not an open gate.
