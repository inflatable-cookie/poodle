# Accordion

> **Surface elevation**: Accordion is a surface consumer (50% strong contrast) — see [surface-elevation.md](./surface-elevation.md).

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `Accordion`
- Layer: `foundation`
- Summary: grouped disclosure sections with explicit trigger, expanded state,
  and panel ownership
- In scope: single or multiple expansion, optional collapse-to-none posture,
  keyboard and focusable trigger semantics, per-item disabled state
- Out of scope: tree navigation, app-specific settings IA, or arbitrary shell
  navigation ownership

## 2. Anatomy

```text
[Root .accordion]  <div role="group"?>
  ├── [Item .accordion__item]  <section data-open>
  │   ├── [Heading .accordion__heading]  <h3>
  │   │   └── [Trigger .accordion__trigger]  <button aria-expanded aria-controls>
  │   │       ├── [Summary .accordion__summary]  <span>
  │   │       │   ├── [Title .accordion__title]  <span>
  │   │       │   └── [Description .accordion__description]  <span> (optional)
  │   │       └── [Indicator .accordion__indicator]  <span aria-hidden>
  │   │           └── [Icon name="chevron-down"]
  │   └── [Panel .accordion__panel]  <div role="region" aria-labelledby> (conditional)
  │       └── [Snippet: children(item, isOpen)]
  └── [...]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | accordion container, optional group role | gap |
| Item | yes | per-item disclosure section | border, radius, background, shadow, padding |
| Heading | yes | semantic h3 wrapper | margin reset |
| Trigger | yes | interactive button for expand/collapse | grid layout, color, cursor, focus ring |
| Summary | yes | title/description container | gap |
| Title | yes | item label text | heading family, weight, size |
| Description | no | supporting description | secondary color, size |
| Indicator | yes | chevron rotation indicator | secondary color, transition |
| Panel | conditional | expanded content region | min-width |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `AccordionItem[]` | `[]` | yes | array of item descriptors |
| `value` | `string \| string[] \| null` | `null` | no | controlled expanded value(s); `null` = uncontrolled |
| `defaultValue` | `string \| string[] \| null` | `null` | no | uncontrolled initial expanded value(s) |
| `selectionMode` | `"single" \| "multiple"` | `"single"` | no | single or multiple expansion |
| `collapsible` | `boolean` | `true` | no | whether all items can be collapsed in single mode |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for the root container |

### AccordionItem Interface

| Property | Type | Required | Notes |
|----------|------|----------|-------|
| `value` | `string` | yes | unique identifier for this item |
| `label` | `string` | yes | visible title text |
| `description` | `string` | no | supporting description text |
| `disabled` | `boolean` | no | suppresses interaction for this item |

### Snippets

| Snippet | Purpose | Arguments |
|---------|---------|-----------|
| `children` | panel content for each item | `(item: AccordionItem, isOpen: boolean)` |

### Controlled And Uncontrolled

- controlled: supplying `value` makes it host-owned through `onValueChange`
- uncontrolled: `defaultValue` with internal state; defaults to `null` (single)
  or `[]` (multiple) when `defaultValue` is `null`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| collapsed | default | panel hidden, indicator points down |
| expanded | item value in active set | panel visible, indicator rotated 180deg |
| disabled | `item.disabled=true` | trigger non-interactive, cursor not-allowed, reduced opacity |
| focus | keyboard focus on trigger | focus ring visible |

### Component States

- `data-open`: `"true"` or `"false"` on each `.accordion__item`
- Items rendered via `{#each items as item (item.value)}` with keyed identity

### Behavior Machine

Behavior classification: machine-backed (shared `toggleGroupTransition` in
`@inflatable-cookie/poodle-headless`)

Accordion reuses the ToggleGroup machine: open values are a selection over
items. `selectionMode` maps directly; `collapsible` maps to
`allowDeactivation` (single mode reselect closes the panel only when
collapsible). Disabled items are inert. Every accepted toggle emits
`emitValueChange(nextValue)`.

- Machinery dependencies: none (native buttons provide keyboard/focus).

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onValueChange` | user expands or collapses an item | `string \| string[] \| null` | called on every toggle; suppressed for disabled items |

## 6. Accessibility

### Semantics

- Root: `role="group"` when `selectionMode="multiple"`, no role when single
- Root: `aria-label` from prop when provided
- Heading: `<h3>` wrapping each trigger
- Trigger: `<button type="button">` with `aria-expanded` and `aria-controls`
- Trigger: `id="poodle-accordion-trigger-{accordionId}-{item.value}"`
- Panel: `role="region"` with `aria-labelledby` pointing to trigger id
- Panel: `id="poodle-accordion-panel-{accordionId}-{item.value}"`
- Module-level `nextAccordionId` counter provides unique id namespaces

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` | toggles the focused item |
| `Space` | toggles the focused item |
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

- Root: vertical grid with `space-stack-md` gap between items
- Items: self-contained bordered sections with internal grid layout (internal `accordion-item-gap`, `0.625rem` default)
- Panels: expand inline below their trigger within the item container
- All grid containers use `min-width: 0` for overflow safety

### Composition

- parent expectations: settings pages, FAQ surfaces, docs sections, marketing
  disclosure patterns
- child expectations: arbitrary slot content per panel
- resizing: items stretch to container width; panel content flows naturally

## 8. Token Usage — Exact Values

### Root `.accordion`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-space-stack-md)` |
| `min-width` | `0` |

### Item `.accordion__item`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-accordion-item-gap)` (`0.625rem` default) |
| `min-width` | `0` |
| `padding` | `0.625rem var(--poodle-space-panel-x)` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 36%, transparent)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 40%, var(--poodle-color-background-panel))` |
| `box-shadow` | `inset 0 0.0625rem 0 color-mix(in srgb, var(--poodle-color-text-inverse) 8%, transparent)` |

### Heading `.accordion__heading`

| Property | Value |
|----------|-------|
| `margin` | `0` |

### Trigger `.accordion__trigger`

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
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Trigger focus (`:focus-visible`)

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |
| `border-radius` | `calc(var(--poodle-radius-control) - 0.125rem)` |

### Summary `.accordion__summary`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `min-width` | `0` |

### Title `.accordion__title`

| Property | Value |
|----------|-------|
| `font-family` | `var(--poodle-typography-heading-family)` |
| `font-size` | `1rem` |
| `font-weight` | `700` |
| `line-height` | `1.2` |

### Description `.accordion__description`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.8125rem` |
| `line-height` | `1.45` |

