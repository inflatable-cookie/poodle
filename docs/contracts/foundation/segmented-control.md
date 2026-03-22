# Segmented Control

> **Surface elevation**: SegmentedControl is a surface consumer (72% moderate contrast) — see [surface-elevation.md](./surface-elevation.md).

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `SegmentedControl`
- Layer: `foundation`
- Summary: a single-select control presented as adjacent visual segments within
  a shared track, used for exclusive inline mode switches
- In scope: exclusive selection, directional keyboard movement, disabled
  segments, hidden native radio inputs, module-level ID generation
- Out of scope: arbitrary content tabs, ternary policy semantics (see
  TriStateSwitch), multi-select (see ToggleGroup), panel navigation (see Tabs)

## 2. Anatomy

```text
[Root .segmented-control]  <div role="radiogroup">
  └── [Segment .segmented-control__segment...]  <label>
        ├── [Control .segmented-control__control]  <input type="radio">
        └── [Label .segmented-control__label]  <span>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | radiogroup host track | border, radius, background, padding |
| Segment | yes | one selectable option wrapper | cursor |
| Control | yes | hidden native radio input | visually hidden, receives focus |
| Label | yes | visible segment text | typography, background, color, border-radius, focus ring |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string \| null` | `null` | no | controlled selected value |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial value |
| `options` | `SegmentedControlOption[]` | none | yes | segment list |
| `isDisabled` | `boolean` | `false` | no | disables whole control |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |
| `name` | `string \| undefined` | auto-generated | no | radio group name attribute |

### SegmentedControlOption

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `value` | `string` | yes | option value |
| `label` | `string` | yes | visible label text |
| `isDisabled` | `boolean` | no | disables individual segment |
| `ariaLabel` | `string` | no | accessible name override for abbreviated labels |

### Controlled And Uncontrolled

- controlled: `value` plus `valueChange` event
- uncontrolled: `defaultValue`

### Internal State

- Module-level `nextSegmentedControlId` counter for generating unique radio
  group names when `name` is not provided

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| unselected | default | neutral segment styling with `text-secondary` color |
| selected | value matches option | accent background, inverse text, inset highlight shadow |
| focus | keyboard focus on hidden radio input | focus ring on corresponding label |
| disabled (group) | `isDisabled=true` | all segments muted, non-interactive |
| disabled (segment) | option `isDisabled=true` | individual segment muted, non-interactive |

### Component States

Roving-focus via native radio group behavior and single-selected-option state
are required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | selection changes | `{ value: string }` | one segment selected at a time |

## 6. Accessibility

### Semantics

