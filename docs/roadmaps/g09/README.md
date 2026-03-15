# g09 GPUI First-Class Component Build-Out, Preview Parity, And Visual Fidelity

Status: planned
Updated: 2026-03-15

## Context

`g07` built a GPUI rendering adapter that maps Pug spec structs to GPUI
elements, and `g08` did the same for Jetstream. Both generations declared
three-runtime parity and marked their milestones complete. However, the actual
GPUI preview app still contains dozens of hand-built mockup specimens that use
raw `div()` chains instead of real Pug components, many components lack
interactive behaviour, and no systematic visual comparison against the Svelte
reference has been performed. The GPUI preview app is a functional scaffold, not
a finished product.

`g09` makes the GPUI preview app a first-class deliverable. Every component
that has a Svelte specimen gets a GPUI specimen built from real Pug components.
Every specimen is visually compared against the Svelte reference and
pixel-level discrepancies are resolved. The preview app gains the same
navigation, theme controls, density toggles, and route-addressable state as the
Svelte version. The result is a GPUI preview app that looks and works
identically to the Svelte preview app.

## Starting State

- 41 GPUI component structs in `pug_gpui_components` (30 core + 4 layout +
  7 date/time)
- 77 Svelte primitives, 42 Svelte composites, 8+ workstation surfaces
  (127 total Svelte components)
- GPUI preview app has ~65 specimen files, but many are hand-built mockups
  using raw `div()` chains without real component wrappers
- 14 specimens have been upgraded to use real Pug components; ~50+ remain
  as mockups
- no theme/density/control-size overlay controls in GPUI preview
- no route-addressable navigation or section tabs in GPUI preview
- no visual parity verification against Svelte reference
- Svelte preview has 112 specimen files, 4-section navigation, display
  controls sidebar, and 6-screen shared demo app

## Exit State

- every Svelte primitive, composite, and workstation component that is
  appropriate for GPUI has a first-class `PugComponent` struct in
  `pug_gpui_components`
- every component has a dedicated specimen in the GPUI preview app using
  real Pug component instances (no hand-built mockups remain)
- GPUI preview app has section tabs (Primitives, Composites, Demo, Tokens),
  sidebar navigation, and per-component pages matching Svelte layout
- theme selector, density toggle, and control-size selector are functional
  in the GPUI preview
- route-addressable state (theme, density, size, section, component) is
  preserved across navigation
- 6-screen shared demo app is implemented in GPUI matching the Svelte demo
- systematic visual comparison is performed for every component and
  discrepancies are resolved to within acceptable tolerance
- cross-runtime parity report is updated with full evidence
- zero hand-built mockup specimens remain in the codebase

## Milestone Status

| ID | Milestone | Depends On | Class | Status |
|----|-----------|------------|-------|--------|
| 001 | Component gap audit — Svelte surface vs GPUI components | g08.014 | Foundation | Planned |
| 002 | Missing primitive components — structural and informational | 001 | Core build | Planned |
| 003 | Missing primitive components — input and temporal | 002 | Core build | Planned |
| 004 | Missing composite components — form and data | 003 | Core build | Planned |
| 005 | Missing composite components — editing, media, and operational | 004 | Core build | Planned |
| 006 | Missing workstation components | 005 | Core build | Planned |
| 007 | Specimen upgrade — structural and layout primitives | 002 | Specimens | Planned |
| 008 | Specimen upgrade — action and input primitives | 003 | Specimens | Planned |
| 009 | Specimen upgrade — selection and feedback primitives | 003 | Specimens | Planned |
| 010 | Specimen upgrade — overlay and date/time primitives | 003 | Specimens | Planned |
| 011 | Specimen upgrade — composites batch 1 (form, data, detail) | 004 | Specimens | Planned |
| 012 | Specimen upgrade — composites batch 2 (editing, media, workstation) | 005, 006 | Specimens | Planned |
| 013 | Preview app shell — section tabs, sidebar, per-component pages | 007-012 | App shell | Planned |
| 014 | Preview app controls — theme, density, control size, route state | 013 | App shell | Planned |
| 015 | Shared demo app — 6-screen GPUI implementation | 014 | Alignment | Planned |
| 016 | Visual parity audit — systematic Svelte vs GPUI comparison | 015 | Hardening | Planned |
| 017 | Cross-runtime parity report update and evidence refresh | 016 | Hardening | Planned |
| 018 | Generation closeout | 017 | Closure | Planned |

