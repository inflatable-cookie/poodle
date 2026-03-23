# Time Zone Select

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `TimeZoneSelect`
- Layer: `foundation`
- Summary: a timezone-value control for choosing a named time zone, using a
  styled native select with an indicator chevron inside a field-chrome shell
- In scope: timezone selection, placeholder behavior, disabled state, optional
  host-provided option set, default timezone list fallback
- Out of scope: offset math, locale-specific timezone display policy,
  scheduling workflows, custom searchable overlay

## 2. Anatomy

```text
[Shell .time-zone-select]  <div>
  ├── [Control .time-zone-select__control]  <select>
  │     ├── [Placeholder <option>] (conditional, when placeholder set and no value)
  │     └── [Option <option>]...
  └── [Indicator .time-zone-select__indicator]  <span> (decorative chevron)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Shell | yes | field chrome wrapper | border, radius, background, focus ring |
| Control | yes | native select element | typography, text color, appearance reset |
| Indicator | yes | decorative disclosure chevron | icon color, typography |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string \| undefined` | `undefined` | no | HTML id for the select element |
| `value` | `string \| null` | `null` | no | controlled selected timezone identifier |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial timezone |
| `placeholder` | `string \| null` | `"Select time zone"` | no | shown when no value selected |
| `options` | `TimeZoneOption[]` | `defaultTimeZoneOptions()` | no | timezone option list |
| `isDisabled` | `boolean` | `false` | no | disables the select |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |
| `describedBy` | `string \| null` | `null` | no | aria-describedby target |
| `name` | `string \| undefined` | `undefined` | no | form field name |

### Type Definitions

```
TimeZoneOption: { value: string; label: string; isDisabled?: boolean }
```

### Controlled And Uncontrolled

- controlled: `value` plus `valueChange` event
- uncontrolled: `defaultValue`
- when no options provided, component uses `defaultTimeZoneOptions()` fallback

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| placeholder | no value selected, placeholder prop set | placeholder text in secondary color |
| selected | value matches an option | timezone label displayed in primary color |
| focus-within | select receives focus | border-color and box-shadow change on shell |
| disabled | `isDisabled=true` | reduced opacity on shell, non-interactive |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| value selected | user picks a timezone | `valueChange` fires with timezone identifier |
| placeholder shown | no value and placeholder set | placeholder option displayed |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | user selects a different timezone | `{ value: string }` | fires on native change event |

## 6. Accessibility

### Semantics

- Role: native `<select>` element provides built-in accessibility
- Required attributes: accessible name from external label or `ariaLabel`
- Optional attributes: `aria-describedby` from `describedBy`
- `disabled` attribute set on select when `isDisabled`
- Labeling rules: placeholder text is not the accessible name

### Keyboard

| Key | Behavior |
|-----|----------|
| `Space` / `Enter` | opens native select dropdown (platform-dependent) |
| `Arrow Down` / `Arrow Up` | navigates options within native dropdown |
| `Escape` | closes native dropdown |
| `Tab` | exits the control |

### Focus And Announcement

- focus entry: shell receives visible focus treatment (border-color, box-shadow changes)
- focus exit: focus treatment clears
- live-region behavior: none; native select handles value announcement
- GPUI-native accessibility mapping notes: GPUI must expose select semantics with option list, selected value, and expanded state through native accessibility tree

## 7. Layout

### Sizing

- Shell min-height follows `size-control-height` token
- Shell uses grid layout with two columns: select fills available space, indicator is auto-sized
- Indicator is pointer-events: none to allow click-through to native select

### Composition

- parent expectations: forms, settings rows, zoned datetime pickers, Field wrapper
- child expectations: options only
- resizing rules: shell stretches to parent width; value display truncates if needed

## 8. Token Usage — Exact Values

### Shell `.time-zone-select`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `minmax(0, 1fr) auto` |
| `align-items` | `center` |
| `gap` | `var(--flint-space-inline-sm)` |
| `min-height` | `var(--flint-size-control-height)` |
| `padding` | `0 var(--flint-space-control-x)` |
| `border` | `0.0625rem solid var(--flint-color-border-default)` |
| `border-radius` | `var(--flint-radius-control)` |
| `background` | `var(--flint-color-background-surface)` |
| `transition` | `border-color, box-shadow, background` |

