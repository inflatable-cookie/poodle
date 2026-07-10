# Toolbar

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `Toolbar`
- Layer: `foundation`
- Summary: a semantic grouping container for compact action controls with
  toolbar semantics, roving focus, and configurable orientation
- In scope: orientation (horizontal/vertical), grouped labeling, roving
  keyboard focus between descendant controls, subtle visual container chrome
- Out of scope: workstation-specific panel headers, menu bars, individual
  tool-item contracts

## 2. Anatomy

```text
[Root .toolbar]  <div role="toolbar">
  └── [Children] (focusable tool items)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | toolbar container with roving focus | border, radius, background, padding, gap |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | no | layout direction and keyboard axis |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"chrome"` | no | semantic size offset from inherited presentation |
| `ariaLabel` | `string \| null` | `null` | no | accessible name when no visible title exists |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |

### Snippets

| Snippet | Purpose |
|---------|---------|
| `children()` | focusable tool items (buttons, toggles, selects) |

### Controlled And Uncontrolled

- layout and focus-management container, no value model

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | subtle container chrome with tool items |
| focus-within | any descendant receives focus | no additional visual change on root (focus belongs to child) |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| horizontal | `orientation="horizontal"` | items flow left-to-right, ArrowLeft/Right navigate |
| vertical | `orientation="vertical"` | items flow top-to-bottom, ArrowUp/Down navigate |

### Behavior Machine

Behavior classification: adapter-owned interaction (g11.004 sweep)

Keyboard handling delegated to focused children; no owned machine state.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | — | — | events belong to slotted child controls |

## 6. Accessibility

### Semantics

- Role: `toolbar` (`role="toolbar"` on root)
- `aria-label`: from prop (required when no visible title exists)
- `aria-orientation`: set to match `orientation` prop (note: not currently implemented in Svelte; only `data-orientation` is set)
- `tabindex="0"` on root for initial focus entry into the toolbar group
- Slotted items: `tabindex="-1"` except the currently roving-focused item (`tabindex="0"`)

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | enters toolbar (focuses roving-active item); next Tab exits toolbar |
| `Arrow Right` | moves focus to next item (horizontal mode, wrapping) |
| `Arrow Left` | moves focus to previous item (horizontal mode, wrapping) |
| `Arrow Down` | moves focus to next item (vertical mode, wrapping) |
| `Arrow Up` | moves focus to previous item (vertical mode, wrapping) |
| `Home` | moves focus to first item (not currently implemented) |
| `End` | moves focus to last item (not currently implemented) |

### Focus And Announcement

- focus entry: Tab into toolbar focuses the roving-active item (last focused or first)
- focus exit: Tab out of toolbar moves to next focusable element outside
- roving focus: uses `getFocusableElements()` to discover slotted focusable children
- live-region behavior: none
- GPUI-native accessibility mapping notes: GPUI must expose toolbar role with orientation and roving focus semantics through native accessibility tree

## 7. Layout

### Sizing

- Root: inline-flex, auto-sizes to content
- Compact spacing via small gap and padding
- overflow behavior: items do not wrap; parent must ensure sufficient space

### Composition

- parent expectations: panels, headers, form surfaces, utility rows
- child expectations: buttons, icon buttons, toggles, selects, separators
- resizing rules: toolbar auto-sizes to its content; parent constrains width

## 8. Token Usage — Exact Values

