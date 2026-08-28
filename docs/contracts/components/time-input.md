# Time Input

Status: detailed contract
Updated: 2026-08-29

## 1. Purpose

- Component name: `TimeInput`
- Layer: `foundation`
- Summary: a time-only value control for local wall-clock entry using native
  web editing and a segmented custom editor where the runtime has no native
  time input
- In scope: canonical time values, adapter-owned partial drafts, min/max
  constraints including overnight ranges, whole-second step sizing, disabled
  state, controlled and uncontrolled value models
- Out of scope: timezone conversion, date ownership, recurrence, schedule
  workflows, custom time picker overlays

## 2. Anatomy

```text
Web
[Input .time-input]  <input type="time">

Custom native projection
[Root .time-input]  <group>
  [Hour segment] : [Minute segment] [: [Second segment]]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Input / Root | yes | one visual time-entry control | background, border, radius, color, typography, focus ring |
| Segment | custom native only | labelled hour, minute, or conditional second editor | text, focus, invalid presentation |
| Separator | custom native only | non-interactive colon between segments | secondary text |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string \| null` | `null` | no | HTML id for label association |
| `value` | `string \| null \| undefined` | `undefined` | no | controlled committed value in canonical 24-hour `HH:MM` or `HH:MM:SS` form; leave undefined for uncontrolled mode |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial value |
| `min` | `string \| null` | `null` | no | inclusive lower bound; `min > max` defines a range crossing midnight |
| `max` | `string \| null` | `null` | no | inclusive upper bound; `min > max` defines a range crossing midnight |
| `step` | `number` | `60` | no | positive whole-second increment, anchored at `min` when present or midnight otherwise |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `disabled` | `boolean` | `false` | no | disables editing and interaction |
| `ariaLabel` | `string \| null` | `null` | no | required when no external label exists |
| `describedBy` | `string \| null` | `null` | no | aria-describedby target |

### Controlled And Uncontrolled

- controlled: `value` plus `onValueChange` callback; leave `value` undefined for uncontrolled mode
- uncontrolled: `defaultValue` sets the initial value; component owns its own state
- do not mix controlled and uncontrolled modes simultaneously
- incomplete and invalid text/segments are adapter-owned drafts, never a second
  public value channel
- an external controlled-value replacement discards the local draft
- authored values outside the documented grammar or constraints are invalid
  component input; adapters do not normalize them into a different time

### Canonical Value And Constraints

- `HH:MM` and `HH:MM:SS` are zero-padded 24-hour forms: hours `00`–`23`,
  minutes and seconds `00`–`59`
- fractional seconds are outside the value grammar; `step` is a positive whole
  number of seconds
- the step grid starts at `min` when present, otherwise `00:00:00`
- with no bounds, stepping wraps across midnight
- with `min <= max`, the allowed range is linear and stepping stops at its
  inclusive endpoints
- with `min > max`, the allowed range is `[min, 24:00) ∪ [00:00, max]`;
  stepping may cross midnight but never enters the excluded daytime gap
- the seconds segment is present when `step < 60` or any authored current,
  default, minimum, or maximum value includes seconds
- a complete direct edit must satisfy the grammar, bounds, and step grid before
  it becomes committed

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | no value set | input shows platform placeholder |
| populated | value is set | time value displayed |
| focus | input receives focus | focus ring via outline |
| drafting | user is entering a partial replacement | local draft remains visible; controlled value is unchanged |
| invalid draft | draft is incomplete, out of bounds, or off step | danger treatment and invalid accessibility state; no callback |
| disabled | `disabled=true` | reduced opacity, non-interactive |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty -> populated | user enters time or value prop set | onValueChange fires |
| populated -> empty | value cleared | onValueChange fires with null |
| committed -> drafting | a segment becomes incomplete or invalid | committed value is retained; no callback |
| drafting -> committed | all visible segments form a constraint-valid time | canonical value is emitted once |
| drafting -> reverted | invalid draft blurs or receives Escape | last committed value is restored; no callback |
| controlled replacement | host supplies another `value` | local draft is discarded and authored value is shown |

### Behavior Machine

Behavior classification: machine-backed

