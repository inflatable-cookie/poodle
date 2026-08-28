# g16 Component Continuation Runway

Status: active planning map — TimeInput decision promoted as g16.029
Compiled: 2026-08-28
Source: `component-continuation-register.md`

This file routes the seven candidate lanes from the 175-row audit. It does not
copy the register, change evidence, or turn a missing cell into a defect.

## Lane Map

| Order | Lane | Current state | Promotion gate |
| ---: | --- | --- | --- |
| 1 | TimeInput native entry | planned as `g16.029` | approved contract; promote after `g16.021` merges because shared core/headless exports overlap |
| 2 | NumberInput and EditableLabel editing models | decision-blocked | confirm NumberInput's typed committed value plus host-owned raw draft; separately resolve EditableLabel activation/draft/commit/focus |
| 3 | Dependable drag-and-drop family | programme-owned | compiled separately as `g16.021`–`g16.028`; do not issue component-local drag repairs |
| 4 | Fader, Knob, and XYPad interaction | unknown | run one bounded continuous-gesture audit before naming a repair |
| 5 | GPUI accessibility evidence | programme choice | operator selects it as a programme and accepts the manual/runtime evidence boundary |
| 6 | Cross-runtime visual comparison | programme choice | operator selects a fixture/comparison tranche without reviving specimen snapshots or a component IR |
| 7 | No current implementation lane | closed/evidence-only | 162 rows remain out of implementation until concrete evidence names work |

## Next Component Decision

TimeInput's native editing decision is approved. The contract now keeps its
canonical time string, makes partial/invalid drafts adapter-owned, defines
valid-value-only callbacks, whole-second stepping and overnight ranges, and
requires a segmented 24-hour GPUI editor plus a clean Rust
`TimeFieldSpec`→`TimeInputSpec` rename. `g16.029` contains the bounded
implementation lane.

NumberInput and EditableLabel remain behind separate value/draft decisions.
The recommendation for NumberInput stays the recorded typed committed number
plus host-owned raw draft/empty state. That recommendation is not yet authority.

## Parallelism

`g16.021` remains the active worker. Do not dispatch `g16.029` beside it: both
cards edit shared TypeScript/Rust exports and conformance infrastructure.
Promote and dispatch `g16.029` after `g16.021` merges, before choosing another
overlapping core lane. No accessibility, visual, motion, Longhorn-lab, or
Jetstream worker is ready from this map.

## Promotion Rule

For a candidate to become a numbered roadmap card, record the operator choice
in its contract or architecture first, then supply bounded scope, acceptance,
validation, evidence movement, stop conditions, and continuation. Do not send a
worker to discover the decision inside implementation.
