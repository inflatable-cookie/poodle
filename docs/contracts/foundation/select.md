# Select

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Select`
- Layer: `foundation`
- Summary: a single-select control using a native `<select>` element with
  custom styling, supporting flat options and option groups
- In scope: single value selection, placeholder, flat options, grouped options
  (optgroup), disabled options, native select accessibility
- Out of scope: custom dropdown overlays, multi-select, searchable/filterable
  lists (see Combobox), arbitrary menu content

## 2. Anatomy

```text
[Root .select]  <div>
  ├── [Control .select__control]  <select>
  │     ├── [Placeholder <option>] (conditional, when placeholder set and no value)
  │     ├── [Option <option>]... (flat options)
  │     └── [Option Group <optgroup>]... (grouped options)
  │           └── [Option <option>]...
  └── [Indicator .select__indicator]  <span> (decorative chevron)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | styled wrapper providing field chrome | background, border, radius, shadow, focus ring |
| Control | yes | native `<select>` element | typography, text color, appearance reset |
| Indicator | yes | decorative disclosure chevron (Icon component) | icon color |
| Option | yes | selectable value | text color, font-weight |
| Option Group | no | labeled group of options | font-weight, text color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string \| undefined` | `undefined` | no | HTML id for the select element |
| `value` | `string \| null` | `null` | no | controlled selected value |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial value |
| `placeholder` | `string \| null` | `null` | no | shown when no value selected |
| `options` | `SelectItems` | — | yes | array of `SelectOption` or `SelectOptionGroup` |
| `isDisabled` | `boolean` | `false` | no | disables the select |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |
| `describedBy` | `string \| null` | `null` | no | aria-describedby target |
| `name` | `string \| undefined` | `undefined` | no | form field name |

### Type Definitions

```
SelectOption: { value: string; label: string; isDisabled?: boolean; group?: string }
SelectOptionGroup: { label: string; options: SelectOption[] }
SelectItems: SelectOption[] | SelectOptionGroup[]
```

### Controlled And Uncontrolled

- controlled: `value` plus `valueChange` event
- uncontrolled: `defaultValue`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| placeholder | no value selected, placeholder prop set | placeholder text in secondary color, `data-placeholder="true"` |
| selected | value matches an option | option label displayed in primary color |
| focus | select receives focus | focus ring via border-color change, background shift, box-shadow |
| disabled | `isDisabled=true` | reduced opacity on root, non-interactive |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| value selected | user picks an option | `valueChange` fires with selected option value |
| placeholder shown | no value and placeholder set | placeholder option displayed, disabled in dropdown |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | user selects a different option | `{ value: string }` | fires on native change event |

## 6. Accessibility

### Semantics

- Role: native `<select>` element provides built-in accessibility
- Required attributes: accessible name from external label or `ariaLabel`
- Optional attributes: `aria-describedby` from `describedBy`
- `disabled` attribute set on root select when `isDisabled`
- Disabled individual options use native `disabled` attribute on `<option>`
- Labeling rules: placeholder text is not the accessible name

### Keyboard

| Key | Behavior |
|-----|----------|
| `Space` / `Enter` | opens native select dropdown (platform-dependent) |
| `Arrow Down` / `Arrow Up` | navigates options within native dropdown |
| `Escape` | closes native dropdown |
| `Tab` | exits the control |

### Focus And Announcement

- focus entry: root receives visible focus treatment (border-color, background, box-shadow changes)
- focus exit: focus treatment clears
- live-region behavior: none; native select handles value announcement
- GPUI-native accessibility mapping notes: GPUI must expose select/combobox semantics with option list, selected value, and expanded state through the native accessibility tree

## 7. Layout

### Sizing

- Root min-height follows `size-control-height` token
- Root uses grid layout with two columns: select fills available space, indicator is auto-sized
- Indicator is pointer-events: none to allow click-through to native select

### Composition

- parent expectations: forms, filter bars, settings rows, toolbars
- child expectations: options and optgroups only
- resizing rules: select stretches to parent width; value display truncates if needed

## 8. Token Usage — Exact Values

### Root `.select`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `minmax(0, 1fr) auto` |
| `align-items` | `center` |
| `gap` | `var(--pug-space-inline-sm)` |
| `min-height` | `var(--pug-size-control-height)` |
| `padding` | `0 var(--pug-space-control-x)` |
| `border` | `0.0625rem solid var(--pug-color-border-default)` |
| `border-radius` | `var(--pug-treatment-interactive-subtle-radius, var(--pug-radius-control))` |
| `background` | `var(--pug-treatment-interactive-subtle-fill, var(--pug-color-background-surface))` |
| `box-shadow` | `var(--pug-treatment-interactive-subtle-shadow, none)` |
| `transition` | `border-color, box-shadow, background` |

### Root — focus-within

| Property | Value |
|----------|-------|
| `border-color` | `var(--pug-color-accent-focusRing)` |
| `background` | `var(--pug-treatment-interactive-subtle-fill-focus, var(--pug-color-background-surface))` |
| `box-shadow` | `var(--pug-treatment-interactive-subtle-shadow-focus, 0 0 0 var(--pug-border-width-focus) color-mix(in srgb, var(--pug-color-accent-focusRing) 28%, transparent))` |

### Root — has disabled select

| Property | Value |
|----------|-------|
| `opacity` | `var(--pug-state-opacity-disabled)` |

### Control `.select__control`

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `width` | `100%` |
| `height` | `calc(var(--pug-size-control-height) - (var(--pug-border-width-default) * 2))` |
| `padding` | `0` |
| `border` | `0` |
| `background` | `transparent` |
| `color` | `var(--pug-color-text-primary)` |
| `font-family` | `var(--pug-typography-body-family)` |
| `font-size` | `var(--pug-typography-body-size)` |
| `line-height` | `var(--pug-typography-body-lineHeight)` |
| `outline` | `0` |
| `appearance` | `none` |

### Control — placeholder state `.select[data-placeholder="true"] .select__control`

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-text-secondary)` |