- Role: `role="radiogroup"` on root element
- Each segment uses a hidden `<input type="radio">` for native radio semantics
- `aria-label` on root from prop
- Per-segment `aria-label` from option when visible labels are abbreviated
- Hidden inputs: `position: absolute; opacity: 0; pointer-events: none`

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow Left/Right` | moves focus and selection between segments |
| `Arrow Up/Down` | optional parity when visually vertical in future variants |
| `Space` or `Enter` | selects focused segment (native radio behavior) |
| `Tab` | enters/exits the control through one tabbable segment |

### Focus And Announcement

- focus entry: one segment participates in the tab order (native radio group
  behavior)
- focus exit: roving focus remains on the active or last-focused segment
- live-region behavior: none
- GPUI-native accessibility mapping notes: GPUI must expose radiogroup-like
  semantics, selected state, and roving focus explicitly

## 7. Layout

### Sizing

- Root uses CSS grid with equal-width columns for all segments
- Segment labels truncate with ellipsis when content overflows
- Control height derived from shared control-height token minus internal padding

### Composition

- parent expectations: shell tool groups, view mode toggles, compact filters
- child expectations: option list only in this baseline contract
- resizing rules: the control remains visually unified regardless of segment
  count; all segments share equal width via `grid-auto-columns: minmax(0, 1fr)`

## 8. Token Usage — Exact Values

### Root `.segmented-control`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-auto-flow` | `column` |
| `grid-auto-columns` | `minmax(0, 1fr)` |
| `gap` | `0.125rem` |
| `padding` | `0.125rem` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 84%, transparent)` |
| `border-radius` | `var(--pug-radius-control)` |
| `background` | `color-mix(in srgb, var(--pug-surface) 93%, var(--pug-color-text-primary))` |

### Segment `.segmented-control__segment`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `grid` |
| `min-width` | `0` |
| `cursor` | `pointer` |

### Control `.segmented-control__control` (hidden radio input)

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `opacity` | `0` |
| `pointer-events` | `none` |

### Label `.segmented-control__label`

| Property | Value |
|----------|-------|
| `display` | `block` |
| `min-width` | `0` |
| `min-height` | `calc(var(--pug-size-control-height) - 0.25rem)` |
| `padding` | `0 0.75rem` |
| `border-radius` | `calc(var(--pug-radius-control) - 0.125rem)` |
| `color` | `var(--pug-color-text-secondary)` |
| `font-family` | `var(--pug-typography-label-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `600` |
| `line-height` | `calc(var(--pug-size-control-height) - 0.25rem)` |
| `text-align` | `center` |
| `overflow` | `hidden` |
| `text-overflow` | `ellipsis` |
| `white-space` | `nowrap` |
| `transition` | `background, color, box-shadow` at `var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard)` |

### Label — selected state (`:checked + .segmented-control__label`)

| Property | Value |
|----------|-------|
| `background` | `var(--pug-color-accent-base)` |
| `color` | `var(--pug-color-text-inverse)` |
| `box-shadow` | `inset 0 0.0625rem 0 color-mix(in srgb, white 12%, transparent)` |

### Label — focus visible (`:focus-visible + .segmented-control__label`)

| Property | Value |
|----------|-------|
| `outline` | `var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Label — disabled (`:disabled + .segmented-control__label`)

| Property | Value |
|----------|-------|
| `opacity` | `var(--pug-state-opacity-disabled)` |
| `cursor` | `not-allowed` |

## 9. Svelte Notes

- Uses native `<input type="radio">` elements hidden with `opacity: 0` for
  built-in radiogroup keyboard behavior
- Module-level `let nextSegmentedControlId = 0` generates unique group names
- `data-selected` attribute on segment for styling hooks
- Label styling is driven by adjacent sibling selectors (`:checked +`,
  `:focus-visible +`, `:disabled +`)
- Treatment token: uses `--pug-radius-control` directly

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::segmented_control`
- GPUI implementation must intentionally preserve roving-focus and accessible
  exclusive-choice semantics rather than treating each segment as a free button
- The hidden radio input pattern does not apply; GPUI must expose radiogroup
  semantics through its accessibility tree directly
- Equal-width segment layout should use GPUI's flex or grid equivalent

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] exclusive-choice semantics match (radiogroup role)
- [ ] roving focus and directional navigation match
- [ ] abbreviated-label accessible names match
- [ ] disabled group and disabled segment behavior match
- [ ] valueChange event payload matches

### Tier 2: Visual Parity

- [ ] root border, radius, and background match
- [ ] selected segment accent background and inverse text match
- [ ] selected segment inset shadow matches
- [ ] focus ring appearance matches
- [ ] disabled opacity matches
- [ ] label typography matches (family, size, weight)
- [ ] equal-width segment layout matches

### Tier 3: Implementation Freedom

- [ ] hidden radio input vs GPUI native control is internal
- [ ] module-level ID generation strategy is platform-owned
- [ ] transition timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Hidden radio inputs vs GPUI native controls | web uses native radio for a11y; GPUI exposes semantics directly | allowed | keep selection semantics strict |
| ID generation strategy | module-level counter vs Rust ID approach | allowed | both must produce unique group names |
| CSS transition timing | GPUI may not support CSS-style transitions | allowed | match where possible |

## 13. Specimen Definitions

### Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | `value="grid"`, `ariaLabel="View mode"`, options: Grid, List, Table | Three equal-width segments in a bordered track; Grid segment shows selected state with accent background, inverse text, and inset highlight shadow |

### With Disabled Option

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With disabled option | `defaultValue="all"`, `ariaLabel="Status filter"`, options: All, Active, Archived, Draft (`isDisabled=true`) | Four segments; All shows selected state; Draft segment at reduced opacity with not-allowed cursor |

### Fully Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Fully disabled | `isDisabled=true`, `defaultValue="list"`, `ariaLabel="Disabled control"`, options: Grid, List, Table | All three segments at reduced opacity, not-allowed cursor; List shows selected styling but entire control is non-interactive |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: mode toggles, shell view switches, compact filter strips
- future follow-up: verify implementations keep `SegmentedControl` semantically
  distinct from `Tabs`
