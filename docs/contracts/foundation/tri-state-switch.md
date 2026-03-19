# Tri-State Switch

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `TriStateSwitch`
- Layer: `foundation`
- Summary: a ternary selection control for explicit exclude/default/include
  style flows, presented as a three-segment track with semantic coloring
- In scope: three distinct states with fixed order (excluded, default,
  included), directional movement, labeled state options, customizable labels,
  semantic color coding per state
- Out of scope: binary on/off toggles (see Switch), arbitrary multi-option
  segmentation (see SegmentedControl), free-form multi-select (see ToggleGroup)

## 2. Anatomy

```text
[Root .tri-state-switch]  <div role="radiogroup">
  └── [Segment .tri-state-switch__segment...]  <button role="radio">
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | radiogroup host track | border, radius, background, padding |
| Segment | yes | one of three mutually exclusive choices | background, color, border-radius, focus ring |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `TriStateValue` | `"default"` | no | controlled ternary state |
| `options` | `Record<TriStateValue, string>` | `{ excluded: "Exclude", default: "Default", included: "Include" }` | no | labels for each state |
| `isDisabled` | `boolean` | `false` | no | disables interaction |
| `ariaLabel` | `string` | none | yes | names the ternary control (required) |

### TriStateValue

```
type TriStateValue = "excluded" | "default" | "included"
```

### Controlled And Uncontrolled

- controlled by default in this contract; value is always provided
- uncontrolled mode can be added later if needed, but semantics remain ternary
- Fixed segment order: excluded, default, included (never reorderable)

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| excluded selected | `value="excluded"` | danger-tinted background, primary text, inset shadow |
| default selected | `value="default"` | elevated-tinted background, primary text, inset shadow |
| included selected | `value="included"` | success-tinted background, primary text, inset shadow |
| unselected | not current value | transparent background, secondary text |
| focus | keyboard focus on segment | focus ring |
| disabled | `isDisabled=true` | all segments muted, non-interactive |

### Component States

Roving-focus via radiogroup keyboard behavior and single-selected-option state
are required. Exactly one of three segments is always selected.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | selected segment changes | `{ value: TriStateValue }` | one segment selected at a time |

## 6. Accessibility

### Semantics

- Role: `role="radiogroup"` on root element
- Each segment: `role="radio"` with `aria-checked="true"` when selected
- `aria-label` on root from prop (required)
- Per-segment accessible name from the options labels
- Labeling rules: symbol-only options must still expose meaningful accessible
  names such as excluded/default/included

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow Left/Right` | moves selection/focus across segments |
| `Space` | selects the focused segment |
| `Tab` | enters or exits the control through one tabbable segment |

### Focus And Announcement

- focus entry: one segment participates in the tab sequence (roving focus)
- focus exit: current segment remains selected and roving focus is preserved
- live-region behavior: none; state changes announced through radio semantics
- GPUI-native accessibility mapping notes: GPUI must expose exclusive-choice
  semantics and meaningful names for segments, not just their glyphs

## 7. Layout

### Sizing

- Root uses CSS inline-grid with equal-width columns for all three segments
- Three segments share a stable common height derived from control-height token
- Segment labels are not truncated (short by design)

### Composition

- parent expectations: filters, policy overrides, inclusion/exclusion controls
- child expectations: fixed ternary option set (always exactly three segments)
- resizing rules: segments remain visually grouped as one exclusive-choice
  control; never wraps or breaks apart

## 8. Token Usage — Exact Values

### Root `.tri-state-switch`

