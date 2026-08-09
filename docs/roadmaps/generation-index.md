# Roadmap Generation Index

## Active Execution Track

- `g12`
  - Status: completed
  - Range: `001` to `019` on disk
  - Notes: React full parity (`001`–`008`) is complete — all 132 components
    ported to `@inflatable-cookie/poodle-react` against a single shared stylesheet, with a
    full per-component preview gallery mirroring the Svelte preview's slugs.
    Web verification and native hardening through `g12.015` are complete.
    `g12.016` completed the public DockRegion drag extension and exact preview
    artifact checkpoint requested by Longhorn. `g12.017` recorded interaction
    evidence, with its IME follow-up left unpromoted. `g12.018` completed public
    overlay geometry requested by Longhorn. `g12.019` completed the GPUI node
    backend, preview migration, and deletion of the duplicate
    `packages/gpui/components` tier. Batch A and Batch B waves 1–2 are
    complete; Batch B Wave 3's standalone
    TextInput/Field migration is pixel-green. The first static embedded
    Button/IconButton and static header/list slices are also pixel-green. The
    FormActions/Button-label slice is pixel-green. The
    AlertDialog/ConfirmAction, Tooltip/Popover, and Menu/CommandPalette trigger
    slices plus the Dialog/Drawer live Button slice are pixel-green. The old
    Button census is zero; all 32 Checkbox/Switch/TriStateSwitch sites are
    node-backed, pixel-green, and click-proven. All 19 Slider/RangeSlider sites
    are node-backed, pixel-green, and drag-proven; all seven RadioGroup sites
    are node-backed, pixel-green, and selection-proven. All 18 Pagination and
    Stepper sites are node-backed and pixel-green; Pagination navigation and
    controlled limit state are click-proven. Wave 3 is complete after the final
    Field site moved to shared render nodes with exact pixels. Wave 4's 29-site
    standalone Pill and 43-site standalone ListCard slices are node-backed,
    handler-proven, and pixel-green; the ListCard slice also migrated seven
    embedded Pill sites; the final embedded ListCard and six Pill sites are
    node-backed and pixel-green too, leaving both old-tier censuses at zero.
    All 36 DetailItem sites are node-backed and pixel-green too, with no
    old-tier DetailItem specimen import remaining.
    The 18-site DetailSection/DetailSectionGroup/DetailShell family is also
    node-backed and pixel-green, with all three old-tier specimen import
    censuses at zero. The 33-site ListCardCounter/MetricTile/Code display batch
    is node-backed and pixel-green too, with all three old-tier specimen
    censuses at zero. The 16-site ListContainer/ListGrid/Surface list-shell
    batch is node-backed and exact too, with all three old-tier specimen
    censuses at zero. Wave 4 is complete after the 25-site
    Box/Grid/Stack/Spacer/Separator structural batch also moved to shared nodes
    with exact pixels and five zero old-tier censuses. Wave 5's 20-site
    AlertDialog/ConfirmAction, Tooltip/Popover, and Menu/CommandPalette overlay
    batch is node-backed, handler-proven, and pixel-green, with six zero
    old-tier censuses. The following 24-site Accordion/Collapsible/ContextMenu/
    HoverCard/Menubar/NavigationMenu batch is node-backed, interaction-proven,
    and pixel-green too, with six more zero old-tier censuses. The following
    23-site DatePicker/DateRangePicker/DateTimePicker/DateTimeRangePicker batch
    is node-backed, interaction-proven, and pixel-green too, with four more
    zero old-tier censuses. The following 26-site Calendar/
    DateTimeZonePicker/TimeField/TimeZoneSelect batch is node-backed,
    interaction-proven, and pixel-green too, with four more zero old-tier
    censuses. The following 24-site ColorPicker/DurationInput/NumberInput
    batch is node-backed and interaction-proven, with exact ColorPicker and
    NumberInput captures and a recorded deterministic DurationInput
    digit-raster residual. The following 27-site CodeInput/TokenInput/
    FileUpload batch is node-backed and pixel-exact with zero old-tier
    constructor sites. The following 21-site Tabs/TabStrip batch is also
    node-backed and pixel-exact with both old-tier censuses at zero. The
    following six-site Breadcrumbs slice is also node-backed and pixel-exact
    with its old-tier census at zero. The following seven-site TextLink slice
    is also node-backed and pixel-exact with its census at zero. The following
    seven-site SelectionSummary slice is node-backed and pixel-exact with its
    census at zero too. Wave 11's Meter/Rating/Table slice is also node-backed
    and pixel-exact with all three old-tier constructor censuses at zero.
    Wave 12's PaginationSummary/ValidationSummary/EmptyState/ResizeHandle
    utility slice is also node-backed and pixel-exact with zero old-tier
    constructor sites; Progress is node-backed and Region is now node-backed
    too, with its dashed-border raster residual deferred.
    Wave 13's MetaBar/MetaItem/NavCard/Callout/StatusBar slice is node-backed
    with zero old-tier constructor sites; NavCard and Callout are exact while
    metadata/status text-raster deltas remain parked. Wave 14's
    PasswordRequirements/ErrorBoundary/InlineListSection/CollapseToggle slice
    is node-backed with zero old-tier constructor sites; ErrorBoundary,
    InlineListSection, and CollapseToggle are exact while the 0.0077%
    PasswordRequirements check/cross raster delta remains deferred. Wave 15's
    Toolbar slice is node-backed with zero old-tier constructor sites and an
    exact focused capture. Wave 16's OrderBy/RefSelect slice is node-backed
    with zero old-tier constructor sites; their 0.0122% and 0.2216% text/icon
    raster deltas remain deferred. Wave 17's standalone FormActions slice is
    exact. Its PageHeader slice is node-backed
    with zero old-tier constructor sites; its 0.0453% text/icon/banner-edge
    residual remains deferred. Wave 18's AppHeader slice is node-backed with zero
    old-tier constructor sites and an exact focused capture.
    Wave 19's FilterToolbar slice is node-backed with an exact focused capture,
    including its Select/TextInput child slots. Wave 21's
    FormLayout slice is node-backed with aligned field/control geometry; its
    0.7501% text/button raster residual is deferred. Wave 22's
    FieldSet slice is node-backed with an exact focused capture. Wave 23's
    ThemeSelect slice is also node-backed with an exact focused capture. Wave
    24's ModelPicker slice is node-backed with aligned panel geometry; its
    0.2638% text/control-raster residual is deferred, with a matching 0.1210%
    embedded AgentChatInput trigger/control residual. Wave 26's FormDialog
    slice is node-backed with a 0.1980% modal/text-raster residual. Wave 29's
    MediaThumbnail/EmbedPreview/MediaPreview slice is node-backed with aligned
    media geometry; its 0.1492%, 0.2103%, and 0.1154% text/icon raster residuals
    are deferred. Wave 30's CardRadioGroup/EmbedInput/PageLoading slice is
    node-backed; EmbedInput is exact, CardRadioGroup retains a 0.9761%
    selected-state/text residual, and PageLoading remains gate-skipped.
    Wave 31's MediaPicker slice is node-backed with aligned browse/upload
    geometry and a deferred 0.5516% icon/text residual. Wave 32's
    DataTable/AgentQuestion/AgentTranscript slice is node-backed too;
    AgentQuestion is exact, while DataTable retains a 0.7217% text/layout-raster
    residual and AgentTranscript a 0.0131% text-raster residual.
    Wave 33's SidebarNav/MediaBrowsePanel/ToastStack slice is node-backed too;
    the first two retain 0.2954% and 0.1829% text/icon residuals, while
    ToastStack preserves its corner overlay with a deferred 1.1702%
    text/icon/animation-raster residual.
    Wave 34's ToastHost is node-backed too, preserving placement with a deferred
    0.5661% text/icon/animation-raster residual.
    Wave 35's Dialog/Drawer overlay slice is node-backed too; Dialog preserves
    custom header/footer slots, both close paths use queued node events, and
    both focused captures are exact.
    Wave 36's DebugDialog/ActionDiscoveryPanel/BulkActionBar slice is
    node-backed too, with only the documented text/icon-raster residuals.
    Wave 37's AgentChatInput composer is node-backed with ModelPicker, toolbar,
    and footer slots; its 0.1377% text/control-raster residual is deferred.
    Wave 38's FilterBuilder/MarkdownEditor slice is node-backed too;
    MarkdownEditor keeps text and mode events in the node event queue, while
    the focused 1.0752% and 0.2304% residuals remain deferred
    text/layout-raster parity.
    Wave 39's EditableList/RelationPicker slice is node-backed too;
    RelationPicker keeps drill-entry and back-path events in the node queue,
    with focused 0.0063% and 1.3903% residuals deferred as text/geometry
    parity.
    Wave 40's EditableLabel slice is node-backed too; its text-change, submit,
    and cancel intents use the node interaction contract and preview event
    queue, with a focused 0.5334% text-raster residual deferred.
    Wave 20's FormShell slice is node-backed with only a deferred 0.0054%
    text/icon raster residual.

