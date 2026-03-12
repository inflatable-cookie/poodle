# Accordion

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `Accordion`
- Layer: `foundation`
- Summary: grouped disclosure sections with explicit trigger, expanded state,
  and panel ownership
- In scope: single or multiple expansion, optional collapse-to-none posture,
  keyboard and focusable trigger semantics
- Out of scope: tree navigation, app-specific settings IA, or arbitrary shell
  navigation ownership

## 2. Anatomy

```text
[Accordion]
  ├── [Item]
  │   ├── [Trigger]
  │   └── [Panel]
  └── [...]
```

## 3. Props And Inputs

- `items`: `AccordionItem[]`
- `value`: `string | string[] | null`
- `defaultValue`: `string | string[] | null`
- `selectionMode`: `"single" | "multiple"`
- `isCollapsible`: `boolean`
- `ariaLabel`: `string | null`

## 4. States

- collapsed
- expanded
- disabled item

## 5. Events

- `valueChange`

## 6. Accessibility

- triggers expose `aria-expanded` and `aria-controls`
- expanded panels expose region semantics tied back to their trigger
- keyboard focus remains on trigger buttons while disclosure state changes

## 7. Layout

- stacked item grouping
- visible separation between adjacent items
- panel content expands inline below each trigger

## 8. Token Usage

- panel surface, border, spacing, disclosure indicator, focus ring

## 9. Svelte Notes

- the public API may stay data-driven with slot-based panel rendering rather
  than mirroring a compound substrate surface directly

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::accordion`

## 11. Parity Checklist

- [ ] expansion semantics match
- [ ] disabled-item behavior matches
- [ ] single versus multiple expansion posture matches

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| panel rendering may stay slot-driven in Svelte | runtime composition differs | allowed | keep trigger and expansion semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: settings pages, FAQ surfaces, docs sections, marketing
  or web product disclosure patterns

## Next Task

Use `Accordion` where grouped disclosure is the real semantic pattern instead
of styling unrelated cards or details blocks into fake accordions later.
