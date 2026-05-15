# Tri-State Switch

Status: detailed contract
Updated: 2026-03-24

## 1. Purpose

- Component name: `TriStateSwitch`
- Layer: `foundation`
- Summary: a ternary selection control for explicit exclude/default/include
  flows, presented as a shared pill track with a sliding active capsule and
  per-state semantic coloring
- In scope: three distinct states with fixed order (excluded, default,
  included), directional movement, labeled state options, customizable labels,
  semantic color coding per state, semantic size roles, density-aware spacing,
  and explicit `xs | sm | md | lg | xl` size overrides
- Out of scope: binary on/off toggles (see Switch), arbitrary multi-option
  segmentation (see SegmentedControl), free-form multi-select (see ToggleGroup)

## 2. Anatomy

```text
[Root .tri-state-switch]  <div role="radiogroup">
  ├── [Selection .tri-state-switch__selection]  <span aria-hidden="true">
  └── [Option .tri-state-switch__option]  <label>
        ├── [Control .tri-state-switch__control]  <input type="radio">
        └── [Segment .tri-state-switch__segment]  <span>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | radiogroup host track | border, radius, background, padding |
| Selection | yes | sliding active capsule showing the chosen state | background, border, radius, shadow, motion |
| Option | yes | label wrapper for one ternary state | cursor, positioning |
| Control | yes | hidden native radio input | visually hidden, focus semantics |
| Segment | yes | visible state label above the shared track | color, typography, focus ring |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `TriStateValue` | `"default"` | no | controlled ternary state |
| `options` | `Record<TriStateValue, string>` | `{ excluded: "Exclude", default: "Default", included: "Include" }` | no | labels for each state |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl" \| null` | `null` | no | explicit control size override |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `"compact" \| "default" \| "comfortable" \| null` | `null` | no | explicit density override for track inset and segment padding |
| `disabled` | `boolean` | `false` | no | disables interaction |
| `ariaLabel` | `string` | none | yes | names the ternary control (required) |
| `excludedColor` | `string \| null` | `null` | no | optional color override for the selected excluded state |
| `defaultColor` | `string \| null` | `null` | no | optional color override for the selected default state |
| `includedColor` | `string \| null` | `null` | no | optional color override for the selected included state |

### TriStateValue

```
type TriStateValue = "excluded" | "default" | "included"
```

### Controlled And Uncontrolled

- controlled by default in this contract; value is always provided
- uncontrolled mode can be added later if needed, but semantics remain ternary
- Fixed segment order: excluded, default, included (never reorderable)
- per-state color overrides map to local CSS custom properties on the root and
  affect only the selected segment backgrounds for that instance

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| excluded selected | `value="excluded"` | active capsule moves left, danger-tinted fill and border, excluded label turns danger |
| default selected | `value="default"` | active capsule moves center with neutral fill, default label uses primary text |
| included selected | `value="included"` | active capsule moves right, success-tinted fill and border, included label turns success |
| custom semantic colors | any state color override prop set | active capsule and selected label derive from the provided local semantic colors |
| unselected | not current value | muted label text on the shared track background |
| focus | keyboard focus on radio | focus ring on the visible segment label |
| disabled | `disabled=true` | whole control muted, non-interactive |

### Component States

Native radio-group keyboard behavior and single-selected-option state are
required. Exactly one of three states is always selected.

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onValueChange` | selected segment changes | `TriStateValue` | one segment selected at a time |

## 6. Accessibility

### Semantics

- Role: `role="radiogroup"` on root element
- Each control: native `<input type="radio">` with the parent radiogroup
- `aria-label` on root from prop (required)
- Per-state accessible name from the options labels
- Labeling rules: symbol-only options must still expose meaningful accessible
  names such as excluded/default/included

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow Left/Right` | moves selection/focus across states |
| `Space` | selects the focused state |
| `Tab` | enters or exits the control through the checked radio |

### Focus And Announcement

- focus entry: the checked radio participates in the tab sequence
- focus exit: current state remains selected
- live-region behavior: none; state changes announced through radio semantics
- GPUI-native accessibility mapping notes: GPUI must expose exclusive-choice
  semantics and meaningful names for segments, not just their glyphs

## 7. Layout

### Sizing

- Root uses CSS inline-grid with equal-width columns for all three states
- The active capsule spans one-third of the track and moves horizontally
- Labels share a stable common height derived from the resolved semantic size
- Track inset and segment padding respond to density
- State labels are short and should remain untruncated

### Composition

- parent expectations: filters, policy overrides, inclusion/exclusion controls
- child expectations: fixed ternary option set (always exactly three segments)
- resizing rules: segments remain visually grouped as one exclusive-choice
  control; never wraps or breaks apart

## 8. Token Usage — Exact Values

