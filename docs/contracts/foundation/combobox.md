# Combobox

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Combobox`
- Layer: `foundation`
- Summary: a queryable single-select control that combines a text input with a
  filterable suggestion list overlay
- In scope: query text entry, client-side filtering, suggestion list with
  optional descriptions, selection commit, open/close state, highlighted option
  tracking
- Out of scope: multi-select tagging, command-palette ranking, remote/async
  search, complex relation picking

## 2. Anatomy

```text
[Root .combobox]  <div role="combobox">
  ├── [Input .combobox__input]  <input aria-autocomplete="list">
  └── [List .combobox__list]  <div role="listbox"> (conditional, when open)
        ├── [Option .combobox__option]...  <button role="option">
        │     ├── [Label text]
        │     └── [Description .combobox__description] (optional)
        └── [Empty .combobox__empty] (conditional, when no matches)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | combobox container with ARIA role | position, min-width |
| Input | yes | text query input | border, radius, background, typography, focus ring |
| List | no | suggestion overlay | position, border, radius, background, shadow, padding |
| Option | no | selectable suggestion row | padding, radius, background, color, cursor |
| Description | no | secondary text under option label | color, font-size |
| Empty | no | "no results" message | color, font-size, padding |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string \| null` | `null` | no | controlled selected value (option value string) |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial selected value |
| `options` | `ComboboxOption[]` | — | yes | list of suggestion options |
| `placeholder` | `string \| null` | `null` | no | hint text when input is empty |
| `isDisabled` | `boolean` | `false` | no | disables the input and suppresses opening |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |

### Type Definitions

```
ComboboxOption: {
  value: string;
  label: string;
  description?: string;
  isDisabled?: boolean;
}
```

### Controlled And Uncontrolled

- controlled: `value` plus `valueChange` event
- uncontrolled: `defaultValue`
- query text is always internal state; `queryChange` event exposes it for parent observation

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | default or after selection/dismiss | input shows selected label or is empty; list hidden |
| open | focus on input or user begins typing | list appears below input with filtered options |
| highlighted | keyboard navigation or hover | highlighted option has accent background mix |
| selected | option committed | input shows selected label, list closes |
| disabled | `isDisabled=true` | input non-interactive, list cannot open |
| empty results | query matches no options | empty message shown in list |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| query active | user types in input | options filtered by query, `queryChange` fires |
| highlight tracked | ArrowDown/ArrowUp or hover | one option visually highlighted |
| value committed | Enter on highlighted or click option | `valueChange` fires, list closes |
| dismissed | Escape or click outside | list closes without changing value |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | user selects an option | `{ value: string }` | fires on commit only, not on highlight |
| `queryChange` | user types in the input | `{ query: string }` | fires on every input change |
| `openChange` | list opens or closes | `{ open: boolean }` | fires on open and close transitions |

## 6. Accessibility

### Semantics

- Root: `role="combobox"`, `aria-expanded` (true/false), `aria-haspopup="listbox"`, `aria-controls` pointing to listbox id
- Input: `aria-autocomplete="list"`, `aria-activedescendant` pointing to highlighted option id
- List: `role="listbox"`, unique id referenced by `aria-controls`
- Option: `role="option"`, `aria-selected` on the highlighted/selected option
- Disabled options: `aria-disabled="true"`
- Module-level `nextComboboxId` counter generates unique ids for ARIA relationships

### Keyboard

| Key | Behavior |
|-----|----------|
| typing | filters options, opens list if closed |
| `Arrow Down` | highlights next option; opens list if closed |
| `Arrow Up` | highlights previous option |
| `Enter` | selects the highlighted option, closes list |
| `Escape` | closes list without selecting, restores previous value in input |
| `Tab` | closes list if open, exits control |

### Focus And Announcement

- focus entry: input receives focus ring; list may open based on implementation
- focus transition: highlight moves via `aria-activedescendant` (no DOM focus change)
- focus restoration: closing the list keeps focus on the input
- live-region behavior: none; `aria-activedescendant` handles option announcement
- GPUI-native accessibility mapping notes: GPUI must expose combobox role, expanded state, listbox relationship, active-descendant tracking, and option selection through native accessibility APIs

## 7. Layout

### Sizing

- Root has `min-width: 14rem` to ensure usable query input width
- Input height follows `size-control-height` token
- List is absolutely positioned below input with a gap

### Composition

- parent expectations: forms, filter bars, searchable selects, settings
- child expectations: options with optional descriptions
- resizing rules: list anchors to root width (left: 0, right: 0); input stretches to parent

## 8. Token Usage — Exact Values

### Root `.combobox`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `grid` |
| `min-width` | `14rem` |

### Input `.combobox__input`

| Property | Value |
|----------|-------|
| `min-height` | `var(--pug-size-control-height)` |
| `padding` | `0 var(--pug-space-control-x)` |
| `border` | `0.0625rem solid var(--pug-color-border-default)` |
| `border-radius` | `var(--pug-radius-control)` |
| `background` | `var(--pug-color-background-surface)` |
| `color` | `var(--pug-color-text-primary)` |
| `font-family` | `var(--pug-typography-body-family)` |
| `font-size` | `var(--pug-typography-body-size)` |
| `line-height` | `var(--pug-typography-body-lineHeight)` |

### Input — focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### List `.combobox__list`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `top` | `calc(100% + 0.375rem)` |
| `left` | `0` |
| `right` | `0` |
| `z-index` | `var(--pug-overlay-z-menu)` |
| `display` | `grid` |
| `gap` | `0.125rem` |
| `padding` | `0.25rem` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--pug-color-border-default) 72%, transparent)` |
| `border-radius` | `var(--pug-radius-surface)` |
| `background` | `color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel))` |
| `box-shadow` | `var(--pug-elevation-overlay)` |

