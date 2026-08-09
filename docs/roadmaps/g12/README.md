# g12 — React Full Parity And Web Verification Depth

**Status: active.** The React parity program (`001`–`008`) and
the bounded work through `018` are complete. `017` remains
native-interaction evidence; its IME follow-up is still unpromoted, and its
"headless GPUI click driver is blocked" note is superseded — the driver works
(see that card). `019` (GPUI node backend) is **complete** as of 2026-08-07:
all batches landed, the old GPUI tier is deleted, and `ci:native` is green with
`drift:events` and a zero-unnamed accessibility audit added since. The
wave-by-wave narrative below is kept as history.
Wave 3's TextInput/Field and static Button/IconButton slices through the
header/list family plus its FormActions/Button-label slice are pixel-green; its
AlertDialog/ConfirmAction trigger slice is pixel-green; its Tooltip/Popover
and Menu/CommandPalette trigger slices plus its Dialog/Drawer live Button
slice are also pixel-green. The old Button census is zero; the 32-site
Checkbox/Switch/TriStateSwitch family is pixel-green and click-proven; the
19-site Slider/RangeSlider family is pixel-green and drag-proven; all seven
RadioGroup sites are pixel-green and selection-proven; all 18
Pagination/Stepper sites are pixel-green, with Pagination navigation and
controlled limit state click-proven. Wave 3 is complete after the final Field
site moved to shared render nodes with exact pixels. Wave 4 has started: all
29 standalone Pill sites and all 43 standalone ListCard sites are node-backed,
handler-proven, and pixel-green; the ListCard slice also migrated its seven
embedded Pill sites. The final embedded ListCard and six Pill sites are now
node-backed and pixel-green too, leaving both old-tier censuses at zero.
All 36 DetailItem sites are node-backed and pixel-green too, with no old-tier
DetailItem specimen import remaining.
The 18-site DetailSection/DetailSectionGroup/DetailShell family is node-backed
and pixel-green too, with all three old-tier specimen import censuses at zero.
The 33-site ListCardCounter/MetricTile/Code display batch is also node-backed
and pixel-green, with all three old-tier specimen import censuses at zero.
The 16-site ListContainer/ListGrid/Surface list-shell batch is node-backed and
pixel-green too, with all three old-tier specimen import censuses at zero.
Wave 4 is complete after the 25-site Box/Grid/Stack/Spacer/Separator structural
batch also moved to shared nodes with exact pixels and five zero old-tier
censuses. Wave 5's 20-site AlertDialog/ConfirmAction, Tooltip/Popover, and
Menu/CommandPalette overlay batch is node-backed, handler-proven, and
pixel-green, with six zero old-tier censuses.
The following 24-site Accordion/Collapsible/ContextMenu/HoverCard/Menubar/
NavigationMenu batch is node-backed, interaction-proven, and pixel-green too,
with all six old-tier constructor censuses at zero.
The following 23-site DatePicker/DateRangePicker/DateTimePicker/
DateTimeRangePicker batch is node-backed, interaction-proven, and pixel-green
too, with all four old-tier constructor censuses at zero.
The following 26-site Calendar/DateTimeZonePicker/TimeField/TimeZoneSelect batch
is node-backed, interaction-proven, and pixel-green too, with all four old-tier
constructor censuses at zero.
The following 24-site ColorPicker/DurationInput/NumberInput value-input batch
is node-backed and interaction-proven; ColorPicker and NumberInput are exact,
while DurationInput retains a deterministic 0.0033% digit-raster residual with
no baseline change. The following 27-site CodeInput/TokenInput/FileUpload
input-composite batch is node-backed and pixel-exact with zero old-tier
constructor sites. The following 21-site Tabs/TabStrip batch is also
node-backed and pixel-exact with both old-tier censuses at zero. The following
six-site Breadcrumbs slice is also node-backed and pixel-exact with its old-tier
census at zero. The following seven-site TextLink slice is also node-backed and
pixel-exact with its old-tier census at zero. The following seven-site
SelectionSummary slice is also node-backed and pixel-exact with its old-tier
census at zero. Wave 11's Meter/Rating/Table feedback/data slice is also
node-backed and pixel-exact with all three old-tier constructor censuses at
zero; Rating selection is event-bridged and Meter/Table preserve their native
fill and flex-layout recipes.
Wave 12's PaginationSummary/ValidationSummary/EmptyState/ResizeHandle utility
slice is also node-backed and pixel-exact with zero old-tier constructor sites;
Progress is node-backed across all sites, and Region is now node-backed too;
its documented dashed-border raster residual remains deferred.
Wave 13's MetaBar/MetaItem/NavCard/Callout/StatusBar slice is node-backed with
zero old-tier constructor sites; NavCard and Callout are exact, while the
metadata/status text-raster deltas remain parked.
Wave 14's PasswordRequirements/ErrorBoundary/InlineListSection/CollapseToggle
slice is node-backed with zero old-tier constructor sites; ErrorBoundary,
InlineListSection, and CollapseToggle are exact, while PasswordRequirements'
0.0077% check/cross raster delta remains parked.
Wave 15's Toolbar slice is node-backed with zero old-tier constructor sites and
an exact focused capture.
Wave 16's OrderBy/RefSelect slice is node-backed with zero old-tier constructor
sites; their focused text/icon raster deltas remain parked at 0.0122% and
0.2216%. Wave 17's PageHeader slice is also node-backed with zero old-tier
constructor sites; its 0.0453% text/icon/banner-edge residual remains
deferred. The standalone FormActions specimen is exact; AppHeader is now
node-backed with an exact focused capture. PickerShell is now node-backed with
aligned ready-body geometry; its 0.5576% focused text/control-raster residual
is deferred.
FilterToolbar is now node-backed with an exact focused capture, including its
Select/TextInput child slots. ModelPicker is now node-backed with aligned
model/axis panel geometry; its 0.2638% focused text/control-raster residual is
deferred. The embedded AgentChatInput slots carry a matching deferred 0.1210%
trigger/control residual.
FormShell is now node-backed with section slots and action rows; its focused
capture has only a deferred 0.0054% text/icon raster residual.
FormLayout is now node-backed with aligned field/control geometry; its 0.7501%
text/button raster residual is deferred. FormDialog is now node-backed with a
0.1980% modal/text-raster residual. MediaThumbnail, EmbedPreview, and
MediaPreview are now node-backed too; their 0.1492%, 0.2103%, and 0.1154%
focused deltas are deferred text/icon raster parity.
CardRadioGroup, EmbedInput, and PageLoading are node-backed as well. EmbedInput
is focused exact; CardRadioGroup retains a deferred 0.9761% selected-state/text
residual, and PageLoading remains skipped by the native gate.
MediaPicker is node-backed too; its browse/upload geometry is aligned with a
deferred 0.5516% icon/text residual.
DataTable, AgentQuestion, and AgentTranscript are node-backed too; AgentQuestion
is focused exact, while DataTable retains a 0.7217% text/layout-raster residual
and AgentTranscript a 0.0131% text-raster residual.
SidebarNav and MediaBrowsePanel are node-backed too, with 0.2954% and 0.1829%
text/icon residuals. ToastStack preserves its corner overlay and retains a
deferred 1.1702% text/icon/animation-raster residual.
ToastHost is node-backed too, preserving placement with a deferred 0.5661%
text/icon/animation-raster residual.
Dialog and Drawer are node-backed too; Dialog keeps custom header/footer slots,
and both focused captures are exact.
DebugDialog, ActionDiscoveryPanel, and BulkActionBar are node-backed too, with
only the documented text/icon-raster residuals.
AgentChatInput is node-backed with ModelPicker, toolbar, and footer slots; its
0.1377% text/control-raster residual is deferred.
FilterBuilder and MarkdownEditor are node-backed through the shared renderers;
MarkdownEditor preserves text/mode events through the node event queue. Their
focused 1.0752% and 0.2304% residuals remain deferred text/layout-raster parity.
EditableList and RelationPicker are node-backed too; RelationPicker preserves
drill-entry and back-path events through the node queue. Their focused 0.0063%
and 1.3903% residuals remain deferred text/geometry parity.
EditableLabel is node-backed too; its text-change, submit, and cancel intents
use the node interaction contract and preview event queue. Its focused 0.5334%
text-raster residual remains deferred; Wave 40's full gate compared all 136
components with no capture failures or baseline writes.
FieldSet is now node-backed with an exact focused capture. ThemeSelect is also
node-backed with an exact focused capture.

