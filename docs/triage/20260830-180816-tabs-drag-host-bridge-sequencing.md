# Tabs Drag Host-Bridge Sequencing

Status: open — operator decision required before g16.023 dispatch
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

## Recommendation

Choose option 1. It preserves a small real-component proof now, keeps Tree and
the native substrate runway moving, and revises Tabs only when its actual
DockRegion consumer can migrate in the same bounded host-bridge lane.

## Promotion Route

After operator approval, revise spec 069's migration order, narrow `g16.023`
to EditableList, and make the Tabs migration explicit in `g16.026`. Then mark
`g16.023` ready and create its worker handoff. Until then it remains blocked.
