# g08.008 — Overlay Primitives

Status: Completed
Updated: 2026-03-14

## Objective

Implement `RenderComponent` for 13 overlay and navigation primitive specs.

## Deliverables

### RenderComponent implementations (render_overlay.rs)

| Spec | Node ID | Widget | Notes |
|------|---------|--------|-------|
| DialogSpec | `dialog` | Panel | Modal overlay |
| DrawerSpec | `drawer` | Panel | Edge-anchored overlay |
| PopoverSpec | `popover` | Panel | Positioned overlay |
| MenuSpec | `menu` | List | Dropdown menu |
| TooltipSpec | `tooltip` | Label | Hover hint |
| TabsSpec | `tabs` | Panel | Tab navigation |
| AccordionSpec | `accordion` | Panel | Collapsible sections |
| CollapsibleSpec | `collapsible` | Panel | Single collapsible |
| HoverCardSpec | `hover-card` | Panel | Hover preview card |
| ContextMenuSpec | `context-menu` | List | Right-click menu |
| TabStripSpec | `tab-strip` | Panel | Tab button row |
| NavigationMenuSpec | `navigation-menu` | List | Navigation tree |
| MenubarSpec | `menubar` | Panel | Horizontal menu bar |

### Test coverage

13 tests verifying spec_type and widget_kind propagation.

## Verification

```
cargo test — 13 overlay tests passing
```