Paired pure TypeScript/Rust semantics own canonical parsing and formatting,
seconds visibility, bound membership, step alignment and stepping, draft versus
commit transitions, clearing, blur/Escape reversion, and stale controlled-value
replacement. Web and GPUI adapters own platform events, focus, local draft
storage, drawing, and accessibility projection. No callback or I/O runs inside
the machine.

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onValueChange` | user forms a complete constraint-valid time, or clears the whole control | `string \| null` | emits canonical `HH:MM`/`HH:MM:SS` or `null`; partial and invalid drafts never emit |

## 6. Accessibility

### Semantics

- Web role: native `<input type="time">` provides built-in accessibility
- Custom native role: one labelled group containing labelled hour, minute, and
  conditional second spin-button-like segments
- Required attributes: accessible name from external label or `ariaLabel`
- Optional attributes: `aria-describedby` from `describedBy`
- `disabled` attribute set when `disabled`
- `min`, `max`, `step` attributes set on native input when provided
- an incomplete, out-of-range, or off-step local draft exposes invalid state
  until corrected or reverted

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow Up` / `Arrow Down` | increment/decrement the time by configured `step`, respecting the allowed range |
| `Tab` / `Shift+Tab` | native web traversal; custom editors move between visible segments, then leave the control |
| number keys | direct entry into the focused time segment |
| `Backspace` / `Delete` | clear the focused segment; clearing every segment emits `null` |
| `Escape` | discard an incomplete/invalid draft and restore the last committed value |

### Focus And Announcement

- focus entry: input or first custom segment receives visible focus ring
- focus exit: focus ring clears
- invalid/incomplete drafts revert on focus exit rather than clamping silently
- live-region behavior: none; native time input handles value announcement
- GPUI-native accessibility mapping notes: the labelled group and each segment
  expose current value plus applicable bounds through the native accessibility
  tree; the renderer uses the existing `SpinButton` node role

## 7. Layout

### Sizing

- minimum height follows `size-control-height` token
- width determined by parent container
- overflow behavior: text truncates within input

### Composition

- parent expectations: forms, settings rows, datetime pickers, Field wrapper
- child expectations: none (self-contained)
- resizing rules: input stretches to parent width

## 8. Token Usage — Exact Values

### Input `.time-input`

| Property | Value |
|----------|-------|
| `min-height` | `var(--poodle-size-control-height)` |
| `padding` | `0 var(--poodle-space-control-x)` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-primary)` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `line-height` | `var(--poodle-typography-body-lineHeight)` |

### Input — focus

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Input — disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Size adjustments

| Size | Property | Value |
|------|----------|-------|
| `xs` (`[data-size="xs"]`) | `min-height` | `calc(var(--poodle-size-control-height) - 0.5rem)` |
| `xs` | `padding` | `0 calc(var(--poodle-space-control-x) - 0.125rem)` |
| `xs` | `font-size` | `0.75rem` |
| `sm` (`[data-size="sm"]`) | `min-height` | `calc(var(--poodle-size-control-height) - 0.375rem)` |
| `sm` | `padding` | `0 calc(var(--poodle-space-control-x) - 0.0625rem)` |
| `sm` | `font-size` | `0.8125rem` (equals the md body-size baseline) |
| `lg` (`[data-size="lg"]`) | `min-height` | `calc(var(--poodle-size-control-height) + 0.375rem)` |
| `lg` | `padding` | `0 calc(var(--poodle-space-control-x) + 0.125rem)` |
| `lg` | `font-size` | `0.9375rem` |
| `xl` (`[data-size="xl"]`) | `min-height` | `calc(var(--poodle-size-control-height) + 0.5rem)` |
| `xl` | `padding` | `0 calc(var(--poodle-space-control-x) + 0.1875rem)` |
| `xl` | `font-size` | `1rem` |

The size `min-height` values above express the intended offsets from the control-height token. Svelte currently resolves these to literal rem values (`xs 1.5rem`, `sm 1.75rem`, `lg 2.75rem`, `xl 3.25rem`) rather than `calc()` on the token; visually equivalent at the default `md=2.25rem`, but re-theming the base token will not flow into Svelte until it switches to `calc()`.

### Density adjustments

Density adjusts horizontal padding only; it never changes height or vertical padding.

| Density | Property | Value |
|---------|----------|-------|
| `compact` (`[data-density="compact"]`) | `padding` | `0 calc(var(--poodle-space-control-x) - 0.125rem)` |
| `default` | `padding` | `0 var(--poodle-space-control-x)` (baseline) |
| `comfortable` (`[data-density="comfortable"]`) | `padding` | `0 calc(var(--poodle-space-control-x) + 0.125rem)` |

## 9. Svelte Notes

- Uses native `<input type="time">` for platform accessibility and time-entry UX
- `appearance: none` may be needed for consistent cross-browser styling
- Public value uses local time strings in HH:MM or HH:MM:SS form
- Browser-native time picker UI is allowed; Poodle does not override it
- Native browser draft UI stays adapter-owned. Callbacks are gated through the
  shared value/constraint semantics so invalid or off-step values do not leak
  into the portable callback.
- A browser validity failure uses the same derived invalid presentation and
  blur reversion as the custom native editor.
- New appearance overrides must use component-scoped Recipe hooks with
  semantic-token fallbacks.
- `data-size` data attribute on the input reflects the resolved size
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::time_input`
- Spec struct: `TimeInputSpec` in `poodle-specs`
- Renderer: `poodle_render::time_input` / `time_input_with_change`
- GPUI provides one segmented time-entry control because there is no native
  `input[type="time"]`
