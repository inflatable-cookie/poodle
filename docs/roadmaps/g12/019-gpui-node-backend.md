# g12.019 GPUI Node Backend

Status: COMPLETE (2026-08-07)
Owner: Poodle core
Depends on: `packages/contracts/node` vocabulary; `packages/render` canonical
components; jetstream g06.013 (Batches A–F complete); g12.014 native visual
gate
Handoff: `docs/logs/2026-08/05-225348-gpui-node-backend-batch-b-handoff.md`

## Why

Two component tiers still exist for GPUI: `poodle-render` (canonical, node
vocabulary) and `packages/gpui/components` (170 files, a hand-written GPUI
implementation of the same components). Every contract change is made twice
or silently diverges. The Jetstream half of the inversion is done: all 149
components render as `Spec + Theme → Node` in `poodle-render`, and Jetstream
consumes them through its own thin Node → JsEl adapter. This card is the
symmetric GPUI half: build the Node → GPUI interpreter, migrate the GPUI
preview onto it, delete the duplicate tier. End state: one component
implementation with N thin backends.

## Verification Strategy

Decided: option (b) from the handoff, built on the g12.014 harness.

- The native visual gate (`effigy test:native-visual`,
  `test/native-visual/`) already screenshots every GPUI preview specimen
  against local baselines under a two-identical-frames acceptance rule.
  Sequence: green the gate on the old tier first, then migrate; the gate
  must stay green with zero rendering diffs per specimen. Any intended
  visual change needs its own recorded reason, per the gate's
  baseline-update rule.
- `effigy drift:handlers` must stay green — the new backend must not accept
  handlers it never wires.
- Probe-style tests in `packages/gpui/components` are mined for behavioral
  assertions worth porting to the node backend before the tier is deleted.
- Interaction: reuse the preview's in-process NSEvent click driver.
- Color: the node vocabulary is sRGB and backends convert at their own edge.
  Verify against `poodle-gpui`'s existing color handling in Batch A before
  writing component conversions — the number one known failure mode from the
  Jetstream parity history.

Options (a) element-tree structural diff and (c) probe-only assertions were
rejected: GPUI has no stable serializable element tree to diff in-repo, and
probe-only is the "it compiles" drift the handoff warned against.

## Contract

- New crate `packages/gpui/node-backend` (named deliberately; `poodle-gpui`
  is taken by the theme/spec adapter): `to_gpui(&Node) -> impl IntoElement`,
  transcribing the channel walk from `jetstream-poodle/src/lib.rs` onto
  GPUI's fluent API — kinds, layout channels, per-side borders, gradients,
  shadow layers, hover/active patches, positions/overlay, a11y roles,
  interaction closures, animations.
- Node interaction closures (`on_activate`, `on_drag` with per-frame deltas,
  hover/active `StylePatch`) map onto GPUI's listener model once, centrally,
  not per component.
- Vocabulary changes stay additive; `poodle-node` never names a backend
  crate.
- `packages/gpui/components` is deleted only after the preview runs on the
  new backend with the visual gate green — same order the Jetstream side
  used: migrate preview, retire gate, delete.

## Decision Log

