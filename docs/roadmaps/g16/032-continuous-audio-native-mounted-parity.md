# g16.032 — Continuous Audio Native Mounted Parity

Status: ready — `g16.031` merged in PR #99; do not overlap `g16.025`
Opened: 2026-08-28
Depends on: completed `g16.031`; landed paired audio machines and callback
effects
Governing refs: `../../architecture/008-audio-control-family.md`,
`../../contracts/components/knob.md`, `../../contracts/components/fader.md`,
`../../contracts/components/xy-pad.md`,
`../../architecture/006-headless-core-and-machine-model.md`,
`../../contracts/001-working-rules.md`

## Goal

Mount Knob, Fader, and XYPad as real interactive GPUI controls over the paired
Rust machines from `g16.031`. Add the smallest reusable Node vocabulary needed
for captured continuous-value gestures, wheel/reset/entry routes, semantic
focus, and the controls' exact accessibility projection.

The renderer continues to consume serializable visual state. Runtime geometry,
capture, focus, keyboard, wheel, text entry, callbacks, and host-owned rebuilds
stay in adapters and handlers.

## Native Boundary

The 2026-08-28 audit found three distinct current facts:

- `poodle-node` `on_scrub` already proves captured 1D press/move/release for
  Slider and RangeSlider.
- generic `on_drag` only emits start/move under crates.io GPUI 0.2.2; it cannot
  close a value gesture and carries neither modifier state nor normalized
  local position.
- current audio specs/renderers are static: they accept no handlers, mount no
  machine, and under-project accessibility. The GPUI specimens therefore show
  structure and pixels only.

Build on the scrub lesson, not on the payload drag session kernel. One backend
may use GPUI's internal drag primitive for pointer capture, but no payload,
drop target, operation, cross-window, or file concept enters this API.

## Locked Node Vocabulary

Add one renderer-neutral continuous-value event carrying:

- phase: press, move, release, or cancel;
- pointer position normalized to the receiving node's local rectangle on both
  axes, with x increasing right and y increasing up;
- per-dispatch logical-pixel delta on both axes;
- current Shift/Alt/platform-accelerator modifier facts; and
- no DOM, GPUI, window, pointer-id, payload, file, or application type.

The Node interaction installs at most one handler for this event. The backend
owns measurement, primary-pointer admission, capture beyond bounds, unique
gesture lifetime, and exactly-once release/cancel. It must not dispatch a
second press while one value gesture is active.

Keep existing `on_scrub` for Slider/RangeSlider and existing `on_drag` for
ResizeHandle. Do not migrate unrelated controls or add aliases. Reuse their
mounted regressions as retained protection.

Add only the small semantic activation routes the contracts require and the
Node vocabulary does not currently expose:

- wheel intent with normalized direction plus modifiers; and
- double activation for reset.

Enter/Escape text-entry behavior uses the existing input submit/cancel/edit,
selection, and focus channels. Arrow/Page/Home/End behavior extends the
existing key vocabulary only where the contracts require a missing physical
key.

## Component Mounting

Introduce idiomatic handler structs for Knob, Fader, and XYPad. They expose
the existing contract effects only:

- live value change;
- value commit;
- gesture begin; and
- gesture end.

The production render path must:

- retain machine state across host-owned rebuilds;
- route normalized point/delta/modifiers to the correct paired transition;
- route keyboard, wheel, double-click reset, and Knob/Fader type-in through
  the same machine;
- keep VisualState as the sole drawing input; and
- reject every user-mutation route when disabled while preserving host value
  replacement, automation state, focus/hover reporting, entry cancellation,
  and the terminal of a gesture accepted while enabled.

The GPUI specimens own explicit state and rebuild so interaction is visible.
They are examples, not a hidden test matrix.

## Accessibility

- Knob and Fader expose one focusable Slider with label, min, max, current
  numeric value, formatted value text, disabled state, and Fader orientation.
- XYPad exposes a labelled Group containing two focusable Slider semantics,
  one per axis, with independent labels, bounds, numeric values, and formatted
  value text.
- Visual nodes stay hidden from assistive technology.
- Focus rings follow the same semantic root that receives keyboard input.

This is focused semantic projection and mounted inspection, not broad native
screen-reader certification. The ledger's GPUI accessibility axis remains
manual.

## Execution Plan

- [ ] **Batch 1 — continuous-value Node event.** Add the event, backend
      dispatch, capture/cancel lifetime, wheel/double activation, and focused
      backend tests while retaining Slider/RangeSlider/ResizeHandle behavior.
- [ ] **Batch 2 — Fader mount.** Add handlers, machine-backed rebuild state,
      axis/detent/fine behavior, keyboard/wheel/reset/entry, accessibility,
      and one mounted regression.
- [ ] **Batch 3 — Knob mount.** Add vertical/circular mapping, fine rebase,
      keyboard/wheel/reset/entry, accessibility, and one mounted regression.
