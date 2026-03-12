# Combobox

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `Combobox`
- Layer: `foundation`
- Summary: a queryable single-select control that combines text input with a
  suggestion list
- In scope: query text, filtered suggestion list, selection, open state
- Out of scope: multi-select tagging, command-palette ranking, complex relation
  picking

## 2. Anatomy

```text
[Root]
  ├── [Input]
  └── [Suggestion List]
        └── [Option...]
```

## 3. Props And Inputs

- `value`: `string | null`
- `defaultValue`: `string | null`
- `options`: combobox options
- `placeholder`: `string | null`
- `isDisabled`: `boolean`
- `ariaLabel`: `string | null`

## 4. States

- closed
- open
- highlighted option
- selected option
- disabled

## 5. Events

- `onValueChange`
- `onQueryChange`
- `onOpenChange`

## 6. Accessibility

- role: combobox with listbox suggestions
- required semantics: expanded state, query input semantics, selected option
- keyboard: arrow navigation, enter commit, escape dismiss

## 7. Layout

- input owns width
- suggestion list anchors below the field unless placement rules later expand

## 8. Token Usage

- text-input, listbox, option-highlight, and overlay roles

## 9. Svelte Notes

- may compose input plus listbox behavior without leaking substrate-specific
  compound APIs

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::combobox`

## 11. Parity Checklist

- [ ] query and selected-value semantics match
- [ ] open and dismiss behavior matches
- [ ] option navigation and commit behavior matches

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| filtering strategy may evolve | ranking and matching internals are implementation details | allowed | keep commit semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: searchable selects, token pickers, compact asset lookup

## Next Task

Keep `Combobox` distinct from workstation `CommandPalette` semantics and from
relation-picker composites that own richer workflow behavior.
