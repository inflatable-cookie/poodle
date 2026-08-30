# Tabs Drag Host-Bridge Sequencing

Status: resolved — option 1 approved and promoted
Captured: 2026-08-30
Source: g16.023 dispatch-readiness check after PR #101

## Finding

`g16.023` cannot currently satisfy all of its boundaries. Tabs still exposes
`onDragPrepare(value, PointerEvent)`, `onDragStart(value, DragEvent)`, and
`onDragEnd(value, DragEvent)`. DockRegion consumes those callbacks to prepare
and start its external/cross-window drag path.

The card also requires Tabs to stop using native HTML drag authority, retain no
dual controller, preserve public callbacks, and leave DockRegion and the host
bridge untouched. Once Tabs reorder moves to the Pointer Events substrate,
there is no honest native `DragEvent` to pass to those callbacks. Keeping HTML
drag solely to manufacture the events preserves the competing drag system the
programme exists to remove.

## Options

1. Narrow `g16.023` to EditableList. It becomes the first simple-reorder
   component proof. Defer Tabs until `g16.026`, where its host-preparation seam
   and DockRegion can migrate together onto the opaque cross-window bridge.
2. Pull a new semantic Tabs host-preparation callback into `g16.023`. This is a
   public API and sequencing change before the paired host bridge exists.
3. Keep the native callbacks and HTML drag alongside the substrate. This
   violates the no-dual-controller and clean-migration decisions and is not
   recommended.

## Decision

The operator approved option 1 on 2026-08-30. `g16.023` is narrowed to
EditableList. Tabs moves to `g16.026`, where its actual DockRegion consumer and
the opaque cross-window bridge migrate in one bounded lane.

## Promotion Route

Promoted into spec 069, `g16.023`, `g16.026`, and the g16/front-door runway.
This note is historical context, not an open gate.