### Root `.tri-state-switch`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `inline-grid` |
| `width` | `max-content` |
| `grid-template-columns` | `repeat(3, minmax(0, 1fr))` |
| `align-items` | `stretch` |
| `padding` | `var(--poodle-tri-state-track-inset)` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `999px` |
| `background` | `color-mix(in srgb, var(--poodle-color-text-primary) 18%, var(--poodle-color-background-surface))` |
| `isolation` | `isolate` |
| `--poodle-tri-state-excluded-color` | `var(--poodle-color-status-danger)` |
| `--poodle-tri-state-default-color` | `var(--poodle-color-text-primary)` |
| `--poodle-tri-state-included-color` | `var(--poodle-color-status-success)` |
| `--poodle-tri-state-excluded-track` | `color-mix(in srgb, var(--poodle-tri-state-excluded-color) 18%, var(--poodle-color-background-surface))` |
| `--poodle-tri-state-default-track` | `color-mix(in srgb, var(--poodle-tri-state-default-color) 10%, var(--poodle-color-background-surface))` |
| `--poodle-tri-state-included-track` | `color-mix(in srgb, var(--poodle-tri-state-included-color) 18%, var(--poodle-color-background-surface))` |
| `--poodle-tri-state-height` | resolved from semantic size |
| `--poodle-tri-state-x` | resolved from density |
| `--poodle-tri-state-track-inset` | resolved from density |
| `--poodle-tri-state-min-width` | resolved from semantic size |

### Selection `.tri-state-switch__selection`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `top` | `var(--poodle-tri-state-track-inset)` |
| `bottom` | `var(--poodle-tri-state-track-inset)` |
| `left` | `var(--poodle-tri-state-track-inset)` |
| `width` | `calc((100% - (var(--poodle-tri-state-track-inset) * 2)) / 3)` |
| `border` | `0.0625rem solid transparent` |
| `border-radius` | `999px` |
| `box-shadow` | `inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent), 0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent)` |
| `transform` | `translateX(calc(var(--poodle-tri-state-active-index) * 100%))` |
| `transition` | `transform, background, border-color, box-shadow` at `var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

### Option `.tri-state-switch__option`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `cursor` | `pointer` |

### Control `.tri-state-switch__control`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `opacity` | `0` |
| `pointer-events` | `none` |

### Segment `.tri-state-switch__segment`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `min-height` | `calc(var(--poodle-tri-state-height) - (var(--poodle-tri-state-track-inset) * 2))` |
| `min-width` | `var(--poodle-tri-state-min-width)` |
| `padding` | `0 var(--poodle-tri-state-x)` |
| `border-radius` | `999px` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `var(--poodle-typography-label-size)` |
| `font-weight` | `var(--poodle-typography-label-weight)` |
| `line-height` | `var(--poodle-typography-label-lineHeight)` |
| `transition` | `color, opacity` at `var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

### Selected label states

| Property | Value |
|----------|-------|
| excluded | `color: var(--poodle-tri-state-excluded-color)` |
| default | `color: var(--poodle-color-text-primary)` |
| included | `color: var(--poodle-tri-state-included-color)` |

### Segment — focus visible (`:focus-visible`)

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Root — disabled (`[data-disabled="true"]`)

| Property | Value |
|----------|-------|
| `opacity` | `var(--poodle-state-opacity-disabled)` |

## 9. Svelte Notes

- Uses native `<input type="radio">` elements inside labels for keyboard and
  accessibility semantics
- Root uses `role="radiogroup"` with `aria-label` from prop
- Fixed state order is enforced: excluded, default, included
- Hidden radios drive the visible state labels via adjacent-sibling selectors
- A shared absolute-positioned selection capsule slides between the three states
- Semantic coloring uses status-danger, text-primary, and status-success tokens
  plus color-mix track fills
- Transition uses motion-duration-interaction and motion-easing-standard tokens

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::tri_state_switch`
- GPUI implementation must preserve named state labeling and exclusive-choice
  keyboard semantics
- The three semantic background colors (danger, neutral text-primary mix, and
  success) must be replicated using GPUI's color-mix or equivalent blending
- Fixed three-segment order must be enforced at the API level
- GPUI should remain shrink-wrapped to its content rather than stretching to
  fill the parent row
- GPUI should expose per-state focus targets and directional arrow-key movement
  explicitly, since there is no native radio-group primitive underneath
- GPUI contract and preview should expose the same per-state label and semantic
  color override inputs as the Svelte implementation, even if the transport
  shape differs from Svelte's `options` record

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] ternary value meaning matches (excluded, default, included)
- [ ] fixed segment order matches
- [ ] radiogroup role and per-segment radio semantics match
- [ ] symbolic segment accessible labels match
- [ ] directional navigation and roving focus match
- [ ] disabled behavior matches
- [ ] onValueChange callback payload matches

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
| Disabled | `value="included"`, `disabled=true`, `ariaLabel="Disabled switch"` | "Include" segment selected with success-tinted background; all segments muted and non-interactive |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: rule filters, inclusion/exclusion workflows, policy
  override controls
- future follow-up: coordinate with filter bars and relation pickers later;
  keep semantically distinct from SegmentedControl (TriStateSwitch always has
  exactly three states with semantic meaning)
