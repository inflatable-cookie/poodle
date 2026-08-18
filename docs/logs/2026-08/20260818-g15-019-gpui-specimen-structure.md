# g15.019 — GPUI Specimen Structure (August batch log)

Date: 2026-08-18
Card: `docs/roadmaps/g15/019-gpui-specimen-structure.md`
Handoff: `docs/handoffs/20260818-183755-g15-019-gpui-specimen-structure.md`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-4da0cdec`
Branch: `t3code/gpui-specimen-structure`

## Summary

The GPUI catalogue now publishes the axis panes the merged web census admits,
and only those. Three changes carry it:

1. `specimen_layout` takes an explicit `SpecimenAxes` value. A page states the
   panes it supplies; a pane with no renderer produces no tab, and a retained
   tab the page no longer admits normalises back to Examples.
2. 74 pages were corrected against the census — 59 gained both axes, four
   gained `Sizes`, eight gained `Densities`, and three stopped advertising
   panes their component does not take.
3. `poodle_render::audio_specimens` no longer returns a combined page. It hands
   out Examples, one representative at a requested size, and one at a requested
   density; GPUI composes them through its own axis-aware layout.

Six named pages gained honest captions. Twelve pages that had been keeping an
axis matrix inside `Examples` had it moved into the matching pane, not
duplicated.

**This is structural proof only.** No native page was rendered. `g15.026` owns
the library seam and the 174-page live headless probe.

## What Changed

### 1. Native axis admission is explicit

`packages/gpui/preview/src/specimens/specimen_axes.rs` is new and holds the
decision on its own: `AxisAdmission::tabs` (which tabs exist) and
`AxisAdmission::resolve_tab` (which tab renders, given whatever the page last
stored). It depends on `poodle_specs` alone, so the focused native regressions
can exercise it headlessly.

`specimen_layout.rs` was broken once. The old signature took a sizes closure
and a densities closure positionally and always built both tabs — Avatar and
Progress passed density closures that ignored their argument, and Tooltip
passed both. The new signature takes a `SpecimenAxes` built from a single
constructor:

```rust
SpecimenAxes::examples_only()
    .with_sizes(|size, theme| ...)
    .with_densities(|density, theme| ...)
```

`with_sizes_where` / `with_densities_where` take an `Option`-returning renderer
for a component whose own enum stops short of the five control steps — Text and
Eyebrow stop at `md`, and their `lg` / `xl` rows are dropped rather than faked,
matching what the Svelte specimen renders.

There is no default-both path, no alias, and no compatibility twin. All 67
call sites were migrated in the same change, including the bounded scene
adapter (`scene_specimen.rs`), which now derives admission from the fixture's
own `size_axis` / `density_axis` lists instead of always constructing both
panes and filling unsupported steps with empty divs.

### 2. The shared audio specimen shape is split

`packages/render/src/audio_specimens.rs` exposed twelve `pub fn <control>(theme)`
functions, each returning one page with the curated examples *and* both axis
sweeps appended. That combined return is gone. In its place:

```rust
pub enum AudioSpecimen { /* the twelve controls */ }