| Property | Value |
|----------|-------|
| `display` | `inline-grid` |
| `grid-auto-flow` | `column` |
| `gap` | `0.125rem` |
| `padding` | `0.125rem` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 84%, transparent)` |
| `border-radius` | `var(--pug-radius-control)` |
| `background` | `color-mix(in srgb, var(--pug-color-background-surface) 82%, transparent)` |

### Segment `.tri-state-switch__segment`

| Property | Value |
|----------|-------|
| `min-height` | `calc(var(--pug-size-control-height) - 0.25rem)` |
| `padding` | `0 0.75rem` |
| `border` | `0` |
| `border-radius` | `calc(var(--pug-radius-control) - 0.125rem)` |
| `background` | `transparent` |
| `color` | `var(--pug-color-text-secondary)` |
| `cursor` | `pointer` |
| `font-family` | `var(--pug-typography-label-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `600` |
| `line-height` | `1` |
| `transition` | `background, color, box-shadow` at `var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard)` |

### Segment — selected (shared across all states)

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-text-primary)` |
| `box-shadow` | `inset 0 0.0625rem 0 color-mix(in srgb, white 12%, transparent)` |

### Segment — excluded + selected

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--pug-color-status-danger) 18%, var(--pug-color-background-surface))` |

### Segment — default + selected

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--pug-color-background-elevated) 88%, var(--pug-color-background-surface))` |

### Segment — included + selected

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--pug-color-status-success) 18%, var(--pug-color-background-surface))` |

### Segment — focus visible (`:focus-visible`)

| Property | Value |
|----------|-------|
| `outline` | `var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Segment — disabled (`:disabled`)

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--pug-state-opacity-disabled)` |

## 9. Svelte Notes

- Uses native `<button>` elements with `role="radio"` and `aria-checked`
- Root uses `role="radiogroup"` with `aria-label` from prop
- Fixed segment order is enforced: excluded, default, included
- `data-value` attribute on each segment for styling hooks
- `data-selected` attribute on the currently selected segment
- Semantic background coloring uses status-danger (excluded), background-elevated
  (default), and status-success (included) via color-mix
- Transition uses motion-duration-interaction and motion-easing-standard tokens

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::tri_state_switch`
- GPUI implementation must preserve symbolic-option accessible labeling and
  exclusive-choice keyboard semantics
- The three semantic background colors (danger, elevated, success) must be
  replicated using GPUI's color-mix or equivalent blending
- Fixed three-segment order must be enforced at the API level
- Roving focus must be implemented explicitly (no native radio group available)

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] ternary value meaning matches (excluded, default, included)
- [ ] fixed segment order matches
- [ ] radiogroup role and per-segment radio semantics match
- [ ] symbolic segment accessible labels match
- [ ] directional navigation and roving focus match
- [ ] disabled behavior matches
- [ ] valueChange event payload matches

### Tier 2: Visual Parity

- [ ] root border, radius, and background match
- [ ] excluded selected background matches (18% status-danger mix)
- [ ] default selected background matches (88% background-elevated mix)
- [ ] included selected background matches (18% status-success mix)
- [ ] selected text color matches (text-primary)
- [ ] selected inset shadow matches
- [ ] unselected text color matches (text-secondary)
- [ ] focus ring appearance matches
- [ ] disabled opacity matches
- [ ] segment typography matches (family, size, weight)

### Tier 3: Implementation Freedom

- [ ] roving focus implementation is platform-owned
- [ ] transition timing is platform-owned
- [ ] data attribute naming is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Roving focus via native radio vs explicit GPUI implementation | web uses button+role=radio; GPUI must implement keyboard management directly | allowed | keep exclusivity and labels strict |
| Color-mix blending | GPUI may approximate color-mix differently | allowed | visual result must be comparable |
| CSS transition timing | GPUI may not support CSS-style transitions | allowed | match where possible |

## 13. Specimen Definitions

### Group: Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | `value="default"`, `ariaLabel="Filter mode"` | Three-segment switch with default labels (Exclude, Default, Include); "Default" segment selected with elevated background; live value display below updates on click |

### Group: Custom labels

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Custom labels | `options={ excluded: "Hide", default: "All", included: "Show" }`, `ariaLabel="Visibility filter"` | Three segments labeled "Hide", "All", "Show" instead of default labels; otherwise identical appearance |

### Group: Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | `value="included"`, `isDisabled=true`, `ariaLabel="Disabled switch"` | "Include" segment selected with success-tinted background; all segments muted and non-interactive |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: rule filters, inclusion/exclusion workflows, policy
  override controls
- future follow-up: coordinate with filter bars and relation pickers later;
  keep semantically distinct from SegmentedControl (TriStateSwitch always has
  exactly three states with semantic meaning)
