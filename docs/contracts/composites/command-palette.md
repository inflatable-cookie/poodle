# CommandPalette

Status: contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `CommandPalette`
- Layer: `composites`
- Summary: a modal command-discovery surface for searching and invoking actions;
  composes `ActionDiscoveryPanel` with `SearchField` in a dialog overlay
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
  │     └── SearchField
  ├── [Status]  role="status" aria-live="polite"
  └── [ActionDiscoveryPanel]  results list
```

| Part | Required | Description |
|------|----------|-------------|
| Overlay | yes | semi-transparent backdrop with blur; click dismisses |
| Dialog | yes | centered modal container |
| Header | yes | title, optional description, close button |
| Invocation Hint | no | keyboard shortcut reminder (e.g. "Cmd+K") |
| Query | yes | SearchField for filtering commands |
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
| open/loading | `open=true`, `state="loading"` | dialog with skeleton loading in results area |
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
- Responsive (<=45rem): width adjusts, header goes single-column, reduced padding

## 9. Composition

- Composes: `SearchField`, `Icon`, `ActionDiscoveryPanel`
- The SearchField dispatches `valueChange`, `clear`, `cancel`, and `submit`
  events which are mapped to palette events
- ActionDiscoveryPanel is bound with `activeId` for two-way active tracking

## 10. Specimen Definitions

### Command Palette

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Command Palette | `open` bound to toggle state, `items` with 7 actions across 3 groups (File: Save/Open File/Close Tab, Edit: Find in Files/Find and Replace, View: Toggle Terminal/Toggle Sidebar) with shortcut hints; trigger Button ("Open Command Palette") | Button to open palette; modal palette overlay with search input, grouped command results with titles and keyboard shortcuts; closable via Escape, backdrop click, or close button |
