# CommandPalette

Status: contract
Updated: 2026-03-26

## 1. Purpose

- Component name: `CommandPalette`
- Layer: `composites`
- Summary: a modal command-discovery surface for searching and invoking actions;
  composes `ActionDiscoveryPanel` with `SearchInput` in a dialog overlay
- In scope: modal overlay, query input, grouped action results, active-result
  movement, command commit, loading/error/empty/no-results posture, focus
  trapping, body scroll lock, invocation hint display
- Out of scope: command registry persistence, fuzzy-search algorithm internals,
  telemetry, or app-specific command namespaces

## 2. Types

Uses `CommandActionItem` and `DiscoveryState` from `ActionDiscoveryPanel`.

## 3. Anatomy

```text
[Overlay]  backdrop, click-to-close
[Dialog]  role="dialog" aria-modal="true"
  ├── [Header]
  │     ├── [Title Group]
  │     │     ├── <h3> title
  │     │     └── <p> description  (optional)
  │     └── [Meta]
  │           ├── [Invocation Hint]  (optional, kbd-styled)
  │           └── [Close Button]  aria-label="Close command palette"
  ├── [Query]
  │     └── SearchInput
  ├── [Status]  role="status" aria-live="polite"
  └── [ActionDiscoveryPanel]  results list
```

| Part | Required | Description |
|------|----------|-------------|
| Overlay | yes | semi-transparent backdrop with blur; click dismisses |
| Dialog | yes | centered modal container |
| Header | yes | title, optional description, close button |
| Invocation Hint | no | keyboard shortcut reminder (e.g. "Cmd+K") |
| Query | yes | SearchInput for filtering commands |
| Status | yes | live status region announcing result count and active item |
| ActionDiscoveryPanel | yes | grouped command results list |

## 4. Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `open` | `boolean` | `false` | no | controls visibility; two-way bindable |
| `title` | `string` | `"Command palette"` | no | dialog heading |
| `description` | `string \| null` | `null` | no | secondary text below title |
| `query` | `string` | `""` | no | current search query value |
| `items` | `CommandActionItem[]` | `[]` | no | action items to display |
| `state` | `DiscoveryState` | `"ready"` | no | controls panel posture |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for dialog; falls back to `title` |
| `invocationHint` | `string \| null` | `null` | no | keyboard shortcut hint displayed in header (e.g. "Cmd+K") |
| `size` | `ControlSize \| null` | `null` | no | explicit semantic size override for header chips, close control, and nested search/results chrome |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic role used to resolve inherited size scale |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for the palette shell and nested result spacing |

## 5. Events

| Event | When It Fires | Payload |
|-------|---------------|---------|
| `queryChange` | user types in the search field or clears it | `{ value: string }` |
| `commandSelect` | user selects a command (click, Enter) | `{ id: string }` |
| `openChange` | palette closes (Escape, backdrop click, close button) | `{ open: boolean }` |
| `activeChange` | active command changes via keyboard or mouse | `{ id: string \| null }` |

## 6. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | `open=false` | nothing rendered |
| open/ready | `open=true`, `state="ready"` | dialog with query field and grouped results |
| open/loading | `open=true`, `state="loading"` | dialog with query field and ActionDiscoveryPanel skeleton loading in results area |
| open/error | `open=true`, `state="error"` | dialog with error empty state |
| open/empty | `open=true`, `state="empty"` | dialog with empty state message |
| open/no-results | `open=true`, `state="no-results"` | dialog with no-results empty state |

## 7. Accessibility

### Semantics

- Dialog: `role="dialog"`, `aria-modal="true"`, `aria-label` (falls back to title)
- `aria-describedby` linked to description paragraph when present
- Status region: `role="status"`, `aria-live="polite"`, `aria-atomic="true"`
- Status text dynamically reports: item count, active command name, or state message

### Keyboard

| Key | Behavior |
|-----|----------|
| `Escape` | closes the palette |
| `ArrowDown` | moves active to next enabled item |
| `ArrowUp` | moves active to previous enabled item |
| `Home` | moves active to first enabled item |
| `End` | moves active to last enabled item |
| `Enter` | selects the currently active command |
| `Tab` / `Shift+Tab` | trapped within the dialog |

### Focus Management

- On open: focus moves to the query input; first enabled item becomes active
- On close: focus restores to the previously focused element
- Body scroll is locked while the palette is open (overflow: hidden on html and body)
- Active item auto-resets to first enabled item when items change and current
  active is no longer valid

## 8. Layout

- Overlay: fixed, full viewport, `z-index: 40`, background with 44% black + blur
- Dialog: fixed, centered via `translate(-50%, -50%)`, `z-index: 41`
- Width: `min(45rem, calc(100vw - 2rem))`
- Max-height: `min(78vh, 52.5rem)`
- Grid rows: `auto auto auto minmax(0, 1fr)`
- Hint pill and close button now respond to semantic size rather than fixed compact geometry
- Responsive (<=45rem): width adjusts, header goes single-column, reduced padding

