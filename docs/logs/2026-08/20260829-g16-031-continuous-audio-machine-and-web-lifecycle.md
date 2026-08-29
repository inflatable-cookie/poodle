# g16.031 — Continuous Audio Machine And Web Lifecycle

Status: complete — awaiting operator review in PR #99
Date: 2026-08-29
PR: https://github.com/inflatable-cookie/poodle/pull/99
Branch: `t3code/continuous-audio-web-lifecycle`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-4f132f5e`
Card: `docs/roadmaps/g16/031-continuous-audio-machine-and-web-lifecycle.md`
Handoff: `docs/handoffs/20260829-161419-g16-031-continuous-audio-web-lifecycle.md`
Audit: `docs/logs/2026-08/20260828-continuous-audio-gesture-audit.md`

## Outcome

Knob, Fader, and XYPad now run one continuous-gesture model in the TypeScript
and Rust cores, proved by one shared corpus, and the Svelte and React adapters
own exactly one gesture at a time.

No ledger cell moved. This card proves paired semantics and web adapter
behaviour; native mounted proof belongs to `g16.032`.

## Locked model, now true in both languages

- One accepted begin per gesture. A second begin while one is open is inert and
  cannot re-anchor it.
- Release and cancellation are the same terminal and both are inert once the
  gesture is closed. Repeated terminals, stale pointers, lost capture, and
  teardown can neither strand nor duplicate the pair.
- A disabled control rejects every user mutation. Host value replacement,
  automation state, hover/focus reporting, entry cancellation, and the terminal
  of a gesture accepted while enabled stay live, each pinned by its own shared
  case.
- Coarse/fine switching re-anchors at the current value and current pointer.
  The transition that flips the modifier only rebases; travel resumes from the
  next sample. Rust no longer interpolates fine movement from the current
  value.
- Knob vertical mapping consumes anchored pointer delta over `dragSensitivity`;
  circular mapping consumes the absolute 270 degree sweep. Each mode ignores
  the other's input.
- Fader detents use an inclusive normalized radius, and the first declared
  detent wins an exact tie. Orientation chooses the axis and never enters the
  value law.
- XYPad presses at the accepted position, anchors fine travel per axis, rebases
  both axes together, and commits its pair atomically.

## Paired API

`@inflatable-cookie/poodle-core` — `packages/core/src/audio/value-controls.ts`
and `packages/core/src/audio/xy-pad.ts`.

- new event `DRAG_CANCEL` on `AudioValueEvent` and `XYPadEvent`; every other
  public event, effect, context field, prop, and callback name is unchanged.

`poodle_headless::audio` gained the Rust equivalents it was missing:

- `KnobContext` / `knob_transition` and `FaderContext` / `fader_transition`
  over a shared `AudioValueContext`, replacing the single generic
  `audio_value_transition`;
- `KnobDragMode`, `FaderOrientation`, `ValueBound`, `XYPadAxis`;
- `AudioValueEvent` now carries `Hover`, `Focus`, `SetValue`, `DragBegin`,
  `DragMove`, `DragSetNorm`, `DragEnd`, `DragCancel`, `Wheel`, `Reset`,
  `KeyNudge`, `KeyBound`, `EntryOpen`, `EntryCancel`, `EntryCommit`;
- `AudioValueEffect::RequestEntryFocus`;
- `AudioValueContext` carries `format`, `hover`, `focus`, `entry_open`,
  `drag_start_value`, `drag_start_position`;
- `XYPadContext` carries `hover`, `focus`, the four drag anchors, and renamed
  `step_x`/`step_y` to `keyboard_step_x`/`keyboard_step_y`;
- `XYPadEvent::Nudge`/`Bound` take one axis, a multiplier, and a bound instead
  of a paired direction, matching the web axis sliders;
- pointer geometry the Rust side never had: `AudioPoint`, `AudioRect`,
  `hit_test_rect`, `hit_test_circle`, `knob_point_to_norm`,
  `fader_point_to_norm`, `xy_pad_point_to_norm`.

Nothing consumed the removed `audio_value_transition`; `g16.032` mounts the two
named transitions instead.

## Shared vectors

`packages/contracts/headless/vectors/machines.json` gained one bounded
`audioControls` section, hand authored:

| Group | Cases | Ordered steps |
| --- | --- | --- |
| knob | 14 | 75 |
| fader | 10 | 48 |
| xyPad | 11 | 48 |
| geometry | 17 | — |

Each case starts from the control's default context with listed overrides and
runs ordered steps; every step pins the effects it emitted in order and, where
the case claims it, a subset of the resulting context. Both runners execute the
same file: `packages/core/test/conformance.test.ts` and
`packages/contracts/headless/tests/conformance.rs#audio_controls_conformance`.
All expected values match bit for bit in both languages with no tolerance.

The corpus stays on `linear` and `bipolar-center` laws so both runtimes do the
same arithmetic. Logarithmic anchoring and detents under a non-linear law are
covered by focused per-language tests with tolerance instead.

## Web lifecycle

All six adapters now:

- accept one primary pointer and refuse a second pointer-down;
- ignore stale pointer ids on move and release;
- close through one `cancelGesture` path shared by `pointercancel`,
  `lostpointercapture`, and component teardown (`onDestroy` in Svelte, an
  unmount effect in React).

Svelte Knob and Fader gained React's one-blur suppression, and both frameworks
now reset that flag whenever the entry opens, so a flag left set by an
unmounted entry cannot swallow a later commit.