**React parity: COMPLETE.** All 132 components ported to `@inflatable-cookie/poodle-react` and
Playwright-verified against the Svelte preview. The React preview is a full
per-component gallery matching the Svelte preview (shell, controls, Tokens
inspector, usage docs, 131/131 specimen slugs) plus docs / parity /
accessibility report generators. `@inflatable-cookie/poodle-react` has a consumer README
(`packages/react/components/README.md`). Docs and parity data are authored
canonically in the Svelte preview and re-exported live into React — no fork.

Goal: `@inflatable-cookie/poodle-react` grows from the 3-component pilot (g11.007) to full
library parity (132 components), as a strategic second web target — no
consuming app yet, so the acceptance bar is the React preview plus
interaction verification, not a consumer rollout.

Decisions inherited from g11.007:

- Hand-written TSX shells (~90 LOC each) over `@inflatable-cookie/poodle-core` machines.
  Mitosis rejected twice (spec 062); no compiler layer.
- Tokens, recipes, and the contrast axis are plain CSS — shared unchanged.
- The Svelte implementation remains the visual proof reference.

Runway:

- `001-shared-styles-package.md` — extract every remaining Svelte
  `<style>` block and move all component CSS to `@inflatable-cookie/poodle-core/styles`, imported
  by both frameworks. Single styling source; React never duplicates CSS.
