# Roadmaps

Status: active
Updated: 2026-08-07

Roadmaps hold executable milestone work for Poodle.

## Rules

- active milestone files live in generation folders such as `g01/`
- file names use `NNN-slug.md` with numbering local to the generation
- references use roadmap IDs such as `g09.001`
- generation rollover is manual only
- treat generations as substantial sequencing eras, not one-or-two-file
  buckets; as a healthy default, expect roughly 20 to 40 roadmap files in one
  generation before rollover is even worth discussing
- treat rollover as full generation closeout, not a convenience reset: close,
  supersede, or rehome every roadmap in the current generation first, then
  purge stale strict-planning artifacts from the active `docs/specs/` tree
- backlog items belong in `backlog/`
- architecture belongs in `../architecture/`, not here

## Current Index State

- highest on-disk generation folder: `g12`
- current executable generation plan: `g12`
- `g09` is complete
- `g10` is complete
- `g11` is complete
- `g12.001`–`g12.016` are complete
- `g12.017` remains interaction evidence with an unpromoted IME follow-up
- `g12.018` is complete
- `g12.019` (GPUI node backend) is in progress; Batch B waves 1–2 are complete,
  Wave 3's TextInput/Field and static Button/IconButton slices through the
  header/list family, its FormActions/Button-label slice, and its
  AlertDialog/ConfirmAction trigger slice are pixel-green; its Tooltip/Popover
  and Menu/CommandPalette trigger slices plus its Dialog/Drawer live Button
  slice are also pixel-green; the old Button census is zero, the 32-site
  Checkbox/Switch/TriStateSwitch family is pixel-green and click-proven, and
  the 19-site Slider/RangeSlider family is pixel-green and drag-proven; all
  seven RadioGroup sites are pixel-green and selection-proven; all 18
  Pagination/Stepper sites are pixel-green, with Pagination navigation and
  controlled limit state click-proven; Wave 3 is complete after the final
  Field site moved to shared render nodes with exact pixels; Wave 4's 29-site
  standalone Pill and 43-site standalone ListCard slices are node-backed,
  handler-proven, and pixel-green, including seven nested Pill sites in the
  ListCard specimen; the last embedded ListCard and six Pill sites are also
  node-backed and pixel-green, leaving both old-tier censuses at zero; all 36
  DetailItem sites are node-backed and pixel-green too; the 18-site
  DetailSection/DetailSectionGroup/DetailShell family is node-backed and
  pixel-green with all three old-tier censuses at zero; the 33-site
  ListCardCounter/MetricTile/Code display batch is also node-backed and
  pixel-green, with all three old-tier specimen censuses at zero; the 16-site
  ListContainer/ListGrid/Surface batch is node-backed and exact too, with all
  three old-tier specimen censuses at zero; Wave 4 is complete after the
  25-site Box/Grid/Stack/Spacer/Separator structural batch also moved to shared
  nodes with exact pixels and five zero old-tier censuses; Wave 5's 20-site
  AlertDialog/ConfirmAction, Tooltip/Popover, and Menu/CommandPalette overlay
  batch is node-backed, handler-proven, and pixel-green, with six zero
  old-tier censuses; the following 24-site Accordion/Collapsible/ContextMenu/
  HoverCard/Menubar/NavigationMenu batch is node-backed, interaction-proven,
  and pixel-green too, with six more zero old-tier censuses; the following
  23-site DatePicker/DateRangePicker/DateTimePicker/DateTimeRangePicker batch is
  node-backed, interaction-proven, and pixel-green too, with four more zero
  old-tier censuses; the following 26-site Calendar/DateTimeZonePicker/
  TimeField/TimeZoneSelect batch is node-backed, interaction-proven, and
  pixel-green too, with four more zero old-tier censuses; the following
  24-site ColorPicker/DurationInput/NumberInput batch is node-backed and
  interaction-proven, with exact ColorPicker/NumberInput captures and a
  recorded deterministic DurationInput digit-raster residual; the following
  27-site CodeInput/TokenInput/FileUpload batch is node-backed and pixel-exact
  with zero old-tier constructor sites; the following 21-site Tabs/TabStrip
  batch is also node-backed and pixel-exact with both old-tier censuses at
  zero; the following six-site Breadcrumbs slice is node-backed and
  pixel-exact with its old-tier census at zero; the following seven-site
  TextLink slice is also node-backed and pixel-exact with its census at zero;
  the following seven-site SelectionSummary slice is node-backed and
  pixel-exact with its census at zero too; Wave 11's Meter/Rating/Table slice
  is also node-backed and pixel-exact with all three old-tier constructor
  censuses at zero; Wave 12's PaginationSummary/ValidationSummary/EmptyState/
  ResizeHandle slice is also node-backed and pixel-exact with zero old-tier
  constructor sites; Wave 13's MetaBar/MetaItem/NavCard/Callout/StatusBar slice
  is node-backed with zero old-tier constructor sites, with only the deferred
  metadata/status text-raster deltas remaining; Wave 14's
  PasswordRequirements/ErrorBoundary/InlineListSection/CollapseToggle/Toolbar/
  OrderBy/RefSelect slice is
  node-backed with zero old-tier constructor sites, with only the deferred
  PasswordRequirements check/cross raster delta remaining; Wave 17's
  PageHeader slice is also node-backed with zero old-tier constructor sites,
  with a 0.0453% text/icon/banner-edge residual deferred. The standalone
  FormActions specimen is exact; Wave 18's AppHeader slice is node-backed with
  an exact focused capture. Wave 25's PickerShell slice is node-backed with
  aligned ready-body geometry; its 0.5576% text/control-raster residual is
  deferred.
  Wave 19's FilterToolbar slice is node-backed with an exact focused capture,
  including its Select/TextInput child slots. Wave 21's
  FormLayout slice is node-backed with aligned field/control geometry; its
  0.7501% text/button raster residual is deferred. Wave 22's
  FieldSet slice is node-backed with an exact focused capture. Wave 23's
  ThemeSelect slice is also node-backed with an exact focused capture. Wave
  24's ModelPicker slice is node-backed with aligned panel geometry; its
  0.2638% text/control-raster residual is deferred, with a matching 0.1210%
  embedded AgentChatInput trigger/control residual. Wave 26's FormDialog slice
  is node-backed with a 0.1980% modal/text-raster residual.
  Wave 29's MediaThumbnail/EmbedPreview/MediaPreview slice is node-backed with
  aligned media geometry; its 0.1492%, 0.2103%, and 0.1154% text/icon raster
  residuals are deferred. Wave 30's CardRadioGroup/EmbedInput/PageLoading slice
  is node-backed; EmbedInput is exact, CardRadioGroup retains a 0.9761%
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
  custom header/footer slots, both close paths use queued node events, and both
  focused captures are exact.
  Wave 36's DebugDialog/ActionDiscoveryPanel/BulkActionBar slice is node-backed
  too, with only the documented text/icon-raster residuals.
  Wave 37's AgentChatInput composer is node-backed with ModelPicker, toolbar,
  and footer slots; its 0.1377% text/control-raster residual is deferred.
  Wave 38's FilterBuilder/MarkdownEditor slice is node-backed too; MarkdownEditor
  keeps text and mode events in the node event queue, while the focused
  1.0752% and 0.2304% residuals remain deferred text/layout-raster parity.
  Wave 39's EditableList/RelationPicker slice is node-backed too; RelationPicker
  keeps drill-entry and back-path events in the node queue, with focused
  0.0063% and 1.3903% residuals deferred as text/geometry parity.
  Wave 40's EditableLabel slice is node-backed too; its text-change, submit,
  and cancel intents use the node interaction contract and preview event
  queue, with a focused 0.5334% text-raster residual deferred.
  Wave 20's FormShell slice is node-backed with only a deferred 0.0054%
  text/icon raster residual.

