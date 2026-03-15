# g10 Jetstream First-Class Component Build-Out, Preview App, And Visual Fidelity

Status: planned
Updated: 2026-03-15

## Context

`g08` built a Jetstream rendering adapter that maps Pug spec structs to
Jetstream `UiTree` nodes and demonstrated components in a 4-screen demo scene.
However, the Jetstream adapter currently has only the adapter crate with test
coverage — it has no standalone preview application, no per-component specimens,
no theme/density controls, and no systematic visual comparison against the
Svelte or GPUI reference implementations.

`g10` makes the Jetstream preview app a first-class deliverable, mirroring what
`g09` accomplishes for GPUI. Every Jetstream-appropriate component gets a
specimen in a navigable preview app. The preview app gains section navigation,
display controls, and a demo scene matching the Svelte and GPUI versions.
Systematic visual comparison ensures Jetstream output is consistent with the
other runtimes within documented native-adaptation tolerances.

Jetstream's constraints (retained-mode `UiTree`, flexbox-only layout, solid
colors, no gradients, no transforms, limited text rendering) mean some
components require native adaptation or are intentionally excluded. These are
documented in the delta register from `g08.011` and updated here.

## Starting State

- `pug-jetstream` adapter crate maps specs to `UiTree` nodes for 117
  components (63 primitives + 41 composites + 13 workstation)
- 142 tests passing in `pug-jetstream`
- 4-screen demo scene exists (main menu, settings, HUD, pause dialog)
- no standalone preview application — components are only exercised in tests
  and the demo scene
- no per-component specimens, no visual comparison tooling
- Jetstream `game_ui` provides: `UiTree`, `UiNode`, `Widget` variants,
  `UiStyle`, `FocusState`, `Theme`, `UiEvent`, `ScreenStack`, GPU render pass
- Jetstream g04.016 rendering infrastructure is in place (text, images,
  clipping, scrolling, extended styling)

## Exit State

- standalone Jetstream preview application with section navigation, sidebar
  catalogue, and per-component specimen pages
- every Jetstream-appropriate component has a dedicated specimen demonstrating
  variants, sizes, and interactive states
- theme selector, density toggle, and control size selector are functional
- demo scene expanded to match the 6-screen structure from Svelte and GPUI
- systematic visual comparison performed against Svelte and GPUI references
- cross-runtime parity report updated with full Jetstream evidence
- delta register updated with all intentional native adaptations
- zero untested or undemonstrated components in the Jetstream adapter

## Milestone Status

| ID | Milestone | Depends On | Class | Status |
|----|-----------|------------|-------|--------|
| 001 | Jetstream preview app scaffold and navigation shell | g08.014 | Foundation | Planned |
| 002 | Component registry and specimen framework | 001 | Foundation | Planned |
| 003 | Structural and layout primitive specimens | 002 | Specimens | Planned |
| 004 | Action and input primitive specimens | 003 | Specimens | Planned |
| 005 | Selection and feedback primitive specimens | 004 | Specimens | Planned |
| 006 | Overlay and date/time primitive specimens | 005 | Specimens | Planned |
| 007 | Form, data, and browse composite specimens | 006 | Specimens | Planned |
| 008 | Editing, media, and operational composite specimens | 007 | Specimens | Planned |
| 009 | Workstation surface specimens | 008 | Specimens | Planned |
| 010 | Display controls — theme, density, control size | 009 | App shell | Planned |
| 011 | Demo scene expansion — 6-screen parity | 010 | Alignment | Planned |
| 012 | Visual parity audit — Svelte/GPUI vs Jetstream comparison | 011 | Hardening | Planned |
| 013 | Delta register update and native adaptation documentation | 012 | Hardening | Planned |
| 014 | Cross-runtime parity report — Jetstream evidence refresh | 013 | Hardening | Planned |
| 015 | Accessibility and input model verification | 014 | Hardening | Planned |
| 016 | Generation closeout | 015 | Closure | Planned |

## Dependency Shape