- `g11`
  - Status: completed
  - Range: `001` to `008`
  - Notes: Svelte modernization and audited consumer rollout, plus the
    headless-core dual-layer program — framework-free state-machine core,
    interface-stable Svelte adapter layer, appearance-recipe overrides and
    full recipe-hook coverage, Rust machine mirror, and the multi-framework
    adapter pilot that seeded g12. Master spec:
    `docs/specs/062-headless-core-and-dual-layer-strategy.md`. Recorded
    residual debt stays in `g11.004` (extraction register) and `g11.006`
    (domain-math port, Jetstream adoption).

- `g10`
  - Status: completed
  - Range: `001` to `021`
  - Notes: Jetstream feasibility, Svelte overhaul closeout, unified component
    packaging, GPUI parity recovery, token fidelity, contract sync, spec struct
    coverage, and GPUI accessibility baseline are complete. `g10.012` is now
    closed as historical runtime-truth documentation, not the live queue.

## Completed Foundations

- `g09`
  - Status: completed
  - Range: `001` to `009`
  - Notes: architecture unification, GPUI continuation, semantic sizing or
    density rollout, and the original cross-runtime verification tranche are
    complete enough that `g09` no longer acts as the live queue

- `g01`
  - Status: completed
  - Range: `001` to `014`
  - Notes: repository bootstrap, token model, contract system, primitive suite, workstation shells, Underlay bridge, and first parity baseline