### Indicator `.select__indicator`

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-icon-muted)` |
| `pointer-events` | `none` |

### Option Group `.select__control optgroup`

| Property | Value |
|----------|-------|
| `font-weight` | `600` |
| `color` | `var(--pug-color-text-secondary)` |

### Option `.select__control option`

| Property | Value |
|----------|-------|
| `font-weight` | `normal` |
| `color` | `var(--pug-color-text-primary)` |

## 9. Svelte Notes

- Uses a native `<select>` element for full platform accessibility without a custom overlay
- `appearance: none` on the select removes native browser chrome; the custom indicator provides the disclosure chevron
- `data-placeholder="true"` attribute on root signals placeholder state for CSS targeting
- Treatment tokens (`--pug-treatment-interactive-subtle-*`) provide themed styling with fallbacks to base tokens
- Placeholder rendered as a disabled `<option>` with `selected` when no value is set
- Option groups rendered as native `<optgroup>` elements

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::select`
- GPUI must model the select as a trigger that opens a platform-appropriate option list
- Must expose selected value, expanded state, and option list through native accessibility tree
- Treatment token fallback chain can be modeled as: use treatment token if set, else base token
- The indicator chevron is decorative and does not need separate accessibility exposure

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value and valueChange semantics match
- [ ] placeholder behavior matches (displayed when no value, secondary color)
- [ ] disabled state matches (whole control and individual options)
- [ ] option groups supported on both platforms
- [ ] accessible name from label or ariaLabel matches

### Tier 2: Visual Parity

- [ ] control height uses control-height token
- [ ] treatment token system matches (subtle radius, fill, shadow with fallbacks)
- [ ] focus-within treatment matches (border-color, background, box-shadow)
- [ ] placeholder color (text-secondary) matches
- [ ] indicator color (icon-muted) matches
- [ ] disabled opacity matches
- [ ] optgroup font-weight (600) and color (text-secondary) match

### Tier 3: Implementation Freedom

- [ ] native `<select>` dropdown vs GPUI custom overlay stays internal
- [ ] transition timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| native select dropdown appearance differs per platform | native `<select>` renders platform-native dropdowns | allowed | keep value/selection semantics strict |
| GPUI may use custom overlay instead of native select | GPUI has no native `<select>` equivalent | allowed | must preserve option group support and keyboard navigation |
| treatment token fallback chain | CSS var fallback vs Rust conditional | allowed | same visual result required |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Default (flat options)

One select with flat option list:

| Placeholder | Options | Initial |
|-------------|---------|---------|
| Choose a fruit | Apple, Banana, Cherry, Dragonfruit, Elderberry | none selected |

### Grouped options

One select with grouped options:

| Placeholder | Groups | Notes |
|-------------|--------|-------|
| Choose an item | Fruits (Apple, Banana, Cherry), Vegetables (Carrot, Broccoli, Spinach [disabled]), Grains (Rice, Wheat) | Spinach is disabled within its group |

### Disabled

One disabled select with pre-selected value:

| Placeholder | Options | Initial | Props |
|-------------|---------|---------|-------|
| Choose a fruit | Apple, Banana, Cherry | Banana | `isDisabled: true` |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: settings, filters, inspectors, form fields
- future follow-up: align with Popover overlay rules if GPUI implementation uses custom overlay; consider validation state support if needed
