# Select

Status: detailed contract
Updated: 2026-03-26

## 1. Purpose

- Component name: `Select`
- Layer: `foundation`
- Summary: a single-select control using a native `<select>` element with
  custom styling, supporting flat options, option groups, lazy option loading,
  and filter-friendly clear/reset behavior
- In scope: single value selection, placeholder, flat options, grouped options
  (optgroup), disabled options, lazy option/group loading, clearable reset
  state, native select accessibility
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
| `items` | `LegacySelectItem[] \| null` | `null` | no | compatibility alias for flat option arrays using `disabled` instead of `isDisabled` |
| `groups` | `LegacySelectGroup[] \| null` | `null` | no | compatibility alias for grouped option data using `items` / nested `groups` |
| `disabled` | `boolean` | `false` | no | disables the select |
| `required` | `boolean` | `false` | no | forwards native required semantics |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |
| `describedBy` | `string \| null` | `null` | no | aria-describedby target |
| `name` | `string \| undefined` | `undefined` | no | form field name |
| `clearable` | `boolean` | `false` | no | keeps the placeholder option selectable so callers can reset to `defaultValue` |
| `valueLabel` | `string \| null` | `null` | no | temporary label for the current selection before lazy options load |
| `loadItems` | `(() => Promise<LegacySelectItem[]>) \| null` | `null` | no | lazy flat option loader |
| `loadGroups` | `(() => Promise<LegacySelectGroup[]>) \| null` | `null` | no | lazy grouped option loader |
| `loadKey` | `string \| null` | `null` | no | invalidates cached lazy options when it changes |
| `onchange` | `((value: string) => void) \| null` | `null` | no | callback prop for existing caller styles; `valueChange` remains the canonical event |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |

### Type Definitions

```
SelectOption: { value: string; label: string; isDisabled?: boolean; group?: string }
SelectOptionGroup: { label: string; options: SelectOption[] }
SelectItems: SelectOption[] | SelectOptionGroup[]
LegacySelectItem: { value: string; label: string; disabled?: boolean; isDisabled?: boolean }
LegacySelectGroup: { label: string; items?: LegacySelectItem[]; groups?: LegacySelectGroup[] }
```

### Controlled And Uncontrolled

- controlled: `value` plus `valueChange` event
- uncontrolled: `defaultValue`
- lazy: `loadItems` / `loadGroups` populate internal options once per `loadKey`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| placeholder | no value selected, placeholder prop set | placeholder text in secondary color, `data-placeholder="true"` |
| clearable reset | `clearable=true` | placeholder option remains selectable and maps to `defaultValue` |
| selected | value matches an option | option label displayed in primary color |
| focus | select receives focus | focus ring via border-color change, background shift, box-shadow |
| disabled | `disabled=true` | reduced opacity on root, non-interactive |
| loading | lazy loader pending | native fallback option shows `Loading…` |
| load error | lazy loader fails | native fallback option shows the error message |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| value selected | user picks an option | `valueChange` fires with selected option value |
| placeholder shown | no value and placeholder set | placeholder option displayed, disabled in dropdown |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | user selects a different option | `{ value: string }` | fires on native change event |
| `change` | user selects a different option | `{ value: string }` | alias event for existing caller styles |

## 6. Accessibility

### Semantics

- Role: native `<select>` element provides built-in accessibility
- Required attributes: accessible name from external label or `ariaLabel`
- Optional attributes: `aria-describedby` from `describedBy`
- `disabled` attribute set on root select when `disabled`
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
| `gap` | `var(--poodle-space-inline-sm)` |
| `min-height` | `var(--poodle-size-control-height)` |
| `padding` | `0 var(--poodle-space-control-x)` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-treatment-interactive-subtle-radius, var(--poodle-radius-control))` |
| `background` | `var(--poodle-treatment-interactive-subtle-fill, var(--poodle-color-background-surface))` |
| `box-shadow` | `var(--poodle-treatment-interactive-subtle-shadow, none)` |
| `transition` | `border-color, box-shadow, background` |

### Root — focus-within

| Property | Value |
|----------|-------|
| `border-color` | `var(--poodle-color-accent-focusRing)` |
| `background` | `var(--poodle-treatment-interactive-subtle-fill-focus, var(--poodle-color-background-surface))` |
| `box-shadow` | `var(--poodle-treatment-interactive-subtle-shadow-focus, 0 0 0 var(--poodle-border-width-focus) color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent))` |

### Root — has disabled select

| Property | Value |
|----------|-------|
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Control `.select__control`

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `width` | `100%` |
| `height` | `calc(var(--poodle-size-control-height) - (var(--poodle-border-width-default) * 2))` |
| `padding` | `0` |
| `border` | `0` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-primary)` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `line-height` | `var(--poodle-typography-body-lineHeight)` |
| `outline` | `0` |
| `appearance` | `none` |

### Control — placeholder state `.select[data-placeholder="true"] .select__control`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |

### Indicator `.select__indicator`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-icon-muted)` |
| `pointer-events` | `none` |

### Option Group `.select__control optgroup`

| Property | Value |
|----------|-------|
| `font-weight` | `600` |
| `color` | `var(--poodle-color-text-secondary)` |

### Option `.select__control option`

| Property | Value |
|----------|-------|
| `font-weight` | `normal` |
| `color` | `var(--poodle-color-text-primary)` |

### Size adjustments

| Size | min-height | padding | control height | font-size |
|------|------------|---------|----------------|-----------|
| `xs` | `calc(control-height - 0.5rem)` | `0 calc(space-control-x - 0.125rem)` | `calc(control-height - 0.5rem - border * 2)` | `0.75rem` |
| `sm` | `calc(control-height - 0.375rem)` | `0 calc(space-control-x - 0.0625rem)` | `calc(control-height - 0.375rem - border * 2)` | `typography-body-size` |
| `md` | `control-height` | `0 space-control-x` | `calc(control-height - border * 2)` | `typography-body-size` |
| `lg` | `calc(control-height + 0.375rem)` | `0 calc(space-control-x + 0.125rem)` | `calc(control-height + 0.375rem - border * 2)` | `0.9375rem` |
| `xl` | `calc(control-height + 0.5rem)` | `0 calc(space-control-x + 0.1875rem)` | `calc(control-height + 0.5rem - border * 2)` | `1rem` |

## 9. Svelte Notes

- Uses a native `<select>` element for full platform accessibility without a custom overlay
- `appearance: none` on the select removes native browser chrome; the custom indicator provides the disclosure chevron
- `data-placeholder="true"` attribute on root signals placeholder state for CSS targeting
- Treatment tokens (`--poodle-treatment-interactive-subtle-*`) provide themed styling with fallbacks to base tokens
- Placeholder rendered as a disabled `<option>` with `selected` when no value is set
- Option groups rendered as native `<optgroup>` elements
- Emits `data-size` on root element reflecting the resolved size
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::select`
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
- [ ] all five sizes visually match (height, padding, font-size per size table)

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
| Choose a fruit | Apple, Banana, Cherry | Banana | `disabled: true` |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: settings, filters, inspectors, form fields
- future follow-up: align with Popover overlay rules if GPUI implementation uses custom overlay; consider validation state support if needed
