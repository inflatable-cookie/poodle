# g09.015 — Shared Demo App: 6-Screen GPUI Implementation

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g09.014
Primary repos: `pug`

## Goals

- [ ] implement the 6-screen shared demo app in GPUI matching the Svelte
  version's content and layout
- [ ] demonstrate real-world composition patterns using Pug components

## Execution Checklist

- [ ] implement Screen 1: Overview Shell — `PugWorkspaceShell`, `PugAppHeader`,
  `PugStateTile` grid, `PugBanner`, `PugProgress` bars, `PugNavCardGrid`,
  `PugLogList`, `PugShellStatusBar`
- [ ] implement Screen 2: Form and Validation — `PugField` + `PugTextInput`,
  `PugTextArea`, `PugSelect`, `PugCheckbox`, `PugRadioGroup`, `PugSwitch`,
  `PugFormActions`, validation error states, `PugBanner` for form-level errors
- [ ] implement Screen 3: Browse and Table — `PugDataTable` with sortable
  columns and row selection, `PugFilterToolbar`, `PugPaginationSummary`,
  `PugBulkActionBar`, `PugEmptyState` for empty search results
- [ ] implement Screen 4: Detail and Related Data — `PugDetailShell`,
  `PugDetailSection`, `PugDetailRow`, `PugMediaThumbnail`, `PugMediaPreview`,
  `PugBadge`, `PugSeparator`
- [ ] implement Screen 5: Picker and Media — `PugPickerShell`,
  `PugRelationPicker`, `PugSelectionSummary`, `PugMediaPicker`,
  `PugAudioPlayer`, `PugVideoPlayer`
- [ ] implement Screen 6: Command and Workspace — `PugWorkspaceShell`,
  `PugCommandPalette`, `PugDockRegion`, `PugSplitView`, `PugPanelSurface`,
  `PugPanelHeader`, `PugPanelTabs`, `PugActionDiscoveryPanel`
- [ ] wire demo tab navigation between the 6 screens
- [ ] apply display controls (theme, density, size, disabled/invalid/busy)
  to the entire demo — all screens react to control changes
- [ ] implement "PrimitiveCoverageDeck" showing every primitive alongside
  demo content for reference completeness
- [ ] verify each screen renders without panic with all theme/density/size
  combinations

## Acceptance Criteria

- [ ] all 6 screens are navigable from the Demo section tab
- [ ] each screen exercises a meaningful composition of Pug components (not
  placeholder text)
- [ ] display controls (theme, density, control size) propagate to all demo
  screens
- [ ] state toggles (disabled, invalid, busy) affect interactive elements
  across all screens
- [ ] total component usage across all screens exceeds 60 unique component
  types
- [ ] `cargo check` passes for the preview crate

## Next Task

Open `g09.016` and perform the visual parity audit.