### Shell — focus-within

| Property | Value |
|----------|-------|
| `border-color` | `var(--flint-color-accent-focusRing)` |
| `box-shadow` | `0 0 0 var(--flint-border-width-focus) color-mix(in srgb, var(--flint-color-accent-focusRing) 28%, transparent)` |

### Shell — disabled

| Property | Value |
|----------|-------|
| `opacity` | `var(--flint-state-opacity-disabled)` |

### Control `.time-zone-select__control`

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `width` | `100%` |
| `height` | `calc(var(--flint-size-control-height) - (var(--flint-border-width-default) * 2))` |
| `padding` | `0` |
| `border` | `0` |
| `background` | `transparent` |
| `color` | `var(--flint-color-text-primary)` |
| `font-family` | `var(--flint-typography-body-family)` |
| `font-size` | `var(--flint-typography-body-size)` |
| `line-height` | `var(--flint-typography-body-lineHeight)` |
| `outline` | `0` |
| `appearance` | `none` |

### Control — placeholder state

| Property | Value |
|----------|-------|
| `color` | `var(--flint-color-text-secondary)` |

### Indicator `.time-zone-select__indicator`

| Property | Value |
|----------|-------|
| `color` | `var(--flint-color-icon-muted)` |
| `font-family` | `var(--flint-typography-code-family)` |
| `font-size` | `0.75rem` |
| `line-height` | `1` |
| `pointer-events` | `none` |

## 9. Svelte Notes

- Uses a native `<select>` element for full platform accessibility
- `appearance: none` on the select removes native browser chrome; the custom indicator provides the disclosure chevron
- `data-placeholder="true"` attribute on shell signals placeholder state for CSS targeting
- Placeholder rendered as a disabled `<option>` with `selected` when no value is set
- `defaultTimeZoneOptions()` utility provides a reasonable IANA timezone list as fallback
- Transition applies to border-color, box-shadow, and background for smooth focus treatment

## 10. GPUI Notes

- expected crate/module surface: `flint_gpui::primitives::time_zone_select`
- Spec struct: `TimeZoneSelectSpec` in primitives crate
- GPUI must model the select as a trigger that opens a platform-appropriate option list
- Must expose selected value, expanded state, and option list through native accessibility tree
- `defaultTimeZoneOptions()` equivalent must be available in Rust
- The indicator chevron is decorative and does not need separate accessibility exposure

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value and valueChange semantics match
- [ ] placeholder behavior matches (displayed when no value, secondary color)
- [ ] disabled state matches
- [ ] accessible name from label or ariaLabel matches
- [ ] describedBy relationship matches
- [ ] defaultTimeZoneOptions() provides equivalent timezone sets

### Tier 2: Visual Parity

- [ ] shell height uses control-height token
- [ ] shell grid layout matches (minmax column, auto indicator)
- [ ] focus-within treatment matches (border-color, box-shadow with 28% mix)
- [ ] placeholder color (text-secondary) matches
- [ ] indicator color (icon-muted) and font (code-family, 0.75rem) match
- [ ] disabled opacity matches
- [ ] control typography (body-family, body-size, body-lineHeight) matches

### Tier 3: Implementation Freedom

- [ ] native `<select>` dropdown vs GPUI custom overlay stays internal
- [ ] default timezone list ordering may vary
- [ ] transition timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| native select dropdown appearance differs per platform | native `<select>` renders platform-native dropdowns | allowed | keep value/selection semantics strict |
| GPUI may use custom overlay instead of native select | GPUI has no native `<select>` equivalent | allowed | must preserve timezone option navigation |
| default timezone list ordering may differ | platform timezone registries vary | allowed | keep public timezone value meaning strict |

## 13. Specimen Definitions

### Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | `ariaLabel="Time zone"` | Time zone select with placeholder text and chevron indicator; selecting a zone displays selected value below |

### With Pre-selected Zone

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With pre-selected zone | `defaultValue="America/New_York"`, `ariaLabel="Pre-filled time zone"` | Time zone select showing "America/New_York" as the selected value |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | `isDisabled` | Time zone select with reduced opacity, non-interactive |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: scheduler setup, publishing settings, zoned datetime pickers,
  ZonedDateTimePicker composite
- future follow-up: consider searchable timezone picker if list length proves unwieldy
