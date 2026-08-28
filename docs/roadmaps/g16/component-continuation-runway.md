# g16 Component Continuation Runway

Status: active planning map — no component implementation card ready
Compiled: 2026-08-28
Source: `component-continuation-register.md`

This file routes the seven candidate lanes from the 175-row audit. It does not
copy the register, change evidence, or turn a missing cell into a defect.

## Lane Map

| Order | Lane | Current state | Promotion gate |
| ---: | --- | --- | --- |
| 1 | TimeInput native entry | decision-blocked | choose native segment/value/draft behavior and update the component contract |
| 2 | NumberInput and EditableLabel editing models | decision-blocked | confirm NumberInput's typed committed value plus host-owned raw draft; separately resolve EditableLabel activation/draft/commit/focus |
| 3 | Dependable drag-and-drop family | programme-owned | compiled separately as `g16.021`–`g16.028`; do not issue component-local drag repairs |
| 4 | Fader, Knob, and XYPad interaction | unknown | run one bounded continuous-gesture audit before naming a repair |
| 5 | GPUI accessibility evidence | programme choice | operator selects it as a programme and accepts the manual/runtime evidence boundary |
| 6 | Cross-runtime visual comparison | programme choice | operator selects a fixture/comparison tranche without reviving specimen snapshots or a component IR |
| 7 | No current implementation lane | closed/evidence-only | 162 rows remain out of implementation until concrete evidence names work |

## Next Component Decision

TimeInput is first because it is independent of drag-and-drop and can become a
bounded foundation card once native entry semantics are explicit. The web
contract delegates to `<input type="time">`; GPUI has no equivalent native
control. The operator must choose the native editing model before a worker can
implement or claim mounted parity.

NumberInput and EditableLabel remain behind separate value/draft decisions.
The recommendation for NumberInput stays the recorded typed committed number
plus host-owned raw draft/empty state. That recommendation is not yet authority.

## Parallelism

`g16.021` may run while the TimeInput decision is discussed: it touches the
generic TypeScript/Rust semantic kernel and shared vectors, not component
contracts or implementations. No other component, accessibility, visual,
motion, Longhorn-lab, or Jetstream worker is ready from this map.

## Promotion Rule

For a candidate to become a numbered roadmap card, record the operator choice
in its contract or architecture first, then supply bounded scope, acceptance,
validation, evidence movement, stop conditions, and continuation. Do not send a
worker to discover the decision inside implementation.