The canonical summary of roadmap status is `generation-index.md`.
If a generation README conflicts with that file, treat the index as the source of truth until the generation README is reconciled.

## Available Generations

- `g01` foundation, token system, contracts, primitive suite, workstation shells, and first Underlay bridge baseline
- `g02` advanced composites, docs/catalog depth, cleanup, and release baseline
- `g03` hardening, migration policy, parity automation, downstream adoption, validation, and extension support
- `g04` Underlay component parity, new component families, feature depth, and specialist media or editing surfaces
- `g05` GPUI foundation, spec crates, cross-runtime parity baseline, and demo-app alignment
- `g06` shared multi-renderer contract layer, typed token resolution, layout or event abstractions, and full component-surface expansion
- `g07` GPUI rendering build-out, adapter crate, primitive and composite rendering, workstation shell updates, and parity reporting
- `g08` consolidated GPUI production-quality and compliance program
- `g09` completed GPUI continuation and semantic sizing/density generation
- `g10` completed Jetstream-feasibility / GPUI-production-hardening generation
- `g11` completed Svelte modernization and consumer rollout generation
- `g12` active after React parity, verification depth, native hardening,
  DockRegion extension, and public overlay geometry observation; `g12.019`
  (GPUI node backend) is in progress

## Working Rule

Use the top-level generation index to determine what is current, then open the relevant generation folder.
Do not assume the highest-numbered generation folder is the active one.

## Rollover guardrail

Do not open `gNN+1` while the current generation still has live roadmap files
or stale strict-planning debris in the active specs tree.

Before rollover:

- every roadmap in the closing generation must be explicitly closed, paused,
  superseded, or moved to backlog
- the roadmap front doors must agree that the old generation is no longer the
  live queue
- stale strict-planning artifacts for that generation must be purged from the
  active `docs/specs/` tree

## Next Task

Continue the next `g12.019` old-tier constructor wave. Wave 40 moved
EditableLabel onto the node backend with queued text-change, submit, and cancel
intents; its 0.5334% focused text-raster residual remains deferred. The
DurationInput
native-visual residual is parked at a deterministic 0.0033% until backend text
parity is addressed. Meter, Rating, Table, PaginationSummary, ValidationSummary,
Progress, EmptyState, ResizeHandle, MetaBar, MetaItem, NavCard, Callout,
StatusBar, TextLink, Breadcrumbs, Tabs, TabStrip, CodeInput, TokenInput,
FileUpload, SelectionSummary, PasswordRequirements, ErrorBoundary,
InlineListSection, CollapseToggle, Toolbar, OrderBy, RefSelect, and PageHeader are now node-backed with zero old-tier
constructor sites; NavCard, Callout, ErrorBoundary, InlineListSection, and
CollapseToggle, and Toolbar are exact, while metadata/status,
PasswordRequirements, OrderBy, RefSelect, and PageHeader text/icon differences
remain deferred. PickerShell is now node-backed with aligned ready-body
geometry; its 0.5576% focused text/control-raster residual is deferred. The
standalone FormActions specimen is node-backed with an exact focused capture;
FormDialog is now node-backed with a 0.1980% modal/text-raster residual.
AppHeader is node-backed too, with an exact focused capture across its
identity, action, and utility slots.