- `002-react-infra-and-conversion-playbook.md` — React preview harness
  (hash-routed specimens like the Svelte preview), shared type strategy,
  documented conversion recipe.
- `003`–`008` — family batches: primitives/display, controls/forms,
  overlays/navigation, layout/form shells, data/date, media/workstation
  composites.
- `008-parity-verification.md` — preview coverage sweep, Playwright
  behavior parity samples against the Svelte preview, docs. **Complete.**
- `009-visual-regression-gate.md` — cross-framework pixel diff (Svelte vs
  React, same slug, same axes). The structural gates cannot see a
  component render at the wrong size; the ListCard `data-size` bug proved
  it. **Complete** — 256/256 sweep pairs and 180/180 axis pairs green,
  after 14 divergences found and fixed across specimens, shells and
  `@inflatable-cookie/poodle-core/styles`.
- `010-agent-composer.md` — the agent composer family across all four
  targets: `Meter shape="ring"`, `ModelPicker` (model + host-declared
  capability axes in one popover) and `AgentChatInput`. **Complete.**
- `011-overlay-portalling.md` — every anchored overlay portals to the theme
  root and is positioned in viewport coordinates. Found from g12.010: a
  scrolling ancestor clips a surface whatever its z-index, and a transformed
  one traps even `position: fixed`. 23 components across both web frameworks,
  behind one shared primitive. **Complete.**
- `012-workstation-tier-removal.md` — delete the retired `poodle-workstation`
  spec crate and its GPUI remnants. Six of its thirteen specs duplicated
  `poodle-specs`; the other seven had no component, contract or Svelte
  counterpart on any target. Closes the "Remaining" section g09.006 left open.
  **Complete.**
- `013-native-spec-surface-parity.md` — nothing measured whether a documented
  prop reached `poodle-specs`, so nothing could tell whether GPUI and Jetstream
  had tracked the web. New `contract-spec-drift` gate found 93 real gaps; all
  93 closed, plus a renderer pass so both targets draw what they can now
  reach. **Complete.**
- `014-native-visual-gate.md` — pixel baselines for the GPUI preview, closing
  the hole `g12.009` left on the native side. Found that GPUI *was* runnable
  here all along; the "build-only" note was wrong. Then Jetstream's headless
  offscreen render landed and settled the question: 90s and zero flake, against
  20min and ~3% for window capture. **Complete.**
