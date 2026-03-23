# Toolbar

Status: detailed contract
Updated: 2026-03-15

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
  └── [Default Slot] (focusable tool items)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | toolbar container with roving focus | border, radius, background, padding, gap |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | no | layout direction and keyboard axis |
| `ariaLabel` | `string \| null` | `null` | no | accessible name when no visible title exists |

### Slots

| Slot | Purpose |
|------|---------|
| default | focusable tool items (buttons, toggles, selects) |

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
| `gap` | `0.25rem` |
| `padding` | `0.25rem` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--flint-color-border-subtle) 78%, transparent)` |
| `border-radius` | `var(--flint-radius-surface)` |
| `background` | `color-mix(in srgb, var(--flint-color-background-panel) 94%, transparent)` |

### Root — vertical orientation

| Property | Value |
|----------|-------|
| `flex-direction` | `column` |
| `align-items` | `stretch` |

## 9. Svelte Notes

- Root rendered as `<div role="toolbar">` with `tabindex="0"`
- `aria-orientation` attribute set from `orientation` prop
- Roving focus implemented via `getFocusableElements()` utility to discover
  slotted focusable children dynamically
- Arrow key handlers on root intercept directional keys and move focus
- Wrapping: focus wraps from last item to first and vice versa
- `data-orientation` data attribute for CSS targeting
- Items are expected to be focusable elements (buttons, inputs) that participate
  in the roving tabindex pattern

## 10. GPUI Notes

- expected crate/module surface: `flint_gpui::primitives::toolbar`
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

- [ ] gap (0.25rem) and padding (0.25rem) match
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
| Bold | Button | `variant="ghost"`, `size="sm"`, `leadingIcon="bold"`, `ariaLabel="Bold"` | Small ghost icon button |
| Italic | Button | `variant="ghost"`, `size="sm"`, `leadingIcon="italic"`, `ariaLabel="Italic"` | Small ghost icon button |
| Underline | Button | `variant="ghost"`, `size="sm"`, `leadingIcon="underline"`, `ariaLabel="Underline"` | Small ghost icon button |
| (separator) | Separator | `orientation="vertical"` | Vertical divider line |
| Align left | Button | `variant="ghost"`, `size="sm"`, `leadingIcon="text-align-start"`, `ariaLabel="Align left"` | Small ghost icon button |
| Align center | Button | `variant="ghost"`, `size="sm"`, `leadingIcon="text-align-center"`, `ariaLabel="Align center"` | Small ghost icon button |
| Align right | Button | `variant="ghost"`, `size="sm"`, `leadingIcon="text-align-end"`, `ariaLabel="Align right"` | Small ghost icon button |

### With Primary Action

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With primary action | `ariaLabel="Actions toolbar"` | Horizontal toolbar with secondary text buttons (Discard, Save draft), a vertical separator, then a primary text button (Publish) |

#### Toolbar Items

| Item | Type | Props / Config | Expected Visual |
|------|------|---------------|-----------------|
| Discard | Button | `variant="secondary"`, `size="sm"` | Small secondary button with text |
| Save draft | Button | `variant="secondary"`, `size="sm"` | Small secondary button with text |
| (separator) | Separator | `orientation="vertical"` | Vertical divider line |
| Publish | Button | `size="sm"` (primary default) | Small primary button with text |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: formatting bars, shell utility rows, compact tool groups,
  panel headers
- future follow-up: consider separator/divider support within toolbar if needed;
  keep product-specific shell headers in composite or workstation contracts