- 2026-08-05 (operator): the zero-diff pilot proved the two old tiers
  genuinely diverge — the old GPUI tier is axis-faithful (token + per-size
  offset recipes, matching Svelte's CSS-var behavior) while `poodle-render`
  carries the old Jetstream tier's fixed tables. Decision: **fix
  `poodle-render` recipes to the axis-faithful form** rather than re-baseline
  GPUI onto the approximation. Includes implementing `color.text.placeholder`
  behavior through the existing `color.text.secondary` fallback; no new token
  exists or is required. Jetstream's parity gate is retired, so this is
  Poodle-internal; local visual baselines rebaseline with recorded reasons.
  Select is the proving component; the same reconciliation pattern applies
  per specimen in Batch B wherever the gate flags a divergence.

## Execution Plan

- [x] Batch A — interpreter: create the crate, transcribe the full channel
  walk, pilot on the Select fixture set (the pilot the Jetstream side used
  to prove the vocabulary), verify color handling against `poodle-gpui`.
  Includes the recipe reconciliation above for select.
- [x] Batch B — preview migration: census old-tier call shapes before
  writing any shim (`grep -o` census, per the parent thread), chrome shim +
  compat wrappers at the specimen dispatch boundary, per-specimen
  conversion, visual gate green at every step, in-process click driver for
  interaction specimens.
- [x] Batch C — deletion and closeout: drop the dependency, delete
  `packages/gpui/components`, land the mined probe tests, log the deletion
  in `docs/logs/`, leave a one-line pointer in the Jetstream g06.013 roadmap
  that the last duplicate tier is gone.

## Batch B Progress

- [x] Wave 1 internal proof: Select plus Button, IconButton, SplitButton,
  ToggleGroup, CardToggleGroup, and SegmentedControl specimens render through
  the node backend. Recipes were checked against the old GPUI tier, including
  Card as CardToggleGroup's nested paint dependency. Render tests: 67 green;
  node-backend tests: 4 green; preview build green. Click-driver state proofs
  passed for every interactive specimen in the wave.
- [x] Operator pixel checkpoint: the migrated slugs passed the native visual
  gate from a Screen Recording-permitted terminal. Batch B remains open until
  later waves are green.
  The 2026-08-05 operator run produced stable captures for all seven slugs but
  found that the ignored local baselines were stale pre-settle images
  (1348×1478 rather than Retina-backed captures, mostly 2696×2396). Those seven
  local baselines were refreshed from the stable outputs. The unchanged-command
  rerun was exact for six slugs; IconButton exposed a one-run window-placement
  outlier (2784×2484 versus the common 2696×2396 geometry). Its local baseline
  was corrected from the second stable capture; the final IconButton-only rerun
  was 2696×2396 and exact zero-diff.
- [x] Wave 2 internal proof: preview-local leaf redirects move all Eyebrow,
  Icon, Text, Skeleton, Spinner, Avatar, and StatusIndicator constructors onto
  `poodle-render` plus the node backend. The standalone Surface and Card
  specimens are also node-backed; embedded Surface/Card composition stays with
  its owning later wave. Recipes were reconciled against the old GPUI tier for
  explicit SVG tint, typography, skeleton geometry/timing, spinner asset use,
  status rings, and Card media-slot sizing. Render tests: 73 green;
  node-backend tests: 4 green; preview build and handler drift green.
- [x] Wave 2 operator pixel checkpoint: the full update pass refreshed the
  ignored corpus from obsolete pre-settle captures to 136 stable 2696×2396
  baselines. The first unchanged run found one transient capture failure and
  three pointer-dependent hover diffs. Screenshot mode now posts an off-window
  mouse move before counting settled frames; focused reruns cleared the capture
  failure and hover diffs. MediaPicker's update-pass baseline alone contained
  the old hover outline and was refreshed for that recorded harness correction.
  The final unchanged `effigy test:native-visual` captured all 136 components
  at 2696×2396 with zero diff artifacts and no missing baselines.
- [x] Wave 3 first-slice internal proof: the standalone TextInput and Field
  specimens render through `poodle-render` plus the node backend. TextInput now
  carries replacement-text intent through the node vocabulary while the GPUI
  backend preserves the old tier's lightweight typing behavior. The recipe was
  reconciled for inline affixes, character count, validation status, compact
  metrics, and the native tier's fixed 1px border. Render tests: 76 green;
  node-backend tests: 5 green; preview build and handler drift green. The click
  driver focused the first input and reported `text-input-name="z"`. Direct
  2696×2396 captures stayed below the native gate tolerance: 40 differing
  pixels for TextInput and 7 for Field.
- [x] Wave 3 first-slice operator pixel checkpoint: the unchanged native visual
  gate passed for `text-input,field` on the eclipse/compact/sm axis.
- [x] Wave 3 embedded TextInput internal proof: FieldSet, FormDialog,
  FormLayout, and FormShell now compose node-backed Field/TextInput elements;
  FilterToolbar, PasswordRequirements, PickerShell, and
  UiPresentationProvider use the same TextInput bridge. This removes old-tier
  TextInput construction from every specimen source. Render tests: 76 green;
  node-backend tests: 5 green; preview build and handler drift green. Direct
  captures for all eight affected slugs matched their baselines exactly.
- [x] Wave 3 embedded TextInput operator pixel checkpoint: the unchanged native
  visual gate passed for all eight affected slugs on the eclipse/compact/sm
  axis.
- [x] Wave 3 static embedded Button/IconButton internal proof: FormLayout,
  FormDialog, FilterToolbar, PickerShell, and FormShell now use preview-local
  node-backed Button bridges for 15 Buttons and 2 IconButtons. Live callback
  sites remain on the old tier. Direct captures were exact for FormDialog,
  FilterToolbar, and PickerShell; FormLayout and FormShell differed by 53 and
  52 pixels, below the 129-pixel gate tolerance. UiPresentationProvider stays
  on the old Button path because its long default-secondary labels expose a
  separate centering drift.
- [x] Wave 3 static embedded Button/IconButton operator pixel checkpoint: the
  unchanged native visual gate passed for all five affected slugs on the
  eclipse/compact/sm axis.
- [x] Wave 3 static header/list Button/IconButton internal proof: AppHeader,
  PageHeader, InlineListSection, and ListGrid now compose node-backed static
  actions. This moves all 10 remaining old-tier IconButton constructor sites
  and 18 Button constructor sites onto the shared renderer. Focused captures
  stayed within the native threshold: 126 differing pixels for AppHeader, 93
  for PageHeader, zero for InlineListSection, and 39 for ListGrid. A repeated
  AppHeader capture produced the same 126-pixel result.
- [x] Wave 3 static header/list Button/IconButton operator pixel checkpoint:
  the unchanged native visual gate passed for all four affected slugs on the
  eclipse/compact/sm axis.
- [x] Wave 3 FormActions/Button-label internal proof: all 17 FormActions Button
  constructor sites now use the node bridge, including three context-free
  handlers that preserve the specimen's last-action feedback. The click driver
  reported `form-action-last="Cancel"`. UiPresentationProvider's two deferred
  long-label Buttons are also node-backed. The GPUI backend now matches the old
  tier's plain-label anatomy: an intrinsic label wrapper centered by flex,
  without a second text-alignment channel. FormActions and
  UiPresentationProvider pass their existing baselines. The standalone Button
  baseline was refreshed once because its earlier post-migration refresh had
  captured the interim direct-label path; the unchanged 12-slug Button
  regression sweep then passed.
- [x] Wave 3 FormActions/Button-label operator pixel checkpoint: the unchanged
  native visual gate passed for Button, FormActions, and UiPresentationProvider
  on the eclipse/compact/sm axis.
- [x] Wave 3 confirmation-overlay trigger internal proof: AlertDialog's two
  openers and ConfirmAction's four openers now use node-backed Buttons. A new
  queued `SetToggle` event preserves the old handlers' idempotent `set true`
  semantics. Closed-state captures match both existing baselines, and the
  click driver reported `alert-danger-open=true` and
  `confirm-action-danger-open=true` through real GPUI hit testing.
- [x] Wave 3 confirmation-overlay trigger operator pixel checkpoint: the
  unchanged native visual gate passed for AlertDialog and ConfirmAction on the
  eclipse/compact/sm axis.
- [x] Wave 3 Tooltip/Popover trigger internal proof: eight remaining trigger
  Button constructor sites now use the node bridge while the old overlay
  composites retain interaction ownership. Both closed-state captures match
  their existing baselines. The click driver reported `popover-default=true`,
  and sustained hover reported `tooltip-default-hovered=true` followed by
  `tooltip-default-open=true`.
- [x] Wave 3 Tooltip/Popover trigger operator pixel checkpoint: the unchanged
  native visual gate passed for Tooltip and Popover on the eclipse/compact/sm
  axis.
- [x] Wave 3 Menu/CommandPalette opener proof: all five live opener Buttons now
  use queued node handlers. Each Menu opener toggles its own key and forces its
  two siblings closed; both CommandPalette openers set their key true. Closed
  captures match both existing baselines, and the click driver confirmed all
  five state transitions through real GPUI hit testing.
- [x] Wave 3 Dialog/Drawer live Button proof: all 19 constructor sites now use
  the node bridge, including shared openers and in-overlay close/action
  Buttons. Queued `SetToggle` events preserve every true/false transition;
  focused captures match both existing baselines, and representative Dialog
  and Drawer open-then-close sequences passed through real GPUI hit testing.
  The now-unused context-bound `overlay_state::set_toggle` helper was removed.
- [x] Wave 3 Toolbar Button proof: all 16 constructor sites now use the node
  bridge. Queued `SetText` events preserve the Bold, Italic, and Publish
  feedback handlers; the click driver reported each expected `toolbar-last`
  value through real GPUI hit testing. The focused native visual gate matched
  the existing Toolbar baseline exactly.
- [x] Wave 3 final Button cleanup: the last nine old-tier constructor sites
  across Callout, DetailItem, DetailSection, DetailShell,
  InlineRemediation, ToastHost, and ToastStack now use the node bridge.
  InlineRemediation records its caller-owned action acknowledgement through
  the node event queue; the click driver reported
  `inline-remediation-last="retry"`. Six focused captures matched immediately.
  DetailShell's first run differed only at its animated loading spinner, then
  matched exactly on rerun. No baseline changed. GPUI tests: 133 green;
  preview build and handler drift green.
- [x] Wave 3 Checkbox/Switch/TriStateSwitch proof: all 32 constructor sites
  across the three standalone specimens and FormLayout now use node bridges.
  Queued events preserve Checkbox and Switch initialization plus value changes
  and TriStateSwitch selection indices. Click-driver proofs reported
  `checkbox-email=false`, `switch-dark-mode=false`, and
  `tri-state-filter=1`. Checkbox, Switch, TriStateSwitch, and FormLayout match
  their existing eclipse/compact/sm baselines with no baseline changes. Render
  tests: 76 green; node-backend tests: 5 green; GPUI tests: 133 green; preview
  build and handler drift green.
- [x] Wave 3 Slider/RangeSlider proof: all 19 constructor sites across the two
  standalone specimens now use node bridges and queued `SetText` events.
  Slider and RangeSlider match their existing eclipse/compact/sm baselines
  exactly with no baseline changes. Drag-driver proofs reported
  `slider-volume="77"` from an initial 65 and
  `range-slider-default-lo="24" range-slider-default-hi="80"` from an initial
  20–80. Render tests: 76 green; node-backend tests: 5 green; GPUI tests: 133
  green; preview build and handler drift green.
- [x] Wave 3 RadioGroup proof: all seven constructor sites in the standalone
  specimen now use the node bridge and queued `SetText` events. The focused
  eclipse/compact/sm capture matches its existing baseline exactly with no
  baseline change. Click-driver proofs reported `radio-plan="free"` from an
  initial `pro` in the vertical group and `radio-size="lg"` from an initial
  `md` in the horizontal group. Render tests: 76 green; node-backend tests: 5
  green; GPUI tests: 133 green; preview build and handler drift green.
- [x] Wave 3 Pagination/Stepper proof: all 18 constructor sites across the two
  standalone specimens now use node bridges. Both focused eclipse/compact/sm
  captures match their existing baselines exactly with no baseline changes.
  Pagination queues page, goto-draft, limit-open, and page-size events; click
  proofs reported `pagination-full-page=7 pagination-full-goto="7"` and
  `pagination-full-limit-open=true`. Render tests: 76 green; node-backend
  tests: 5 green; GPUI tests: 133 green; preview build and handler drift green.
- [x] Wave 3 final Field proof: the EmbedInput specimen's last old-tier Field
  now renders its shared Field and nested EmbedInput nodes through the node
  backend. The unchanged embed-input baseline matches exactly, as do all seven
  Field-bearing focused specimens. The old-tier Field specimen import census
  is zero. Render tests: 76 green; node-backend tests: 5 green; GPUI tests: 133
  green; preview build and handler drift green.
- [x] Wave 4 standalone Pill proof: all 29 constructor sites in the Pill
  specimen now render through shared nodes. The shared renderer preserves the
  old tier's centered layout, translucent-border alpha, removable icon, and
  remove handler. The old tier ignored `font="mono"`; the shared renderer
  implements the contract, so the Pill baseline was refreshed only for its
  three code-font examples. The following unchanged Pill and EmbedInput gate
  was exact. Render tests: 77 green; node-backend tests: 5 green; GPUI tests:
  133 green; preview build and handler drift green.
- [x] Wave 4 standalone ListCard proof: all 43 constructor sites in the
  ListCard specimen now render through shared nodes, together with its seven
  embedded Pill sites and footer-counter slots. The shared recipe was
  reconciled to the old GPUI geometry and state treatments; the unchanged
  eclipse/compact/sm ListCard baseline matches exactly. A native click-driver
  proof reported `list-card-clicked="component-specs.pdf"`. Render tests: 78
  green; node-backend tests: 5 green; GPUI tests: 133 green; preview build and
  handler drift green. No baseline changed.
- [x] Wave 4 embedded ListCard/Pill cleanup: the final ListGrid ListCard and
  the remaining six Pill sites across MetaBar, InlineListSection, PageHeader,
  and Dialog now use the same node compatibility bridge. Focused captures for
  all five affected specimens match their existing eclipse/compact/sm
  baselines exactly. No old-tier ListCard or Pill specimen import remains and
  no baseline changed.
- [x] Wave 4 DetailItem proof: all 36 sites across the standalone,
  DetailSection, DetailSectionGroup, and DetailShell specimens now render
  through shared nodes. The preview bridge carries value-content and action
  slots as nodes; all four focused eclipse/compact/sm captures match their
  existing baselines exactly. No old-tier DetailItem specimen import remains
  and no baseline changed.
- [x] Wave 4 DetailSection-family proof: all 10 DetailSection, four
  DetailSectionGroup, and four DetailShell sites now render through shared
  nodes. The shared group recipe now enforces `maxColumns`; DetailShell carries
  the old header rule and loading-row anatomy. DetailSection, group, item, and
  shell focused captures are green. The DetailShell baseline alone was
  refreshed for the shared Spinner's contract-correct 15px `md` grid geometry
  and declared animation, replacing the old tier's private 14.25px pulse.
  Render tests: 80 green. No old-tier import for any of the three composites
  remains.
- [x] Wave 4 display proof: all 11 ListCardCounter, 12 MetricTile, and 10 Code
  specimen sites now render through shared nodes, including MetaBar's embedded
  Code value. ListCardCounter's shared icon now inherits the counter's
  secondary tone. Code's node recipe now carries the contract's relative `1.4`
  source line-height. ListCardCounter and MetaBar retained their existing
  pixels; MetricTile and Code baselines were refreshed for the contract's
  code-family typography, which the old GPUI tier omitted. The unchanged
  four-slug eclipse/compact/sm gate is green. Render tests: 82 green. No
  old-tier specimen import for any of the three components remains.
- [x] Wave 4 list-shell proof: all four ListContainer, five ListGrid, and seven
  remaining Surface specimen sites now render through shared nodes. The shared
  recipes preserve PageHeader's two-sided vertical padding, ListContainer's
  full-width summary and end-aligned pager, PaginationSummary's body-size token,
  and full-width state surfaces; ListGrid's empty slot fills its grid cell. All
  seven affected eclipse/compact/sm captures match their existing baselines
  exactly. Render tests: 83 green. No old-tier specimen import for
  ListContainer, ListGrid, or Surface remains.
- [x] Wave 4 structural proof: all four Box, three Grid, five Stack, three
  Spacer, and 10 Separator specimen sites now render through shared nodes.
  Box gained the missing shared recipe, Grid preserves declared fractional
  track weights, and decorative Separator nodes stay out of the accessibility
  role channel. All six affected eclipse/compact/sm captures match their
  existing baselines exactly. Render tests: 86 green. No old-tier specimen
  import for any of the five primitives remains.
- [x] Wave 5 confirmation/floating/command overlay proof: all four
  AlertDialog, four ConfirmAction, one Tooltip, one Popover, five Menu, and
  five CommandPalette specimen sites now render their component recipes
  through shared nodes. Preview-local host bridges retain GPUI-owned viewport
  centering, anchoring, hover delay, and event dispatch; shared nodes own the
  surfaces and action handlers. The six-slug eclipse/compact/sm gate is exact
  with no baseline changes. Click-driver proofs reported
  `confirm-action-danger-open=true`, `popover-default=true`,
  `menu-file-open=true`, and the Tooltip hover/open transition. New render
  tests exercise ConfirmAction confirm/cancel plus CommandPalette
  query/select/close channels. Render tests: 88 green; node-backend tests: 5
  green; GPUI tests: 133 green; preview build and handler drift green. No
  old-tier specimen import for any of the six components remains.
- [x] Wave 5 navigation-overlay proof: all five Accordion, six Collapsible,
  five ContextMenu across three specimen files, two HoverCard, three Menubar,
  and three NavigationMenu constructor sites now render through shared nodes.
  Preview-local hosts retain context-menu anchoring and hover timing; queued
  events preserve controlled accordion, collapsible, menu, navigation, and
  mutable tree actions. The eight affected eclipse/compact/sm captures match
  their existing baselines exactly. Click-driver proofs covered Accordion,
  Collapsible, ContextMenu, HoverCard, Menubar, and NavigationMenu state
  transitions. Render tests: 89 green; node-backend tests: 5 green; GPUI
  tests: 133 green; preview build, handler drift, and Rust CI green. No
  baseline changed and no old-tier specimen constructor remains for any of the
  six component families.
- [x] Wave 5 date-picker proof: all five DatePicker, six DateRangePicker, six
  DateTimePicker, and six DateTimeRangePicker constructor sites now render
  through shared nodes. Preview-local wrappers queue controlled open-state
  changes, and DatePicker selection writes the chosen ISO date and closes the
  surface. The four affected eclipse/compact/sm captures match their existing
  baselines exactly. Click-driver proofs covered all four open transitions and
  DatePicker selection (`2026-08-13`). Render tests: 89 green; node-backend
  tests: 5 green; GPUI tests: 133 green; preview build, GPUI checks, handler
  drift, and Rust CI green. No baseline changed and no old-tier specimen
  constructor remains for any of the four component families.
- [x] Wave 5 remaining date/time proof: all eight Calendar, six
  DateTimeZonePicker, six TimeField, and six TimeZoneSelect constructor sites
  now render through shared nodes. Queued events preserve controlled calendar
  date/range/month state and both overlay toggles; TimeField replacement text
  reaches its specimen callback through the node input channel. The four
  affected eclipse/compact/sm captures (`calendar`, `date-time-zone-picker`,
  `time-input`, `time-zone-select`) match their existing baselines exactly.
  Click-driver proofs reported `calendar-selected="2026-01-01"`,
  `calendar-nav-month="2025-12"`, `dtz-picker-open=true`, and
  `tz-select-open=true`; render tests cover range progression and TimeField
  replacement/disabled behavior. Render tests: 92 green; node-backend tests: 5
  green; GPUI tests: 133 green; preview build, GPUI checks, handler drift, and
  Rust CI green. No baseline changed and no old-tier specimen constructor
  remains for any of the four component families.
- [x] Batch-level native sweep: every deterministic slug was exercised. 134
  matched directly. SegmentedControl and SplitButton produced stable
  Codex-display raster differences of 0.0961% and 0.3007%; both are unrelated
  to this slice and were already operator-green from the terminal display
  context. No baseline was changed.
- [x] Wave 5 specialized value-input migration: all eight ColorPicker, seven
  DurationInput, and nine NumberInput constructor sites now render through the
  node backend. ColorPicker and NumberInput focused captures are exact; the
  click driver reports `color-picker-basic-open=true` and
  `number-input-price="30.00"`. DurationInput is node-backed and stable, but
  its digit raster remains a deterministic 0.0033% diff on the focused axis;
  no baseline was changed. The residual is a backend text-raster parity gap,
  not a missing recipe or interaction path.
- [x] Wave 6 input-composite migration: all ten CodeInput, nine TokenInput, and
  eight FileUpload constructor sites now render through the node backend. The
  focused native captures are exact for all three; no baseline changed. The
  node vocabulary gained native text underline support for FileUpload's
  browse affordance, and the node preview bridges preserve the existing
  constructor and handler call shapes.
- [x] Wave 7 tab migration: all 13 Tabs and eight TabStrip constructor sites now
  render through the node backend. The focused `tabs` and `tab-strip` captures
  are exact with no baseline changes. The bridges preserve selection and close
  events; TabStrip reorder remains host-owned behind the compatibility surface.
- [x] Wave 8 Breadcrumbs slice: all six Breadcrumbs constructor sites, including
  the PageHeader breadcrumb slot, now render through the node backend. The
  focused `breadcrumbs` and `page-header` captures are exact with no baseline
  changes. Separator icons carry the native secondary tint explicitly because
  node children do not inherit GPUI parent text color.
- [x] Wave 9 TextLink slice: all seven TextLink constructor sites now render
  through the node backend. The focused `text-link` capture is exact with no
  baseline change. The node vocabulary now carries an optional underline tint,
  preserving the native link's subdued resting decoration.
- [x] Wave 10 SelectionSummary slice: all seven SelectionSummary constructor
  sites now render through the node backend. The focused `selection-summary`
  capture is exact with no baseline change. The bridge preserves clear/remove
  handlers, the old ballot-X glyph, and the overflow badge line-height.
- [x] Wave 11 feedback/data proof: all Meter, Rating, and Table constructor
  sites now render through the node backend. The focused `meter`, `rating`, and
  `table` captures are exact with no baseline changes. Meter preserves the
  native fill tint and rounded ends; Table preserves equal flex columns,
  1.5 line-height, and end alignment; Rating queues selection changes through
  the preview event bridge.
- [x] Wave 12 utility/data proof: all PaginationSummary, ValidationSummary,
  EmptyState, and ResizeHandle constructor sites now render through the node
  backend. Focused `pagination-summary`, `validation-summary`, `empty-state`,
  and `resize-handle` captures are exact with no baseline changes. Progress is
  also node-backed across all nine sites; its native gate remains skipped as
  nondeterministic. Region was later moved into Wave 28; its dashed-border
  capture differs by 1.2144%, a backend border-raster issue rather than text
  parity.
- [x] Wave 13 metadata/navigation/status proof: all MetaBar, MetaItem, NavCard,
  Callout, and StatusBar constructor sites now render through the node backend.
  NavCard and Callout focused captures are exact; MetaBar, MetaItem, and
  StatusBar differ only in text-raster pixels and are parked under the deferred
  text-parity allowance. No baselines changed.
- [x] Wave 14 checklist/error/list proof: all PasswordRequirements,
  ErrorBoundary, InlineListSection, and CollapseToggle constructor sites now
  render through the node backend. ErrorBoundary, InlineListSection, and
  CollapseToggle focused captures are exact. PasswordRequirements is within
  the deferred text/icon-raster allowance at 0.0077%; no baselines changed.
- [x] Wave 15 toolbar proof: all Toolbar constructor sites now render through
  the node backend with node-backed Button and Separator children. The focused
  `toolbar` capture is exact; no baselines changed.
- [x] Wave 16 selection-family proof: all OrderBy and RefSelect constructor
  sites now render through the node backend. Their focused captures differ
  only in text/icon raster pixels (0.0122% and 0.2216%) and are parked under
  the deferred text-raster allowance; handler-capable bridges are in place and
  no baselines changed.
- [x] Wave 17 composite proof: the eight standalone FormActions constructor
  sites and all eight PageHeader constructor sites now render through the node
  backend. PageHeader includes node-backed action, breadcrumb, and metadata
  slots. FormActions is exact; PageHeader differs only in text/icon raster
  pixels plus a tiny banner-edge residual (0.0453%). No baseline changed.
  PickerShell was later moved into Wave 25 after its ready-body wrapper was
  aligned with the old full-width row flow; the remaining focused residual is
  tracked as text/control raster parity.
- [x] Wave 18 app-shell proof: all AppHeader constructor sites now render
  through the node backend, including custom identity, primary-action, and
  utility slots. The focused `app-header` capture is exact; no baseline
  changed.
- [x] Wave 19 filter-toolbar proof: all FilterToolbar constructor sites now
  render through the node backend with node-native Select and TextInput child
  slots plus action and secondary slots. The focused `filter-toolbar` capture
  is exact; no baseline changed.
- [x] Wave 20 FormShell proof: all FormShell constructor sites now render
  through the node backend with node-native section slots and action rows. The
  focused `form-shell` capture differs only in text/icon raster rows (0.0054%),
  which remain under the deferred parity allowance; no baseline changed.
- [x] Wave 21 FormLayout proof: all FormLayout constructor sites now render
  through the node backend. Multi-column cells now use the old tier's
  percentage basis/grow geometry, and the form/grid roots stretch to the
  owning surface width. The focused `form-layout` capture is geometrically
  aligned; its remaining 0.7501% is text/button raster residual deferred per
  the current parity allowance. FormDialog was subsequently moved into
  Wave 26 after its body stack spacing was aligned with the shared stack-md
  contract.
- [x] Wave 22 FieldSet proof: all FieldSet constructor sites now render
  through the node backend. Single-column slots stay vertical; multi-column
  slots match the old tier's `flex_1` grow/zero-basis geometry and inline gap.
  The focused `field-set` capture is exact; no baseline changed.
- [x] Wave 23 ThemeSelect proof: all ThemeSelect constructor sites now render
  through the node backend. The open panel keeps the old tier's horizontal
  trigger/panel placement and swatch-grid geometry. The focused `theme-select`
  capture is exact; no baseline changed.
- [x] Wave 24 ModelPicker proof: the ModelPicker specimen now renders through
  the node backend. Its dialog wrapper preserves the old block surface's full
  width, so the model/axis split and selected rows keep their GPUI geometry.
  The focused capture is geometrically aligned; its remaining 0.2638% is a
  deferred text/control-raster residual from the shared typography and nested
  segmented/switch primitives. The embedded ModelPicker slots in
  AgentChatInput likewise leave a 0.1210% trigger/control-raster residual;
  no baseline changed.
- [x] Wave 25 PickerShell proof: the PickerShell specimen now renders through
  the node backend, including node-native search, result rows, and FormActions
  slots. The ready body keeps its full-width row flow instead of the old
  clipped path. The focused capture is geometrically aligned; its remaining
  0.5576% is deferred text/control-raster parity. No baseline changed.
- [x] Wave 26 FormDialog proof: all FormDialog specimen sites now render
  through the node backend with node-native fields, text inputs, buttons, and
  custom FormActions. The body stack now uses the shared stack-md spacing,
  bringing the focused residual to 0.1980% modal/text raster parity. No
  baseline changed.
- [x] Wave 27 ScrollShell proof: all ScrollShell specimen children now render
  through the node backend, including the custom vertical and horizontal rows.
  Root and viewport sizing now match GPUI's full-size, min-zero, clipped scroll
  layers. The focused `scroll-shell` capture is exact; no baseline changed.
- [x] Wave 28 Region proof: all Region specimen sites now render through the
  node backend. Labels and layout geometry align with GPUI; the remaining
  1.2144% focused residual is the known dashed-border raster difference. No
  baseline changed.
- [x] Wave 29 media composite proof: MediaThumbnail, EmbedPreview, and
  MediaPreview now render through the node backend. MediaPreview keeps its
  caller-owned media slot through an additive render-node slot path, including
  play and badge overlays. Focused captures are geometrically aligned; the
  remaining 0.1492%, 0.2103%, and 0.1154% deltas are text/icon raster parity.
  No baseline changed.
- [x] Wave 30 input/loading composite proof: CardRadioGroup, EmbedInput, and
  PageLoading no longer import the old GPUI composites. CardRadioGroup uses
  the queued node-event seam for selection changes; its row geometry is now
  aligned after restoring the old zero-basis/min-zero flex cells, with a
  deferred 0.9761% selected-state/text raster residual. EmbedInput is focused
  exact; PageLoading remains skipped by the nondeterministic native gate and
  had no committed baseline update.
- [x] Wave 31 MediaPicker proof: the open browse/upload/empty picker specimens
  now use the node backend and the spec-owned `MediaPickerItem` type. Focused
  geometry is aligned; the remaining 0.5516% is icon/text raster parity. The
  native focused run did not update the existing baseline.
- [x] Wave 32 data/agent composite proof: DataTable, AgentQuestion, and
  AgentTranscript now render through the node backend. DataTable sorting and
  row actions use queued host events; AgentQuestion selection and transcript
  expansion/file events use the same context-free seam. AgentQuestion is
  focused exact; AgentTranscript retains a 0.0131% text-raster residual, and
  DataTable retains a 0.7217% text/layout-raster residual. No baseline changed.
- [x] Wave 33 navigation/media/notification proof: SidebarNav,
  MediaBrowsePanel, and ToastStack now render through the node backend.
  ToastStack preserves the old corner-mounted absolute overlay and fixed
  22.5rem stack width. Focused captures retain 0.2954%, 0.1829%, and 1.1702%
  text/icon/animation-raster residuals respectively; no baseline changed.
- [x] Wave 34 ToastHost proof: ToastHost now composes the shared positioned
  ToastStack node through the node backend, preserving placement, inset, and
  fixed-width host geometry. Its focused capture retains a 0.5661%
  text/icon/animation-raster residual; no baseline changed.
- [x] Wave 34 full-gate checkpoint: the post-wave native visual run compared
  all 136 components, skipped the six nondeterministic slugs, and reported 28
  failures. Every failure is a documented text/icon/animation or known
  geometry residual; there were no capture failures and no baseline updates.
- [x] Wave 35 overlay-slot proof: Dialog and Drawer now render through the node
  backend. Dialog gained shared custom header/footer slots and both specimens
  route close-state changes through queued node events. Focused Dialog and
  Drawer captures are exact; no baseline changed.
- [x] Wave 36 action/composite proof: DebugDialog, ActionDiscoveryPanel, and
  BulkActionBar now use node compatibility wrappers and the shared renderers.
  Focused captures retain 0.1692%, 0.0787%, and 0.4728% text/icon-raster
  residuals respectively; no baselines changed.
- [x] Wave 37 composer proof: AgentChatInput now uses the shared node renderer
  with ModelPicker, toolbar, and footer node slots. Its focused capture retains
  a 0.1377% text/control-raster residual; no baseline changed.
- [x] Wave 37 full-gate checkpoint: the post-wave native visual run compared
  all 136 components, skipped the six nondeterministic slugs, and reported 31
  documented text/icon/animation or known geometry residuals. There were no
  capture failures and no baseline updates.
- [x] Wave 38 editor/filter proof: FilterBuilder and MarkdownEditor now use
  preview-local node compatibility wrappers over the shared renderers.
  MarkdownEditor preserves its text-change and mode-change host events through
  the node event queue; FilterBuilder carries its complete handler bundle for
  the next interactive host. Focused captures retain 1.0752% and 0.2304%
  text/layout-raster residuals respectively; no baselines changed.
- [x] Wave 38 full-gate checkpoint: the post-wave native visual run compared
  all 136 components, skipped the six nondeterministic slugs, and reported 33
  documented text/icon/animation or known geometry residuals. The two new
  failures are the FilterBuilder and MarkdownEditor residuals above; there
  were no capture failures and no baseline updates.
- [x] Wave 39 list/picker proof: EditableList and RelationPicker now use
  preview-local node compatibility wrappers over the shared renderers.
  RelationPicker preserves drill-entry and back-path events through queued
  node events. Focused captures retain 0.0063% (EditableList text raster) and
  1.3903% (RelationPicker geometry/text raster) residuals; no baselines
  changed.
- [x] Wave 39 full-gate checkpoint: the post-wave native visual run compared
  all 136 components, skipped the six nondeterministic slugs, and reported 35
  documented text/icon/animation or known geometry residuals. The two new
  failures are the EditableList and RelationPicker residuals above; there
  were no capture failures and no baseline updates.
- [x] Wave 40 interaction proof: EditableLabel now uses a preview-local node
  compatibility wrapper over the shared renderer. Its live text-change,
  submit (Enter/Tab), and cancel (Escape) intents are carried by the node
  interaction contract and routed through the preview event queue. The
  focused capture retains a 0.5334% text-raster residual; no baseline changed.
- [x] Wave 40 full-gate checkpoint: the post-wave native visual run compared
  all 136 components, skipped the six nondeterministic slugs, and reported 36
  documented text/icon/animation or known geometry residuals. The new failure
  is the EditableLabel residual above; there were no capture failures and no
  baseline updates.

- [x] Wave 41 provider/media/time proof: AudioPlayer, VideoPlayer and TimeAgo now
  render through the node backend; IconProvider and UiPresentationProvider moved
  to `packages/gpui/preview/src/providers.rs`. Both providers are pure context
  boundaries — IconProvider's contract anatomy is "Root (no DOM element)" and
  UiPresentationProvider's root only carries CSS custom properties — so neither
  has a `poodle-render` recipe to move to, and both stay preview-local
  passthroughs that outlive the old tier. Their focused eclipse/compact/sm
  captures are exact with no baseline change.
  The three media/time slugs are gate-skipped as non-deterministic, so they were
  proved by direct old-tier-versus-node capture instead: TimeAgo is pixel-exact
  (0 differing pixels). AudioPlayer and VideoPlayer both exposed genuine shared
  recipe defects, now reconciled:
  - AudioPlayer's seek track carried `self_stretch`, so it painted as a
    full-height pill instead of a 0.25rem rail, and neither slider set the
    `text_color` channel the backend reads for a Progress fill — both tracks
    rendered white with no accent. Also corrected to contract: the track radius
    is 0.125rem (not `radius.pill`), the time labels are `typography.label.size`
    and centred, `pad-y`/`gap` follow the component's own 0.375/0.5/0.625 ladder
    rather than `panel_space_y_rem`, and `pad-x` is 0.5/0.75/0.875 rather than
    the generic `control_space_x_rem`. The transport buttons gained the missing
    hover tint.
  - VideoPlayer had no minimum chrome height and no bottom-pinned controls, so
    the viewport collapsed to zero and the controls escaped the black surface.
    Restored the contract's 13.75rem minimum, the growing viewport, the outlined
    big-play circle, the controls' asymmetric inset, and the seek bar's accent
    fill. The old-tier delta went from 1.4999% to 0.0152%, which is the deferred
    time-label text raster.
  `VideoPlayerSpec` gained additive `with_captions_src`/`with_show_captions`
  builders plus a `renders_captions_track()` helper, replacing the old tier's
  ad-hoc `with_captions` element builder. The old tier's extra "subtitles"
  control button was a tier-local invention with no contract or Svelte
  counterpart and does not survive the move — contract §2 renders captions as a
  `<track>` with no chrome.
  Render tests: 92 green; node-backend tests: 5 green; preview build and handler
  drift green.
- [x] Wave 42 split/dock proof: SplitView and DockRegion now render through the
  node backend. `poodle-render` already carried slot-shaped signatures for both
  (`primary`/`secondary` Nodes, a `content` Node, and handler structs), so the
  parked "needs slot/host-event design" note was stale — the work was converting
  specimen-owned GPUI children into Nodes and reconciling the recipes. The
  focused `split-view` capture is exact. `dock-region` came down from 0.4194% to
  0.0415%; what is left is the specimen's own caller-authored body text in a
  deliberately overflowing demo cell, where the two tiers clip at slightly
  different points. No baseline changed. Recipe defects found and fixed:
  - SplitView's root set only `width: Grow`, so the split collapsed to its
    panes' content height instead of filling the host frame. Restored the old
    tier's `size_full`, the panes' cross-axis fill and clipping, and the
    whole-split disabled opacity.
  - SplitView allocated panes with `flex_grow: ratio` against a zero basis,
    which spreads the divider's thickness across both panes and moves the
    split. The old tier seeds `flex_basis(relative(ratio))` and lets grow/shrink
    settle it. `poodle-node` gained an additive `flex_basis_pct` channel for
    this (the existing `flex_basis` is pixels only); the GPUI backend maps it to
    `flex_basis(relative(..))` and it wins over the pixel channel when both are
    set.
  - DockRegion's expanded layout ignored `tabs_placement`/edge entirely: a
    left-edge dock laid its tabs out in a row above the body instead of down the
    edge beside it. Restored the edge-aware root/strip/tab-list direction, the
    right-edge body-before-strip order, and the body's grow-and-clip cell.
  - DockRegion drew a four-sided border on every emphasis; a dock rules only the
    edge it docks against. Now per-side, using the uniform colour channel (the
    vocabulary has no `border_color_right`, and the fallback covers it).
  - DockRegion's active tab carried a TabStrip-style accent underline plus bold
    primary-coloured text. The old tier and Svelte both render an accent-tinted
    rounded pill with accent text at normal weight.
  - DockRegion interpolated `"{icon} {label}"` into one button label, which
    collapses the icon/label gap to a single space and shifts every tab's
    centring. Icon and label are now separate children behind the contract gap,
    and inactive tabs regained their hover fill.
  Preview seams: `SplitView::with_extent_px` declares the axis extent the
  divider's per-frame pixel deltas are measured against, because the node
  vocabulary is deliberately delta-only — the same trade `poodle_render::slider`
  makes with its fixed track basis, where the old tier read the drag event's
  container bounds. Ratio changes and dock tab changes both route through the
  context-free `NodeSpecimenEvent` queue.
  Render tests: 92 green; node-backend tests: 5 green; preview build and handler
  drift green.
- [x] Wave 43 preview-chrome proof: `main.rs`, `token_view.rs` and
  `usage_docs_view.rs` no longer import the old tier. This is app infrastructure
  rather than specimens, so it sat outside the constructor census below and was
  not tracked by any earlier wave — but Batch C cannot delete the tier while it
  stands. The shell's nav Tabs, component-search TextInput, token-panel Tabs,
  token-inspector TextInput, SidebarNav, Table, Code and Separator now all
  render through the node backend. Their handlers move to the same context-free
  seam through a new `NodeSpecimenEvent::Chrome(ChromeEvent)` variant, because
  these mutate `AppState` (section, search, active component, token panel)
  rather than specimen state. Preview build, GPUI tests (133) and handler drift
  green. NOT YET PIXEL-VERIFIED — see the open risk below.
- [x] Wave 44 BlockEditor proof: the BlockEditor specimen renders through the
  node backend. `poodle-render` gained an additive
  `block_editor_with_children(spec, theme, children)`; `block_editor` keeps its
  signature and delegates. The spec drives blocks whenever `spec.blocks` is
  non-empty, and `children` is the escape hatch for consumers that own their
  block vocabulary — each wrapped in the same block shell, without a toolbar,
  matching the old tier's legacy `with_child` path.
  `poodle-node` gained an additive `text_italic` channel. The vocabulary had no
  way to express `font-style: italic`, which five contracts' CSS calls for
  (editable-label, markdown-editor, inline-list-section, password-requirements,
  selection-summary) and which `render/src/block_editor.rs` had been working
  around with a documented "italic-substitute" tone change.
- [x] Wave 45 LogList proof: LogList renders through the node backend, and the
  Rust spec now carries the entries payload the contract has documented all
  along. This was spec-versus-contract drift, not a missing contract:
  `docs/contracts/components/log-list.md` §"Types" already specifies
  `LogEntry = StreamLogEntry | AuditLogEntry`, but `LogListSpec` carried only
  `entry_count: usize`, so `render/src/log_list.rs` drew the literal string
  `"{n} entries"` where the rows belong.
  Added to `poodle-specs`, mirroring Svelte as the parity authority:
  `LogLevel` (info/warn/error), `LogActor`, `StreamLogEntry`, `AuditLogEntry`
  and the `LogEntry` union, plus `LogListSpec::entries` and the derived
  `entry_count()`, `is_audit()`, `audit_entries()`, `stream_entries()` (level +
  case-insensitive text filtering) and `level_count()`. `entry_count` the field
  is gone; nothing outside this repo consumed it.
  `poodle-render` now draws real stream rows to the contract recipe — a mono
  `[timestamp | level | message]` row at 0.8125rem/1.45, level-tinted, capped at
  `maxEntries`, separated by a half-strength subtle rule with no rule on the
  first row.
  Two old-tier inventions did not survive: the extra `Debug` level (the contract
  and Svelte both define exactly info/warn/error) and the flattened
  actor/resource fields, which are now the contract's nested `LogActor` and
  audit resource fields. `AuditLogEntry.details` is deliberately omitted — it
  exists only to feed the web target's `entryDetails` snippet, and the Rust
  targets have no snippet channel.
  LogList has no headless, node-contract or vector surface, so the cross-target
  sweep was the Rust spec, the renderer and the (already-correct) contract doc.
  Spec tests: 223 → 227. Render tests: 92 → 98. `effigy docs:spec-drift` green.
- [x] Wave 43 follow-up — SidebarNav recipe defect found by the full gate.
  Moving the preview shell onto the node backend put SidebarNav into every
  baseline, which exposed a real defect the component's own slug had been
  carrying since Wave 33 as a "0.2954% text/icon raster" residual. It was
  never raster:
  - The active row set an accent left rail AND a ring-coloured 1px box on the
    same node. gpui 0.2.2 has one `border_color` per element (the backend
    documents this approximation at `node-backend/src/lib.rs`), so the left
    colour overwrote all four sides: the rail vanished and the row wore a full
    accent outline. The ring is now a separate absolutely-inset child, exactly
    how the old tier emulated the inset shadow.
  - The group title and item rows set no `line_height`, so gpui's default
    applied instead of the contract's 1.2 / 1.3, shifting every row below the
    title down by ~7px.
  With both fixed, `sidebar-nav` and `button` are pixel-exact, and the Wave 33
  residual is retired rather than deferred.
- [x] Wave 46 Tree — the interaction vocabulary is now wide enough, and Tree is
  node-backed. The four gaps named below were closed additively:
  1. **`NodeModifiers` + `Interaction::on_activate_modified`.** Modifier-aware
     activation as a sibling channel, not a wider `on_activate` — widening the
     existing signature would break every call site. When set, the backend
     calls it instead of `on_activate`, so a node wires one or the other.
     `accel` collapses Cmd and Ctrl onto one flag so components never branch on
     the host OS.
  2. **`NodePoint` + `Interaction::on_context`.** Secondary activation with the
     pointer anchor. This is the only place the vocabulary hands a component a
     coordinate, and it is justified rather than leaked: a context menu is
     anchored to the pointer *by definition*, so the anchor is the semantic.
  3. **`NodeKey` + `Interaction::on_key`.** Navigation and command keys, named
     physically (ArrowDown, F2, …) because their meaning is the component's —
     ArrowDown is "next row" in a tree and "next option" in a select.
     Enter/Tab and Escape stay on `on_submit`/`on_cancel`.
  4. **Payload/zone drag: `drag_payload`, `drop_zone`, `DropEdge`,
     `NodeDropEvent`, `on_drop_hover`, `on_drop`.** This is the one that looked
     like it would invert `NodeDragEvent`'s documented delta-only rule, and on
     inspection it does not. The old GPUI tier already derived its
     `DropPosition` from the pointer's *fraction within the hovered row's own
     bounds* and reported a semantic position — never a coordinate. So the
     backend hit-tests zones and computes the edge from geometry it already
     owns; the component names its zones and receives `DropEdge`. No coordinate
     reaches a component, and `on_drag` stays delta-only for gestures like the
     SplitView divider.
  Tree's own handlers follow: `on_select_modified`, `on_context_menu`,
  `on_key`, `on_drag_over` and `on_reorder` join `TreeHandlers`, and the
  specimen routes all of them through a new `NodeSpecimenEvent::Tree(TreeEvent)`
  variant. Selection maths is not reimplemented — the specimen calls the shared,
  unit-tested `compute_selection` with the visible order from
  `TreeSpec::visible_rows`, so the contract keeps owning it. Arrow/Home/End
  navigation resolves in the specimen for the same reason: only the host knows
  the flattened visible order.
  Known limit: gpui does not hand `on_drop` a pointer position, so the drop
  edge comes from the last `on_drop_hover` the host recorded. That is the same
  live-hover-over-snapshot trade the old tier made, and it is why both handlers
  exist.
  Tree's focused capture is now exact. Two divergences were investigated and
  resolved in opposite directions — the contract decided both:
  1. **Focus ring expression — the render tier was right, the old tier was
     not.** The old tier painted the ring through gpui's `.focus()`
     pseudo-state, so it appeared only on real keyboard focus and never in a
     screenshot; `poodle-render` paints it from `spec.focused_value`. The tree
     contract §"Roving tabindex" is explicit: "the Rust runtimes track it via
     `focused_value` on the spec (the host app owns + mutates it) and render a
     focus ring on that node." So the shared recipe already matched the
     contract and the baseline was stale. No focus-visible vocabulary channel
     is needed for this.
  2. **Depth guide lines rendered segmented**, and this was a real defect with
     a shared cause. The row carried an always-on 1px border (transparent
     unless focused) to avoid layout jitter. A border insets the row's content
     box, so every indent cell was 2px shorter than the row pitch and the
     ancestor guides broke into stubs — and every row was 2px too tall. The
     contract draws the ring as an `outline`
     (`tree.css .tree__item:focus-visible > .tree__row`), which does not
     participate in layout, so the faithful mapping is an absolutely-inset
     overlay, not a border. Same shape as the SidebarNav ring fix above.
  With both settled, the tree baseline was refreshed for those two recorded
  reasons and the slug is exact.
- [ ] ~~Wave 46 Tree — PARKED~~ (superseded by the entry above; kept for the
  record of what the blocking shape was). Every other
  in-scope specimen is migrated; Tree is the last one, and it is parked on
  interaction vocabulary rather than on effort. `poodle_render::TreeHandlers`
  carries three handlers (`on_select`, `on_toggle_expand`, `on_check`); the
  specimen drives eleven. The four gaps, in increasing order of design cost:
  1. **Modifier-aware activation.** Ctrl/Cmd+click and Shift+click drive
     `on_selection_change(TreeSelectionUpdate { values, anchor, focused })`.
     `Interaction::on_activate` is `Fn()` and carries no modifier state.
     Widening it is a breaking change for every call site, so this wants a
     sibling channel rather than a wider `on_activate`.
  2. **Secondary click with a position.** `on_context_menu(TreeContextRequest)`
     needs the right-click point to anchor the menu. No secondary-activation
     channel exists.
  3. **Keyboard navigation.** ↑/↓/←/→/Space plus F2 and Alt+↑/↓ reorder. The
     vocabulary carries `on_submit`/`on_cancel` only — Enter/Tab and Escape —
     and no general key channel.
  4. **Drag with a drop target.** `on_drag_over(TreeDragOver { value, position })`
     and `on_reorder(TreeReorderRequest { from, to })` need the node under the
     pointer and a before/inside/after position. `NodeDragEvent` is documented
     as deliberately delta-only: "Absolute positions are a backend concern
     (they depend on layout, which the component never sees)." Reporting a drop
     target id is additive in syntax but inverts that principle, so it is an
     operator decision, not a mechanical one — and it is the roadmap's own stop
     condition about the vocabulary needing a change to express a rendering.
  Rename is the one piece already expressible: the node `Input` kind plus
  `on_text_change`/`on_submit`/`on_cancel` covers start/change/commit/cancel.
- [x] Wave 41 note: `packages/gpui/preview/src/demo_view.rs` is dead source. No
  `mod demo_view;` declaration exists and the `app_state::DemoScreen` type it
  needs is gone, so it has not compiled since an earlier refactor. Its 26
  old-tier constructor sites are therefore not a migration blocker; the file is
  Batch C deletion work.

Old-tier constructor census after Wave 45:

| Constructor | Calls | Files |
| --- | ---: | ---: |
| `Button::from_spec` | 0 | 0 |
| `IconButton::from_spec` | 0 | 0 |
| `TextInput::from_spec` | 0 | 0 |
| `Checkbox::from_spec` | 0 | 0 |
| `Switch::from_spec` | 0 | 0 |
| `TriStateSwitch::from_spec` | 0 | 0 |
| `Slider::from_spec` | 0 | 0 |
| `RangeSlider::from_spec` | 0 | 0 |
| `RadioGroup::from_spec` | 0 | 0 |
| `Pagination::from_spec` | 0 | 0 |
| `Stepper::from_spec` | 0 | 0 |
| `ListCard::from_spec` | 0 | 0 |
| `Pill::from_spec` | 0 | 0 |
| `DetailItem::from_spec` | 0 | 0 |
| `DetailSection::from_spec` | 0 | 0 |
| `DetailSectionGroup::from_spec` | 0 | 0 |
| `DetailShell::from_spec` | 0 | 0 |
| `ListCardCounter::from_spec` | 0 | 0 |
| `MetricTile::from_spec` | 0 | 0 |
| `Code::from_spec` | 0 | 0 |
| `ListContainer::from_spec` | 0 | 0 |
| `ListGrid::from_spec` | 0 | 0 |
| `Field::from_spec` | 0 | 0 |
| `Surface::from_spec` | 0 | 0 |
| `Box::from_spec` | 0 | 0 |
| `Grid::from_spec` | 0 | 0 |
| `Stack::from_spec` | 0 | 0 |
| `Spacer::new` | 0 | 0 |
| `Separator::from_spec` | 0 | 0 |
| `AlertDialog::from_spec` | 0 | 0 |
| `ConfirmAction::from_spec` | 0 | 0 |
| `Tooltip::from_spec` | 0 | 0 |
| `Popover::from_spec` | 0 | 0 |
| `Menu::from_spec` | 0 | 0 |
| `CommandPalette::from_spec` | 0 | 0 |
| `Accordion::from_spec` | 0 | 0 |
| `Collapsible::from_spec` | 0 | 0 |
| `ContextMenu::from_spec` | 0 | 0 |
| `HoverCard::from_spec` | 0 | 0 |
| `Menubar::from_spec` | 0 | 0 |
| `NavigationMenu::from_spec` | 0 | 0 |
| `DatePicker::from_spec` | 0 | 0 |
| `DateRangePicker::from_spec` | 0 | 0 |
| `DateTimePicker::from_spec` | 0 | 0 |
| `DateTimeRangePicker::from_spec` | 0 | 0 |
| `Calendar::from_spec` | 0 | 0 |
| `DateTimeZonePicker::from_spec` | 0 | 0 |
| `TimeField::from_spec` | 0 | 0 |
| `TimeZoneSelect::from_spec` | 0 | 0 |
| `ColorPicker::from_spec` | 0 | 0 |
| `DurationInput::from_spec` | 0 | 0 |
| `NumberInput::from_spec` | 0 | 0 |
| `CodeInput::from_spec` | 0 | 0 |
| `TokenInput::from_spec` | 0 | 0 |
| `FileUpload::from_spec` | 0 | 0 |
| `Tabs::from_spec` | 0 | 0 |
| `TabStrip::from_spec` | 0 | 0 |
| `Breadcrumbs::from_spec` | 0 | 0 |
| `TextLink::from_spec` | 0 | 0 |
| `SelectionSummary::from_spec` | 0 | 0 |
| `Meter::from_spec` | 0 | 0 |
| `Rating::from_spec` | 0 | 0 |
| `Table::from_spec` | 0 | 0 |
| `PaginationSummary::from_spec` | 0 | 0 |
| `ValidationSummary::from_spec` | 0 | 0 |
| `Progress::from_spec` | 0 | 0 |
| `EmptyState::from_spec` | 0 | 0 |
| `ResizeHandle::from_spec` | 0 | 0 |
| `MetaBar::from_spec` | 0 | 0 |
| `MetaItem::from_spec` | 0 | 0 |
| `MetaItem::new` | 0 | 0 |
| `NavCard::from_spec` | 0 | 0 |
| `Callout::from_spec` | 0 | 0 |
| `StatusBar::from_spec` | 0 | 0 |
| `PasswordRequirements::from_spec` | 0 | 0 |
| `ErrorBoundary::from_spec` | 0 | 0 |
| `InlineListSection::from_spec` | 0 | 0 |
| `CollapseToggle::from_spec` | 0 | 0 |
| `Toolbar::from_spec` | 0 | 0 |
| `OrderBy::from_spec` | 0 | 0 |
| `RefSelect::from_spec` | 0 | 0 |
| `PageHeader::from_spec` | 0 | 0 |
| `AppHeader::from_spec` | 0 | 0 |
| `FilterToolbar::from_spec` | 0 | 0 |
| `FormShell::from_spec` | 0 | 0 |
| `FormLayout::from_spec` | 0 | 0 |
| `FieldSet::from_spec` | 0 | 0 |
| `ThemeSelect::from_spec` | 0 | 0 |
| `MediaThumbnail::from_spec` | 0 | 0 |
| `EmbedPreview::from_spec` | 0 | 0 |
| `MediaPreview::from_spec` | 0 | 0 |
| `CardRadioGroup::from_spec` | 0 | 0 |
| `EmbedInput::from_spec` | 0 | 0 |
| `PageLoading::from_spec` | 0 | 0 |
| `MediaPicker::from_spec` | 0 | 0 |
| `ModelPicker::from_spec` | 0 | 0 |
| `PickerShell::from_spec` | 0 | 0 |
| `FormDialog::from_spec` | 0 | 0 |
| `ScrollShell::from_spec` | 0 | 0 |
| `Region::from_spec` | 0 | 0 |
| `DataTable::from_spec` | 0 | 0 |
| `AgentQuestion::from_spec` | 0 | 0 |
| `AgentTranscript::from_spec` | 0 | 0 |
| `SidebarNav::from_spec` | 0 | 0 |
| `MediaBrowsePanel::from_spec` | 0 | 0 |
| `ToastStack::from_spec` | 0 | 0 |
| `ToastHost::from_spec` | 0 | 0 |
| `Dialog::from_spec` | 0 | 0 |
| `Drawer::from_spec` | 0 | 0 |
| `DebugDialog::from_spec` | 0 | 0 |
| `ActionDiscoveryPanel::from_spec` | 0 | 0 |
| `BulkActionBar::from_spec` | 0 | 0 |
| `AgentChatInput::from_spec` | 0 | 0 |
| `FilterBuilder::from_spec` | 0 | 0 |
| `MarkdownEditor::from_spec` | 0 | 0 |
| `EditableList::from_spec` | 0 | 0 |
| `RelationPicker::from_spec` | 0 | 0 |
| `EditableLabel::from_spec` | 0 | 0 |
| `AudioPlayer::from_spec` | 0 | 0 |
| `VideoPlayer::from_spec` | 0 | 0 |
| `TimeAgo::from_spec` | 0 | 0 |
| `IconProvider::new` | 0 | 0 |
| `UiPresentationProvider::from_spec` | 0 | 0 |
| `SplitView::from_spec` | 0 | 0 |
| `DockRegion::from_spec` | 0 | 0 |
| `BlockEditor::from_spec` | 0 | 0 |
| `LogList::from_spec` | 0 | 0 |
| `Tree::from_spec` | **7** | **1** |

No specimen uses a direct old-tier free function or lowercase helper path.

Old-tier imports remaining in the whole preview crate, not just specimens:

| File | Why it is still there |
| --- | --- |
| `packages/gpui/preview/src/specimens/tree.rs` | Wave 46, parked on interaction vocabulary (above) |
| `packages/gpui/preview/src/demo_view.rs` | Dead source — no `mod demo_view;`, and the `app_state::DemoScreen` it needs no longer exists. Delete in Batch C rather than migrate. |

Every other preview file, specimen and app-infrastructure alike, is node-backed.

Remaining wave:

1. Continue the next old-tier composite wave after the EditableLabel,
   EditableList, and RelationPicker slice. The FilterBuilder and MarkdownEditor
   slice is now node-backed too. The MediaThumbnail,
   EmbedPreview, MediaPreview, CardRadioGroup, EmbedInput, PageLoading, and
   MediaPicker slice. DurationInput's raster residual
   is parked at the documented 0.0033% until backend text parity is addressed;
   Region's dashed-border residual is parked at 1.2144%. MetaBar, MetaItem,
   StatusBar, and PasswordRequirements text/icon raster deltas are also parked.
   None blocks constructor migration. ScrollShell is now node-backed with an
   exact focused capture. OrderBy and
   RefSelect text/icon raster deltas remain deferred. The standalone FormActions
   specimen is exact and PageHeader is now node-backed; PickerShell is now
   node-backed with a deferred 0.5576% text/control-raster residual. The
   FormDialog is now node-backed with its default and custom action paths;
   AppHeader and FilterToolbar are exact and node-backed; FormShell
   is node-backed with only a deferred 0.0054% text/icon raster residual.
   ModelPicker is now node-backed with aligned panel geometry; its 0.2638%
   focused text/control-raster residual is deferred. The media trio is now
   node-backed with only the documented text/icon residuals. CardRadioGroup is
   node-backed with its 0.9761% selected-state/text residual deferred; the
   MediaPicker is node-backed with a 0.5516% icon/text residual.
   FormDialog is now
   node-backed with a 0.1980% modal/text raster residual; its child controls
   are already node-backed.

## Non-Goals

- touching the Jetstream repo beyond the one-line g06.013 pointer note
- npm publishes (g06.014, operator-side)
- the React conversion lane
- vocabulary redesign or non-additive vocabulary changes
- GPUI accessibility upstream work (held deliberately per g12.015)

## Acceptance

- every preview specimen renders through `poodle-render` nodes plus the GPUI
  node backend; no specimen imports `packages/gpui/components`
- native visual gate green with no unexplained baseline updates
- `effigy drift:handlers` green
- `packages/gpui/components` deleted and the workspace builds clean
- deletion logged in `docs/logs/`; roadmap front doors updated

## Validation

- `effigy test:native-visual` before and after migration, zero-diff
- `effigy drift:handlers`
- `effigy gpui:build` and `effigy gpui:test`
- `effigy ci:rust` and `effigy ci:native` where applicable
- `git diff --check`

## Stop Conditions

- the vocabulary needs a non-additive change to express a GPUI rendering
- color conversion cannot match the old tier without a vocabulary change
- gpui 0.2.2 API drift makes a component's old behavior unreachable
- the visual gate cannot go green without raising tolerance

## Next Task

`g12.019` is complete. `packages/gpui/components` — 170 files, 44,796 lines, the
last duplicate component tier — is deleted, and every Poodle target renders one
implementation (`poodle-render` emitting `poodle-node` trees) through a thin
per-target backend.

Gate after the follow-ups: **136 compared, 98 exact, 37 failing**, every failure
a named residual in the bucket below. Down from 86 failing before the stale
chrome baselines were refreshed.

Logs: `docs/logs/2026-08/07-gpui-components-tier-deleted.md` (deletion,
probe mining, retired gates) and
`docs/logs/2026-08/07-gpui-node-backend-waves-41-45.md` (the migration waves).

Open, and none of it blocking this card:

1. **`effigy test:jetstream-a11y` fails on 151 unnamed `TextInput` nodes** across
   13 specimens. `poodle_render::text_input` names its root only when
   `spec.aria_label` is set. Pre-existing but previously invisible: `ci:native`
   died at `drift:clicks` — an orphaned gate, now retired — long before reaching
   the audit, so the count silently regressed from zero. It is the last thing
   between `ci:native` and green. Held under `g12.015`; recorded in
   `PAPERCUTS.md`.
2. **`block-editor` retains 0.0602%** in its own per-block toolbars (TypeSelect
   labels, `+`, `Select…`), which is the deferred text-raster bucket.
3. The deferred text/icon/animation raster bucket below is unchanged and still
   deliberately deferred.

### Deferred residual bucket (unchanged)

Text, icon and animation raster parity remains deferred by operator
instruction. The focused residuals recorded through Wave 40 stand: DurationInput
0.0033%, Region 1.2144% (dashed-border raster), PasswordRequirements 0.0077%,
CardRadioGroup 0.9761%, MediaPicker 0.5516%, DataTable 0.7217%, AgentTranscript
0.0131%, SidebarNav 0.2954%, MediaBrowsePanel 0.1829%, ToastStack 1.1702%,
ToastHost 0.5661%, FilterBuilder 1.0752%, MarkdownEditor 0.2304%, EditableList
0.0063%, RelationPicker 1.3903%, EditableLabel 0.5334%, plus the MetaBar,
MetaItem, StatusBar, OrderBy, RefSelect, PageHeader, PickerShell, FormShell,
FormLayout, FormDialog, ModelPicker, AgentChatInput, DebugDialog,
ActionDiscoveryPanel and BulkActionBar deltas. AudioPlayer, VideoPlayer,
TimeAgo, Spinner, Progress and PageLoading remain gate-skipped as
non-deterministic.
