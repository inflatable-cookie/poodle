# g07.005 — GPUI Overlay, Disclosure, Navigation, and Menu Primitives

Status: Completed
Updated: 2026-03-14

## Objective

Implement RenderComponent for overlay, disclosure, navigation, and menu primitives.

## Components (13)

AccordionSpec, CollapsibleSpec, DialogSpec, DrawerSpec, PopoverSpec, TooltipSpec,
HoverCardSpec, MenuSpec, ContextMenuSpec, TabsSpec, TabStripSpec, NavigationMenuSpec,
MenubarSpec

## Implementation

New module `render_overlay.rs` with 13 `RenderComponent<Spec>` implementations.
Dialog resolves backdrop fill token.

## Tests

13 new tests (67 total).

## Verification

- [x] All 13 overlay/navigation primitives have RenderComponent implementations
- [x] AdapterManifest updated
- [x] 67 tests passing