## Dependency Shape

```text
g08.014 Previous Generation Complete
  -> 001 Component Gap Audit
      -> 002 Structural/Informational Primitives
          -> 003 Input/Temporal Primitives
              -> 004 Form/Data Composites
                  -> 005 Editing/Media/Operational Composites
                      -> 006 Workstation Components
      -> 007 Specimens: Structural  (after 002)
      -> 008 Specimens: Action/Input (after 003)
      -> 009 Specimens: Selection/Feedback (after 003)
      -> 010 Specimens: Overlay/DateTime (after 003)
          -> 011 Specimens: Composites Batch 1 (after 004)
              -> 012 Specimens: Composites Batch 2 (after 005, 006)
                  -> 013 App Shell
                      -> 014 Display Controls + Route State
                          -> 015 Demo App
                              -> 016 Visual Parity Audit
                                  -> 017 Parity Report
                                      -> 018 Closeout
```

## Execution Lanes

### Lane A: Component Build-Out

`001 -> 002 -> 003 -> 004 -> 005 -> 006`

### Lane B: Specimen Upgrades (parallelizable with Lane A after 002/003)

`007 -> 008 -> 009 -> 010 -> 011 -> 012`

### Lane C: App Shell and Demo

`013 -> 014 -> 015`

### Lane D: Parity and Closeout

`016 -> 017 -> 018`

## Milestone Details

### 001 — Component Gap Audit: Svelte Surface vs GPUI Components

Produce an exhaustive gap register comparing the full Svelte component surface
(77 primitives + 42 composites + workstation) against the current GPUI
`pug_gpui_components` crate (41 components). For each missing component,
classify it as: (a) new component struct needed, (b) existing component covers
it via configuration, or (c) not applicable to GPUI. Produce a prioritized
implementation list with estimated batch assignments.

### 002 — Missing Primitive Components: Structural and Informational

Implement first-class `Pug*` component structs for structural and informational
primitives that exist in Svelte but are missing from GPUI: Banner, CallOut,
Eyebrow, Pill, Code, Skeleton, Meter, Rating, HoverCard, TriStateSwitch,
EditableLabel, Inline, Spacer. Each component must implement `IntoElement`,
resolve tokens via `GpuiThemeProvider`, and follow the established builder
pattern.

### 003 — Missing Primitive Components: Input and Temporal

Implement missing input and temporal primitives: NumberEntry, PinInput,
Toolbar, RangeSlider, DurationInput, TimeAgo, TimeZoneSelect,
ZonedDateTimePicker, FileUpload, ColorPicker. Each follows the standard
`Pug*` struct pattern with spec-backed token resolution and interactive
callbacks where appropriate.

### 004 — Missing Composite Components: Form and Data

Implement composite components for form, data, and browse patterns:
DataTable, ListShell, GridShell, DetailShell, DetailSection, DetailRow,
FilterToolbar, PickerShell, RelationPicker, SelectionSummary,
PaginationSummary, BulkActionBar, OrderBy, FormDialog, ConfirmAction.

### 005 — Missing Composite Components: Editing, Media, and Operational

Implement composites for editing, media, and operational patterns:
MarkdownEditor, BlockEditor, EmbedInput, EmbedPreview, EmbedShell,
AudioPlayer, VideoPlayer, MediaPicker, MediaThumbnail, MediaPreview,
LogList, PageLoading, EmptyState, ToastStack, StateTile, SlugField,
InlineEditableField.

### 006 — Missing Workstation Components

Implement remaining workstation surface components: WorkspaceShell,
AppHeader, CommandPalette, CommandPaletteShell, PanelHeader, PanelSurface,
PanelTabs, DockRegion, SplitView, ShellStatusBar, SurfaceTabs,
ActionDiscoveryPanel. These compose multiple primitives and composites into
application-level layout surfaces.

### 007 — Specimen Upgrade: Structural and Layout Primitives