```text
g08.014 Previous Generation Complete
  -> 001 Preview App Scaffold
      -> 002 Component Registry + Specimen Framework
          -> 003 Structural Specimens
              -> 004 Action/Input Specimens
                  -> 005 Selection/Feedback Specimens
                      -> 006 Overlay/DateTime Specimens
                          -> 007 Form/Data Composite Specimens
                              -> 008 Editing/Media Composite Specimens
                                  -> 009 Workstation Specimens
                                      -> 010 Display Controls
                                          -> 011 Demo Scene Expansion
                                              -> 012 Visual Parity Audit
                                                  -> 013 Delta Register
                                                      -> 014 Parity Report
                                                          -> 015 Accessibility
                                                              -> 016 Closeout
```

## Execution Lanes

### Lane A: App Foundation

`001 -> 002`

### Lane B: Specimen Build-Out

`003 -> 004 -> 005 -> 006 -> 007 -> 008 -> 009`

### Lane C: App Polish and Demo

`010 -> 011`

### Lane D: Parity and Closeout

`012 -> 013 -> 014 -> 015 -> 016`

## Milestone Details

### 001 — Jetstream Preview App Scaffold and Navigation Shell

Create a standalone Jetstream preview application (separate from the game demo
scene) that runs in its own window/screen. Implement: main window with title
bar, 4 section tabs (Primitives, Composites, Demo, Tokens), sidebar area for
component catalogue, content area for specimen display. The app should use
Jetstream's `UiTree` and `ScreenStack` to manage navigation. Establish the
build target and run script.

### 002 — Component Registry and Specimen Framework

Implement a component registry (Rust struct or data table) mapping slugs to
display names, tiers, categories, and descriptions — mirroring the Svelte
`component-registry.ts`. Define the specimen rendering trait/pattern: each
specimen receives a `&PugTheme` and a `&mut UiTree` and returns a `UiNodeId`
subtree. Create the framework for rendering specimen cards with title,
description, and live component output.

### 003 — Structural and Layout Primitive Specimens

Create specimens for all structural primitives: Box, Stack, Grid, Surface,
Separator, ScrollShell, Banner, CallOut, Inline, Spacer. Each specimen
demonstrates multiple variants (tones, orientations, padding scales) by
building `UiTree` subtrees through the Pug-Jetstream adapter.

### 004 — Action and Input Primitive Specimens

Create specimens for: Button (all variants), IconButton, Field, TextInput,
TextArea, SearchField, FormActions, TimeField, EditableLabel, NumberEntry,
PinInput, Toolbar, SplitButton. Each specimen shows variant, size, and
state combinations.

### 005 — Selection and Feedback Primitive Specimens

Create specimens for: Checkbox, RadioGroup, Switch, TriStateSwitch, Select,
SegmentedControl, Slider, RangeSlider, Progress, Badge, StatusIndicator,
Meter, Rating, Skeleton, Pill, Eyebrow, TimeAgo, DurationInput, Code,
ColorPicker, FileUpload.

### 006 — Overlay and Date/Time Primitive Specimens

Create specimens for: Accordion, Collapsible, Dialog, Drawer, Popover,
HoverCard, Tooltip, Menu, ContextMenu, Tabs, TabStrip, NavigationMenu,
Menubar, Calendar, RangeCalendar, DatePicker, DateRangePicker, TimeField,
DateTimePicker, DateTimeRangePicker, TimeZoneSelect, ZonedDateTimePicker.
Overlays use Jetstream's `ScreenStack` for modal/transparent screens.

### 007 — Form, Data, and Browse Composite Specimens

Create specimens for: DataTable, ListShell, GridShell, DetailShell,
DetailSection, DetailRow, FilterToolbar, PickerShell, RelationPicker,
SelectionSummary, PaginationSummary, BulkActionBar, OrderBy, FormDialog,
ConfirmAction.

### 008 — Editing, Media, and Operational Composite Specimens

Create specimens for: MarkdownEditor, BlockEditor (simplified for Jetstream),
EmbedInput, EmbedPreview, EmbedShell, AudioPlayer, VideoPlayer, MediaPicker,
MediaThumbnail, MediaPreview, LogList, PageLoading, EmptyState, ToastStack,
StateTile, SlugField, InlineEditableField. Document any components that are
intentionally simplified or excluded for the game engine context.