The cancel guard takes a defaulted parameter rather than an optional one: the
Svelte package build strips the type annotation but leaves the `?`, so the
optional form shipped as invalid JavaScript. Only `effigy test:web-pack-install`
catches that; the component suites compile from source and pass either way.

Terminal cleanup resolves from an adapter-owned synchronous snapshot of the
machine, written before any host callback runs, rather than from a render's
context. Without it React strands a gesture whose host unmounts the control
from inside `onGestureBegin`: React batches the begin render away, so cleanup
saw `drag: "none"` and emitted no `onGestureEnd`. Svelte did not have the
defect — its reactive context survives teardown — but now uses the same
explicit storage, so the callback-parity claim covers this timing rather than
resting on one framework's teardown order. Both suites carry the regression;
removing the React snapshot fails it and nothing else.

## Callback traces

`packages/svelte/components/test/AudioControlsLifecycle.svelte.test.ts` and
`packages/react/components/test/AudioControlsLifecycle.test.tsx` assert the same
ten claims and the same traces:

| Claim | Trace |
| --- | --- |
| Knob gesture, refused second pointer, release, then the lost capture a release causes | begin ×1, change `[0.6]`, commit `[0.6]`, end ×1 |
| Knob lost capture, twice, then release | begin ×1, end ×1 |
| Knob host unmounts from `onGestureBegin` | begin ×1, commit `[0.5]`, end ×1 |
| Knob teardown mid-gesture | begin ×1, commit `[0.5]`, end ×1 |
| Fader cancel with stale ids | change `[0.25], [0.8]`, commit `[0.8]`, begin ×1, end ×1 |
| XYPad press with a refused second pointer | change `[0.25, 0.75], [0.5, 0.5]`, commit `[0.5, 0.5]`, begin ×1, end ×1 |
| Knob Enter | 1 blur, commit `[0.75]` |
| Knob Escape | 1 blur, no commit, value 0.5 |
| Knob unresolved blur | commit `[0.75]` |
| Fader Enter then Escape | commit `[0.8]` only, value 0.8 |

Each entry case pins the blur count as well as the commit count, so a pass
cannot come from a blur that never fired.

## Contract reconciliation

Architecture 008's gesture contract now states the one-open-gesture rule, the
shared terminal, the disabled-mid-gesture exception, the rebase rule, the Knob
mode split, the inclusive detent radius with first-declared tie resolution, and
the paired-vector location. Knob, Fader, and XY Pad contracts state the same in
their machine and callback sections.

## Orchestrator review, 2026-08-29

Three blocking findings on head `ad27306a6`, all addressed:

1. **React could strand a gesture.** The unmount callback closed over a
   render's context; a host that removes the control from inside
   `onGestureBegin` unmounts before React commits the render that opened the
   gesture, so cleanup ran `DRAG_CANCEL` against `drag: "none"` and emitted no
   `onGestureEnd`. Fixed by the live snapshot above, with the reproduction as a
   regression in both suites.
2. **DragNumberField was partially migrated.** `dragNumberTransition` is
   restored byte-identical to `main` and its lifecycle test removed. Its repair
   belongs to a later card, with both adapters.
3. **"Disabled is inert on every route" was false.** Both cores deliberately
   keep host and presentation routes and the gesture terminal. The card,
   architecture 008, the three contracts, and this log now define disabled
   *user-mutation* inertia and name the exceptions, and three new shared cases
   pin them. Both runners carry automation state so `SET_AUTOMATION` is real
   evidence rather than an untested claim.

A follow-up `svelte-check` error in the new Svelte regression (a deferred
`let view` binding widened the query return type) was fixed by destructuring
the render result.

## Non-claims

- no ledger cell moved and no evidence level changed;
- no native mounting, Node vocabulary, renderer, or GPUI/Jetstream change;
- no payload drag-and-drop dependency;
- no public web prop, callback, or specimen change; specimens needed no repair;
- DragNumberField is untouched: `dragNumberTransition` is byte-identical to
  `main`, and it keeps the pre-card lifecycle on purpose. It shares the value
  helpers, not the one-begin/cancel rules, because migrating its machine
  without its two web adapters would leave them disagreeing. Its repair belongs
  to a later card;
- no accessibility or visual-comparison claim.

## Validation

Focused: `packages/core` audio value-control, XY pad, and conformance suites;
`packages/contracts/headless` lib and `conformance` tests; the two new mounted
lifecycle suites.

Board, all confirmed by task exit code: `effigy test:core` (1081 pass),
`effigy test:contracts`, `effigy ci:rust`, `effigy ci:web`, full
`bunx vitest run` (334 web files, 2366 tests), `effigy docs:check` including
`docs:lint`, `check:parity-evidence-ledger`, and the
contract/callback/spec/value-domain drift gates, `effigy qa`, and
`git diff --check origin/main...HEAD`. Nothing windowed, native-visual,
Jetstream-preview, release, or workflow-mutating was run.

`effigy test:web-pack-install` caught the one defect the component suites could
not: the packaged Svelte build stripped the cancel guard's type annotation and
left the optional marker, so `Fader`, `Knob`, and `XYPad` shipped invalid
JavaScript to consumers. Fixed with a default parameter before the board went
green.

## Next

Do not start native mounting from this PR. After operator-authorized merge,
`g16.032` implements the bounded Node/GPUI continuous-value seam and the three
mounted proofs, and must not overlap `g16.025`.
