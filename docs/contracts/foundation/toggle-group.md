# Toggle Group

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `ToggleGroup`
- Layer: `foundation`
- Summary: a grouped toggle surface for single-select or multi-select utility
  actions
- In scope: single and multiple selection, grouped labeling, disabled items
- Out of scope: tab-panel navigation and segmented shell chrome

## 2. Anatomy

```text
[Root]
  └── [Toggle Item...]
```

## 3. Props And Inputs

- `value`: `string | string[] | null`
- `defaultValue`: `string | string[] | null`
- `options`: grouped toggle options
- `selectionMode`: `"single" | "multiple"`
- `isDisabled`: `boolean`
- `ariaLabel`: `string | null`

## 4. States

- unselected
- selected
- focus
- disabled

## 5. Events

- `onValueChange`

## 6. Accessibility

- role: `radiogroup` in single mode, grouped toggle buttons in multi mode
- required semantics: per-item selected state
- keyboard: `Tab` enters or exits the group; directional movement may be added
  later if the group adopts roving focus

## 7. Layout

- items may wrap
- group spacing stays compact and utility-oriented

## 8. Token Usage

- toggle-family background, selected, focus, and disabled roles

## 9. Svelte Notes

- may compose `Toggle`-like internals, but the grouped selection API remains
  Pug-owned

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::toggle_group`

## 11. Parity Checklist

- [ ] single vs multiple meaning matches
- [ ] selected-state exposure matches
- [ ] disabled-item behavior matches

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| focus-management details may differ initially | grouped utility controls may evolve | allowed | tighten during parity review |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: filters, utility bars, formatting controls

## Next Task

Keep `ToggleGroup` semantically distinct from `SegmentedControl` and `Tabs`
even when the visual density is similar.
