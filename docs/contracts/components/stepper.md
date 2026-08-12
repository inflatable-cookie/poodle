# Stepper

> **Surface elevation**: Stepper is a surface consumer (72% moderate contrast) — see [surface-elevation.md](./surface-elevation.md).

Status: detailed contract
Updated: 2026-07-28

## 1. Purpose

- Component name: `Stepper`
- Layer: `foundation`
- Summary: a route through a multi-step process — horizontal by default,
  vertical where the process reads as a checklist — showing where the user is,
  which steps are done, and — where steps do real work — whether each one is
  running or has failed
- In scope: per-step status, current-step selection, per-step re-run action,
  keyboard navigation between steps, horizontal and vertical orientation,
  collapsing a vertical stepper to a one-line summary
- Out of scope: step content panels (the consumer renders those; a vertical
  stepper does not become an accordion), branching or optional-step logic,
  progress percentages (see Progress)

### Collapse Hides The Stepper, Not Step Content

The collapsed form is the *stepper* folded into one line, and that is the whole
of it. It does not become an accordion: there are still no content panels, and
expanding reveals the same step rows that were always there. The distinction
matters because "collapsible vertical list of steps" is one rename away from the
component this contract explicitly says Stepper is not.

### Status Is A Property, Not A Position

**A step's status is given, never derived from its index.** The obvious
implementation is `index < currentIndex ? complete : pending`, and it is wrong
for any wizard whose steps do work: a step that ran for two minutes and was
rejected must read as *failed*, and deriving from position would render it as
"not yet reached" — actively misleading rather than merely imprecise.

So `status` is a required field on every step, and `value` separately names
which step is current. The two are orthogonal: the current step may be
`running`, and a `failed` step may sit behind the current one.

Consumers whose steps are cheap and linear can still produce the simple
behaviour by mapping their own index comparison onto `status`. That mapping
belongs to them, because only they know whether a step not yet reached is
`pending` or a step already passed is `complete` or `failed`.

## 2. Anatomy

```text
[Root .poodle-stepper]  <nav>
  └── [Step .poodle-stepper__step]  <ol><li>
        ├── [Trigger .poodle-stepper__trigger]  <button aria-current="step">
        │     ├── [Marker .poodle-stepper__marker]  <span>
        │     │     └── index number, check glyph, failure glyph, or Spinner
        │     └── [Label .poodle-stepper__label]  <span>
        └── [Rerun .poodle-stepper__rerun]  <button>  optional
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | bordered track owning the shared border and radius | border, radius, background |
| Step | yes | one column of the track; carries the divider to its right and the current-step fill | border-right, background |
| Trigger | yes | selects this step; the whole row is the hit target | min-height, padding, gap, typography, color, background |
| Marker | yes | circular status glyph or 1-based index | size, border, radius, typography, status color |
| Label | yes | the step's name | typography, truncation |
| Rerun | conditional | re-runs a completed step; present only when `onRerun` is supplied and the step's `status` is `complete` | icon size, color, hover fill |
| Summary | conditional | present only when `collapsible` and `orientation="vertical"`; the disclosure trigger, and the whole of the control when collapsed | min-height, padding, gap, typography |
| Chevron | conditional | direction reflects collapse state | icon size, color |
| Rail | conditional | one segment per step, coloured by that step's status | gap |
| Rail segment | conditional | a single step, as a dash: coloured by its status, full-length when current and half-length otherwise | width, thickness, radius, status color |
| Summary label | conditional | the current step's label | typography, truncation |
| Summary count | conditional | completed steps over total | typography, color |

```text
[Root .poodle-stepper][data-collapsible="true"]  <nav>
  ├── [Summary .poodle-stepper__summary]  <button aria-expanded>
  │     ├── [Chevron .poodle-stepper__summary-chevron]  <span aria-hidden>
  │     ├── [Rail .poodle-stepper__rail]  <span aria-hidden>
  │     │     └── [Rail segment .poodle-stepper__rail-segment]  ×steps
  │     ├── [Summary label .poodle-stepper__summary-label]  <span>
  │     └── [Summary count .poodle-stepper__summary-count]  <span aria-hidden>
  └── [Step …]  ×steps  — omitted entirely when collapsed
