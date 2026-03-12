# Toggle

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `Toggle`
- Layer: `foundation`
- Summary: a pressable action control that exposes a persistent pressed or
  selected state
- In scope: pressed state, disabled state, icon-only or text usage
- Out of scope: mutually exclusive grouped selection and toolbar roving-focus

## 2. Anatomy

```text
[Root]
  └── [Content]
```

## 3. Props And Inputs

- `isPressed`: `boolean | null`
- `defaultPressed`: `boolean`
- `variant`: `"primary" | "secondary" | "ghost"`
- `size`: `"sm" | "md" | "lg"`
- `isDisabled`: `boolean`
- `ariaLabel`: `string | null`

## 4. States

- default
- pressed
- focus
- disabled

## 5. Events

- `onPressedChange`

## 6. Accessibility

- role: `button`
- required semantics: `aria-pressed`
- keyboard: `Enter` and `Space` activate

## 7. Layout

- control height follows shared control-size tokens
- content may be icon-only or text-backed

## 8. Token Usage

- button-family background, border, focus, and disabled roles

## 9. Svelte Notes

- may wrap a button substrate, but Pug owns pressed-state semantics

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::toggle`

## 11. Parity Checklist

- [ ] pressed semantics match
- [ ] keyboard activation matches
- [ ] icon-only naming rules match

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: mode toggles, formatting toggles, compact shell actions

## Next Task

Use `Toggle` for standalone pressed-state controls and `ToggleGroup` when
selection becomes grouped or multi-select.
