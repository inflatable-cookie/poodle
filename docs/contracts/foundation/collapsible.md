# Collapsible

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `Collapsible`
- Layer: `foundation`
- Summary: a single disclosure container with a trigger and revealable content
- In scope: controlled or uncontrolled open state, heading copy, trigger
  affordance, and inline reveal posture
- Out of scope: grouped selection logic, routing, or full accordion ownership

## 2. Anatomy

```text
[Collapsible]
  ├── [Trigger]
  └── [Content]
```

## 3. Props And Inputs

- `open`: `boolean | null`
- `defaultOpen`: `boolean`
- `title`: `string | null`
- `description`: `string | null`
- `isDisabled`: `boolean`
- `ariaLabel`: `string | null`

## 4. States

- collapsed
- expanded
- disabled

## 5. Events

- `openChange`

## 6. Accessibility

- trigger exposes `aria-expanded` and `aria-controls`
- content exposes region semantics tied to the trigger
- trigger remains keyboard-operable through button semantics

## 7. Layout

- one trigger row
- content revealed inline beneath the trigger
- indicator communicates open versus closed posture

## 8. Token Usage

- surface, border, spacing, indicator color, focus ring

## 9. Svelte Notes

- may accept trigger slot overrides as long as trigger semantics remain owned by
  the primitive

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::collapsible`

## 11. Parity Checklist

- [ ] open-state semantics match
- [ ] disabled behavior matches
- [ ] trigger and region ownership matches

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| slot-based trigger content may differ by runtime | composition freedom is acceptable | allowed | keep disclosure semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: settings groups, sidebars, drawers, docs notes, compact
  web disclosure surfaces

## Next Task

Use `Collapsible` where a single revealable content block is the real pattern,
and only promote to `Accordion` when grouped disclosure semantics actually
matter.
