# desengs.com Reuse Candidates — Pass 2

Status: open — research lead only; not execution authority
Captured: 2026-09-01
Owner: Muse / desengs scout
Source: [desengs.com](https://desengs.com) remaining Build / Use entries
See also: `20260901-192850-desengs-reuse-candidates.md` (pass 1; do not
re-argue)
Promotion owner: Poodle Northstar orchestrator

Second scout pass over the leftover Build and closely related Use listings,
plus craft essays that map to a concrete Poodle gap. This note is not
execution authority.

## Scope And Skips

Inspected 2026-09-01. First-party `/inspiration`, `/minimum`, `/dsgnrs` stay
galleries. Jobs, newsletters, people-to-follow, books, and career/taste
essays skipped.

Skip as already judged in pass 1 or already queued:

- HITL / AskUserQuestions — Fluid Functionalism
  [AskUserQuestions](https://www.fluidfunctionalism.com/docs/ask-user-questions)
  is the same family as pass 1 candidate 1 (`AgentQuestion` / `AgentPlan`)
- citations, agent task list, NumberFlow, Cuelume
- icon morph, shimmer, Transitions.dev, block slider, interior.dev
- Dot Matrix / Thinking orbs, liquid-glass, Liquid Gooey, crd-ui, Magic UI,
  Hashvatar
- Interface Cheat Sheet (held)
- soundcn (sample library); WebHaptics (later native/mobile)

Skip this pass after live inspection:

- [⌘K](https://github.com/dip/cmdk) — `CommandPalette` +
  `ActionDiscoveryPanel` already cover modal command discovery
- [Remediate](https://www.remediate.ski/) — in-app screenshot/video/voice
  capture. Not `MessageCenter` or `InlineRemediation` / `RemediationBanner`
  (those are notification and form-recovery). Product feedback tool, not a
  Poodle primitive
- [Agentation](https://www.agentation.com/) — element-pick annotation overlay
  for coding agents. Not AgentQuestion/plan. Devtool, not a published
  composite
- [@web-kits/audio](https://audio.raphaelsalaja.com/) — synthesis engine for
  pass 1 candidate 5, not a second cue family
- [Calligraph](https://calligraph.raphaelsalaja.com/) — number/text slot
  morph; number path is pass 1 candidate 4. Text path is Transitions.dev
  text-swap / thinking-states
- [Torph](https://torph.lochie.me/) — string morph. Same text-swap lane, not
  shimmer/typewriter, but already ranked in the Transitions.dev catalogue
- [Fancy Components](https://www.fancycomponents.dev/) and
  [devl.dev](https://devl.dev/) — decorative experiment dumps (scramble-in,
  shaders). No missing generalized family
- [dither-kit](https://www.tripwire.sh/dither-kit) — full chart kit plus
  dithered buttons/avatars. `MetricTile` already owns a static sparkline.
  Bar/area/pie/radar plus dither bloom is a style kit, not a Poodle admission
- [DialKit](https://joshpuckett.me/dialkit) — floating live-tune panel.
  `DebugDialog` is a JSON dump; `Knob` / `Fader` / `Slider` / `ColorPicker`
  already exist. A public Dial clone is a design-tool. Specimen/lab hosts can
  compose the existing controls. Spring/easing editors would widen
  architecture 012's closed duration/easing set
- Fluid Functionalism proximity hover / font-weight travel / springs —
  pointer-only preview; keyboard and GPUI cannot share it. Same rejection
  class as Transitions.dev avatar-group hover. Sidebar peek/resize is already
  `DockRegion` + `SplitView` + `ResizeHandle`
- [Gradient Border Plugin](https://gradient-border.floriankiem.com/),
  [shadowLab](https://shadowlab.mocarski.design/),
  [Easing Graphs](https://www.easing.dev/) — authoring playgrounds. Elevation
  and compact motion tokens already exist; 012 does not take arbitrary
  cubic-bezier or stacked-shadow catalogues
- [UI Playbook](https://uiplaybook.dev/) — ten common components Poodle
  already owns
- [Component Gallery](https://component.gallery/) — carousel is the one
  missing family (`Card` already scopes carousels out). Marketing/media
  slider, not a workstation primitive. `MediaBrowsePanel` chose grid +
  load-more
- Rauno [Invisible Details](https://rauno.me/craft/interaction-design) —
  iOS gesture physics essay. No unused Poodle primitive
- Other Rauno [Web Interface Guidelines](https://interfaces.rauno.me/)
  items — already covered by Poodle a11y/tokens or the cheat-sheet hold
  (press-scale, tabular-nums, hover media, copy-to-checkmark on `Code`)

## Candidates

### 1. Live interpolating series

What: a real-time line that lerps incoming `{ time, value }` points, with
scrub, pause, loading-to-data morph, and a tip badge. Not a static sparkline
and not an audio waveform.

URL: [Liveline](https://github.com/benjitaylor/liveline)

Poodle already has:

- `MetricTile` — optional static SVG sparkline; out of scope for full charts
- `WaveformDisplay` — audio peak-pyramid inspector; timeline-scale waveform
  explicitly out of scope
- `Meter` / `AudioMeter` / `GainReductionMeter` / `Progress` — instant
  values, not a time window
- architecture 012 — canvas/path drawing needs a named role and a static
  fallback

Lacks: a live series primitive with stable identity, interpolating updates,
and reduced/frozen snapshots.

Why reusable: component. Workstations and agent dashboards need a live
windowed series. That job is generalized. Candles, order books, and "degen"
particles are product chrome.

Later try: one research card that asks whether `MetricTile`'s sparkline
gains a live mode or a separate `LiveSeries` display primitive. Inputs:
host-owned points, current value, window length, optional scrub. Full:
bounded lerp inside a named canvas/path exception. Reduced/frozen: last
committed polyline, no pulse. Reject candles, orderbook, particle bursts,
and trading toggle chrome. Do not vendor Liveline.

### 2. In-place toast lifecycle

What: one toast identity that starts pending and resolves to success or
error in place, instead of stacking a second toast.

URL: [Sonner](https://sonner.emilkowal.ski/) (promise toast)

Poodle already has:

- `ToastStack` — presentational stack; tones, dismiss, optional action;
  auto-dismiss timers out of scope here
- `ToastHost` — placement, auto-dismiss, sticky tones, store mapping.
  `ToastHostStoreItem` has id, title, message, tone, action, sticky. No
  pending / progress / resolve field
- `MessageCenter` — durable messages plus live activity rows with
  `Progress`; not a transient toast

Lacks: a toast that stays sticky while pending, then becomes a tone and
re-enters the auto-dismiss clock, without a second announcement of a new
item.

Why reusable: pattern. Same ToastHost store, additive item state. Not a
new widget. Distinct from pass 1 sound cues.

Later try: a later component/motion card for optional `lifecycle`
(`pending | settled`) on one toast id. Pending is sticky and may show
`Spinner` / `Progress`. Resolve updates tone and copy on the same id;
live region does not reannounce a new toast. Reduced/frozen: skip enter
travel, keep the copy change. Do not add swipe, expand-all, richColors, or
a `toast.promise` helper. Do not copy the Sonner API.

### 3. Nested-menu pointer intent

What: a path-based safe region so a pointer traveling diagonally into a
submenu does not close it by crossing siblings.

URL: [Web Interface Guidelines](https://interfaces.rauno.me/) — nested
menus / "prediction cone"

Poodle already has:

- `Menu` / `ContextMenu` / `Menubar` — cascading `children` flyouts.
  `Menu` opens on hover and closes when the pointer hits any sibling. No
  safe-path or intent cone
- Keyboard already works (`Arrow Right` / `Left`)
- GPUI notes allow native window menus for menubar submenus

Lacks: pointer-intent geometry for web nested menus. Distinct from the
cheat-sheet hold (press-scale, icon cross-fade).

Why reusable: pattern. One submenu hover rule across Menu, ContextMenu,
and Menubar. Not a new component.

Later try: a later research card for a web-only pointer-intent rule on
submenu parents: keep the flyout open while the pointer stays inside a
triangle/path from the parent row to the flyout. Keyboard unchanged.
Reduced/frozen: no visual change; this is hit-testing, not motion.
Native may keep OS menus. Measure accidental close vs sticky-wrong-flyout
before promoting. Do not add magnetic/proximity hover from Fluid
Functionalism.

## Promotion Route

1. Orchestrator intake only. No contract, token, package, or roadmap edit
   follows from this note.
2. If a family is wanted, open one research or planning card per family.
   Do not bundle live series, toast lifecycle, and menu intent with pass 1.
3. Keep each card behind architecture 012 and existing toast/menu
   ownership. Jetstream stays deferred.
4. Remove this note when every family is rejected or promoted into its
   owning research dossier or card.

No implementation, restyle, vendor import, or Next Task move is authorized.
