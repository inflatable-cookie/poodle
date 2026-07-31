# Collapsible

> **Surface elevation**: Collapsible is a surface consumer (50% strong contrast) — see [surface-elevation.md](./surface-elevation.md).

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `Collapsible`
- Layer: `foundation`
- Summary: a single disclosure container with a trigger and revealable content
- In scope: controlled or uncontrolled open state, heading copy, trigger
  affordance, inline reveal posture, disabled state, trigger snippet override
- Out of scope: grouped selection logic, routing, or full accordion ownership

## 2. Anatomy

```text
[Root .collapsible]  <section data-open data-disabled>
  ├── [Trigger .collapsible__trigger]  <button aria-expanded aria-controls>
  │   ├── [Heading .collapsible__heading]  <span>
  │   │   ├── [Snippet: trigger({ isOpen })]  (or default heading)
  │   │   ├── [Title .collapsible__title]  <span> (when no trigger snippet)
  │   │   └── [Description .collapsible__description]  <span> (optional, when no trigger snippet)
  │   └── [Indicator .collapsible__indicator]  <span aria-hidden>
  │       └── [Icon name="chevron-down" size="sm"]
  └── [Content .collapsible__content]  <div role="region" aria-labelledby> (conditional)
      └── [Snippet: children()]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | disclosure section container | border, radius, background, shadow, padding, gap |
| Trigger | yes | interactive button for expand/collapse | grid layout, color, cursor, focus ring |
| Heading | yes | title/description container or trigger snippet target | gap |
| Title | no | heading text (when no trigger snippet) | heading family, weight, size |
| Description | no | supporting description (when no trigger snippet) | secondary color, size |
| Indicator | yes | chevron rotation indicator | secondary color, transition |
| Content | conditional | expanded content region | min-width, padding-top |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `open` | `boolean \| null \| undefined` | `undefined` | no | controlled open state when supplied; omit for uncontrolled mode |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial open state |
| `title` | `string \| null` | `null` | no | visible heading text |
| `description` | `string \| null` | `null` | no | visible supporting description |
| `disabled` | `boolean` | `false` | no | suppresses interaction |
| `highlighted` | `boolean` | `false` | no | applies accent highlight to the container |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `ariaLabel` | `string \| null` | `null` | no | accessible label when no title present |

### Snippets

| Snippet | Purpose | Props |
|---------|---------|-------|
| `children()` | collapsible content | none |
| `trigger({ isOpen })` | custom trigger heading content | `{ isOpen: boolean }` |

### Controlled And Uncontrolled

- controlled: supplying `open` makes it host-owned through `onOpenChange`
- uncontrolled: `defaultOpen` with internal state management

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | default | content hidden, indicator points down, gap collapses to 0 |
| open | `open=true` or toggled | content visible, indicator rotated 180deg, gap active |
| disabled | `disabled=true` | reduced opacity on root, cursor not-allowed on trigger |
| highlighted | `highlighted=true` | accent border and halo on root |
| focus | keyboard focus on trigger | focus ring visible |

### Component States

- `data-open`: `"true"` or `"false"` on root `.collapsible`
- `data-disabled`: `"true"` or `"false"` on root `.collapsible`
- `data-highlighted`: `"true"` or `"false"` on root `.collapsible`

### Behavior Machine

Behavior classification: machine-backed (`disclosureTransition` in
`@poodle/headless`)

- Context: `open` (controllable), `disabled`
- Events: `TOGGLE` (trigger click), `SET_OPEN` (programmatic, silent)
- Transitions: `TOGGLE` flips `open` and emits `emitOpenChange(open)`;
  disabled is inert
- Machinery dependencies: none (native button provides keyboard/focus).

## 5. Callbacks

| Callback | When It Runs | Payload | Notes |
|-------|---------------|---------|-------|
| `onOpenChange` | user toggles disclosure | `boolean` | suppressed while disabled (button disabled) |

## 6. Accessibility

### Semantics

- Root: `<section>` element
- Trigger: `<button type="button">` with `aria-expanded` and `aria-controls`
- Trigger: `id="poodle-collapsible-trigger-{collapsibleId}"`
- Trigger: `aria-label` from prop when no `title` is present
- Content: `role="region"` with `aria-labelledby` pointing to trigger id
- Content: `id="poodle-collapsible-content-{collapsibleId}"`
- Module-level `nextCollapsibleId` counter provides unique id namespaces

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` | toggles open state |
| `Space` | toggles open state |
| `Tab` | moves focus to next focusable element |
| `Shift+Tab` | moves focus to previous focusable element |