### Indicator `.accordion__indicator`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `0.75rem` |
| `line-height` | `1` |
| `transition` | `transform var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

### Indicator open (`.accordion__item[data-open="true"] .accordion__indicator`)

| Property | Value |
|----------|-------|
| `transform` | `rotate(180deg)` |

### Panel `.accordion__panel`

| Property | Value |
|----------|-------|
| `min-width` | `0` |
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
- Uses `<Icon name="chevron-down" />` for the indicator (no explicit `size`)
- Module-level `nextAccordionId` counter (incremented per instance) generates
  unique ARIA id pairs for trigger/panel association
- Items rendered via keyed `{#each}` block on `item.value`
- Panel conditionally rendered with `{#if openValues.includes(item.value)}`
  (mount/unmount, not show/hide)
- Data attribute `data-open` on each item section drives indicator rotation
- Controlled mode: `value !== null`; uncontrolled mode: internal
  `uncontrolledValue` tracks state
- Single mode with `collapsible=false`: toggling an open item keeps it open
- The public API is data-driven (items array + slot) rather than compound
  component composition

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::accordion`
- GPUI must expose button role with expanded state on each trigger
- Region association between trigger and panel must be maintained
- Group role required in multiple mode
- Per-item disabled state must suppress interaction and reduce opacity
- Chevron rotation animation maps to indicator transform
- Panel expand/collapse must animate height over ~180ms (equivalent to Svelte
  `slide` transition)

## 10a. Jetstream Notes

- `Accordion::from_spec(spec, theme).on_change(...)`, carrying the value of the
  item whose trigger was pressed.
- The item, not the resulting expanded set: single- and multi-expand are the
  host's policy, and returning a set would move that decision into the
  component.
- The whole header row is the trigger, not just the chevron.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] expansion semantics match (`aria-expanded`, `aria-controls`)
- [ ] region role and `aria-labelledby` on panels match
- [ ] group role present in multiple mode, absent in single mode
- [ ] disabled item behavior matches (cursor, opacity, non-interactive)
- [ ] single vs multiple expansion posture matches
- [ ] collapsible behavior matches in single mode
- [ ] `onValueChange` callback payload matches
- [ ] controlled and uncontrolled modes match

### Tier 2: Visual Parity

- [ ] all five sizes visually match per size table
- [ ] item border color-mix (36% border-subtle) matches
- [ ] item background color-mix (40% background-elevated, background-panel) matches
- [ ] item inset box-shadow (8% text-inverse) matches
- [ ] item border-radius uses radius-surface
- [ ] item padding (`0.625rem` block, `space-panel-x` inline) matches
- [ ] trigger grid layout (1fr auto) matches
- [ ] title typography (heading-family, 1rem, 700, 1.2) matches
- [ ] description typography (0.8125rem, 1.45, secondary) matches
- [ ] indicator chevron rotation (180deg) and transition match
- [ ] focus ring appearance matches (width, color, offset, radius)
- [ ] disabled opacity uses state-opacity-disabled

### Tier 3: Implementation Freedom

- [ ] module-level id counter mechanism is platform-owned
- [ ] panel mount/unmount vs show/hide is platform-owned
- [ ] data attribute naming is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| panel rendering may stay slot-driven in Svelte | runtime composition differs | allowed | keep trigger and expansion semantics strict |
| CSS color-mix vs GPUI color blending | different color systems per platform | allowed | same visual result required |
| module-level id counter vs GPUI id generation | different id generation mechanisms | allowed | unique ARIA id pairs required |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Single selection

Accordion with `type: "single"` — only one item open at a time:

| Item title | Panel content | Initial state |
|-----------|---------------|---------------|
| Getting started | "Follow these steps to set up your project and start building." | open (`defaultValue="getting-started"`) |
| API reference | "Complete documentation for all available endpoints and methods." | closed |
| Accessibility | "Guidelines for building accessible components with proper ARIA support." | closed |

### Multiple selection

Accordion with `type: "multiple"` — multiple items can be open simultaneously:

| Item title | Panel content | Initial state |
|-----------|---------------|---------------|
| Design tokens | "Tokens define the visual language of your application." | open (`defaultValue={["design", "keyboard"]}`) |
| Keyboard shortcuts | "Common shortcuts for navigating and interacting with components." | open (`defaultValue={["design", "keyboard"]}`) |
| Known issues | "Current limitations and workarounds for known bugs." | closed |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: settings pages, FAQ surfaces, docs sections, marketing
  or web product disclosure patterns
- future follow-up: coordinate with Collapsible for single-item disclosure
  patterns