### Option `.combobox__option`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.125rem` |
| `width` | `100%` |
| `padding` | `0.375rem 0.5rem` |
| `border` | `0` |
| `border-radius` | `calc(var(--pug-radius-control) - 0.125rem)` |
| `background` | `transparent` |
| `color` | `var(--pug-color-text-primary)` |
| `cursor` | `pointer` |
| `font` | `inherit` |
| `text-align` | `left` |

### Option — highlighted / hover

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--pug-color-accent-base) 16%, transparent)` |

### Option — disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--pug-state-opacity-disabled)` |

### Description `.combobox__description`

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-text-secondary)` |
| `font-size` | `0.6875rem` |
| `line-height` | `1.35` |

### Empty `.combobox__empty`

| Property | Value |
|----------|-------|
| `padding` | `0.5rem` |
| `color` | `var(--pug-color-text-secondary)` |
| `font-size` | `0.6875rem` |
| `line-height` | `1.35` |

## 9. Svelte Notes

- Module-level `nextComboboxId` counter generates unique ids for each instance to wire ARIA relationships (`aria-controls`, `aria-activedescendant`)
- Client-side filtering: options are filtered by matching query against label (case-insensitive)
- Focus on input opens the list; click outside or Escape closes it
- When a value is selected, the input text is set to the selected option's label
- Disabled options are rendered but not selectable or highlightable
- Options are rendered as `<button>` elements with `role="option"` for click handling

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::combobox`
- GPUI must implement the combobox pattern with input, overlay list, and option highlighting
- Must expose combobox role, expanded state, active-descendant, and option list through native accessibility APIs
- List overlay positioning: anchor below input with gap; may need overflow/clipping awareness
- The `color-mix` formulas for list border, background, and option highlight must be replicated or approximated

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value and valueChange semantics match (commit on selection only)
- [ ] queryChange fires on every input change
- [ ] openChange fires on open/close transitions
- [ ] ArrowDown/ArrowUp highlight navigation matches
- [ ] Enter commits highlighted option
- [ ] Escape closes without selecting
- [ ] disabled options are visible but not selectable
- [ ] ARIA: combobox role, expanded, haspopup, controls, activedescendant, option role, selected

### Tier 2: Visual Parity

- [ ] input uses control-height, control-x padding, body typography
- [ ] input focus ring matches (outline with focusRing color, 0.125rem offset)
- [ ] list overlay: absolute positioning, 0.375rem gap below input
- [ ] list border color-mix (72% border-default) matches
- [ ] list background color-mix (98% elevated, panel) matches
- [ ] list elevation shadow matches
- [ ] option highlight color-mix (16% accent-base) matches
- [ ] option padding (0.375rem 0.5rem) and inner border-radius match
- [ ] description/empty font-size (0.6875rem) and color (text-secondary) match
- [ ] disabled option opacity matches

### Tier 3: Implementation Freedom

- [ ] filtering algorithm details (substring, prefix, fuzzy) stay internal
- [ ] overlay positioning/clipping strategy is platform-owned
- [ ] id generation strategy is implementation-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| filtering strategy may differ | matching/ranking internals are implementation details | allowed | keep commit and selection semantics strict |
| overlay positioning details | GPUI overlay system differs from CSS absolute positioning | allowed | must appear anchored below input visually |
| color-mix approximation in GPUI | GPUI may not have CSS color-mix; equivalent blending acceptable | allowed | visual result must match |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: searchable selects, token pickers, compact asset lookup, settings
- future follow-up: consider async/remote filtering support; keep distinct from CommandPalette semantics
