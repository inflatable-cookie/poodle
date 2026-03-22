# StatusBar

Status: seed contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `StatusBar`
- Layer: `foundation`
- Summary: a lightweight shell utility/status row for workspace summary, connection state, and context metadata
- In scope: leading and trailing status regions, workspace summary text, utility metadata, and shell-level status packing
- Out of scope: transient notifications, remediation banners, app-specific transport/status widgets, or global command registries

## 2. Anatomy

```text
[Root .status-bar]  <footer>
  ├── [Summary] (summary text)
  ├── [Leading slot] (left-aligned status items)
  └── [Trailing slot] (right-aligned metadata items)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | footer element wrapping the status bar | background, border, padding |
| Summary | no | summary text for workspace state | typography |
| Leading | no | slot for left-aligned status items | flex layout |
| Trailing | no | slot for right-aligned metadata items | flex layout |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `summary` | `string \| null` | `null` | no | summary text displayed in the status bar |
| `ariaLabel` | `string \| null` | `null` | no | accessible name for the status bar region |

### Slots

| Slot | Purpose |
|------|---------|
| `leading` | left-aligned status items (branch indicator, error count, etc.) |
| `trailing` | right-aligned context metadata (cursor position, encoding, language, etc.) |

## 4. Accessibility

- Root element is `<footer>` providing landmark semantics
- `aria-label` from prop when provided
- status-bar content must remain textual and keyboard reachable where interactive controls are present
- shell utility metadata must not become the only place a critical error is communicated
- status ordering should remain stable as connection or sync state changes
- GPUI-native accessibility mapping notes: GPUI must preserve utility-region labeling and status ordering even when shell chrome is custom-rendered

## 5. Specimen Definitions

### Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | `summary="Ready"`, leading slot with "main" branch indicator and "0 errors" status, trailing slot with "Ln 42, Col 18", "UTF-8", and "TypeScript" metadata items | Full-width status bar with summary text, leading status items (branch, error count) on the left, trailing context metadata (cursor position, encoding, language) on the right |

## 6. Next Task

Use `StatusBar` for persistent shell summary and utility metadata while keeping urgent remediation on banners and transient confirmation in toasts.
