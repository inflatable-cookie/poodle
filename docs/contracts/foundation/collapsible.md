# Collapsible

> **Surface elevation**: Collapsible is a surface consumer (50% strong contrast) — see [surface-elevation.md](./surface-elevation.md).

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Collapsible`
- Layer: `foundation`
- Summary: a single disclosure container with a trigger and revealable content
- In scope: controlled or uncontrolled open state, heading copy, trigger
  affordance, inline reveal posture, disabled state, trigger slot override
- Out of scope: grouped selection logic, routing, or full accordion ownership

## 2. Anatomy

```text
[Root .collapsible]  <section data-open data-disabled>
  ├── [Trigger .collapsible__trigger]  <button aria-expanded aria-controls>
  │   ├── [Heading .collapsible__heading]  <span>
  │   │   ├── [Slot: trigger { isOpen }]  (named slot, or default heading)
  │   │   ├── [Title .collapsible__title]  <span> (when no trigger slot)
  │   │   └── [Description .collapsible__description]  <span> (optional, when no trigger slot)
  │   └── [Indicator .collapsible__indicator]  <span aria-hidden>
  │       └── [Icon name="chevron-down" size="sm"]
  └── [Content .collapsible__content]  <div role="region" aria-labelledby> (conditional)
      └── [Slot: default]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | disclosure section container | border, radius, background, shadow, padding, gap |
| Trigger | yes | interactive button for expand/collapse | grid layout, color, cursor, focus ring |
| Heading | yes | title/description container or trigger slot target | gap |
| Title | no | heading text (when no trigger slot) | heading family, weight, size |
| Description | no | supporting description (when no trigger slot) | secondary color, size |
| Indicator | yes | chevron rotation indicator | secondary color, transition |
| Content | conditional | expanded content region | min-width, padding-top |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `open` | `boolean \| null` | `null` | no | controlled open state; `null` = uncontrolled |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial open state |
| `title` | `string \| null` | `null` | no | visible heading text |
| `description` | `string \| null` | `null` | no | visible supporting description |
| `isDisabled` | `boolean` | `false` | no | suppresses interaction |
| `ariaLabel` | `string \| null` | `null` | no | accessible label when no title present |

### Slots

| Slot | Purpose | Slot Props |
|------|---------|------------|
| default | collapsible content | none |
| trigger (named) | custom trigger heading content | `{ isOpen: boolean }` |

### Controlled And Uncontrolled

- controlled: `open` (non-null) plus `openChange` event
- uncontrolled: `defaultOpen` with internal state management

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | default | content hidden, indicator points down, gap collapses to 0 |
| open | `open=true` or toggled | content visible, indicator rotated 180deg, gap active |
| disabled | `isDisabled=true` | reduced opacity on root, cursor not-allowed on trigger |
| focus | keyboard focus on trigger | focus ring visible |

### Component States

- `data-open`: `"true"` or `"false"` on root `.collapsible`
- `data-disabled`: `"true"` or `"false"` on root `.collapsible`

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `openChange` | user toggles disclosure | `{ open: boolean }` | suppressed while disabled (button disabled) |

## 6. Accessibility

### Semantics

- Root: `<section>` element
- Trigger: `<button type="button">` with `aria-expanded` and `aria-controls`
- Trigger: `id="pug-collapsible-trigger-{collapsibleId}"`
- Trigger: `aria-label` from prop when no `title` is present
- Content: `role="region"` with `aria-labelledby` pointing to trigger id
- Content: `id="pug-collapsible-content-{collapsibleId}"`
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

- Root: vertical grid with 0.5rem gap when open, 0 gap when closed
- Content appears inline below the trigger
- All grid containers use `min-width: 0` for overflow safety

### Composition

- parent expectations: settings groups, sidebars, drawers, docs notes, compact
  web disclosure surfaces
- child expectations: arbitrary slot content
- resizing: root stretches to container width; content flows naturally

## 8. Token Usage — Exact Values

### Root `.collapsible` (open state)

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.5rem` |
| `min-width` | `0` |
| `padding` | `0.875rem 1rem` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 42%, transparent)` |
| `border-radius` | `var(--pug-radius-surface)` |
| `background` | `color-mix(in srgb, var(--pug-surface) 50%, var(--pug-color-background-elevated))` |
| `box-shadow` | `inset 0 0.0625rem 0 color-mix(in srgb, var(--pug-color-text-inverse) 8%, transparent)` |

### Root closed (`.collapsible[data-open="false"]`)

| Property | Value |
|----------|-------|
| `gap` | `0` |

### Root disabled (`.collapsible[data-disabled="true"]`)

| Property | Value |
|----------|-------|
| `opacity` | `var(--pug-state-opacity-disabled)` |

### Trigger `.collapsible__trigger`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `minmax(0, 1fr) auto` |
| `align-items` | `center` |
| `gap` | `0.75rem` |
| `width` | `100%` |
| `padding` | `0` |
| `border` | `0` |
| `background` | `transparent` |
| `color` | `var(--pug-color-text-primary)` |
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
| `outline` | `var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |
| `border-radius` | `calc(var(--pug-radius-control) - 0.125rem)` |

### Heading `.collapsible__heading`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.3125rem` |
| `min-width` | `0` |

### Title `.collapsible__title`

| Property | Value |
|----------|-------|
| `font-family` | `var(--pug-typography-heading-family)` |
| `font-size` | `1rem` |
| `font-weight` | `700` |
| `line-height` | `1.2` |

### Description `.collapsible__description`

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-text-secondary)` |
| `font-size` | `0.8125rem` |
| `line-height` | `1.45` |

### Indicator `.collapsible__indicator`

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-text-secondary)` |
| `font-family` | `var(--pug-typography-code-family)` |
| `font-size` | `0.75rem` |
| `line-height` | `1` |
| `transition` | `transform var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard)` |

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

## 9. Svelte Notes

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

- expected crate/module surface: `pug_gpui::primitives::collapsible`
- GPUI must expose button role with expanded state on trigger
- Region association between trigger and content must be maintained
- Disabled state must suppress interaction, show not-allowed cursor, and reduce
  root opacity
- Chevron rotation animation maps to indicator transform
- Content expand/collapse must animate height over ~180ms (equivalent to Svelte
  `slide` transition)
- Gap collapse (0.5rem to 0) when closed must be replicated

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] open-state semantics match (`aria-expanded`, `aria-controls`)
- [ ] region role and `aria-labelledby` on content match
- [ ] disabled behavior matches (opacity on root, cursor on trigger)
- [ ] openChange event payload matches
- [ ] controlled and uncontrolled modes match
- [ ] trigger slot override preserves button semantics
- [ ] `aria-label` fallback when no title matches

### Tier 2: Visual Parity

- [ ] root border color-mix (42% border-subtle) matches
- [ ] root background color-mix (88% elevated, surface) matches
- [ ] root inset box-shadow (8% text-inverse) matches
- [ ] root border-radius uses radius-surface
- [ ] root padding (0.875rem 1rem) matches
- [ ] gap collapse (0.5rem open, 0 closed) matches
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