- `015-native-accessibility-options.md` — `003-native-accessibility.md`
  recorded that neither runtime exposes an accessibility API; this costed the
  way out per engine and then built the half that was tractable. The two were
  never in the same position: gpui 0.2.2 is the latest published version and
  has nothing, while Jetstream ran winit 0.30 against a version-compatible
  `accesskit_winit`, over a `UiTree` that already carried bounds, roles and
  parent/child links. **Jetstream now has a live AccessKit surface** — tree
  projection, adapter, and actions routed through the same handlers as pointer
  input — with 108 Poodle components attaching their `aria_label` to it. GPUI
  still waits on upstream, because the work it needs is the work upstream would
  obsolete. **Complete for Jetstream; GPUI held deliberately.**
- `016-public-dock-drag-extension-and-preview-artifact.md` — add the public
  async pre-drag/source lifecycle and external-drop target seam needed by
  desktop hosts, then prove the exact packed Svelte preview artifact outside
  sibling source resolution. **Complete.**
- `017-native-interaction-parity.md` — the "render-only natives" claim in eleven
  contracts was false: GPUI wires clicks in 71 of ~97 components, and 35 of them
  accepted a handler they never read. The agent chat set is now interactive on
  GPUI and `drift:handlers` gates the dead-handler class. Jetstream needs no
  engine work — its runtime dispatches clicks and the preview already feeds
  pointer state — only a decision about handler shape.
  **GPUI done and gated; Jetstream scoped.**
- `018-public-overlay-geometry-observation.md` — explicit immutable viewport
  snapshots for built-in Popover/Menu surfaces across Svelte and React, without
  public DOM or host-runtime coupling. **Complete; requested by Longhorn.**
- `020-public-package-consolidation.md` — six publish-intent npm packages to
  three, grouped by framework binding: `poodle-core`, `poodle-svelte`,
  `poodle-react`. **Complete.**
- `021-icon-catalogue-boundary.md` — `poodle-core` stops vendoring the 1,703
  Lucide icons. Measured: 84% of the tarball, 8% of it used, and a namespace
  import in `icon-registry.ts` puts all of them in every consumer bundle.
  Poodle keeps the twelve its own components need. **Ready; blocks Longhorn's
  first publication.**
- `019-gpui-node-backend.md` — the symmetric GPUI half of the
  Poodle↔Jetstream inversion: a Node → GPUI interpreter crate, GPUI preview
  migration onto `poodle-render` nodes, then deletion of
  `packages/gpui/components` — the last duplicate component tier. Verified by
  the g12.014 native visual gate held at zero diff. **In progress; Batch A
  and Batch B waves 1–2 complete; Wave 3's old Button census is zero and the
  32-site Checkbox/Switch/TriStateSwitch family is node-backed, pixel-green,
  and click-proven; the 18-site Pagination/Stepper family is node-backed and
  pixel-green; Waves 3–4 are complete; Wave 5's first 93 constructor sites are
  node-backed and pixel-green; Waves 6–10 cover CodeInput/TokenInput/FileUpload,
  Tabs/TabStrip, Breadcrumbs, TextLink, and SelectionSummary with exact focused
  captures and zero old-tier constructor sites.**

- Ongoing: `check:svelte` (svelte-check over `@inflatable-cookie/poodle-svelte`, driven
  through the isolated `install-smoke` consumer) now runs in `ci:web` and
  `ci-web.yml`. Added after five type errors in `Rating.svelte` were found
  from a consumer repo rather than from Poodle's own gates.

## Next Task

``019-gpui-node-backend.md`` is COMPLETE. `packages/gpui/components` — 170 files, 44,796 lines, the
last duplicate component tier — is deleted. Every Poodle target now renders one
implementation (`poodle-render` emitting `poodle-node` trees) through a thin
per-target backend.

Final native visual gate: 136 compared, 98 exact, 37 failing, every failure a
named residual. Probe tests were mined into `packages/render/src/presentation.rs`
(98 → 109 render tests) before the tier went; the handler-drift gate was
repointed at `poodle-render` rather than dropped with its old subject, and the
orphaned `drift:clicks` was retired.

Next up, not part of this card: `effigy test:jetstream-a11y` fails on 151
unnamed `TextInput` nodes — pre-existing, and unmasked now that `ci:native` gets
past its dead tasks. It is the last thing between `ci:native` and green.

Logs: `docs/logs/2026-08/07-gpui-components-tier-deleted.md` and
`docs/logs/2026-08/07-gpui-node-backend-waves-41-45.md`.
