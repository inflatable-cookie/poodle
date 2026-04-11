# Separator

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Separator`
- Layer: `foundation`
- Summary: a low-emphasis visual division primitive between adjacent content
  groups, supporting both decorative and semantic modes
- In scope: horizontal and vertical separators, decorative versus semantic
  separators, subtle and default tone emphasis
- Out of scope: resize handles, draggable splitters, tab indicators, split-view
  dividers

## 2. Anatomy

```text
[Root .separator]  <div>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | dividing rule element | separator color, stroke width |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `orientation` | `Orientation: "horizontal" \| "vertical"` | `"horizontal"` | no | axis of the dividing rule |
| `decorative` | `boolean` | `true` | no | when false, exposes semantic separator role to assistive technology |
| `tone` | `SeparatorTone: "subtle" \| "default"` | `"subtle"` | no | divider color emphasis |

### Controlled And Uncontrolled

- no controlled value model

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| subtle (default) | `tone="subtle"` | low-emphasis divider using mixed border color |
| default | `tone="default"` | stronger divider using full border-default color |
| horizontal (default) | `orientation="horizontal"` | spans full width, minimal height |
| vertical | `orientation="vertical"` | minimal width, stretches to container height |

### Component States

No internal state.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | visual primitive only |

## 6. Accessibility

### Semantics

- Role: none when `decorative=true`; `role="separator"` when `decorative=false`
- Required attributes:
  - `aria-hidden="true"` when `decorative=true`
  - `aria-orientation` set to orientation value when `decorative=false`
- Optional attributes: none
- Labeling rules: decorative separators must be hidden from assistive
  technology; semantic separators convey structural division

### Keyboard

| Key | Behavior |
|-----|----------|
| none | no intrinsic keyboard behavior; separator is never focusable |

### Focus And Announcement

- focus entry: never focusable
- focus exit: n/a
- live-region behavior: none

## 7. Layout

### Sizing

- horizontal: spans full available width (`width: 100%`), minimal block size
  (`min-height: 0.0625rem`)
- vertical: minimal inline size (`width: 0.0625rem`), stretches to container
  height (`align-self: stretch`, `min-height: 100%`)
- flex behavior: `flex: 0 0 auto` prevents growth or shrinkage

### Composition

- parent expectations: flex-based layout (Stack, Inline, toolbar), adjacent
  content groups
- child expectations: none (no children)
- resizing rules: follows axis and parent constraints; does not grow or shrink

## 8. Token Usage — Exact Values

### CSS Custom Properties (on .separator)

| Var | Default Value |
|-----|---------------|
| `--poodle-separator-color` | `color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent)` |

### Root (.separator) — base styles

| Property | Value |
|----------|-------|
| `flex` | `0 0 auto` |
| `background` | `var(--poodle-separator-color)` |

### Tone: default — .separator[data-tone="default"]

| Var | Value |
|-----|-------|
| `--poodle-separator-color` | `var(--poodle-color-border-default)` |

### Orientation: horizontal — .separator[data-orientation="horizontal"]

| Property | Value |
|----------|-------|
| `width` | `100%` |
| `min-height` | `0.0625rem` |

### Orientation: vertical — .separator[data-orientation="vertical"]

| Property | Value |
|----------|-------|
| `width` | `0.0625rem` |
| `align-self` | `stretch` |
| `min-height` | `100%` |

### Data Attributes

| Attribute | Source |
|-----------|--------|
| `data-orientation` | `orientation` prop |
| `data-tone` | `tone` prop |

### Accessibility Attributes (conditional)

| Condition | Attributes |
|-----------|------------|
| `decorative=true` | `aria-hidden="true"` |
| `decorative=false` | `role="separator"`, `aria-orientation="{orientation}"` |

## 9. Svelte Notes

- rendered as a `<div>` element
- uses data attributes (`data-orientation`, `data-tone`) to drive CSS variant
  selectors
- the separator is drawn using `background` color rather than `border`, keeping
  the element itself as the visible 1px rule
- `0.0625rem` equals 1px at default root font size
- `color-mix` at 72% creates a softer subtle tone compared to full border color
- when `decorative=false`, the element receives `role="separator"` and
  `aria-orientation`; when `decorative=true`, it receives `aria-hidden="true"`

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::separator`
- Spec struct: `SeparatorSpec` in primitives crate
- GPUI implementation must distinguish decorative rules from semantic
  separators in the native accessibility tree
- decorative rules should not surface as focusable or named accessibility nodes
- the 1px rule can be drawn as a filled rect or line with matching color
- color-mix at 72% must produce equivalent visual result

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] decorative vs semantic meaning matches (aria-hidden vs role="separator")
- [ ] orientation semantics match (aria-orientation when semantic)
- [ ] separator is never focusable in either runtime

### Tier 2: Visual Parity

- [ ] subtle tone color-mix (72% border-subtle) matches
- [ ] default tone uses full border-default color
- [ ] stroke weight matches (0.0625rem / 1px)
- [ ] horizontal spans full width
- [ ] vertical stretches to container height

### Tier 3: Implementation Freedom

- [ ] HTML div with background vs GPUI filled rect stays internal
- [ ] CSS data-attribute selectors vs Rust match stays internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| color-mix transparency blending | GPUI may use direct alpha blending instead of CSS color-mix | allowed | same visual result required |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Horizontal (default)

Vertical stack with text above and below:

| Layout |
|--------|
| "Content above" |
| `<Separator />` |
| "Content below" |

### Vertical

Horizontal row with separators between text:

| Layout |
|--------|
| "Left" | `<Separator orientation="vertical" />` | "Center" | `<Separator orientation="vertical" />` | "Right" |

### Decorative

Single separator with `role: "none"`:

| Props |
|-------|
| `role="none"` (decorative, no semantic meaning) |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: menus, forms, panels, detail layouts, toolbar groups,
  settings sections
- future follow-up: keep draggable splitters and resize handles in separate
  contracts (SplitView)
