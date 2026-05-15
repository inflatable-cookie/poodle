# Time Zone Select

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `TimeZoneSelect`
- Layer: `foundation`
- Summary: a timezone-value control for choosing a named time zone, implemented
  as a thin wrapper around `Select` with timezone-specific option defaults
- In scope: timezone selection, placeholder behavior, disabled state, optional
  host-provided option set, default timezone list fallback
- Out of scope: offset math, locale-specific timezone display policy,
  scheduling workflows, and custom rendering rules beyond what `Select`
  already supports

## 2. Anatomy

```text
[Wrapper .time-zone-select]  <Select>
  └── [Option list]  TimeZoneOption[] -> SelectOption[]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Wrapper | yes | forwards props to `Select` | inherits `Select` token usage |
| Option list | yes | timezone options mapped into `Select` options | label, value, disabled |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string \| undefined` | `undefined` | no | HTML id for the select element |
| `value` | `string \| null \| undefined` | `undefined` | no | controlled selected timezone identifier; leave undefined for uncontrolled mode |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial timezone |
| `placeholder` | `string \| null` | `"Search time zones..."` | no | shown when no value selected |
| `options` | `TimeZoneOption[]` | `defaultTimeZoneOptions()` | no | timezone option list |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `disabled` | `boolean` | `false` | no | disables the select |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |
| `describedBy` | `string \| null` | `null` | no | aria-describedby target |
| `name` | `string \| undefined` | `undefined` | no | form field name |
| `onValueChange` | `(value: string) => void \| undefined` | `undefined` | no | called when the selected time zone changes |
| `onQueryChange` | `(query: string) => void \| undefined` | `undefined` | no | called when the search query changes |
| `onOpenChange` | `(open: boolean) => void \| undefined` | `undefined` | no | called when the searchable dropdown opens or closes |

### Type Definitions

```
TimeZoneOption: { value: string; label: string; disabled?: boolean }
```

### Controlled And Uncontrolled

- controlled: `value` plus `onValueChange` callback; leave `value` undefined for uncontrolled mode
- uncontrolled: `defaultValue`
- when no options provided, component uses `defaultTimeZoneOptions()` fallback

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| placeholder | no value selected, placeholder prop set | placeholder text shown by `Select` |
| selected | value matches an option | selected timezone label shown by `Select` |
| open | searchable dropdown opened | option list and query field shown by `Select` |
| disabled | `disabled=true` | reduced opacity and non-interactive state from `Select` |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| value selected | user picks a timezone | `onValueChange` fires with timezone identifier |
| query updated | user types into searchable mode | `onQueryChange` fires with query text |
| dropdown toggled | overlay opens or closes | `onOpenChange` fires with open state |

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onValueChange` | user selects a different timezone | `string` | fires on select value change |
| `onQueryChange` | search query changes | `string` | searchable mode only |
| `onOpenChange` | dropdown open state changes | `boolean` | custom searchable mode only |

## 6. Accessibility

### Semantics

- Role: inherited from `Select` searchable mode
- Required attributes: accessible name from external label or `ariaLabel`
- Optional attributes: `aria-describedby` from `describedBy`
- disabled semantics delegated to `Select`
- Labeling rules: placeholder text is not the accessible name

### Keyboard

| Key | Behavior |
|-----|----------|
| `Space` / `Enter` | opens dropdown or selects highlighted option |
| `Arrow Down` / `Arrow Up` | navigates options within the dropdown |
| `Escape` | closes dropdown |
| `Tab` | exits the control |

### Focus And Announcement

- focus entry: inherited from `Select`
- focus exit: focus treatment clears
- live-region behavior: none
- GPUI-native accessibility mapping notes: GPUI must preserve `Select` semantics with selected value, expanded state, and option list

## 7. Layout

### Sizing

- sizing and spacing are inherited from `Select`
- searchable mode is always enabled

### Composition

- parent expectations: forms, settings rows, zoned datetime pickers, Field wrapper
- child expectations: options only
- resizing rules: inherited from `Select`

## 8. Token Usage — Exact Values

- Inherits `Select` token usage exactly for size, density, typography, focus, placeholder, disabled, and searchable-overlay behavior
- Adds no wrapper-specific visual tokens beyond timezone option mapping

## 9. Svelte Notes

- Delegates rendering and interaction entirely to `Select`
- `defaultTimeZoneOptions()` utility provides a reasonable IANA timezone list as fallback
- Maps `TimeZoneOption[]` into `Select` options
- searchable mode is always enabled

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::time_zone_select`
- Spec struct: `TimeZoneSelectSpec` in primitives crate
- GPUI must preserve `Select` semantics while using the same default timezone option source
- `defaultTimeZoneOptions()` equivalent must be available in Rust

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value and onValueChange semantics match
- [ ] searchable behavior matches
- [ ] placeholder behavior matches
- [ ] disabled state matches
- [ ] accessible name from label or ariaLabel matches
- [ ] describedBy relationship matches
- [ ] defaultTimeZoneOptions() provides equivalent timezone sets

### Tier 2: Visual Parity

- [ ] visual parity matches `Select`
- [ ] disabled opacity matches
- [ ] all five sizes visually match

### Tier 3: Implementation Freedom

- [ ] underlying option list implementation stays internal
- [ ] default timezone list ordering may vary
- [ ] overlay timing and positioning stay runtime-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| underlying select implementation differs per runtime | wrapper delegates to runtime-specific `Select` implementation | allowed | keep `Select` semantics and timezone option meaning strict |
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
| Disabled | `disabled` | Time zone select with reduced opacity, non-interactive |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: scheduler setup, publishing settings, zoned datetime pickers,
  DateTimeZonePicker composite
- future follow-up: consider searchable timezone picker if list length proves unwieldy