### Focus And Announcement

- focus entry: trigger button receives visible focus ring
- focus exit: focus ring clears immediately
- live-region behavior: none; expansion state announced through `aria-expanded`
- GPUI-native accessibility mapping notes: GPUI must expose button role with
  expanded state and region association through the native accessibility tree

## 7. Layout

### Sizing

- Root: vertical grid with `space.stack.md` gap when open, 0 gap when closed
- Content appears inline below the trigger
- All grid containers use `min-width: 0` for overflow safety

### Composition

- parent expectations: settings groups, sidebars, drawers, docs notes, compact
  web disclosure surfaces
- child expectations: arbitrary snippet or child content
- resizing: root stretches to container width; content flows naturally

## 8. Token Usage — Exact Values

### Root `.collapsible` (open state)

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-space-stack-md)` |
| `min-width` | `0` |
| `padding` | `0.625rem var(--poodle-space-panel-x)` (inline padding overridden by density) |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 36%, transparent)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 40%, var(--poodle-color-background-panel))` (also sets `--poodle-surface`) |
| `box-shadow` | `inset 0 0.0625rem 0 color-mix(in srgb, var(--poodle-color-text-inverse) 8%, transparent)` |

### Root closed (`.collapsible[data-open="false"]`)

| Property | Value |
|----------|-------|
| `gap` | `0` |

### Root disabled (`.collapsible[data-disabled="true"]`)

| Property | Value |
|----------|-------|
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Root highlighted (`.collapsible[data-highlighted="true"]`)

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--poodle-color-accent-base) 55%, transparent)` |
| `box-shadow` | `0 0 0 0.125rem color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent)` |

### Root density (`.collapsible[data-density]`)

| Density | `padding-inline` |
|---------|------------------|
| `compact` | `0.5rem` |
| `default` | `var(--poodle-space-panel-x)` |
| `comfortable` | `1rem` |

### Trigger `.collapsible__trigger`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `minmax(0, 1fr) auto` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-md)` |
| `width` | `100%` |
| `padding` | `0` |
| `border` | `0` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `text-align` | `left` |
| `font` | `inherit` |

### Trigger disabled (`:disabled`)

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |

### Trigger focus (`:focus-visible`)

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |
| `border-radius` | `calc(var(--poodle-radius-control) - 0.125rem)` |

### Heading `.collapsible__heading`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `min-width` | `0` |

### Title `.collapsible__title`

| Property | Value |
|----------|-------|
| `font-family` | `var(--poodle-typography-heading-family)` |
| `font-size` | `1rem` |
| `font-weight` | `700` |
| `line-height` | `1.2` |

### Description `.collapsible__description`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.8125rem` |
| `line-height` | `1.45` |

### Indicator `.collapsible__indicator`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `0.75rem` |
| `line-height` | `1` |
| `transition` | `transform var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

### Indicator open (`.collapsible[data-open="true"] .collapsible__indicator`)

| Property | Value |
|----------|-------|
| `transform` | `rotate(180deg)` |

### Content `.collapsible__content`

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `padding-top` | `0.125rem` |
| `transition` | `slide` — height animates from/to 0 over 180ms on mount/unmount |

### Size adjustments

| Size | title font-size | description font-size |
|------|----------------|----------------------|
| `xs` | `0.8125rem` | `0.6875rem` |
| `sm` | `0.875rem` | `0.75rem` |
| `md` | `1rem` | `0.8125rem` |
| `lg` | `1.0625rem` | `0.875rem` |
| `xl` | `1.125rem` | `0.9375rem` |

## 9. Svelte Notes

- `data-size` attribute on root reflects the resolved size
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)
- Uses `<Icon name="chevron-down" size="sm" />` for the indicator
- Module-level `nextCollapsibleId` counter (incremented per instance) generates
  unique ARIA id pairs for trigger/content association
- Content conditionally rendered with `{#if isOpen}` (mount/unmount, not
  show/hide)
