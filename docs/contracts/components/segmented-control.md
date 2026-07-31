# Segmented Control

> **Surface elevation**: SegmentedControl is a surface consumer (72% moderate contrast) — see [surface-elevation.md](./surface-elevation.md).

Status: detailed contract
Updated: 2026-07-14

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
              ├── [Icon .poodle-icon]  optional
              └── [Label Text .segmented-control__label-text]  optional when icon-only
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | radiogroup host track | border, radius, background, padding |
| Segment | yes | one selectable option wrapper | cursor |
| Control | yes | hidden native radio input | visually hidden, receives focus |
| Label | yes | visible segment content and selected-state surface | typography, background, color, border-radius, focus ring |
| Icon | no | decorative option icon | current color, supporting visual size |
| Label Text | conditional | visible option text; omitted only when an icon-only option has an icon | typography, truncation |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string \| null \| undefined` | `undefined` | no | controlled selected value; leave undefined for uncontrolled mode |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial value |
| `options` | `SegmentedControlOption[]` | none | yes | segment list |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `disabled` | `boolean` | `false` | no | disables whole control |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |
| `name` | `string \| undefined` | auto-generated | no | radio group name attribute |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `equalWidth` | `boolean` | `true` | no | when `false`, segments size to content instead of sharing equal width |

### SegmentedControlOption

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `value` | `string` | yes | option value |
| `label` | `string` | yes | visible label text, or accessible-name and tooltip fallback for an icon-only option |
| `icon` | `IconProp` | no | optional icon rendered before the label |
| `iconOnly` | `boolean` | no | hides visible label text when `icon` exists; retains `label` as the accessible-name and tooltip fallback |
| `disabled` | `boolean` | no | disables individual segment |
| `ariaLabel` | `string` | no | accessible name override for abbreviated labels |
| `title` | `string` | no | tooltip/title attribute for the segment wrapper |

### Controlled And Uncontrolled

- controlled: `value` plus `onValueChange` callback; leave `value` undefined for uncontrolled mode
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
| disabled (group) | `disabled=true` | all segments muted, non-interactive |
| disabled (segment) | option `disabled=true` | individual segment muted, non-interactive |

### Component States

Roving-focus via native radio group behavior and single-selected-option state
are required.

### Behavior Machine

Behavior classification: machine-backed (shared `singleSelectTransition` in
`@poodle/headless`)

Same machine as RadioGroup: single-select over options with per-option and
group disabled guards; `SELECT` sets `value` and emits
`emitValueChange(value)`; same-value and disabled selections are inert.
Native radio inputs provide keyboard and focus behavior.

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onValueChange` | selection changes | `string` | one segment selected at a time |

## 6. Accessibility

### Semantics

- Role: `role="radiogroup"` on root element
- Each segment uses a hidden `<input type="radio">` for native radio semantics
- `aria-label` on root from prop
- Per-segment `aria-label` from option when visible labels are abbreviated
- Icon-only segments use `ariaLabel` when supplied, otherwise their required
  `label`, so the icon is never the sole accessible name
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

- Root uses CSS grid with equal-width columns for all segments by default
- When `equalWidth=false`, segments size to content and the group left-aligns
- Segment labels truncate with ellipsis when content overflows
- Labelled icons sit before text; icon-only segments become square when
  `equalWidth=false`
- Control height derived from shared control-height token minus internal padding

### Composition

- parent expectations: shell tool groups, view mode toggles, compact filters
- child expectations: option list only in this baseline contract
- resizing rules: the control remains visually unified regardless of segment
  count; segments either share equal width via `grid-auto-columns: minmax(0, 1fr)` or size to content when `equalWidth=false`

## 8. Token Usage — Exact Values