### 009 — Workstation Surface Specimens

Create specimens for: WorkspaceShell, AppHeader, ProjectHeader,
CommandPalette, CommandPaletteShell, PanelHeader, PanelSurface, PanelTabs,
DockRegion, SplitView, ShellStatusBar, SurfaceTabs, ActionDiscoveryPanel.
These demonstrate how game applications compose Pug surfaces into their
window layout.

### 010 — Display Controls: Theme, Density, Control Size

Implement display controls in the preview app: theme selector (dropdown or
button cycle through registered themes), density toggle (compact/normal/
spacious), control size selector (sm/md/lg). Changes should re-resolve all
Pug tokens through the Jetstream theme bridge and trigger re-render of the
current specimen. Add state toggles for disabled, invalid, and busy modes.

### 011 — Demo Scene Expansion: 6-Screen Parity

Expand the existing 4-screen demo scene to the full 6-screen structure
matching Svelte and GPUI: (1) Overview Shell, (2) Form and Validation,
(3) Browse and Table, (4) Detail and Related Data, (5) Picker and Media,
(6) Command and Workspace. Each screen uses real Pug components through the
Jetstream adapter. Display controls apply across all screens.

### 012 — Visual Parity Audit: Svelte/GPUI vs Jetstream Comparison

Perform a systematic visual comparison of Jetstream specimens against the
Svelte and GPUI references. For each component, compare: colors (background,
text, border, accent), dimensions (height, padding, gap), border radius,
font size, and interactive state presentation. Fix style mapping issues in
the Jetstream adapter. Document cases where Jetstream constraints prevent
exact match (no gradients, no transforms, limited shadows).

### 013 — Delta Register Update and Native Adaptation Documentation

Update the intentional delta register to document all Jetstream-specific
adaptations: components with reduced functionality, simplified rendering,
or excluded features. For each delta, provide: component name, delta type
(visual/behavioral/excluded), technical justification, and Jetstream
constraint reference. Classify components into full parity, partial parity,
and intentional skip categories.

### 014 — Cross-Runtime Parity Report: Jetstream Evidence Refresh

Regenerate the cross-runtime parity report with full Jetstream evidence.
Include: component coverage (count and list), parity tier classification
for each component, delta register summary, token resolution coverage,
and test results. Update parity artifacts in both the Jetstream and Svelte
preview packages.

### 015 — Accessibility and Input Model Verification

Verify Jetstream-specific input handling: keyboard navigation (tab order,
arrow keys, enter/escape), gamepad navigation (D-pad/stick for focus,
A/B for confirm/cancel), focus state visualization (focus ring, highlight).
Verify that `FocusState` management works correctly for overlays and modal
screens. Document any accessibility differences from the Svelte/GPUI
implementations.

### 016 — Generation Closeout

Verify all milestones complete. Confirm the Jetstream preview app has:
section navigation, sidebar catalogue, per-component specimens, display
controls, 6-screen demo. Verify zero undemonstrated components remain.
Verify all tests pass. Update generation-index.md. Document any deferred
items for future generations.

## Cross-Project Dependencies

| Dependency | Direction | Description |
|-----------|-----------|-------------|
| Pug g08 | Pug → Pug g10 | Jetstream adapter must be complete |
| Pug g09 | Independent | GPUI build-out can run in parallel |
| Jetstream g04.016 | Jetstream → Pug g10 | UI rendering infrastructure must be in place |

## Known Constraints

From the Jetstream rendering constraint document (g06.013) and g08:
- **Layout**: flexbox-like only (no CSS Grid) — grid specs emulated with
  nested row/column panels
- **Text**: single-style runs, LTR, Latin/common scripts — no rich text,
  no complex shaping
- **Colors**: solid colors only — no gradients
- **Images**: GPU texture handles — no SVG rendering
- **Shadows**: single box shadow per node — no stacked or inset shadows
- **Transforms**: none — no rotation, scale, or skew
- **Scrolling**: vertical scroll with clipping — no momentum or snap
- **Input**: keyboard + mouse + gamepad — no touch/multi-touch, no IME

## Next Task

Open `g10.001` and begin the preview app scaffold.