```

### Why Rerun Is A Separate Control

Re-running a step costs whatever the step costs — for a model-backed pipeline
that is real money and minutes. **Selecting a step and re-running it are
therefore different actions and get different controls.** Folding re-run into
the trigger would mean revisiting a finished step to read its output silently
re-spends; a user cannot undo that by navigating away.

The trigger navigates. The Rerun button, and only the Rerun button, re-runs.

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `steps` | `StepperStep[]` | none | yes | ordered step list |
| `value` | `string \| null \| undefined` | `undefined` | no | controlled current step value; leave undefined for uncontrolled mode |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial current step; falls back to the first step |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for horizontal spacing |
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | no | axis the steps flow along; vertical stacks steps as rows in the same shared track |
| `collapsible` | `boolean` | `false` | no | renders the Summary row as a disclosure trigger; **vertical only** — ignored when horizontal |
| `collapsed` | `boolean \| undefined` | `undefined` | no | controlled collapse state; leave undefined for uncontrolled mode |
| `defaultCollapsed` | `boolean` | `false` | no | uncontrolled initial collapse state |
| `disabled` | `boolean` | `false` | no | disables every step |
| `ariaLabel` | `string \| null` | `null` | no | accessible name for the nav landmark; required, since "navigation" alone does not say which process |
| `rerunLabel` | `string` | `"Re-run step"` | no | accessible name for the Rerun control, suffixed with the step label |

### StepperStep

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `value` | `string` | yes | step identity |
| `label` | `string` | yes | visible step name |
| `status` | `"pending" \| "running" \| "complete" \| "failed"` | yes | see §1; never derived from position |
| `isDisabled` | `boolean` | no | when true this step cannot be selected |
| `description` | `string \| null` | no | supplementary text announced after the label |

### Controlled And Uncontrolled

- `value` supplied → controlled; the consumer owns the current step and must
  update it from `onChange`
- `value` undefined → uncontrolled; internal state starts at `defaultValue`, or
  the first step when that is null
- `collapsed` follows the same rule against `defaultCollapsed` and
  `onCollapsedChange`

### Collapse Is Vertical-Only

`collapsible` does nothing in horizontal orientation, and that is deliberate
rather than unimplemented: a horizontal stepper is already one line, so
collapsing it would trade a row of legible step labels for a row of dashes and
buy back no height. An implementation must ignore `collapsible` and `collapsed`
when `orientation="horizontal"` — render the full track and no Summary row.

## 4. States

### Visual States

| State | Trigger | Treatment |
|-------|---------|-----------|
| Current | step's `value` matches the current value | accent-tinted background, primary text, accent marker border |
| Complete | `status: "complete"` | primary text, accent marker with check glyph |
| Running | `status: "running"` | primary text, accent marker containing a Spinner |
| Failed | `status: "failed"` | primary text, danger marker with failure glyph, danger label |
| Pending | `status: "pending"` | secondary text, neutral marker showing the 1-based index |
| Disabled | `isDisabled`, or root `disabled` | `disabled-opacity`, no pointer events |
| Hover | pointer over an enabled trigger | subtle fill across the whole step, not just the trigger; hovering the Rerun control does not raise it, since that control does something other than select |
| Focus | keyboard focus on trigger or rerun | focus ring |

**Current and status are composed, not exclusive.** A step that is both current
and failed shows the accent background *and* the danger marker: the user needs
to know both where they are and that it broke.

### Collapse States

| State | Trigger | Treatment |
|-------|---------|-----------|
| Expanded | `collapsible`, `collapsed=false` | Summary row above the step list, chevron pointing down, divider below the Summary |
| Collapsed | `collapsible`, `collapsed=true` | Summary row only; the step list is not rendered, chevron points right, no divider |
| Rail complete | segment's step is `complete` | accent |
| Rail running | segment's step is `running` | accent at 55% — the same hue as complete, dimmer, because a running step is on its way to being one |
| Rail failed | segment's step is `failed` | danger |
| Rail pending | segment's step is `pending` | `border-strong` |
| Summary disabled | root `disabled` | `disabled-opacity`, not toggleable |
| Summary hover | pointer over the enabled Summary | the same subtle fill a Step gets |
| Summary focus | keyboard focus | focus ring |

**Colour is status, length is position.** The rail carries two codes, and they
get two channels: every segment is coloured by its step's status, and the
current step's segment draws at full length while every other one draws at half.
A second *colour* for the current step would be unreadable — at dash size one
mark cannot hold two colour codes — so length is the channel that was free.

## 5. Callbacks

| Callback | Signature | Notes |
|----------|-----------|-------|
| `onValueChange` | `((value: string) => void) \| undefined` | fired when a step is selected; not fired for disabled steps |
| `onRerun` | `((value: string) => void) \| null` | when supplied, completed steps render a Rerun control; omitted entirely when null |
| `onCollapsedChange` | `(collapsed: boolean) => void` | fired when the Summary toggles; carries the new state |

## 6. Accessibility

### Semantics

- Root is `<nav>` with `aria-label` from `ariaLabel`
- Steps are an `<ol>` of `<li>`, so the count and position are announced —
  "step 3 of 4" is structural, not something the label has to spell out
- Trigger is a `<button>`; the current step carries `aria-current="step"`
- Status is announced through the trigger's accessible description, not through
  colour alone: `running`, `failed` and `complete` append their state to the
  accessible name
- A `failed` step sets `aria-invalid="true"` on its trigger
- The Marker is decorative once status is in the accessible name, so it is
  `aria-hidden="true"` — otherwise a screen reader reads the check glyph and
  then the word "complete"
- Rerun is a separate `<button>` with `aria-label` of `rerunLabel` plus the step
  label, so a row of them is not four identical "Re-run step" buttons
- Summary is a `<button>` carrying `aria-expanded`. Its accessible name is
  `"<current step label>, <n> of <m> steps complete"` — the visible `n/m` is
  `aria-hidden`, since "five slash five" is not a sentence. The Chevron and Rail
  are `aria-hidden` too: both restate what the name already says
- Collapsing removes the `<ol>` from the tree rather than hiding it with CSS, so
  a collapsed stepper does not leave four unreachable buttons in the tab order

### Keyboard

| Key | Action |
|-----|--------|
| `Tab` | moves into the stepper — the Summary first when collapsible — then between trigger and its rerun control |
| `Enter` / `Space` on Summary | toggles the collapse state |
| `ArrowRight` / `ArrowLeft` | horizontal orientation: moves focus between step triggers, skipping disabled steps |
| `ArrowDown` / `ArrowUp` | vertical orientation: the same movement along the vertical axis |
| `Home` / `End` | first / last enabled step |
| `Enter` / `Space` | activates the focused control |

Focus moves without selecting; selection is explicit. A wizard step can be
expensive to open, so arrow keys must not commit to one.

## 7. Layout

### Horizontal (default)

- Root is a grid of equal columns — `grid-auto-flow: column`,
  `grid-auto-columns: minmax(0, 1fr)` — so steps share the width evenly
  regardless of label length
- Each Step carries a right border except the last, drawing dividers inside the
  shared track rather than around each cell
- Trigger fills its column and left-aligns; the Marker is fixed-size and the
  Label truncates
- Rerun sits at the trailing edge of the step, outside the Label's truncation

### Vertical

- The same shared bordered track, flowed as rows: `grid-auto-flow: row`,
  single column, each row sized by its content
- Dividers move to the bottom edge — each Step carries a bottom border except
  the last; no right borders
- Step anatomy is unchanged: same Trigger, Marker, Label and Rerun, same
  left-alignment. A vertical stepper is the horizontal one rotated at the
  track level, not a different component
- Size still owns row height and marker/font sizes; density still owns only
  horizontal padding and gap — rows are contiguous, so there is no vertical
  spacing for density to touch

### Collapsed

- The Summary is a row of the same shared track, sized by the same
  `--poodle-stepper-row-height` as a Step, so collapsing and expanding do not
  change the control's first-row height
- Order along the row: Chevron, Rail, Summary label, Summary count. The label
  takes the remaining width and truncates; Chevron, Rail and count are all
  fixed-size
- The count is pushed to the trailing edge, so a stack of collapsed steppers
  right-aligns its counts regardless of label length
- Expanded, the Summary carries a bottom border in the same hairline as the step
  dividers; collapsed, it carries none — it is the last row

## 8. Token Usage — Exact Values

### Root `.poodle-stepper`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-auto-flow` | `column` |
| `grid-auto-columns` | `minmax(0, 1fr)` |
| `border` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-panel) 92%, transparent)` |
| `overflow` | `hidden` |

### Step `.poodle-stepper__step`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `min-width` | `0` |
| `border-right` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `border-right` (last) | `0` |

### Trigger `.poodle-stepper__trigger`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-stepper-gap)` |
| `flex` | `1 1 auto` |
| `min-width` | `0` |
| `min-height` | `var(--poodle-stepper-row-height)` |
| `padding` | `var(--poodle-stepper-pad-y) var(--poodle-stepper-pad-x)` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `var(--poodle-stepper-font-size)` |
| `text-align` | `left` |
| focus ring (`:focus-visible`) | `var(--poodle-border-width-focus)` solid `var(--poodle-color-accent-focusRing)`, `0.125rem` offset, radius `var(--poodle-radius-control)` — the first and last steps' rings cornered inside the track's rounded corners without it |

