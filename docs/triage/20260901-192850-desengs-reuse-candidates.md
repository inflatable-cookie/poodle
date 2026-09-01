# desengs.com Reuse Candidates

Status: open — research lead only; not execution authority
Captured: 2026-09-01
Owner: Muse / desengs scout
Source: [desengs.com](https://desengs.com) (92-item homepage index plus
`/inspiration`, `/minimum`, `/dsgnrs`)
Promotion owner: Poodle Northstar orchestrator

One inventory of the live DesEngs index. The orchestrator splits families into
later research or planning cards. This note does not implement, restyle, or
authorize a card.

## Scope

Full index assessed 2026-09-01. Homepage is a curated outbound list
(Beautiful UI through Design Engineering at Vercel). First-party
`/inspiration`, `/minimum`, and `/dsgnrs` are galleries and a designer follow
list — not primitives.

Genuinely useful here means a component, transition, style, or pattern Poodle
does not already have, or has in a weaker/wrong placement, that would serve
the generalized library (Svelte / React / GPUI, agent + workstation + audio,
architecture 012). Landing kits, jobs, newsletters, people-to-follow, career
essays, decorative chrome, and items already queued elsewhere are skips.

## Candidates

### Agent chrome

#### 1. Transcript-inline HITL approval

What: pause-the-agent surfaces — option questions, command green-light, short
plan accept, and a recommendation card with confidence plus alternatives.

URLs:

- [Beautiful UI](https://beautifului.dev) — Approval Card, Recommendation Card
- [AICSS Approval Card](https://www.aicss.dev/components/approval-card) —
  variants `questions` | `command` | `plan`
- Fluid Functionalism
  [AskUserQuestions](https://www.fluidfunctionalism.com/docs/ask-user-questions)
  is the same family (composer-style intake, not a second candidate)

Poodle already has:

- `AgentQuestion` / `AgentPlan` — live HITL, but composer-bound inside
  `AgentChatInput`; transcript gets settled records only
- `ConfirmAction` — destructive dialog, not an agent turn gate
- `AgentTranscript` item kinds — message, tool-run, changed-files, answered
  question, decided plan; no live approval block

Lacks: a transcript-inline approval that can gate a command or short plan
without a dialog and without moving the decision into the composer.
Beautiful UI's recommendation card (confidence + alternatives + accept) has
no Poodle sibling.

Why reusable: component / pattern. Same semantic job as the existing
question and plan contracts, different placement and payload.

Later try: one research card — new transcript item kind vs additive
`AgentQuestion` / `AgentPlan` placement. Keep no-scrim, no-focus-trap,
turn-vs-action blocking. Reject a second text input beside the composer.

#### 2. Sourced answers and citation marks

What: streamed or settled assistant prose with inline source markers, a
compact source footer, and optional follow-up chips.

URLs:

- [Beautiful UI](https://beautifului.dev) — Streaming Text
- [AICSS Inline Citations](https://www.aicss.dev/r/inline-citations)
  (licensed; existence proof, not a copy source)

Poodle already has:

- `AgentMessage` — markdown subset, streaming caret, `TextLink`; avatars,
  timestamps, and message actions are out of scope
- `AgentTranscript` live region at the log, not per token
- `Code` — copy toolbar; not a citation

Lacks: citation identity, mark mapping, source list, follow-up prompt chips.
No citation contract under `docs/contracts/components/`.

Why reusable: component. Host-owned source records plus a small mark/list
composite.

Later try: one component card for `Citation` marks + a source list that
`AgentMessage` or a sibling can compose. Host supplies ids, titles, hrefs.
Skip typewriter word-reveal (Transitions.dev P2). Do not shimmer citation
text.

#### 3. Agent task list

What: a live checklist the agent maintains — pending / in-progress / done /
failed rows with counts. Not a proposed plan and not a tool-call run.

URLs:

- [Beautiful UI](https://beautifului.dev) — Task Rows
- [AICSS To-do List](https://www.aicss.dev/r/task-list)

Poodle already has:

- `ToolCall` / `ToolCallGroup` — work row or compressed run; no checklist
  semantics
- `AgentPlan` — accept / revise / dismiss
- `Progress` — bar, not a task tree
- `ChangedFiles` — turn file summary

Lacks: a first-class agent task-list composite with stable row identity and
status vocabulary.

Why reusable: component. Cursor-style agent todos are a generalized
conversation primitive.

Later try: extend-`ToolCallGroup` vs new `AgentTaskList`. Rows: id, label,
status, optional count/detail. Host owns the list. Do not steal `AgentPlan`
decision controls.

### Motion

#### 4. Numeric change motion

What: digits restyle in place when a formatted number changes (spin, trend
direction, prefix/suffix, grouped sync). Accessible; reduced-motion opt-out.

URL: [NumberFlow](https://number-flow.barvian.me/)
([Calligraph](https://calligraph.raphaelsalaja.com/) number slots are the
same family, not a second candidate)

Poodle already has:

- architecture 012 + `MotionPolicyProvider` — no numeric-change role
- `ValueReadout`, `MetricTile`, `ListCardCounter` — snap
- `NumberInput` / `DragNumberField` — editing, not display motion

Lacks: a restriction-only numeric-change treatment. Distinct from queued
block-slider, icon-morph, and shimmer.

Why reusable: transition / style. One optional role covers meters, readouts,
and counters.

Later try: optional `numeric-change` role on display-only numbers. Full:
bounded digit restyle inside the 012 property budget. Reduced/frozen: latest
formatted text immediately. Keep `ValueReadout` VisualState-only child and
single announcement. Do not animate while editing. Do not vendor NumberFlow.

#### 5. Live interpolating series

What: a real-time line that lerps incoming `{ time, value }` points, with
scrub, pause, loading-to-data morph, and a tip badge. Not a static sparkline
and not an audio waveform.

URL: [Liveline](https://github.com/benjitaylor/liveline)

Poodle already has:

- `MetricTile` — optional static SVG sparkline; full charts out of scope
- `WaveformDisplay` — audio peak-pyramid; timeline-scale waveform out of
  scope
- `Meter` / `AudioMeter` / `GainReductionMeter` / `Progress` — instant
  values
- architecture 012 — canvas/path drawing needs a named role and a static
  fallback

Lacks: a live series primitive with stable identity, interpolating updates,
and reduced/frozen snapshots.

Why reusable: component. Workstations and agent dashboards need a live
windowed series. Candles, order books, and particle bursts are product
chrome.

Later try: `MetricTile` sparkline live mode vs a `LiveSeries` display
primitive. Inputs: host-owned points, current value, window, optional scrub.
Full: bounded lerp inside a named canvas/path exception. Reduced/frozen:
last committed polyline, no pulse. Reject candles, orderbook, and trading
chrome. Do not vendor Liveline.

### Feedback

#### 6. Semantic interaction sound

What: a closed set of synthesized interaction cues (press, release, toggle,
success, error, loading, ready) bound by role, not by WAV files.

URL: [Cuelume](https://cuelume.dev)
([@web-kits/audio](https://audio.raphaelsalaja.com/) is the engine under
this job, not a second family)

Poodle already has: visual state, toast, and motion policy. No sound,
haptic, or cue-role contract.

Why reusable: pattern. Same shape as `MotionPolicy`: host-level,
restriction-only, semantic roles, never required for correctness.

Later try: optional `CuePolicy` (`full | muted | silent`) beside motion, not
inside it. Closed role list. Host owns permission, first-gesture unlock, and
volume. Do not depend on Cuelume. Do not add soundcn's sample library.
WebHaptics stays a later native/mobile question on the same card.

#### 7. In-place toast lifecycle

What: one toast identity that starts pending and resolves to success or
error in place, instead of stacking a second toast.

URL: [Sonner](https://sonner.emilkowal.ski/) (promise toast)

Poodle already has:

- `ToastStack` — tones, dismiss, optional action
- `ToastHost` — placement, auto-dismiss, sticky tones. Store item has no
  pending / progress / resolve field
- `MessageCenter` — durable messages plus live activity rows; not a
  transient toast

Lacks: a toast that stays sticky while pending, then becomes a tone and
re-enters the auto-dismiss clock, without announcing a new item.

Why reusable: pattern. Additive ToastHost item state, not a new widget.

Later try: optional `lifecycle` (`pending | settled`) on one toast id.
Pending is sticky and may show `Spinner` / `Progress`. Resolve updates tone
and copy on the same id. Do not add swipe, expand-all, richColors, or a
`toast.promise` helper.

### Menus

#### 8. Nested-menu pointer intent

What: a path-based safe region so a pointer traveling diagonally into a
submenu does not close it by crossing siblings.

URL: [Web Interface Guidelines](https://interfaces.rauno.me/) — nested
menus / "prediction cone"

Poodle already has:

- `Menu` / `ContextMenu` / `Menubar` — cascading `children` flyouts. Hover
  on any sibling closes the flyout. No safe-path or intent cone
- Keyboard already works (`Arrow Right` / `Left`)
- GPUI notes allow native window menus for menubar submenus

Lacks: pointer-intent geometry for web nested menus. Distinct from the
cheat-sheet hold (press-scale, icon cross-fade).

Why reusable: pattern. One submenu hover rule across three contracts.

Later try: web-only pointer-intent rule on submenu parents. Keyboard
unchanged. This is hit-testing, not motion. Native may keep OS menus.
Measure accidental close vs sticky-wrong-flyout. Do not add Fluid
Functionalism proximity hover.

## Assessed and skipped

Full homepage (92) plus first-party pages. Grouped by reason.

### Already in Poodle

- [⌘K](https://github.com/dip/cmdk) — `CommandPalette` +
  `ActionDiscoveryPanel`
- [UI Playbook](https://uiplaybook.dev/) — button, select, tooltip, etc.
- [Component Gallery](https://component.gallery/) — families Poodle owns;
  carousel is the one miss and is marketing/media (`Card` already scopes it
  out; `MediaBrowsePanel` chose grid)
- [OKLCH.fyi](https://oklch.fyi/) — tokens already emit `oklch`
- [You Don't Need Animations](https://emilkowal.ski/ui/you-dont-need-animations)
  — high-frequency no-motion is already architecture 012; `Tooltip` already
  has open delay
- [Details That Make Interfaces Feel Better](https://jakub.kr/writing/details-that-make-interfaces-feel-better)
  — tabular-nums, interruptible transitions, optical align, image outline;
  concentric radius already used in native focus-ring work
- Remaining [Web Interface Guidelines](https://interfaces.rauno.me/) items
  and [Interface Cheat Sheet](https://interfaces.dev/cheat-sheet) — semantic
  tokens, reduced-motion wrap, focus-visible, press-scale held as a later
  style check not a component
- Fluid Functionalism sidebar peek/resize — `DockRegion` + `SplitView` +
  `ResizeHandle`

### Already queued in other triage / research

- [Morphrig](https://morphrig.dev), [morphicons](https://www.morphicons.com/)
  — icon morph; `20260901-125758-post-motion-research-queue.md`
- AICSS Thinking State / thinking-label shimmer — same note
- [Transitions.dev](https://transitions.dev/) — audited in
  `../research/value-tracks/transitions-dev-catalogue.md`; policy is
  architecture 012
- [interior.dev](https://www.interior.dev/) — post-click timing overlaps 012
- [Torph](https://torph.lochie.me/) — string morph; Transitions.dev
  text-swap / thinking-states
- [Interactive SVG Animations](https://www.svg.guide/) — path-draw course;
  icon-morph lane
- Taste / agent-guidance kits —
  [Taste Skill](https://www.tasteskill.dev/),
  [UI Skills](https://www.ui-skills.com/),
  [Impeccable](https://impeccable.style/),
  [rams](https://www.rams.ai/),
  [jakubkrehel/skills](https://github.com/jakubkrehel/skills),
  [emilkowalski/skills](https://github.com/emilkowalski/skills) —
  overlap `20260901-121256-design-guidance-pilot-decision.md`; not a
  published primitive

### Duplicate of a candidate above

- Fluid Functionalism AskUserQuestions — candidate 1
- Calligraph number/text slots — candidate 4 / text-swap skip
- @web-kits/audio, [soundcn](https://soundcn.xyz/) — candidate 6 (engine vs
  sample library)
- [WebHaptics](https://haptics.lochie.me) — later native/mobile on
  candidate 6

### Decorative / landing / product chrome

- [Dot Matrix](https://dotmatrix.zzzzshawn.cloud),
  [Thinking orbs](https://orbs.jakubantalik.com/) — Spinner already has
  ring/grid/dots
- [liquid-glass](https://glass.samasante.com/),
  [Liquid Gooey](https://gooey.jakubantalik.com/),
  [Magic UI](https://magicui.design/),
  [Fancy Components](https://www.fancycomponents.dev/),
  [devl.dev](https://devl.dev/),
  [Design Spells](https://designspells.com) — shaders, scramble, easter eggs
- [crd-ui](https://crd-ui.juanda.co/) — payment card
- [Hashvatar](https://www.hashvatar.com/) — `Avatar` extras
- [dither-kit](https://www.tripwire.sh/dither-kit) — dithered chart kit;
  `MetricTile` already owns a static sparkline
- Fluid Functionalism proximity hover / font-weight travel / springs —
  pointer-only; same rejection class as Transitions.dev avatar-group hover
- [DialKit](https://joshpuckett.me/dialkit) — public live-tune panel.
  `DebugDialog` is JSON; `Knob` / `Fader` / `Slider` / `ColorPicker` exist.
  Specimen hosts can compose them. Spring editors would widen 012
- [Gradient Border Plugin](https://gradient-border.floriankiem.com/),
  [shadowLab](https://shadowlab.mocarski.design/),
  [Easing Graphs](https://www.easing.dev/) — authoring playgrounds;
  elevation and compact easings already exist
- [UI Camera](https://ui.camera) — product photography
- [Remediate](https://www.remediate.ski/) — screenshot/video/voice capture;
  not `MessageCenter` or `InlineRemediation` / `RemediationBanner`
- [Agentation](https://www.agentation.com/) — element-pick annotation
  overlay; coding-agent devtool
- [Design System Checklist](https://www.designsystemchecklist.com/) — meta
  DS planning, not a component
- Paid craft libraries/courses — [Interface Craft](https://www.interfacecraft.dev/),
  [Devouring Details](https://devouringdetails.com/),
  [animations.dev](https://animations.dev)

### Career / community / browse (no Poodle primitive)

- Jobs: Autumn, Zed, Resend, Rockstar Games, Design Engineer Jobs
- Follow / people: Jakub Krehel, floguo, Rauno Freiberg, Emil Kowalski,
  A Collection of Design Engineers, DSGNRS
- Career / taste essays: On AI systems taste and the human filter; The Rise
  of Design Engineering; Is AI going to steal my job; How to become an AI
  Designer; Front-of-the-front-end; The Concept of Taste; Using AI as a
  Design Engineer; The Attributes of a Design Engineer; Becoming a Design
  Engineer; Design Engineering (floguo notes and designengineer.xyz);
  Design Engineering: A State of Mind; Design Engineering 101; Design
  Engineering at Vercel; Developing Taste; 12 Principles of Animation;
  Family Values; Invisible Details of Interaction Design
- Browse / magazines / books / community: Design Books; Design Systems
  Surf; Design for Engineers; Interfaces; ui.land; Design Principles; Laws
  of UX; Design Engineer Tools; userinterface.wiki (page failed to load;
  directory, not a primitive); abtest.design; Design Engineering Club;
  newsletter (curationsystems.substack.com)
- First-party galleries: `/inspiration`, `/minimum`, `/dsgnrs`

## Promotion Route

1. Orchestrator intake only. No contract, token, package, or roadmap edit
   follows from this note.
2. Split into one research or planning card per family if wanted. Do not
   bundle the eight.
3. Keep each card behind architecture 012 and existing agent / toast / menu
   ownership. Jetstream stays deferred.
4. Remove this note when every family is rejected or promoted into its
   owning research dossier or card.

No implementation, restyle, vendor import, or Next Task move is authorized.