impl AudioSpecimen {
    pub fn examples(self, theme: &dyn ThemeProvider) -> Node;
    pub fn size(self, size: ControlSize, theme: &dyn ThemeProvider) -> Node;
    pub fn density(self, density: ControlDensity, theme: &dyn ThemeProvider) -> Node;
}
```

Direct compile consumers were updated in the same change:

- `packages/gpui/preview/src/specimens/audio_controls.rs` composes the three
  parts through the axis-aware layout — one `render` entry point for all twelve
  pages, replacing twelve page-returning wrappers.
- `packages/jetstream/preview/src/specimens/audio_controls.rs` keeps its present
  Examples-only behaviour, now via `AudioSpecimen::X.examples(theme)`. No
  Jetstream parity work was added.

### 3. Layout stays renderer-owned

Axis panes construct native GPUI element trees. Nothing here copies web DOM
structure, and no shared render tree was introduced. Where a component's native
representative could not be a bare surface — Dialog, AlertDialog, Drawer,
CommandPalette are absolutely-positioned overlays — the pane renders a trigger
per step and the overlay at that step's size or density when the trigger is
used, which is what the Svelte specimen's own axis pane does.

## 74-Axis Checklist

Every row was read back from the dispatch table and its page module after the
change: which axis builders the page calls, and whether it routes through
`specimen_layout`. 74 checked, 74 match the census, 0 mismatches.

### Both axes added — 59 pages

| Page | Route | Axes published |
| --- | --- | --- |
| `ConfirmAction` | `confirm-action` | both |
| `TriStateSwitch` | `tri-state-switch` | both |
| `DragNumberField` | `drag-number-field` | both |
| `EmbedInput` | `embed-input` | both |
| `FileUpload` | `file-upload` | both |
| `Select` | `select` | both |
| `SplitView` | `split-view` | both |
| `StatusIndicator` | `status-indicator` | both |
| `ToastHost` | `toast-host` | both |
| `ToastStack` | `toast-stack` | both |
| `SidebarNav` | `sidebar-nav` | both |
| `Tabs` | `tabs` | both |
| `Tree` | `tree` | both |
| `Accordion` | `accordion` | both |
| `AlertDialog` | `alert-dialog` | both |
| `CommandPalette` | `command-palette` | both |
| `Dialog` | `dialog` | both |
| `Drawer` | `drawer` | both |
| `FormDialog` | `form-dialog` | both |
| `BlockEditor` | `block-editor` | both |
| `Field` | `field` | both |
| `MarkdownEditor` | `markdown-editor` | both |
| `BulkActionBar` | `bulk-action-bar` | both |
| `CardToggleGroup` | `card-toggle-group` | both |
| `DataTable` | `data-table` | both |
| `EditableList` | `editable-list` | both |
| `FilterToolbar` | `filter-toolbar` | both |
| `ListCard` | `list-card` | both |
| `LogList` | `log-list` | both |
| `RelationPicker` | `relation-picker` | both |
| `Table` | `table` | both |
| `AudioPlayer` | `audio-player` | both |
| `MediaBrowsePanel` | `media-browse-panel` | both |
| `MediaPicker` | `media-picker` | both |
| `MediaPreview` | `media-preview` | both |
| `VideoPlayer` | `video-player` | both |
| `AppHeader` | `app-header` | both |
| `HistoryCenter` | `history-center` | both |
| `MessageCenter` | `message-center` | both |
| `PageHeader` | `page-header` | both |
| `StatusBar` | `status-bar` | both |
| `AgentQuestion` | `agent-question` | both |
| `AgentTranscript` | `agent-transcript` | both |
| `AudioMeter` | `audio-meter` | both |
| `AudioSwitch` | `audio-switch` | both |
| `EnvelopeEditor` | `envelope-editor` | both |
| `Fader` | `fader` | both |
| `GainReductionMeter` | `gain-reduction-meter` | both |
| `Keyboard` | `keyboard` | both |
| `Knob` | `knob` | both |
| `ModMatrixGrid` | `mod-matrix-grid` | both |
| `ValueReadout` | `value-readout` | both |
| `WaveformDisplay` | `waveform-display` | both |
| `XYPad` | `xy-pad` | both |
| `LicenceActivation` | `licence-activation` | both |
| `LicenceSeats` | `licence-seats` | both |
| `LicenceStatus` | `licence-status` | both |
| `UpdateCenter` | `update-center` | both |
| `UpdateStatus` | `update-status` | both |
### Size pane only — 4 pages

| Page | Route | Axes published |
| --- | --- | --- |
| `Eyebrow` | `eyebrow` | sizes |
| `Text` | `text` | sizes |
| `Meter` | `meter` | sizes |
| `PasswordRequirements` | `password-requirements` | sizes |
### Density pane only — 8 pages

| Page | Route | Axes published |
| --- | --- | --- |
| `Card` | `card` | densities |
| `DetailItem` | `detail-item` | densities |
| `UiPresentationProvider` | `ui-presentation-provider` | densities |
| `MetricTile` | `metric-tile` | densities |
| `NavCard` | `nav-card` | densities |
| `FormActions` | `form-actions` | densities |
| `DetailSection` | `detail-section` | densities |
| `DetailSectionGroup` | `detail-section-group` | densities |
### Over-advertisement corrected — 3 pages

| Page | Route | Axes published |
| --- | --- | --- |
| `Avatar` | `avatar` | sizes |
| `Progress` | `progress` | sizes |
| `Tooltip` | `tooltip` | none |
## Six Caption Repairs

| Page | Caption work |
| --- | --- |
| `Text` | Examples regrouped into five captioned groups (Tones, Weight and leading, Inline phrase, Compact spacing, Clamp); the size ladder moved to the `Sizes` pane |
| `TextLink` | Its four group labels were plain text divs; they are `Eyebrow` captions now, the same caption idiom every other page uses |
| `ListCardCounter` | Had no labels at all; both example groups are captioned (Static footer counters, Inherited typography) |
| `HistoryCenter` | Single uncaptioned center; captioned with what the scenario teaches |
| `MessageCenter` | Single uncaptioned center; captioned with what the scenario teaches |
| `SettingsShell` | Single uncaptioned shell; captioned with what the scenario teaches |

## Axis Matrices Moved Out of Examples

The card counted twelve; twenty-one turned up once every page was read. Each
sweep was removed from `Examples` and re-expressed as one representative per
step in the matching pane: `Table`, `DataTable`, `Field`, `Tree`, `ToastStack`,
`EditableList`, `FilterToolbar`, `BulkActionBar`, `AudioPlayer`, `VideoPlayer`,
`MediaPreview`, `AppHeader`, `StatusBar`, `Card`, `DetailItem`,
`DetailSection`, `DetailSectionGroup`, `MetricTile`, `NavCard`, `FormActions`,
`PasswordRequirements`. No page duplicates a sweep across both places.

## Evidence Added

`packages/render/src/audio_specimens.rs` (`cargo test -p poodle-render`, run by
`effigy check:gpui`) — 4 tests:

- `examples_pane_carries_no_axis_matrix` — for all twelve controls, no caption
  in the Examples pane starts with `Sizes —` or `Densities —`.
- `each_axis_step_returns_one_representative` — every size and density step
  returns a smaller tree than the Examples pane, so no step returns a page.
- `the_requested_size_reaches_the_control` — the knob's own fixed box grows
  monotonically across `xs` → `xl`.
- `the_requested_density_reaches_the_control` — every fader density step
  renders.

`packages/gpui/preview/tests/headless_regressions.rs` (`effigy
regressions:native`) — 4 tests, over a `#[path]`-included `specimen_axes`:

- `a_page_publishes_exactly_the_axis_tabs_it_admits` — both, size-only,
  density-only, and Examples-only.
- `an_admitted_tab_is_the_one_that_renders`.
- `a_retained_tab_the_page_no_longer_admits_falls_back_to_examples` — the
  Avatar / Progress / Tooltip shrink case, plus an unknown tab and no stored
  tab.
- `axis_row_keys_are_distinct_per_step`.

Caption evidence is not asserted in a test. `packages/gpui/preview` still cannot
construct a specimen page outside `main.rs`, so there is no seam a headless test
could read a caption through; `g15.026` owns that seam. The caption work is
recorded in the table above and visible in the diff.

## Validation

| Command | Result |
| --- | --- |
| `cargo test -p poodle-render --manifest-path packages/render/Cargo.toml audio_specimens` | ok — 4 passed, 0 failed |
| `effigy check:gpui` | ok — `cargo check -p poodle-gpui-preview` clean; 325 passed / 0 failed (`poodle-render`); 19 passed / 0 failed (node backend) |
| `effigy regressions:native` | ok — 44 passed, 0 failed (40 before this card, 4 added) |
| `effigy docs:check` | ok |
| `git diff --check origin/main...HEAD` | clean |

Compiler warnings in `poodle-gpui-preview` are the five that predate this card
(`all_components`, four unused `node_compat` builder methods). No new warning
was introduced.

Never run, per the card: any `*-windowed` selector, `test:native-visual`, any
Jetstream selector. No sibling Jetstream symlink was created, no page probe or
native screenshot was taken, no conformance run, no `.github/workflows/` edit,
and no release mutation.

## Known Baseline Findings

Recorded, not absorbed: the repository's existing generated-in-src, god-file,
stale-suppression, stale-graph, and comment-ratio findings. `effigy doctor` was
not run — selector routing was unambiguous throughout.

## Deviations and Limitations

1. **The Jetstream consumer is compile-unverified locally.** `cargo check -p
   poodle-jetstream-preview` fails before compiling anything:

   ```
   error: package collision in the lockfile: packages poodle-layout v0.1.0
   (/Users/tom/.t3/worktrees/poodle/poodle/packages/contracts/layout) and
   poodle-layout v0.1.0 (.../t3code-4da0cdec/packages/contracts/layout) are
   different, but only one can be written to lockfile unambiguously
   ```

   The sibling Jetstream crates path-depend on a different Poodle checkout, so
   the paired build cannot resolve from a worker worktree. The card forbids
   creating a sibling symlink. The change there is twelve mechanical one-line
   substitutions to `AudioSpecimen::X.examples(theme)`, and the underlying
   `poodle-render` API is exercised by the tests above. `ci:jetstream` is the
   board that covers it.

2. **Overlay axis panes render a trigger, not a bare surface.** Dialog,
   AlertDialog, Drawer, and CommandPalette build an absolutely-positioned
   backdrop; five stacked in one pane would cover each other. Each step renders
   its trigger and opens its own overlay at that step, which is the shape the
   Svelte axis pane uses.

3. **Text and Eyebrow publish three of five size rows.** Their own size enums
   stop at `md`. `with_sizes_where` drops the absent steps instead of
   substituting `md`, matching the Svelte specimens, which render nothing for
   `lg` and `xl`.

4. **No live GPUI page review.** Not claimed anywhere in this log.