## 9. Token Usage — Exact CSS Values

#### `.command-palette__overlay`

| Property | Value |
|----------|-------|
| `position` | `fixed` |
| `inset` | `0` |
| `background` | `color-mix(in srgb, black 44%, transparent)` |
| `backdrop-filter` | `blur(0.5rem)` |
| `z-index` | `40` |

#### `.command-palette` (Dialog)

| Property | Value |
|----------|-------|
| `position` | `fixed` |
| `top` | `50%` |
| `left` | `50%` |
| `display` | `grid` |
| `grid-template-rows` | `auto auto auto minmax(0, 1fr)` |
| `gap` | `var(--poodle-space-stack-md)` |
| `width` | `min(45rem, calc(100vw - 2rem))` |
| `max-height` | `min(78vh, 52.5rem)` |
| `min-height` | `0` |
| `padding` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 42%, transparent)` |
| `border-radius` | `calc(var(--poodle-radius-surface) + 0.125rem)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 98%, transparent)` |
| `box-shadow` | `var(--poodle-elevation-dialog)` |
| `overflow` | `hidden` |
| `overscroll-behavior` | `contain` |
| `transform` | `translate(-50%, -50%)` |
| `z-index` | `41` |

#### `.command-palette__header`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `minmax(0, 1fr) auto` |
| `gap` | `var(--poodle-space-inline-md)` |
| `align-items` | `start` |

#### `.command-palette__header h3`, `.command-palette__header p`

| Property | Value |
|----------|-------|
| `margin` | `0` |

#### `.command-palette__header h3`

| Property | Value |
|----------|-------|
| `font-size` | `1.375rem` |
| `line-height` | `1.2` |

#### `.command-palette__header p`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.8125rem` |
| `line-height` | `1.5` |

#### `.command-palette__meta`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `align-items` | `center` |

#### `.command-palette__hint`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `min-height` | `1.5rem` |
| `padding` | `0 0.5rem` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 76%, transparent)` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `0.75rem` |

#### `.command-palette__close`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `1.75rem` |
| `height` | `1.75rem` |
| `padding` | `0` |
| `border` | `0` |
| `border-radius` | `calc(var(--poodle-radius-control) - 0.0625rem)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 62%, transparent)` |
| `color` | `var(--poodle-color-text-secondary)` |
| `cursor` | `pointer` |
| `font` | `inherit` |

#### `.command-palette__close:hover`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 84%, transparent)` |
| `color` | `var(--poodle-color-text-primary)` |

#### `.command-palette__close:focus-visible`

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

#### `.command-palette__status`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.8125rem` |
| `line-height` | `1.5` |

### Light Theme Override: `:global([data-theme="light"]) .command-palette`

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--poodle-color-border-default) 24%, transparent)` |
| `box-shadow` | `0 1.125rem 2.75rem rgba(49, 66, 85, 0.1), inset 0 0.0625rem 0 rgba(255, 255, 255, 0.72)` |

### Responsive Breakpoint: `max-width: 45rem`

#### `.command-palette`

| Property | Value |
|----------|-------|
| `width` | `min(100vw - 1.25rem, 45rem)` |
| `max-height` | `calc(100vh - 1.25rem)` |
| `padding` | `1rem` |

#### `.command-palette__header`

| Property | Value |
|----------|-------|
| `grid-template-columns` | `1fr` |

#### `.command-palette__meta`

| Property | Value |
|----------|-------|
| `justify-content` | `flex-start` |

### Data Attributes Used for CSS Selectors

| Attribute | Element | Purpose |
|-----------|---------|---------|
| `data-theme` | global (`:global([data-theme="light"])`) | light theme override selector |

## 10. Composition

- Composes: `SearchInput`, `Icon`, `ActionDiscoveryPanel`
- The SearchInput dispatches `valueChange`, `clear`, `cancel`, and `submit`
  events which are mapped to palette events
- ActionDiscoveryPanel is bound with `activeId` for two-way active tracking
- Loading treatment is delegated to `ActionDiscoveryPanel`, which remains
  Skeleton-based rather than Spinner-based because it is preserving result-row structure

## 11. Specimen Definitions

### Command Palette

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Command Palette | `open` bound to toggle state, `items` with 7 actions across 3 groups (File: Save/Open File/Close Tab, Edit: Find in Files/Find and Replace, View: Toggle Terminal/Toggle Sidebar) with shortcut hints; trigger Button ("Open Command Palette") | Button to open palette; modal palette overlay with search input, grouped command results with titles and keyboard shortcuts; closable via Escape, backdrop click, or close button |
