# Navigation Menu

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `NavigationMenu`
- Layer: `foundation`
- Summary: a persistent navigation control that owns top-level nav triggers and
  optional associated viewport content
- In scope: top-level navigation items, active open state, associated viewport
  surface, keyboard movement across triggers
- Out of scope: routing, breadcrumbs, shell-specific sidebars, workstation
  panel systems

## 2. Anatomy

```text
[Root]
  ├── [Navigation List]
  │     └── [Navigation Trigger...]
  └── [Viewport]
```

## 3. Props And Inputs

- `value`: `string | null`
- `defaultValue`: `string | null`
- `items`: `Array<{ value: string; label: string; isDisabled?: boolean; description?: string }>`
- `ariaLabel`: `string | null`

## 4. States

- closed
- open item
- disabled item

## 5. Events

- `onValueChange`

## 6. Accessibility

- role: navigation landmark with reachable trigger controls
- required semantics: accessible nav label, current open trigger state, stable
  trigger-to-viewport relationship
- keyboard: left and right movement across triggers, home/end bounds, enter,
  space, or arrow-down opens the associated viewport, escape closes

## 7. Layout

- nav triggers own the persistent row or cluster
- viewport content sits below or adjacent according to host layout policy

## 8. Token Usage

- navigation trigger, surface, border, text, focus, and elevation roles

## 9. Svelte Notes

- public contract owns active item semantics, not routing internals
- viewport content may be slot-driven and host-owned

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::navigation_menu`

## 11. Parity Checklist

- [ ] active-item and open-state semantics match
- [ ] trigger keyboard movement matches
- [ ] viewport relationship and dismissal posture match

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact viewport animation may differ | overlay and motion internals differ | allowed | keep nav meaning and focus rules strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: catalog nav bars, docs-site nav clusters, product
  section menus

## Next Task

Use `NavigationMenu` for persistent top-level navigation disclosure, and keep
route ownership or shell-specific nav structure in higher layers.
