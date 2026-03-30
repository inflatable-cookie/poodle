# StatusBar

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `StatusBar`
- Layer: `foundation`
- Summary: a lightweight shell utility/status row for workspace summary,
  connection state, and context metadata, rendered as a `<footer>` landmark
- In scope: leading and trailing status regions via slots, summary text fallback
  in the leading region, shell-level status packing with space-between layout
- Out of scope: transient notifications, remediation banners, app-specific
  transport/status widgets, global command registries, size/density scaling

## 2. Anatomy

```text
[Root .status-bar]  <footer aria-label="...">
  ├── [Leading .status-bar__leading]  <div>
  │   ├── [Slot: leading] (slot content when provided)
  │   └── [Summary fallback]  <span> (when no leading slot, and summary is set)
  └── [Trailing .status-bar__trailing]  <div> (only rendered when trailing slot has content)
        └── [Slot: trailing]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | `<footer>` element with status bar styling | flex layout, padding, border-top, background, color, font-size, line-height |
| Leading | yes | left-aligned container; shows slot content or summary text fallback | flex layout, gap |
| Trailing | no | right-aligned container; only rendered when trailing slot has content | flex layout, gap |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `summary` | `string \| null` | `null` | no | summary text; displayed in the leading region when no `leading` slot content is provided; also used as fallback `aria-label` |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for the `<footer>`; falls back to `summary`, then to `"Status"` |

### Slots

| Slot | Purpose | Notes |
|------|---------|-------|
| `leading` | left-aligned status items (branch indicator, error count, etc.) | when provided, overrides `summary` text display |
| `trailing` | right-aligned context metadata (cursor position, encoding, language, etc.) | trailing container only renders when this slot has content |

### Controlled And Uncontrolled

- Fully declarative; no internal state
- Content is determined entirely by props and slot content

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | component rendered | status bar with semi-transparent panel background, border-top, secondary text color |
| summary only | `summary` provided, no slots | leading region shows summary text |
| with leading slot | `leading` slot provided | slot content replaces summary text in leading region |
| with trailing slot | `trailing` slot provided | trailing region appears with right-aligned metadata |
| full | both slots + summary | leading slot content shown (summary ignored for display), trailing slot shown |

## 5. Events

No events are dispatched by this component. Interactive controls placed in
slots dispatch their own events.

## 6. Accessibility

### Semantics

- Root: `<footer>` element providing landmark semantics
- `aria-label` resolved in order: `ariaLabel` prop > `summary` prop > `"Status"`
- Status bar content must remain textual and keyboard-reachable where interactive
  controls are placed in slots
- Shell utility metadata must not be the only place a critical error is communicated

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves focus through any interactive controls placed in slots |

### Focus And Announcement

- focus entry: no focusable elements by default; interactive slot content
  participates in tab order
- live-region behavior: none

## 7. Layout

### Sizing

- Root: full-width flex row with wrapping
- No explicit height; determined by content and padding
- No size or density scaling props

### Composition

- parent expectations: bottom of application shell, workspace frame, or panel
- child expectations: slot content only (status items, metadata labels,
  interactive controls)
- resizing: fills parent width, wraps on narrow viewports

## 8. Token Usage -- Exact Values

### Root `.status-bar`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `align-items` | `center` |
| `justify-content` | `space-between` |
| `gap` | `var(--poodle-space-inline-md)` |
| `padding` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `border-top` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent)` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.8125rem` |
| `line-height` | `1.5` |

### Leading `.status-bar__leading`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-sm)` |

### Trailing `.status-bar__trailing`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-sm)` |

### aria-label Resolution

The `aria-label` on the `<footer>` is resolved as:
1. `ariaLabel` prop if provided
2. `summary` prop if provided
3. `"Status"` as final fallback

## 9. Svelte Notes

- Root is a native `<footer>` element
- No `data-size` or `data-density` attributes; this component does not
  participate in size/density scaling
- Leading region always renders; when the `leading` slot has content, it is
  displayed; otherwise, the `summary` prop text is shown in a `<span>`
- Trailing region only renders when the `trailing` slot has content
  (checked via `$$slots.trailing`)
- The component has no internal state and dispatches no events
- Background uses `color-mix` for semi-transparent panel appearance
- Font-size is hardcoded at `0.8125rem` (not token-driven) as the bar is
  intentionally compact

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::status_bar`
- Spec struct: `StatusBarSpec` in primitives crate
- Slot-equivalent: leading and trailing child element lists
- `<footer>` landmark semantics must be mapped to GPUI accessibility API
- Background uses color-mix equivalent in Rust
- No size/density scaling needed

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] `<footer>` landmark with `aria-label` resolution (prop > summary > "Status")
- [ ] leading region shows slot content or summary text fallback
- [ ] trailing region only rendered when content exists
- [ ] no events dispatched by the component itself

### Tier 2: Visual Parity

- [ ] background: 94% panel color mixed with transparent
- [ ] border-top: 0.0625rem solid border-subtle
- [ ] font-size: 0.8125rem, line-height: 1.5
- [ ] color: text-secondary
- [ ] padding: panel-y panel-x
- [ ] gap between leading and trailing: space-inline-md
- [ ] gap within leading/trailing: space-inline-sm
- [ ] flex-wrap on root and inner containers

### Tier 3: Implementation Freedom

- [ ] slot rendering mechanism is platform-owned
- [ ] color-mix implementation method is platform-owned

## 12. Specimen Definitions

### Group: Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | `summary="Ready"`, leading slot: "main" branch indicator + "0 errors" status, trailing slot: "Ln 42, Col 18" + "UTF-8" + "TypeScript" | Full-width status bar with summary text in leading area, branch and error items on the left, cursor/encoding/language metadata on the right |

### Group: Summary only

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Summary only | `summary="3 items selected"`, no slots | Status bar showing only "3 items selected" text in the leading region; no trailing region |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: workspace shells, IDE-style editors, admin dashboards
- future follow-up: connection state indicators, sync status, interactive
  status items
