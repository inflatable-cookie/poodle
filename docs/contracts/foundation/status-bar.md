# ShellStatusBar

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `ShellStatusBar`
- Layer: `workstation`
- Summary: a lightweight shell utility/status row for workspace summary, connection state, and context metadata
- In scope: leading and trailing status regions, workspace summary text, utility metadata, and shell-level status packing
- Out of scope: transient notifications, remediation banners, app-specific transport/status widgets, or global command registries

## 2. Accessibility

- status-bar content must remain textual and keyboard reachable where interactive controls are present
- shell utility metadata must not become the only place a critical error is communicated
- status ordering should remain stable as connection or sync state changes
- GPUI-native accessibility mapping notes: GPUI must preserve utility-region labeling and status ordering even when shell chrome is custom-rendered

## 3. Specimen Definitions

### Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | `summary="Ready"`, leading slot with "main" branch indicator and "0 errors" status, trailing slot with "Ln 42, Col 18", "UTF-8", and "TypeScript" metadata items | Full-width status bar with summary text, leading status items (branch, error count) on the left, trailing context metadata (cursor position, encoding, language) on the right |

## 4. Next Task

Use `ShellStatusBar` for persistent shell summary and utility metadata while keeping urgent remediation on banners and transient confirmation in toasts.