### Root `.segmented-control`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-auto-flow` | `column` |
| `grid-auto-columns` | `minmax(0, 1fr)` |
| `gap` | `0.125rem` |
| `padding` | `0.125rem` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 84%, transparent)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `color-mix(in srgb, var(--poodle-surface) 93%, var(--poodle-color-text-primary))` |

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
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `gap` | `0.375rem` |
| `box-sizing` | `border-box` |
| `min-width` | `0` |
| `min-height` | `calc(var(--poodle-size-control-height) - 0.25rem)` |
| `padding` | `0 0.75rem` |
| `border-radius` | `calc(var(--poodle-radius-control) - 0.125rem)` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `600` |
| `line-height` | `calc(var(--poodle-size-control-height) - 0.25rem)` |
| `text-align` | `center` |
| `overflow` | `hidden` |
| `text-overflow` | `ellipsis` |
| `white-space` | `nowrap` |
| `transition` | `background, color, box-shadow` at `var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

### Label — icon-only (`[data-icon-only="true"]`)

| Property | Value |
|----------|-------|
| `width` | `calc(var(--poodle-segmented-control-height) - 0.25rem)` |
| `padding-inline` | `0` |

### Label — selected state (`:checked + .segmented-control__label`)

| Property | Value |
|----------|-------|
| `background` | `var(--poodle-color-accent-base)` |
| `color` | `var(--poodle-color-text-inverse)` |
| `box-shadow` | `inset 0 0.0625rem 0 color-mix(in srgb, white 12%, transparent)` |

### Label — focus visible (`:focus-visible + .segmented-control__label`)

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Label — disabled (`:disabled + .segmented-control__label`)

| Property | Value |
|----------|-------|
| `opacity` | `var(--poodle-state-opacity-disabled)` |
| `cursor` | `not-allowed` |

## 9. Svelte Notes

- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)
- Uses native `<input type="radio">` elements hidden with `opacity: 0` for
  built-in radiogroup keyboard behavior
- Module-level `let nextSegmentedControlId = 0` generates unique group names
- `data-selected` attribute on segment for styling hooks
- `data-icon-only="true"` on options that supply both `iconOnly=true` and an icon
- Option icons use Poodle `Icon` at the supporting-visual size resolved from the
  control size
- Label styling is driven by adjacent sibling selectors (`:checked +`,
  `:focus-visible +`, `:disabled +`)
- Treatment token: uses `--poodle-radius-control` directly

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::segmented_control`
- GPUI implementation must intentionally preserve roving-focus and accessible
  exclusive-choice semantics rather than treating each segment as a free button
- The hidden radio input pattern does not apply; GPUI must expose radiogroup
  semantics through its accessibility tree directly
- Equal-width segment layout should use GPUI's flex or grid equivalent

## 10a. Jetstream Notes

- `SegmentedControl::from_spec(spec, theme).on_change(...)`, carrying the chosen
  option's value.
- Re-picking the selected segment still fires. Swallowing it here would hide the
  click from a consumer that treats re-selection as "confirm".

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] exclusive-choice semantics match (radiogroup role)
- [ ] roving focus and directional navigation match
- [ ] abbreviated-label accessible names match
- [ ] disabled group and disabled segment behavior match
- [ ] onValueChange callback payload matches

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
| Icon and icon-only options | currently implemented by the authoritative Svelte component; native option specs do not yet carry icon references | provisional | add equivalent native icon references before claiming strict visual parity for this presentation |

## 13. Specimen Definitions

### Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | `value="grid"`, `ariaLabel="View mode"`, options: Grid, List, Table | Three equal-width segments in a bordered track; Grid segment shows selected state with accent background, inverse text, and inset highlight shadow |

### With Disabled Option

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With disabled option | `defaultValue="all"`, `ariaLabel="Status filter"`, options: All, Active, Archived, Draft (option `disabled=true`) | Four segments; All shows selected state; Draft segment at reduced opacity with not-allowed cursor |

### Fully Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Fully disabled | `disabled=true`, `defaultValue="list"`, `ariaLabel="Disabled control"`, options: Grid, List, Table | All three segments at reduced opacity, not-allowed cursor; List shows selected styling but entire control is non-interactive |

### Icon-Only Options

| Label | Props / Config | Expected Visual |
|-------|----------------|-----------------|
| Icon-only options | `defaultValue="effects"`, `equalWidth=false`, `ariaLabel="Plugin kind"`, options: Effects and Instruments with `icon` and `iconOnly=true` | Two compact square segments showing icons; Effects is selected; each native radio is announced by its option label and each wrapper exposes the label as a tooltip |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: mode toggles, shell view switches, compact filter strips
- future follow-up: verify implementations keep `SegmentedControl` semantically
  distinct from `Tabs`