- the pre-1.0 `TimeFieldSpec` / `time_field` public surface is gone; no alias or
  compatibility wrapper remains
- Must expose time value, min/max constraints, and step through accessibility tree
- Focus ring treatment must match outline spec

## 11. Parity Checklist

### Tier 1: Strict Parity

- [x] value and onValueChange semantics match
- [x] min, max, step constraints match
- [x] canonical format, conditional seconds, overnight range, and step anchor match
- [x] incomplete/invalid drafts remain local and revert without callback
- [x] clearing, controlled replacement, and disabled inertia match
- [x] disabled state matches
- [x] accessible name from label or ariaLabel matches
- [ ] describedBy relationship matches

### Tier 2: Visual Parity

- [ ] control height uses control-height token
- [ ] padding uses space-control-x token
- [ ] border and border-radius match
- [ ] background uses background-surface token
- [ ] typography (body-family, body-size, body-lineHeight) matches
- [ ] focus ring (border-width-focus, accent-focusRing, 0.125rem offset) matches
- [ ] disabled opacity matches
- [ ] all five sizes visually match (height, padding, font-size per size table)

### Tier 3: Implementation Freedom

- [x] native time-entry UI vs GPUI custom time editing stays internal
- [x] transition timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| native editing affordances may differ | platform time-entry controls differ | allowed | shared commit/draft/value semantics remain strict |
| GPUI provides segmented custom editing | no native `input[type="time"]` in GPUI | allowed | preserve format, constraints, callback, focus, and accessibility results |
| custom GPUI presentation is 24-hour | locale-specific 12-hour presentation is not part of the portable contract | allowed | revisit only through a separate localization decision |

## 13. Specimen Definitions

### Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | `ariaLabel="Start time"` | Empty time input with platform placeholder; selecting a time displays selected value below |

### With Default Value

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With default value | `defaultValue="14:30"`, `ariaLabel="Meeting time"` | Time input pre-filled with 14:30 |

### With Min/Max Constraints

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With min/max constraints | `defaultValue="09:00"`, `min="08:00"`, `max="18:00"`, `ariaLabel="Office hours"` | Time input constrained to 08:00-18:00 range, showing 09:00 |

### With Seconds

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Seconds step | `defaultValue="09:30:15"`, `step=15`, `ariaLabel="Cue time"` | Seconds are visible and arrow changes move in 15-second increments |

### Overnight Range

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Overnight | `defaultValue="23:30"`, `min="22:00"`, `max="06:00"`, `step=1800`, `ariaLabel="Quiet hours"` | Valid range crosses midnight; stepping reaches 00:00 without entering the excluded daytime gap |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | `defaultValue="12:00"`, `disabled` | Time input showing 12:00, reduced opacity, non-interactive |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: settings rows, booking fields, datetime pickers,
  DateTimePicker composite
- approved native editing decision: 2026-08-28 — segmented 24-hour GPUI editor,
  adapter-owned draft, valid-value-only callback, revert rather than clamp,
  whole-second step, overnight ranges, and clean Rust rename
- future follow-up: consider custom time picker overlay or localization only if
  browser-native presentation proves unsuitable; neither changes this value
  contract implicitly

## Rust Spec Migration

The landed implementation renamed the legacy Rust `TimeFieldSpec` /
`time_field` surface to `TimeInputSpec` / `time_input` and migrated every
in-repository caller. Poodle is pre-1.0, so the old public names are gone
without aliases, wrappers, or silent fallback.
