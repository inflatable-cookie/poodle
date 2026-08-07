# 001 - Working Rules

Status: active
Owner: Poodle core
Depends on: `docs/architecture/product-guardrails.md`

## Contract

- Treat `docs/roadmaps/`, `docs/specs/`, and `docs/logs/` as the execution
  authority chain for active Poodle work.
- Use `docs/specs/` as the strict planning and execution-control layer when the
  roadmap alone is not enough to keep the next owner honest.
- In a strict lane, a bare `continue` should resolve through the previous
  closeout's `Next Task`, which should point at the current ready card or an
  explicit planning gate.
- If there is no ready card, the lane is in planning. Do not improvise from a
  dirty worktree or the most recent chat summary.
- When multiple plausible next seams exist inside `g10`, freeze the active
  posture first, then choose the next owner deliberately.
- Keep currentness surfaces aligned so completed cards do not remain advertised
  as ready.

## Generation Rollover Rule

Treat roadmap generations as substantial sequencing eras, not tiny buckets. In
a long-running repo, expect roughly 20 to 40 roadmap files in one generation
before rollover is even worth discussing.

Treat rollover as full closeout:

- every roadmap in the old generation must be explicitly closed, paused,
  superseded, or moved to backlog
- the roadmap front doors must reflect that closed state before the next
  generation opens
- stale strict-planning artifacts from the closing generation must be archived
  or removed from the active `docs/specs/` tree

If those closeout conditions are not satisfied, repair the current generation
instead of opening a new one.

## Typography Inherit Rule

Use `typography="inherit"` for inline text-like primitives when parent copy
should own the local text scale.

Two modes are allowed:

- text-only inherit: for primitives without shell geometry, inherit font metrics
  directly from the parent
- proportional inherit: for primitives with visible shell geometry, convert the
  component's size preset from token `rem` values into equivalent `em` values
  so text, padding, gaps, and other shell metrics stay proportional

Runtime note:

- CSS runtimes should implement this literally with inherited font metrics and
  `em`-relative shell geometry
- non-CSS runtimes may approximate proportional inherit with equivalent
  ratio-preserving metrics from a 1rem baseline until parent-relative inline
  layout exists; that limitation must stay documented on the runtime side

Do not overload `size` with an `"inherit"` option for this behavior. `size`
continues to mean the component's own semantic size preset.

## Svelte Surface Modernization Rule

Treat the current Svelte component layer as compatibility-first, not as the
target shape for new work.

Rules:

- new or substantially reshaped Svelte components should prefer Svelte 5
  runes-based internals over `export let` plus `$:` compatibility mode
- new public composition surfaces should prefer callback props and snippets over
  introducing new `createEventDispatcher` and legacy slot APIs
- do not add new compatibility alias props like parallel `items` / `options`
  inputs unless there is a specific downstream migration need documented first
- when a legacy component is touched substantially, remove old compatibility
  baggage before adding more surface area if that can be done without breaking
  current consumers

Operational note:

- use `effigy svelte:surface-audit` to keep the legacy surface visible during
  modernization work
- the audit is a report, not a gate; the goal is to stop drift first, then
  reduce the backlog deliberately

## Current Posture