- [ ] **Batch 4 — XYPad mount.** Add atomic two-axis press/move, fine rebase,
      reset, independent keyboard sliders, accessibility children, and one
      mounted regression.
- [ ] **Batch 5 — specimens and closeout.** Make the three GPUI specimen
      examples visibly stateful, update only the three mounted ledger cells,
      and record exact behavior, validation, and non-claims in one log.

## Acceptance Criteria

- [ ] One accepted continuous-value gesture produces one press, zero or more
      moves, and exactly one release or cancel, including outside-bounds and
      lost-host paths.
- [ ] The Node event contains only normalized local position, logical-pixel
      delta, phase, and modifiers; it has no payload drag/drop or runtime type.
- [ ] Existing Slider, RangeSlider, and ResizeHandle mounted regressions stay
      green without migration.
- [ ] Fader proves horizontal/vertical position, detents, fine rebase,
      keyboard, wheel, reset, type-in, callback ordering, disabled inertia,
      and full Slider accessibility through production GPUI dispatch.
- [ ] Knob proves vertical/circular movement, fine rebase, keyboard, wheel,
      reset, type-in, callback ordering, disabled inertia, and full Slider
      accessibility through production GPUI dispatch.
- [ ] XYPad proves coarse press, atomic pair moves/commits, fine rebase, reset,
      independent axis keys, disabled inertia, and two child Slider semantics
      through production GPUI dispatch.
- [ ] Human-facing GPUI specimens visibly rebuild from interaction and remain
      aligned with the curated Svelte/React examples.
- [ ] Only Fader, Knob, and XYPad GPUI mounted-behavior cells move from missing
      to mounted. Starting from the expected post-`g16.030` 49 / 125 total,
      the ledger becomes 52 mounted / 122 missing. Accessibility and visual
      comparison cells do not move.
- [ ] Jetstream receives no behavior claim. Mechanical shared Rust compile
      maintenance is allowed only if the Node vocabulary requires it.

## Writable Scope

- the exact continuous-value, wheel, double-activation, and required physical
  key vocabulary in `packages/contracts/node/`;
- focused Node and `packages/gpui/node-backend/` interaction tests;
- Knob, Fader, and XYPad specs only for state/identity/accessibility data the
  mounted path demonstrably needs;
- `packages/render/src/audio.rs`, its exports, and focused tests for the three
  handler-backed builders;
- GPUI audio specimens, state ownership, headless driver support, and named
  mounted regressions for the three controls;
- mechanical Jetstream compile references only if the shared Node surface
  changes, with no new behavior or evidence claim;
- the three component contracts and architecture 008 for exact landed names;
- the three ledger rows/checker, this card, one August log, g16/front-door
  closeout, and `PAPERCUTS.md` only for new execution friction.

Do not edit TypeScript/Svelte/React behavior, the paired audio-machine vectors,
payload drag-and-drop semantics/adapters, unrelated Node interactions or
components, recipes/tokens except a proven missing semantic state, broad
accessibility or visual programmes, workflows, versions, releases, sibling
repositories, or downstream consumers.

## Validation

Use Effigy selectors discovered after worker startup. At minimum:

- focused Node and GPUI backend interaction tests;
- focused poodle-specs and poodle-render audio tests;
- named mounted Knob, Fader, and XYPad regressions;
- retained mounted Slider, RangeSlider, and ResizeHandle regressions;
- `effigy probe:gpui-specimens`;
- `effigy test:contracts`, `effigy ci:rust`, `effigy ci:native`, and
  `effigy docs:check`;
- contract/callback/capability drift checks and
  `effigy check:parity-evidence-ledger`;
- one final headless `effigy qa`; and
- `git diff --check origin/main...HEAD`.

Everything stays headless. Never run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, publication, or workflow-mutation selectors.

## Stop Conditions

- Crates.io GPUI 0.2.2 cannot provide captured release/cancel or wheel facts
  without a fork, platform-private patch, focus-stealing windowed harness, or
  application-owned global listener.
- Correctness requires exposing runtime coordinates, pointer ids, payload
  drag/drop state, or GPUI types through `poodle-node`.
- The public value/callback contracts or the paired machine semantics from
  `g16.031` must change.
- XYPad accessibility cannot expose two sliders through the current Node
  accessibility tree without a wider accessibility programme.
- Work expands into other audio controls, the payload drag programme, visual
  comparison, Jetstream admission, release, or a sibling repository.
- More than the exact three mounted-behavior ledger cells would move.

## Continuation

Return the exact Node event, backend lifetime evidence, three handler surfaces,
mounted callback/accessibility traces, retained regressions, ledger delta,
validation, and closeout log to the orchestrator. Do not migrate another audio
control or start a broader accessibility/visual programme.