### Marker `.poodle-stepper__marker`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `place-items` | `center` |
| `flex` | `0 0 auto` |
| `width` / `height` | `var(--poodle-stepper-marker-size)` |
| `border` | `0.0625rem solid currentColor` |
| `border-radius` | `999px` |
| `font-size` | `var(--poodle-stepper-marker-font-size)` |
| `font-weight` | `700` |

### Summary `.poodle-stepper__summary`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-stepper-gap)` |
| `min-height` | `var(--poodle-stepper-row-height)` |
| `padding` | `var(--poodle-stepper-pad-y) var(--poodle-stepper-pad-x)` |
| `background` | `transparent` |
| `border-radius` | `var(--poodle-radius-control)` — rounds the full-width row so the inset focus ring and the hover fill do not render square inside the track's rounded corners |
| `font-size` | `var(--poodle-stepper-font-size)` |
| `text-align` | `left` |
| `border-bottom` (expanded) | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `border-bottom` (collapsed) | `0` |
| focus ring (`:focus-visible`) | `var(--poodle-border-width-focus)` solid `var(--poodle-color-accent-focusRing)`, `-0.125rem` inset offset |

### Rail `.poodle-stepper__rail` and `.poodle-stepper__rail-segment`

| Part | Property | Value |
|------|----------|-------|
| Rail | `display` | `flex` |
| Rail | `align-items` | `center` |
| Rail | `flex` | `0 0 auto` |
| Rail | `gap` | `var(--poodle-stepper-rail-gap)` |
| Segment | `width` | `calc(var(--poodle-stepper-rail-segment-width) / 2)` |
| Segment, current | `width` | `var(--poodle-stepper-rail-segment-width)` |
| Segment | `height` | `var(--poodle-stepper-rail-thickness)` |
| Segment | `border-radius` | `999px` |
| Segment | `flex` | `0 0 auto` |