### Root `.toolbar` (horizontal, default)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.375rem` |
| `padding` | `0.25rem 0.375rem` (block 0.25, inline 0.375) |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 78%, transparent)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent)` |

### Size Variants

Size scales both block/inline padding and gap. Base (`md`) values are above.

| Size | `padding` | `gap` |
|------|-----------|-------|
| xs | `0.125rem 0.25rem` | `0.25rem` |
| sm | `0.1875rem 0.3125rem` | `0.3125rem` |
| md | `0.25rem 0.375rem` | `0.375rem` |
| lg | `0.3125rem 0.5rem` | `0.5rem` |
| xl | `0.375rem 0.625rem` | `0.625rem` |

### Density Variants

Density overrides only inline padding and gap; block padding (toolbar height) is
never touched by density.

| Density | `padding-inline` | `gap` |
|---------|------------------|-------|
| compact | `0.25rem` | `0.25rem` |
| default | `0.375rem` (base) | `0.375rem` (base) |
| comfortable | `0.5rem` | `0.5rem` |

### Root — vertical orientation

| Property | Value |
|----------|-------|
| `flex-direction` | `column` |
| `align-items` | `stretch` |

## 9. Svelte Notes

- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)
- `data-size` — resolved control size used to drive size-variant padding/gap
- Root rendered as `<div role="toolbar">` with `tabindex="0"` and `aria-label`
  from the `ariaLabel` prop
- `aria-orientation` is **not currently set** by Svelte; orientation is exposed
  through the `data-orientation` data attribute only. The contract still requires
  `aria-orientation` for accessibility (see §6) — Svelte has not yet shipped it
- Roving focus implemented via `getFocusableElements()` utility to discover
  slotted focusable children dynamically
- Arrow key handlers on root intercept directional keys and move focus
- Wrapping: focus wraps from last item to first and vice versa
- `data-orientation` data attribute for CSS targeting
- Items are expected to be focusable elements (buttons, inputs) that participate
  in the roving tabindex pattern

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::toolbar`
- Spec struct: `ToolbarSpec` in primitives crate
- GPUI must expose toolbar role with aria-orientation through accessibility tree
- Roving focus must be implemented using GPUI focus management primitives
- The border color-mix and background color-mix formulas should be replicated
- Vertical orientation maps to column layout with stretch alignment

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] orientation prop means the same thing
- [ ] ariaLabel application matches
- [ ] toolbar role and aria-orientation match
- [ ] roving focus behavior matches (arrow keys, wrapping, Home/End)
- [ ] Tab enters/exits toolbar as a single stop

### Tier 2: Visual Parity

- [ ] gap (0.375rem base) and padding (0.25rem block / 0.375rem inline base) match, plus the size and density variant tables
- [ ] border color-mix formula (border-subtle 78%) matches
- [ ] border-radius (radius-surface) matches
- [ ] background color-mix formula (background-panel 94%) matches
- [ ] vertical orientation layout (column, stretch) matches

### Tier 3: Implementation Freedom

- [ ] roving focus implementation details are platform-owned
- [ ] focusable element discovery mechanism is platform-owned
- [ ] transition timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| roving focus implementation may differ | runtime focus engines differ | allowed | keep toolbar semantics and keyboard behavior strict |
| color-mix formula rendering | GPUI may approximate color-mix | allowed | match visual result as closely as possible |

## 13. Specimen Definitions

### Horizontal (default)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Horizontal (default) | `ariaLabel="Formatting toolbar"`, `orientation="horizontal"` (default) | Horizontal toolbar container with bordered chrome; contains ghost icon buttons (Bold, Italic, Underline), a vertical separator, then ghost icon buttons (Align left, Align center, Align right) |

#### Toolbar Items

| Item | Type | Props / Config | Expected Visual |
|------|------|---------------|-----------------|
| Bold | Button | `variant="ghost"`, `sizeRole="chrome"`, `leadingIcon="bold"`, `ariaLabel="Bold"` | Toolbar chrome action |
| Italic | Button | `variant="ghost"`, `sizeRole="chrome"`, `leadingIcon="italic"`, `ariaLabel="Italic"` | Toolbar chrome action |
| Underline | Button | `variant="ghost"`, `sizeRole="chrome"`, `leadingIcon="underline"`, `ariaLabel="Underline"` | Toolbar chrome action |
| (separator) | Separator | `orientation="vertical"` | Vertical divider line |
| Align left | Button | `variant="ghost"`, `sizeRole="chrome"`, `leadingIcon="text-align-start"`, `ariaLabel="Align left"` | Toolbar chrome action |
| Align center | Button | `variant="ghost"`, `sizeRole="chrome"`, `leadingIcon="text-align-center"`, `ariaLabel="Align center"` | Toolbar chrome action |
| Align right | Button | `variant="ghost"`, `sizeRole="chrome"`, `leadingIcon="text-align-end"`, `ariaLabel="Align right"` | Toolbar chrome action |

### With Primary Action

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With primary action | `ariaLabel="Actions toolbar"` | Horizontal toolbar with secondary text buttons (Discard, Save draft), a vertical separator, then a primary text button (Publish) |

#### Toolbar Items

| Item | Type | Props / Config | Expected Visual |
|------|------|---------------|-----------------|
| Discard | Button | `variant="secondary"`, `sizeRole="chrome"` | Secondary action offset below the local baseline |
| Save draft | Button | `variant="secondary"`, `sizeRole="chrome"` | Secondary action offset below the local baseline |
| (separator) | Separator | `orientation="vertical"` | Vertical divider line |
| Publish | Button | `sizeRole="prominent"` (primary default) | Primary action offset above the local baseline |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: formatting bars, shell utility rows, compact tool groups,
  panel headers
- future follow-up: consider separator/divider support within toolbar if needed;
  keep product-specific shell headers in composite or workstation contracts