Poodle completed `g12.018`. `g12.019` (GPUI node backend) is in progress:
Batch A and Batch B waves 1–2 are complete. Wave 3's TextInput/Field migration,
including embedded TextInput use, is pixel-green. The first static embedded
Button/IconButton and static header/list slices are also pixel-green. The
FormActions/Button-label slice is pixel-green. The AlertDialog/ConfirmAction
trigger, Tooltip/Popover trigger, and Menu/CommandPalette opener slices are
also pixel-green. The Dialog/Drawer and Toolbar live Button slices are
pixel-green, and no old-tier Button constructor remains in specimen source.
The 32 Checkbox/Switch/TriStateSwitch sites are node-backed, pixel-green, and
click-proven; no old-tier import for those controls remains in specimen source.
The 19 Slider/RangeSlider sites are also node-backed, pixel-green, and
drag-proven; no old-tier import for either control remains in specimen source.
All seven RadioGroup sites are node-backed, pixel-green, and selection-proven;
no old-tier RadioGroup import remains in specimen source. All 18 Pagination
and Stepper sites are also node-backed and pixel-green; Pagination navigation
and controlled limit-menu state are click-proven, and neither control retains
an old-tier specimen import. Wave 3 is complete: the final Field site in the
EmbedInput specimen is node-backed and pixel-green, and no old-tier Field
import remains in specimen source. Wave 4 has started: all 29 standalone Pill
sites and all 43 standalone ListCard sites are node-backed, handler-proven,
and pixel-green. The ListCard specimen's seven embedded Pill sites are also
node-backed. The final ListGrid ListCard and remaining six embedded Pill sites
are now node-backed and pixel-green too; neither component retains an old-tier
specimen import.
All 36 DetailItem sites across the standalone, DetailSection,
DetailSectionGroup, and DetailShell specimens are also node-backed and
pixel-green; no old-tier DetailItem specimen import remains.
The 18-site DetailSection/DetailSectionGroup/DetailShell family is also
node-backed and pixel-green, with all three old-tier specimen import censuses
at zero.
The 33-site ListCardCounter/MetricTile/Code display batch is also node-backed
and pixel-green, with all three old-tier specimen import censuses at zero.
The 16-site ListContainer/ListGrid/Surface list-shell batch is node-backed and
pixel-green too, with all three old-tier specimen import censuses at zero.
Wave 4 is complete: the final 25-site Box/Grid/Stack/Spacer/Separator structural
batch is node-backed and pixel-green, with all five old-tier specimen import
censuses at zero. Wave 5's 20-site AlertDialog/ConfirmAction, Tooltip/Popover,
and Menu/CommandPalette overlay batch is node-backed, handler-proven, and
pixel-green, with all six old-tier specimen import censuses at zero.
The following 24-site Accordion/Collapsible/ContextMenu/HoverCard/Menubar/
NavigationMenu batch is also node-backed, interaction-proven, and pixel-green;
all six old-tier constructor censuses are zero.
The following DatePicker/DateRangePicker/DateTimePicker/DateTimeRangePicker,
Calendar/DateTimeZonePicker/TimeField/TimeZoneSelect, and
ColorPicker/DurationInput/NumberInput waves are node-backed and interaction-
proven; DurationInput retains a deterministic 0.0033% digit-raster residual.
CodeInput/TokenInput/FileUpload, Tabs/TabStrip, Breadcrumbs, TextLink, and
SelectionSummary are now node-backed with exact focused captures and zero
old-tier constructor sites. Meter, Rating, and Table are now node-backed with
exact focused captures and zero old-tier constructor sites too.
PaginationSummary, ValidationSummary, Progress, EmptyState, and ResizeHandle
are now node-backed too, with exact deterministic captures where the native
gate applies. Region remains parked on its documented dashed-border raster
residual.
MetaBar, MetaItem, NavCard, Callout, and StatusBar are now node-backed too;
NavCard and Callout are exact, while metadata/status text-raster deltas remain
parked under the deferred text-parity allowance.
PasswordRequirements, ErrorBoundary, InlineListSection, and CollapseToggle are
now node-backed too; ErrorBoundary, InlineListSection, and CollapseToggle are
exact, while PasswordRequirements retains a deferred 0.0077% check/cross
text-icon raster delta.
Toolbar is now node-backed too, with an exact focused capture and node-backed
Button/Separator children.
OrderBy and RefSelect are now node-backed too, with handler-capable bridges;
their 0.0122% and 0.2216% focused text/icon raster deltas remain deferred.
PageHeader is now node-backed through node-compatible action, breadcrumb, and
metadata slots; its 0.0453% focused text/icon/banner-edge residual remains
deferred. PickerShell is parked on a ready-body geometry mismatch.
Wave 40 moved EditableLabel onto the node backend with queued text-change,
submit, and cancel intents; its focused 0.5334% text-raster residual remains
deferred. The full gate compared all 136 components with no capture failures
or baseline writes.

## Next Task

Continue the next `g12.019` old-tier constructor wave. The DurationInput
native-visual residual is parked at a deterministic 0.0033% until backend text
parity is addressed. Meter, Rating, Table, PaginationSummary, ValidationSummary,
Progress, EmptyState, ResizeHandle, MetaBar, MetaItem, NavCard, Callout,
StatusBar, TextLink, Breadcrumbs, Tabs, TabStrip, CodeInput, TokenInput,
FileUpload, SelectionSummary, PasswordRequirements, ErrorBoundary,
InlineListSection, CollapseToggle, Toolbar, OrderBy, RefSelect, and PageHeader are now node-backed with zero old-tier
constructor sites; NavCard, Callout, ErrorBoundary, InlineListSection, and
CollapseToggle, and Toolbar are exact, while metadata/status,
PasswordRequirements, OrderBy, RefSelect, and PageHeader text/icon differences
remain deferred. PickerShell remains parked on its ready-body geometry mismatch.
The standalone FormActions specimen is node-backed with an exact focused
capture; FormDialog and PickerShell retain their old FormActions slots for
their parked composite migrations.
AppHeader is node-backed too, with an exact focused capture across its
identity, action, and utility slots.