### Summary Label And Count

| Part | Property | Value |
|------|----------|-------|
| Chevron | `color` | `var(--poodle-color-text-secondary)` |
| Summary label | `flex` | `1 1 auto`, `min-width: 0`, ellipsis truncation |
| Summary label | `color` | `var(--poodle-color-text-primary)` |
| Summary count | `flex` | `0 0 auto` |
| Summary count | `color` | `var(--poodle-color-text-tertiary)` |

### Size Ladder

Size controls intrinsic dimensions only; density controls horizontal spacing.

| Size | row-height | marker-size | font-size | marker-font-size | pad-y |
|------|------------|-------------|-----------|------------------|-------|
| `xs` | 2.5rem | 1.125rem | 0.625rem | 0.5625rem | 0.5rem |
| `sm` | 2.875rem | 1.25rem | 0.6875rem | 0.5625rem | 0.625rem |
| `md` | 3.25rem | 1.35rem | 0.75rem | 0.625rem | 0.7rem |
| `lg` | 3.625rem | 1.5rem | 0.8125rem | 0.6875rem | 0.8rem |
| `xl` | 4rem | 1.75rem | 0.875rem | 0.75rem | 0.9rem |

`rail-segment-width` is the **current** step's dash; every other dash is half of
it.

| Size | rail-segment-width | rail-thickness |
|------|--------------------|----------------|
| `xs` | 0.75rem | 0.125rem |
| `sm` | 0.875rem | 0.125rem |
| `md` | 1rem | 0.1875rem |
| `lg` | 1.125rem | 0.1875rem |
| `xl` | 1.25rem | 0.25rem |

| Density | pad-x | gap | rail-gap |
|---------|-------|-----|----------|
| `compact` | 0.625rem | 0.4375rem | 0.1875rem |
| `default` | 0.8rem | 0.55rem | 0.25rem |
| `comfortable` | 1rem | 0.6875rem | 0.3125rem |

The rail's segment width and thickness are size, not density: a dash is an
intrinsic dimension of the mark. The gap between dashes is spacing between
siblings, so it is density.

### State Treatments

