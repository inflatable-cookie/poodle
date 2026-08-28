# g16 Component Continuation Runway

Status: active planning map — TimeInput and NumberInput decisions promoted
Compiled: 2026-08-28
Source: `component-continuation-register.md`

This file routes the seven candidate lanes from the 175-row audit. It does not
copy the register, change evidence, or turn a missing cell into a defect.

## Lane Map

| Order | Lane | Current state | Promotion gate |
| ---: | --- | --- | --- |
| 1 | TimeInput native entry | planned as `g16.029` | approved contract; dispatch after `g16.021` merges because shared core/headless exports overlap |
| 2 | NumberInput and EditableLabel editing models | NumberInput planned as `g16.030`; EditableLabel decision-blocked | complete `g16.029`, then execute the approved NumberInput committed-value/raw-draft migration; resolve EditableLabel separately |
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

NumberInput's clean value-model decision is approved. Its contract now uses a
typed committed `number | null`, an optional raw-draft channel, valid-value-only
changes, explicit commit/revert behavior, and no string-value or redundant
step-source callbacks. `g16.030` contains the bounded clean migration and
mounted proof. EditableLabel remains behind its separate
activation/draft/commit/focus decision.

## Parallelism

`g16.021` remains the active worker. Do not dispatch `g16.029` beside it: both
cards edit shared TypeScript/Rust exports and conformance infrastructure.
Dispatch `g16.029` after `g16.021` merges, then dispatch `g16.030` only after
TimeInput closes because both edit the same exports and domain-vector corpus.
No accessibility, visual, motion, Longhorn-lab, or Jetstream worker is ready
from this map.

## Promotion Rule

For a candidate to become a numbered roadmap card, record the operator choice
in its contract or architecture first, then supply bounded scope, acceptance,
validation, evidence movement, stop conditions, and continuation. Do not send a
worker to discover the decision inside implementation.