- `g02`
  - Status: completed
  - Range: `001` to `016`
  - Notes: advanced composites, product and workstation depth, docs and preview cleanup, API cleanup, packaging, and release baseline

- `g03`
  - Status: completed
  - Range: `001` to `014`
  - Notes: migration policy, parity automation, docs publishing, downstream adoption, ecosystem validation, change control, and extension support

- `g04`
  - Status: completed
  - Range: `001` to `018`
  - Notes: Underlay component parity, new component families, feature depth, and specialist editing or media surfaces

- `g05`
  - Status: completed
  - Range: `001` to `014`
  - Notes: GPUI foundation, spec crates, cross-runtime parity baseline, and demo alignment

- `g06`
  - Status: completed
  - Range: `001` to `015`
  - Notes: shared multi-renderer contract layer, crate restructuring, typed token resolution, layout and event abstractions, style descriptors, adapter traits, and full component-surface expansion

- `g07`
  - Status: completed
  - Range: `001` to `015`
  - Notes: GPUI rendering build-out, adapter crate, theme integration, primitive and composite rendering, workstation shell updates, and cross-runtime parity reporting

- `g08`
  - Status: completed
  - Range: `001` to `011`
  - Notes: consolidated GPUI production-quality, contract-compliance, specimen, accessibility, and visual-parity work

## Working Rule

When roadmap files disagree:

1. treat this index as the top-level source of truth
2. treat `docs/roadmaps/README.md` as the entrypoint
3. treat `g12` as complete; open a new generation only after an explicit
   maintainer sequencing decision

## Rollover policy

Create a new generation only when maintainers explicitly decide the sequencing
baseline needs a real reset.

Generations should be substantial. As a healthy default, expect something
closer to 20 to 40 roadmap files before rollover is worth discussing. Treat
that as a judgment guardrail, not an automatic counter.

Rollover is a closeout event, not a convenience move. Before opening the next
generation:

- close, pause, supersede, or rehome every roadmap in the current generation
- refresh the roadmap front doors so the old generation is visibly closed
- purge stale generation-specific strict-planning artifacts from the active
  `docs/specs/` tree

If that cleanup has not happened, stay in the current generation and finish the
closeout there first.

## Current Release Posture

``g12.019`` is COMPLETE. `packages/gpui/components` — 170 files, 44,796 lines, the
last duplicate component tier — is deleted. Every Poodle target now renders one
implementation (`poodle-render` emitting `poodle-node` trees) through a thin
per-target backend.

Final native visual gate: 136 compared, 98 exact, 37 failing, every failure a
named residual. Probe tests were mined into `packages/render/src/presentation.rs`
(98 → 109 render tests) before the tier went; the handler-drift gate was
repointed at `poodle-render` rather than dropped with its old subject, and the
orphaned `drift:clicks` was retired.

`effigy ci:native` and `effigy test:jetstream-a11y` are green. The Jetstream
audit covers 135 specimens and 20,644 nodes; every role that requires an
accessible name has one. Public-release hardening now owns the live queue
outside this completed generation.

Logs: `docs/logs/2026-08/07-gpui-components-tier-deleted.md` and
`docs/logs/2026-08/07-gpui-node-backend-waves-41-45.md`.