| State | Property | Value |
|-------|----------|-------|
| Current | Step `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 10%, transparent)` — on the **Step**, not the Trigger, so the fill spans the whole column even when a Rerun control sits beside the trigger |
| Current | Trigger `color` | `var(--poodle-color-text-primary)` |
| Current, Complete, Running | marker `border-color`, `color` | `var(--poodle-color-accent-base)` |
| Failed | marker `border-color`, `color` | `var(--poodle-color-status-danger)` |
| Failed | label `color` | `var(--poodle-color-status-danger)` |
| Complete | `color` | `var(--poodle-color-text-primary)` |
| Disabled | `opacity` | `var(--poodle-state-opacity-disabled)` |
| Hover (enabled) | Step `background` | `color-mix(in srgb, var(--poodle-color-text-primary) 6%, transparent)` — on the **Step**, for the same reason as the current fill |
| Hover on current | Step `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent)` — deepens the accent rather than replacing it with the neutral hover |
| Hover (Summary) | Summary `background` | `color-mix(in srgb, var(--poodle-color-text-primary) 6%, transparent)` — the same neutral fill a Step gets |
| Rail segment, `complete` | `background` | `var(--poodle-color-accent-base)` |
| Rail segment, `running` | `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 55%, transparent)` |
| Rail segment, `failed` | `background` | `var(--poodle-color-status-danger)` |
| Rail segment, `pending` | `background` | `var(--poodle-color-border-strong)` |

## 9. Svelte Notes

- The current value is `$state` when uncontrolled and read from props when
  controlled, following the SegmentedControl pattern
- The Spinner component renders the `running` marker rather than a bespoke
  animation

## 10. GPUI Notes

- Status colours resolve through the `StepperSpec` token methods; no literal
  colours in the component
- GPUI has no accessibility API — see `003-native-accessibility.md`. The status
  suffix in the accessible name is carried on the spec and is inert there.
- Orientation maps to the flex axis: vertical flows the same steps as a column
  with bottom dividers.
- Keyboard navigation between triggers does not exist on the natives (no key
  routing), so the per-orientation arrow keys are web-only.

## 10a. Jetstream Notes

- `Stepper::from_spec(spec, theme).on_change(...).on_rerun(...)`, each carrying
  the step's value. This is the component whose GPUI handlers were stored and
  never attached — the defect that started g12.017 — so these tests are the
  reference for what "actually wired" means.
- The rerun sits inside a clickable step, so it takes its own handler, inert
  when unwired: rerunning a step must not also select it.

## 11. Parity Checklist

- [ ] equal-column layout with dividers between steps
- [ ] marker renders index, check, failure glyph and spinner per status
- [ ] current step is visually distinct from complete
- [ ] failed step is distinguishable from pending — the defect the status
      property exists to prevent
- [ ] rerun control appears only for completed steps and only when `onRerun` is
      supplied
- [ ] disabled steps are not selectable
- [ ] accessible name includes the status word
- [ ] `collapsible` renders a Summary row in vertical orientation and nothing in
      horizontal
- [ ] collapsed omits the step list rather than hiding it
- [ ] the rail draws one segment per step, coloured by that step's status
- [ ] the current step's rail segment is twice the length of the others

### GPUI Interaction

`on_change` and `on_rerun` reach real click handlers on GPUI. They were stored
and attached to nothing for a while: the builders type-checked, the
pointing-hand cursor promised a click, and nothing happened when you made one.

`effigy drift:handlers` exists because of that bug — it fails when a GPUI
component accepts a handler it never reads.

## 12. Known Deltas

| Target | Delta | Reason |
|--------|-------|--------|
| GPUI | no accessibility surface | runtime limit — `003-native-accessibility.md` |
| GPUI, Jetstream | selection, rerun and collapse wiring live in the preview event loop | components render from spec state |
| GPUI, Jetstream | the collapsed chevron is the shared `chevron-right` / `chevron-down` icon; the rail is a row of rounded boxes | no CSS, so both are drawn as nodes |
| GPUI | marker glyphs are text characters (`✓ ✕ ◌ ⟳`) rather than icon assets | matches the surrounding GPUI components, which draw glyphs the same way |
| GPUI | the `running` marker is a static glyph, not an animated spinner | GPUI components render from spec state with no frame loop of their own |

## 13. Specimen Definitions

| Specimen | Content |
|----------|---------|
| Default | four steps, two complete, one current, one pending — the Soundcheck arrangement |
| Working | a running step with its spinner |
| Failed | a failed step behind the current one, which is what position-derived state cannot express |
| Rerun | completed steps with the rerun control shown |
| Sizes | the five-step size ladder |
| Disabled | whole control disabled |
| Collapsed | `collapsible`, vertical, starting collapsed — the summary line alone, toggling to the full list |
| Collapsed statuses | a collapsed stepper whose rail carries all four statuses, which is the only place the rail's colour coding is legible at a glance |

## 14. Approval And Adoption Notes

The visual design is carried over from Soundcheck's Sync wizard, which built a
bespoke stepper before this component existed; Soundcheck should adopt this and
delete its local copy. The two additions — status as a property and the re-run
affordance — come from Figmatic, whose steps run for minutes and can be rejected
by a gate. Both are additive: the Soundcheck arrangement renders unchanged.
