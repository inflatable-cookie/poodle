# g13-047 Machine Shape Consolidation, And The Unpinned Nine

Date: 2026-08-13
Card: `docs/roadmaps/g13/batch-cards/047-machine-shape-consolidation.md`
Branch: `thread/g13-047-machine-shape-consolidation`

## What changed

- **Convention written down** in `g11.002` ("Machine Shape Convention"):
  stateful machines (State + Context + Event + Effect + `TransitionResult` /
  `(State, Vec<Effect>)`) and the trivial case (single implicit state, value in
  context; no State type). Read off `hover`/`menu`/`modal`/`popover` in both
  runtimes plus the `g11.002` section list; not a new design.
- **`switch` pinned**: `machines.json` gained a 5-case `switch` block; both
  harnesses now run it (`case "switch"` in `conformance.test.ts`,
  `switch_conformance` in `tests/conformance.rs`).
- **`color`, `date`, `duration`, `nav`, `pagination`, `tree` pinned**: the
  Rust-only `domain.json` vectors are now run by the TS side too
  (`packages/core/test/domain-conformance.test.ts`, mirroring
  `tests/domain_conformance.rs`). `duration` (15 cases) and `nav` (9 cases)
  vectors were generated FROM the TS core, per the file's convention.
- **Gate**: `packages/svelte/preview/scripts/machine-shape-drift.ts`, wired as
  `effigy docs:machine-shape-drift`. Two rules: every machine present in both
  runtimes must be pinned by a vector both harnesses run; every module that
  declares a transition must follow the convention. One baseline entry
  (`rs:text_input`).

No machine source file changed. No public export renamed. No behavior changed.

## R1 classification (recorded)

Card buckets come from a structural grep; per R1 each candidate was classified
by reading the module. "Should conform and does not" came back **empty in both
runtimes** — every off-pattern candidate is the documented trivial case or
machinery, matching the card's own expectation (037: 22/34 false positives).

### TypeScript

| Module | Bucket | Evidence |
|---|---|---|
| hover, menu, modal, popover | conforms already | reference machines; `TransitionResult`, full shape |
| edit, history-center, tabs | conforms already | import `TransitionResult`; State/Context/Event/Effect/Result |
| checkbox, disclosure, single-select, switch, toggle-group | correctly different | single implicit state, value in context — g11.002's documented trivial case; no State type; no `TransitionResult` because there is no state to report. Inventing one is the failure mode R1 forbids |
| tree | correctly different | behavior machinery (flatten, cascade, intents, windowing), not a state machine; no transition fn |
| slider | excluded | b046 owns it; vector is a fixed target there |

### Rust

| Module | Bucket | Evidence |
|---|---|---|
| hover, menu, modal, popover, audio, slider | conforms already | canonical per card; full shape |
| checkbox, disclosure, single_select, switch, toggle_group, tabs | correctly different | the six modules the card names as off-pattern only for lack of State; value lives in context — trivial case |
| tree | correctly different | machinery, not a machine |
| text_input | correctly different | text editing model (caret/selection/keyboard contract), not a behavior machine; baselined in the gate |

## Pin correction (measured, not assumed)

The card's pinning table says 12 pinned / 9 unpinned, listing
`single_select` and `toggle_group` as unpinned. Measured: `machines.json`
already contained `singleSelect` (4 cases) and `toggleGroup` (4 cases) keys,
run by **both** harnesses (`conformance.test.ts` and `conformance.rs`) and
green at baseline. So 14 machines were actually pinned before this card, and
the genuinely unpinned set was **7**: `color`, `date`, `duration`, `nav`,
`pagination`, `switch`, `tree`. All 7 now have vectors on both sides; all 21
duplicated machines are pinned.

## R4a — depth inventory of the already-pinned machines

For each machine pinned before this card's vector work: does its vector
exercise the real surface or just the happy path? (12 per the card's table;
measured 14 — `singleSelect`/`toggleGroup` were already pinned.)

