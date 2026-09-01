# desengs.com Reuse Candidates

Status: open — research lead only; not execution authority
Captured: 2026-09-01
Owner: Muse / desengs scout
Source: [desengs.com](https://desengs.com) (homepage resource list plus
`/inspiration`, `/minimum`, `/dsgnrs`)
Promotion owner: Poodle Northstar orchestrator

Scout pass over the live DesEngs index and the first-party sibling pages.
This note keeps a few Poodle-reusable families. It does not implement, restyle,
or authorize a card. It is not a dump of the site.

## Scope And Skips

Inspected 2026-09-01. The homepage is a curated outbound index. First-party
pages `/inspiration`, `/minimum`, and `/dsgnrs` are galleries and a designer
follow list. No Poodle primitive lives there.

Skip as already queued or already researched:

- icon morph — Morphrig, morphicons; see
  `20260901-125758-post-motion-research-queue.md`
- text shimmer / thinking-label shimmer — AICSS Thinking State; same note
- Transitions.dev — already audited in
  `../research/value-tracks/transitions-dev-catalogue.md`; motion policy is
  architecture 012
- block Slider / RangeSlider — same post-motion note
- interior.dev post-click timing — overlaps the 012 property budget
- Dot Matrix / Thinking orbs catalogues — decorative Spinner variants;
  `Spinner` already has `ring` / `grid` / `dots`
- liquid-glass, Liquid Gooey, crd-ui, Magic UI, Hashvatar — product chrome,
  payment widgets, or Avatar extras

