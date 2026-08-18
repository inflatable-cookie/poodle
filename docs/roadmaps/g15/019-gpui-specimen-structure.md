# g15.019 — GPUI Specimen Structure

Status: **complete** — PR #40 merged at `23150ead`; all 74 scoped axis
corrections and six caption repairs landed, with EmptyState and Icon returned
as explicit follow-up axis-domain gaps
Consumes: `g15.011` partial screening baseline
Depends on: `g15.017` (web axis target)
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`
(Cross-Runtime Agreement), `../../contracts/001-working-rules.md`
(Runtime Parity Authority)

## Outcome

The native catalogue teaches what the web catalogue teaches. Not with the same
layout mechanics — with the same evidence.

`g15.010` closed the last missing GPUI specimen, so every component in the
active cohort has a native page. `g15.011` and the merged `g15.017` web census
leave three exact native structure gaps:

- 59 pages omit eligible axis panes.
- 12 pages keep eligible size/density matrices inside `Examples` instead of
  the matching panes.
- Six pages render their examples with no captions at all.

The six caption gaps overlap the axis set. A readiness recheck also found
three pages that over-advertise axes because the current native helper always
constructs both panes: `Avatar` and `Progress` need size only; `Tooltip` needs
neither. The migration denominator is therefore 74 axis corrections plus six
caption corrections, not the earlier `59 + 6` shorthand.

Per the working rules, a capability present in Svelte and absent from another
active runtime is a gap to port, not an accepted delta. Layout mechanics are
runtime-owned; the evidence is not.

## Exact Scope

Make both axis panes authoritative on these 59 pages. For the 12 pages that
already render the matrix inside `Examples`, this means moving it rather than
inventing it again:

`ConfirmAction`, `TriStateSwitch`, `DragNumberField`, `EmbedInput`,
`FileUpload`, `Select`, `SplitView`, `StatusIndicator`, `ToastHost`,
`ToastStack`, `SidebarNav`, `Tabs`, `Tree`, `Accordion`, `AlertDialog`,
`CommandPalette`, `Dialog`, `Drawer`, `FormDialog`, `BlockEditor`, `Field`,
`MarkdownEditor`, `BulkActionBar`, `CardToggleGroup`, `DataTable`,
`EditableList`, `FilterToolbar`, `ListCard`, `LogList`, `RelationPicker`,
`Table`, `AudioPlayer`, `MediaBrowsePanel`, `MediaPicker`, `MediaPreview`,
`VideoPlayer`, `AppHeader`, `HistoryCenter`, `MessageCenter`, `PageHeader`,
`StatusBar`, `AgentQuestion`, `AgentTranscript`, `AudioMeter`, `AudioSwitch`,
`EnvelopeEditor`, `Fader`, `GainReductionMeter`, `Keyboard`, `Knob`,
`ModMatrixGrid`, `ValueReadout`, `WaveformDisplay`, `XYPad`,
`LicenceActivation`, `LicenceSeats`, `LicenceStatus`, `UpdateCenter`, and
`UpdateStatus`.

Add size only on `Eyebrow`, `Text`, `Meter`, and `PasswordRequirements`.

Add density only on `Card`, `DetailItem`, `UiPresentationProvider`,
`MetricTile`, `NavCard`, `FormActions`, `DetailSection`, and
`DetailSectionGroup`.

Correct the three existing over-advertisements: `Avatar` and `Progress` keep
`Sizes` and lose `Densities`; `Tooltip` loses both axis panes.

Caption the Examples content on `Text`, `TextLink`, `ListCardCounter`,
`HistoryCenter`, `MessageCenter`, and `SettingsShell`.

Implementation scope also includes:

- one breaking migration of the preview-local `specimen_layout` API so pane
  admission is explicit and an unsupported or empty axis cannot be shown;
  migrate every caller once, including the bounded scene adapter, with no
  default-both fallback or compatibility twin
- `poodle_render::audio_specimens`: split examples, sizes, and densities in the
  same migration that adopts those panes; update direct consumers without a
  compatibility twin
- Jetstream stays program-deferred and out of scope

## Goals

- [ ] Every native page shows `Sizes` iff the merged web census says the
      component takes `size`, and `Densities` iff it takes `density`.
- [ ] Eligible panes contain one representative per step; no axis matrix
      remains inside `Examples`.
- [ ] The six caption-less native pages caption their examples.
- [ ] Native pages that legitimately keep a bounded renderer-owned adapter
      publish their outline instead, and the audit row records that choice.
- [ ] No native page reproduces web layout mechanics for their own sake.

## Acceptance

- [ ] All 74 named axis corrections match the merged web census. The worker
      records an exact checked list in the batch log; `g15.026` later replaces
      this structural proof with live headless page evidence.
- [ ] No native page renders uncaptioned examples.
- [ ] `effigy check:gpui` and the headless native regression board pass.
- [ ] No Jetstream selector run; no `*-windowed` selector run.
- [ ] No claim of live GPUI page review. `packages/gpui/preview` cannot yet
      construct a specimen page outside `main.rs`; `g15.026` owns that seam
      and the 174-page live headless probe.

## Stop Conditions

- Native pages start copying web DOM structure.
- The card reaches for a shared render tree. The outline is a document.
- The worker starts building a page-construction probe or screenshot system;
  that belongs to `g15.026`.
- Jetstream parity is smuggled in before its admission gate.

## Writable Scope

- `packages/gpui/preview/src/specimens/*`
- `packages/render/src/audio_specimens.rs` and the direct GPUI/Jetstream
  compile consumers required by its breaking internal shape migration;
  Jetstream receives no new parity work
- focused tests for the preview-local axis-admission decision, where they can
  run headlessly without introducing the `g15.026` page seam
- one batch log

## Validation

- `effigy check:gpui`, `effigy regressions:native`, `effigy docs:check`,
  `git diff --check`
- headless only. Never `test:native-visual`, `qa:jetstream`, or a
  `*-windowed` selector.
