# g16 Component Continuation Runway

Status: active planning map — TimeInput, NumberInput, and continuous audio complete
Compiled: 2026-08-28
Source: `component-continuation-register.md`

This file routes the seven candidate lanes from the 175-row audit. It does not
copy the register, change evidence, or turn a missing cell into a defect.

## Lane Map

| Order | Lane | Current state | Promotion gate |
| ---: | --- | --- | --- |
| 1 | TimeInput native entry | complete as `g16.029` | merged in PR #97; ledger 48 mounted / 126 missing |
| 2 | NumberInput and EditableLabel editing models | NumberInput complete as `g16.030`; EditableLabel decision-blocked | merged in PR #98; resolve EditableLabel separately |
| 3 | Dependable drag-and-drop family | programme-owned | compiled separately as `g16.021`–`g16.028`; do not issue component-local drag repairs |
| 4 | Fader, Knob, and XYPad interaction | complete through `g16.032` / PR #100 | native mounting landed; never overlap a Node/GPUI routing card with `g16.025` |
| 5 | GPUI accessibility evidence | programme choice | operator selects it as a programme and accepts the manual/runtime evidence boundary |
| 6 | Cross-runtime visual comparison | programme choice | operator selects a fixture/comparison tranche without reviving specimen snapshots or a component IR |
| 7 | No current implementation lane | closed/evidence-only | 162 rows remain out of implementation until concrete evidence names work |

## Next Component Decision

TimeInput's native editing decision is closed in `g16.029`. The contract keeps
its canonical time string, makes partial/invalid drafts adapter-owned, defines
valid-value-only callbacks, whole-second stepping and overnight ranges, and
ships a segmented 24-hour GPUI editor plus a clean Rust `TimeInputSpec` /
`time_input` rename.

NumberInput's clean value-model decision is complete. Its contract now uses a
typed committed `number | null`, an optional raw-draft channel, valid-value-only
changes, explicit commit/revert behavior, and no string-value or redundant
step-source callbacks. `g16.030` delivered the bounded clean migration and
mounted proof in PR #98. EditableLabel remains behind its separate
activation/draft/commit/focus decision.

The bounded Fader/Knob/XYPad audit is complete. The old register description
was wrong: Rust scalar and XY machines exist, but they are not behaviorally
paired or mounted. Fine movement is not anchored like TypeScript, Fader detents
and Knob modes are absent, XYPad press semantics differ, web pointer lifetime
can duplicate or strand gestures, and Svelte Knob/Fader entry blur can undo the
intended Enter/Escape boundary. `g16.031` closed paired machine and web
lifecycle behavior. `g16.032` added one bounded Node continuous-value event
and mounted all three in GPUI. Payload drag-and-drop remains separate.

## Parallelism

`g16.029`–`g16.032` are merged. No accessibility, visual, motion,
Longhorn-lab, or Jetstream worker is ready from this map. EditableLabel remains
decision-blocked rather than becoming an implementation card by default.

## Promotion Rule

For a candidate to become a numbered roadmap card, record the operator choice
in its contract or architecture first, then supply bounded scope, acceptance,
validation, evidence movement, stop conditions, and continuation. Do not send a
worker to discover the decision inside implementation.