Replace hand-built mockup specimens for structural components (box, stack,
grid, surface, separator, scroll-shell, banner, callout, inline, spacer)
with specimens using real `PugBox`, `PugStack`, `PugGrid`, `PugSurface`,
`PugSeparator`, `PugScrollShell`, `PugBanner`, `PugCallOut` components.
Each specimen should demonstrate variants, sizes, and interactive states
matching the Svelte specimen.

### 008 — Specimen Upgrade: Action and Input Primitives

Replace mockup specimens for action and input components (split-button,
number-entry, pin-input, toolbar, editable-label, time-field, file-upload,
color-picker) with specimens using real Pug components. Add specimens for
components that currently have `simple_specimen()` placeholders.

### 009 — Specimen Upgrade: Selection and Feedback Primitives

Replace mockup specimens for feedback components (meter, rating, skeleton,
pill, eyebrow, time-ago, duration-input, code) with specimens using real
Pug components. Each specimen must show all variants and interactive states.

### 010 — Specimen Upgrade: Overlay and Date/Time Primitives

Upgrade overlay specimens (hover-card) and ensure all date/time specimens
(calendar, range-calendar, date-picker, date-range-picker, time-field,
date-time-picker, date-time-range-picker) use real component instances with
interactive state. Add specimens for components currently missing them.

### 011 — Specimen Upgrade: Composites Batch 1 (Form, Data, Detail)

Replace the grouped composite specimens (data-table, detail-shell,
list-shell, cards, picker, page-structure, state-display, misc-composites)
with per-component specimens using real Pug composite components. Break
grouped `misc_composites.rs` into individual files. Each composite specimen
should mirror its Svelte counterpart.

### 012 — Specimen Upgrade: Composites Batch 2 (Editing, Media, Workstation)

Replace grouped specimens for media, editors, and workstation components
with per-component specimens using real Pug components. Split `media.rs`,
`editors.rs`, `workspace.rs`, `panel.rs`, `dock_split.rs` into individual
per-component files.

### 013 — Preview App Shell: Section Tabs, Sidebar, Per-Component Pages

Restructure the GPUI preview app to match the Svelte preview's navigation:
4 section tabs (Primitives, Composites, Demo, Tokens), sidebar catalogue
with alphabetical component listing, per-component pages with hero header
(tier, package, description) and live specimen. Replace the current
flat-list or grouped layout with the structured page model.

### 014 — Preview App Controls: Theme, Density, Control Size, Route State

Implement display controls matching the Svelte preview: theme selector
dropdown, density mode toggle (compact/normal/spacious), control size
selector (sm/md/lg), appearance treatment override, state toggles
(disabled, invalid, busy). Implement route-addressable state so theme,
density, size, section, and selected component are preserved in app state
and restorable.

### 015 — Shared Demo App: 6-Screen GPUI Implementation

Implement the 6-screen shared demo app in GPUI matching the Svelte version:
(1) Overview Shell, (2) Form and Validation, (3) Browse and Table,
(4) Detail and Related Data, (5) Picker and Media, (6) Command and
Workspace. Each screen uses real Pug components and exercises composition
patterns. Display controls (theme, density, size, disabled/invalid/busy
toggles) apply to the entire demo.

### 016 — Visual Parity Audit: Systematic Svelte vs GPUI Comparison

Perform a systematic side-by-side visual comparison of every component
specimen between Svelte and GPUI. For each component, verify: background
colors, border colors and widths, border radius, text colors and sizes,
padding and spacing, interactive state changes (hover, focus, active,
disabled). Document discrepancies and fix style mapping issues in the
component implementations until visual parity is achieved within acceptable
native-adaptation tolerances.

### 017 — Cross-Runtime Parity Report Update and Evidence Refresh

Update the cross-runtime parity report with full evidence covering the
expanded GPUI surface. Document: component coverage (count and list),
visual parity tier (strict/visual/native-adaptation), behavioral parity
verification, intentional deltas with justification, and token resolution
coverage. Regenerate parity artifacts.

### 018 — Generation Closeout

Verify all milestones complete. Confirm zero hand-built mockup specimens
remain. Verify GPUI preview app navigation, controls, and demo app match
Svelte version. Document any deferred items for future generations.

## Next Task

Open `g09.001` and begin the component gap audit.
