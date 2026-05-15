# StatusBar

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `StatusBar`
- Layer: `foundation`
- Summary: a lightweight shell utility/status row for workspace summary,
  connection state, and context metadata, rendered as a `<footer>` landmark
- In scope: leading and trailing status regions via snippets, summary text fallback
  in the leading region, shell-level status packing with space-between layout
- Out of scope: transient notifications, remediation banners, app-specific
  transport/status widgets, global command registries

## 2. Anatomy

```text
[Root .status-bar]  <footer aria-label="...">
  ├── [Leading .status-bar__leading]  <div>
  │   ├── [leading()] (snippet content when provided)
  │   └── [Summary fallback]  <span> (when no leading snippet, and summary is set)
  └── [Trailing .status-bar__trailing]  <div> (only rendered when trailing snippet has content)
        └── [trailing()]
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
| `chrome` | `boolean` | `false` | no | when true, renders with an explicit border-top and panel background; when false, the bar blends into its container |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl" \| null` | `null` | no | explicit size override; scales font-size and padding-block |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"chrome"` | no | semantic size offset from inherited presentation |
| `density` | `"compact" \| "default" \| "comfortable" \| null` | `null` | no | explicit density override; scales padding-inline and gap |

### Snippets

| Snippet | Purpose | Notes |
|---------|---------|-------|
| `leading()` | left-aligned status items (branch indicator, error count, and so on) | when provided, overrides `summary` text display |
| `trailing()` | right-aligned context metadata (cursor position, encoding, language, and so on) | trailing container only renders when this snippet has content |

### Controlled And Uncontrolled

- Fully declarative; no internal state
- Content is determined entirely by props and snippet content

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | component rendered | status bar with semi-transparent panel background, border-top, secondary text color |
| summary only | `summary` provided, no snippets | leading region shows summary text |
| with leading snippet | `leading()` provided | snippet content replaces summary text in leading region |
| with trailing snippet | `trailing()` provided | trailing region appears with right-aligned metadata |
| full | both snippets + summary | leading snippet content shown (summary ignored for display), trailing snippet shown |

## 5. Events

No events are dispatched by this component. Interactive controls placed in
snippets dispatch their own events.

## 6. Accessibility

### Semantics

- Root: `<footer>` element providing landmark semantics
- `aria-label` resolved in order: `ariaLabel` prop > `summary` prop > `"Status"`
- Status bar content must remain textual and keyboard-reachable where interactive
  controls are placed in snippets
- Shell utility metadata must not be the only place a critical error is communicated

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves focus through any interactive controls placed in snippets |

### Focus And Announcement

- focus entry: no focusable elements by default; interactive snippet content
  participates in tab order
- live-region behavior: none

## 7. Layout

### Sizing

- Root: full-width flex row with wrapping
- No explicit height; determined by content and padding
- No size or density scaling props

### Composition

- parent expectations: bottom of application shell, workspace frame, or panel
- child expectations: snippet content only (status items, metadata labels,
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
| `padding` | `0.375rem 0.75rem` |
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

### Chrome modifier `.status-bar--chrome`

| Property | Value |
|----------|-------|
| `border-top` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent)` |

### Size modifiers

| Selector | Property | Value |
|----------|----------|-------|
| `.status-bar[data-size="xs"]` | `font-size` | `0.6875rem` |
| `.status-bar[data-size="xs"]` | `padding-block` | `0.25rem` |
| `.status-bar[data-size="sm"]` | `font-size` | `0.75rem` |
| `.status-bar[data-size="sm"]` | `padding-block` | `0.3125rem` |
| `.status-bar[data-size="lg"]` | `font-size` | `0.875rem` |
| `.status-bar[data-size="lg"]` | `padding-block` | `0.4375rem` |
| `.status-bar[data-size="xl"]` | `font-size` | `0.9375rem` |
| `.status-bar[data-size="xl"]` | `padding-block` | `0.5rem` |

### Density modifiers

| Selector | Property | Value |
|----------|----------|-------|
| `.status-bar[data-density="compact"]` | `padding-inline` | `0.5rem` |
| `.status-bar[data-density="compact"]` | `gap` | `0.375rem` |
| `.status-bar[data-density="comfortable"]` | `padding-inline` | `1.125rem` |
| `.status-bar[data-density="comfortable"]` | `gap` | `1rem` |

## 9. Svelte Notes

- Root is a native `<footer>` element
- `data-size` and `data-density` attributes are used to apply size and density variants
- Leading region always renders; when the `leading()` snippet has content, it is
  displayed; otherwise, the `summary` prop text is shown in a `<span>`
- Trailing region only renders when the `trailing()` snippet has content
- The component has no internal state and dispatches no events
- Chrome background uses `color-mix` for semi-transparent panel appearance
- Size and density defaults resolve through the shared UI presentation layer

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::status_bar`
- Spec struct: `StatusBarSpec` in primitives crate
- Snippet-equivalent: leading and trailing child element lists
- `<footer>` landmark semantics must be mapped to GPUI accessibility API
- Background uses color-mix equivalent in Rust
- No size/density scaling needed

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] `<footer>` landmark with `aria-label` resolution (prop > summary > "Status")
- [ ] leading region shows snippet content or summary text fallback
- [ ] trailing region only rendered when content exists
- [ ] no events dispatched by the component itself

### Tier 2: Visual Parity

- [ ] chrome background: 94% panel color mixed with transparent
- [ ] chrome border-top: 0.0625rem solid border-subtle
- [ ] font-size: 0.8125rem, line-height: 1.5
- [ ] color: text-secondary
- [ ] default padding: `0.375rem 0.75rem`
- [ ] gap between leading and trailing: space-inline-md
- [ ] gap within leading/trailing: space-inline-sm
- [ ] flex-wrap on root and inner containers
- [ ] size and density variants match the Svelte table

### Tier 3: Implementation Freedom

- [ ] snippet rendering mechanism is platform-owned
- [ ] color-mix implementation method is platform-owned

## 12. Specimen Definitions

### Group: Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | `summary="Ready"`, leading snippet: "main" branch indicator + "0 errors" status, trailing snippet: "Ln 42, Col 18" + "UTF-8" + "TypeScript" | Full-width status bar with summary text in leading area, branch and error items on the left, cursor/encoding/language metadata on the right |

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

## Rust Spec Note

The `poodle-specs` Rust type for this component is `ShellStatusBarSpec`
(file: `packages/contracts/components/src/shell_status_bar.rs`). The Rust
filename predates the doc rename from `ShellStatusBar` to `StatusBar`;
future refactors may align the Rust filename/type name with the doc, but
both refer to the same contract.
