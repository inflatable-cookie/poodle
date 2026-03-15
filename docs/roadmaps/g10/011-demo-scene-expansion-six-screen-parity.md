# g10.011 — Demo Scene Expansion: 6-Screen Parity

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g10.010
Primary repos: `pug`

## Goals

- [ ] expand the existing 4-screen Jetstream demo scene to the full 6-screen
  structure matching Svelte and GPUI

## Execution Checklist

- [ ] refactor existing 4 demo screens (main menu, settings, HUD, pause)
  into the 6-screen structure:
  - [ ] Screen 1: Overview Shell — WorkspaceShell, AppHeader, StateTile grid,
    Banner, Progress bars, NavCardGrid, LogList, ShellStatusBar
  - [ ] Screen 2: Form and Validation — Field + TextInput, TextArea, Select,
    Checkbox, RadioGroup, Switch, FormActions, validation error states
  - [ ] Screen 3: Browse and Table — DataTable with sortable columns,
    FilterToolbar, PaginationSummary, BulkActionBar, EmptyState
  - [ ] Screen 4: Detail and Related Data — DetailShell, DetailSection,
    DetailRow, MediaThumbnail, MediaPreview, Badge, Separator
  - [ ] Screen 5: Picker and Media — PickerShell, RelationPicker,
    SelectionSummary, MediaPicker, AudioPlayer, VideoPlayer
  - [ ] Screen 6: Command and Workspace — WorkspaceShell, CommandPalette,
    DockRegion, SplitView, PanelSurface, PanelHeader, PanelTabs,
    ActionDiscoveryPanel
- [ ] wire demo tab navigation between the 6 screens using ScreenStack
  or internal state switching
- [ ] apply display controls (theme, density, size, state toggles) to all
  demo screens
- [ ] preserve existing game-context elements (HUD overlay, gamepad
  navigation) as native-adaptation additions
- [ ] verify each screen renders without panic with all theme/density
  combinations

## Acceptance Criteria

- [ ] all 6 screens are navigable from the Demo section
- [ ] each screen exercises a meaningful composition of Pug components
- [ ] display controls propagate to all demo screens
- [ ] total component usage across all screens exceeds 50 unique types
- [ ] `cargo check` passes

## Next Task

Open `g10.012` and perform the visual parity audit.