| Machine | Cases | Surface exercised | Verdict |
|---|---|---|---|
| checkbox | 5 | TOGGLE (disabled, readOnly, mixed guards) + SET_CHECKED — every event and guard | real surface |
| disclosure | 2 | TOGGLE open + disabled guard | happy path; SET_OPEN and close direction unexercised |
| hover | 6 | ENTER/LEAVE/TIMER_FIRE across opening/open/closing states + re-enter cancel | partial; DISMISS and SET_OPEN unexercised |
| menu | 3 | TOGGLE open + ACTION | happy path; OPEN/CLOSE/ESCAPE/OUTSIDE_INTERACT and disabled guard unexercised |
| modal | 5 | OPEN/CLOSE/REQUEST_CLOSE/ESCAPE/BACKDROP_CLICK + both guards | real surface |
| popover | 4 | TOGGLE open order, ESCAPE close, outside guard, disabled | partial; OPEN/CLOSE direct events and initialFocus strategies unexercised |
| singleSelect | 4 | SELECT (enabled, disabled, same-value, unknown guards) | partial; SET_VALUE unexercised |
| toggleGroup | 4 | TOGGLE multiple add/remove, single deactivate/reselect | partial; SET_VALUE and disabled guard unexercised |
| tabs | 5 | SELECT, FOCUS_MOVE (auto/manual), REORDER_STEP, CLOSE closable | partial; ACTIVATE, direct REORDER, prev/first/last moves unexercised |
| slider | 3 | input snap/clamp, commit clamp, degenerate range | **thin** — b045 measured zero two-thumb coverage; excluded (b046 owns it), recorded not fixed |
| agent-plan | 18 (decide 6, labels 4, summary 8) | canDecidePlan, decidePlan, planStatusLabel, planRecordSummary — all 4 exports | real surface |
| agent-question | 22 (resolve 8, toggle 4, progress 6, summary 4) | resolveQuestionAnswer, toggleQuestionSelection, questionProgress, answeredQuestionSummary | partial; 7 of 11 exports (submitsOnSelect, declineQuestion, canSubmitQuestion, showsQuestionProgress, nextQuestionIndex, questionBatchComplete, isChosenOption) unexercised |
| agent-subagent | 24 (terminal 8, spinning 8, labels 8) | isTerminalSubagentStatus, subagentStatusSpins, subagentStatusLabel — all 3 exports | real surface |
| agent-transcript | 26 (grouping 13, windowing 8, pinned 5) | groupTranscriptItems, transcriptWindow, isPinnedToBottom | partial; 6 of 9 exports (toolRunLeadCall, toolRunHiddenCount, toolRunStatus, changedFilesTotals, buildChangedFileTree, changedFileScopes) unexercised |

Summary: 4 real-surface (checkbox, modal, agent-plan, agent-subagent), 8
partial, 1 thin (slider, excluded). The mechanism the roadmap depends on is
carrying about a quarter of the surface at full depth. Follow-up scope, per
R4a: thin/partial vectors should be deepened in a later card, starting with
the dismissable-layer machines (menu, popover) whose dismissal events are
entirely unpinned.

## Divergence findings

None. Every new vector (switch, duration, nav, and the TS side of the
pre-existing domain vectors) passed on first run on both implementations —
TS 191 conformance tests green, Rust 11 + 6 domain tests green.

## Gate

- `effigy docs:machine-shape-drift` passes clean: 21 pinned machines, both
  harnesses reference every vector.
- Proved both halves fail independently: planted an unpinned duplicate
  (`__probe_machine` in both src trees) → "present in both runtimes but not in
  the PINNED registry"; planted an off-pattern transition
  (`__probe_shape.ts`) → "transition without the machine shape". Both probes
  removed; gate green after.
- Baseline: `rs:text_input` — text editing model, not a behavior machine.

## Validation

All step-8 commands exit 0:
`effigy test:core`, `effigy test:components`, `effigy ci:rust`,
`effigy ci:web`, `effigy docs:lint`, `effigy docs:machine-shape-drift`,
`git diff --check`.