Held, not promoted here: Interface Cheat Sheet
([interfaces.dev/cheat-sheet](https://interfaces.dev/cheat-sheet)). Most of it
is already Poodle policy (semantic tokens, reduced-motion wrap, tabular-nums,
focus-visible). Icon cross-fade is the queued morph lane. Press-scale is a
later style check, not a new component.

## Candidates

### 1. Transcript-inline HITL approval

What: pause-the-agent surfaces — option questions, command green-light, short
plan accept, and a recommendation card with confidence plus alternatives.

URLs:

- [Beautiful UI](https://beautifului.dev) — Approval Card, Recommendation Card
- [AICSS Approval Card](https://www.aicss.dev/components/approval-card) —
  variants `questions` | `command` | `plan`

Poodle already has:

- `AgentQuestion` / `AgentPlan` — live HITL, but composer-bound inside
  `AgentChatInput`; transcript gets settled records only
- `ConfirmAction` — destructive dialog, not an agent turn gate
- `AgentTranscript` item kinds — message, tool-run, changed-files, answered
  question, decided plan; no live approval block

Lacks: a transcript-inline approval that can gate a command or short plan
without opening a dialog and without moving the decision into the composer.
Beautiful UI's recommendation card (confidence + alternatives + accept) has no
Poodle sibling.

Why reusable: component / pattern. Same semantic job as the existing question
and plan contracts, different placement and payload (command string, scored
suggestion). Generalized tokens and composites, not a DAW widget.

Later try: one research card that asks whether command/recommendation approval
is a new transcript item kind or an additive `AgentQuestion` / `AgentPlan`
placement. Keep Poodle's no-scrim, no-focus-trap, turn-vs-action blocking
rules. Do not copy AICSS variants or Beautiful UI demo copy. Reject a second
text input beside the composer.

### 2. Sourced answers and citation marks

What: streamed or settled assistant prose with inline source markers, a
compact source footer, and optional follow-up chips.

URLs:

- [Beautiful UI](https://beautifului.dev) — Streaming Text (inline sources,
  actions, follow-ups)
- [AICSS Inline Citations](https://www.aicss.dev/r/inline-citations)
  (licensed; treat as an existence proof, not a copy source)

Poodle already has:

- `AgentMessage` — markdown subset, streaming caret, `TextLink`; avatars,
  timestamps, and message actions are out of scope
- `AgentTranscript` live region at the log, not per token
- `Code` — copy toolbar; not a citation

Lacks: citation identity, superscript/mark mapping, source list, follow-up
prompt chips. No citation contract exists under `docs/contracts/components/`.

Why reusable: component. Host-owned source records plus a small mark/list
composite would serve any agent transcript, not one product.

Later try: a later component card for `Citation` marks + a source list that
`AgentMessage` or a sibling can compose. Host supplies ids, titles, and hrefs.
Skip typewriter word-reveal (already ranked P2 in the Transitions.dev
catalogue). Do not animate or shimmer citation text. Licensed AICSS CSS is
evidence of demand, not a recipe to paste.

### 3. Agent task list

What: a live checklist the agent maintains — pending / in-progress / done /
failed rows with counts, not a proposed plan and not a tool-call run.

URLs:

- [Beautiful UI](https://beautifului.dev) — Task Rows
- [AICSS To-do List](https://www.aicss.dev/r/task-list)

Poodle already has:

- `ToolCall` / `ToolCallGroup` — one work row or a compressed run; argument
  line + output disclosure; no checklist semantics
- `AgentPlan` — accept / revise / dismiss of a proposed plan
- `Progress` — determinate/indeterminate bar, not a task tree
- `ChangedFiles` — turn file summary, not task state

Lacks: a first-class agent task-list composite with stable row identity and
status vocabulary.

Why reusable: component. Cursor-style agent todos are a generalized
conversation primitive. Keep product job names and SKU counts out of Poodle.

Later try: decide extend-`ToolCallGroup` vs a new `AgentTaskList` before any
contract. Rows need id, label, status, optional count/detail. Host owns the
list. Do not steal `AgentPlan` decision controls. Native can be static rows;
web may use existing Spinner/StatusIndicator under architecture 012.

### 4. Numeric change motion

What: digits restyle in place when a formatted number changes (spin, trend
direction, prefix/suffix, grouped sync). Accessible; reduced-motion opt-out.

URL: [NumberFlow](https://number-flow.barvian.me/)

Poodle already has:

- architecture 012 + `MotionPolicyProvider` — policy and first-pilot roles;
  no numeric-change role
- `ValueReadout` — audio-domain formatted text; snaps
- `MetricTile` — label + value + optional sparkline; snaps
- `ListCardCounter` — tabular-nums count; snaps
- `NumberInput` / `DragNumberField` — editing, not display motion

Lacks: a restriction-only numeric-change treatment. Distinct from the queued
block-slider, icon-morph, and shimmer lanes.

Why reusable: transition / style. One optional role would cover meters,
readouts, and counters without a new public widget.

Later try: a later motion/style card for an optional `numeric-change` role on
display-only numbers. Full: bounded digit restyle inside the 012 property
budget (opacity / translation / scale; no blur/mask unless separately
decided). Reduced/frozen: latest formatted text immediately. Keep
`ValueReadout` VisualState-only child and single announcement. Do not vendor
NumberFlow. Do not animate while the user is editing.

### 5. Semantic interaction sound

What: a closed set of synthesized interaction cues (press, release, toggle,
success, error, loading, ready) bound by role, not by WAV files.

URL: [Cuelume](https://cuelume.dev) (17 cues; `bind()` + `data-cuelume-*`)

Poodle already has: visual state, toast, and motion policy. No sound, haptic,
or cue-role contract in `docs/contracts/` or packages.

Why reusable: pattern. Same shape as `MotionPolicy`: host-level, restriction-
only, semantic roles, never required for correctness. Web-capable; native
no-op until a later proof.

Later try: a later research card for an optional `CuePolicy`
(`full | muted | silent`) beside motion, not inside it. Closed role list.
Host owns device permission, first-gesture unlock, and volume. Components
may request a role; they do not load files or discover OS sound settings.
Do not depend on Cuelume. Do not add soundcn's sample library. WebHaptics
stays a separate, later native/mobile question.

## Promotion Route

1. Orchestrator intake only. No contract, token, package, or roadmap edit
   follows from this note.
2. If a family is wanted, open one research or planning card per family.
   Do not bundle HITL, citations, task list, numeric motion, and sound.
3. Keep each card behind architecture 012 laws and the existing agent
   placement rules. Jetstream stays deferred.
4. Remove this note when every family is rejected or promoted into its
   owning research dossier or card.

No implementation, restyle, vendor import, or Next Task move is authorized.
