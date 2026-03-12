# Hover Card

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `HoverCard`
- Layer: `foundation`
- Summary: a richer hover- or focus-invoked preview surface anchored to a
  trigger
- In scope: delayed open and close behavior, anchored preview surface, trigger
  and surface hover continuity
- Out of scope: command menus, click-owned popovers, modal workflows, complex
  form editing

## 2. Anatomy

```text
[Root]
  ├── [Trigger]
  └── [Preview Surface]
```

## 3. Props And Inputs

- `open`: `boolean | null`
- `defaultOpen`: `boolean`
- `openDelayMs`: `number`
- `closeDelayMs`: `number`
- `placement`: overlay placement
- `ariaLabel`: `string | null`

## 4. States

- closed
- open
- dismissing

## 5. Events

- `onOpenChange`

## 6. Accessibility

- role: anchored dialog-like preview surface paired with a reachable trigger
- required semantics: trigger reachability, dismiss on escape, preview content
  remaining reachable while open
- keyboard: focus-in may open, escape dismisses, tab order must continue
  through trigger and content naturally

## 7. Layout

- trigger owns anchor position
- preview surface floats adjacent to the trigger and may use the shared overlay
  layer

## 8. Token Usage

- overlay surface, elevation, border, text, and focus roles

## 9. Svelte Notes

- may share substrate patterns with tooltip or popover internally, but the
  public surface owns hover-delay and richer preview semantics

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::hover_card`

## 11. Parity Checklist

- [ ] open and dismiss timing semantics remain equivalent enough for review
- [ ] trigger and preview reachability match
- [ ] hover continuity and escape dismissal match

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact delay timing may differ slightly | runtime timer behavior differs | allowed | keep open and dismiss meaning strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: identity previews, asset summaries, compact profile
  surfaces

## Next Task

Use `HoverCard` for richer non-modal previews, and keep click-owned or action
surfaces in `Popover`, `Menu`, or higher workflow layers.