- Data attributes `data-open` and `data-disabled` on root drive style switching
- Controlled mode: `open !== null`; uncontrolled mode: internal
  `uncontrolledOpen` tracks state
- Named `trigger` slot allows custom heading content while preserving button
  semantics; receives `{ isOpen }` slot prop
- When trigger slot is used, `title` and `description` props are ignored
- `aria-label` applied to trigger only when no `title` prop is present
- Disabled state applies opacity at root level and `cursor: not-allowed` at
  trigger level (no separate opacity on trigger since root covers it)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::collapsible`
- GPUI must expose button role with expanded state on trigger
- Region association between trigger and content must be maintained
- Disabled state must suppress interaction, show not-allowed cursor, and reduce
  root opacity
- Chevron rotation animation maps to indicator transform
- Content expand/collapse must animate height over ~180ms (equivalent to Svelte
  `slide` transition)
- Gap collapse (0.5rem to 0) when closed must be replicated

## 10a. Jetstream Notes

- `Collapsible::from_spec(spec, theme).content(...).on_open_change(...)`,
  carrying the open state the region is moving **to**.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] open-state semantics match (`aria-expanded`, `aria-controls`)
- [ ] region role and `aria-labelledby` on content match
- [ ] disabled behavior matches (opacity on root, cursor on trigger)
- [ ] onOpenChange callback payload matches
- [ ] controlled and uncontrolled modes match
- [ ] trigger slot override preserves button semantics
- [ ] `aria-label` fallback when no title matches

### Tier 2: Visual Parity

- [ ] all five sizes visually match per size table
- [ ] root border color-mix (36% border-subtle) matches
- [ ] root background color-mix (40% background-elevated, background-panel) matches
- [ ] root inset box-shadow (8% text-inverse) matches
- [ ] root border-radius uses radius-surface
- [ ] root padding (0.625rem vertical + density-driven inline) matches
- [ ] gap collapse (space.stack.md open, 0 closed) matches
- [ ] highlighted state (accent-base 55% border + 12% halo) matches
- [ ] trigger grid layout (1fr auto) matches
- [ ] title typography (heading-family, 1rem, 700, 1.2) matches
- [ ] description typography (0.8125rem, 1.45, secondary) matches
- [ ] indicator chevron rotation (180deg) and transition match
- [ ] focus ring appearance matches (width, color, offset, radius)
- [ ] disabled opacity uses state-opacity-disabled
- [ ] content padding-top (0.125rem) matches

### Tier 3: Implementation Freedom

- [ ] module-level id counter mechanism is platform-owned
- [ ] content mount/unmount vs show/hide is platform-owned
- [ ] data attribute naming is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| slot-based trigger content may differ by runtime | composition freedom is acceptable | allowed | keep disclosure semantics strict |
| CSS color-mix vs GPUI color blending | different color systems per platform | allowed | same visual result required |
| module-level id counter vs GPUI id generation | different id generation mechanisms | allowed | unique ARIA id pairs required |

## 13. Specimen Definitions

### Group: Default (closed)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Closed collapsible | `<Collapsible title="Project settings" description="Configure build options and deploy targets.">` with content | Collapsed disclosure section showing title and description in the trigger; chevron indicator points down; content hidden; gap collapsed to 0 |

### Group: Default open

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Open collapsible | `<Collapsible title="Advanced options" defaultOpen>` with content | Expanded disclosure section showing title in trigger; chevron rotated 180deg; content region visible below trigger with 0.5rem gap |

### Group: Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled collapsible | `<Collapsible title="Locked section" description="Requires admin access." isDisabled>` with content | Collapsed disclosure with reduced opacity (disabled-opacity token); trigger shows not-allowed cursor; clicking does not toggle open state |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: settings groups, sidebars, drawers, docs notes, compact
  web disclosure surfaces
- future follow-up: coordinate with Accordion for grouped disclosure patterns
