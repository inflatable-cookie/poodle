# Continuous Audio Gesture Audit

Date: 2026-08-28
Scope: Fader, Knob, XYPad across contracts, TypeScript core, Svelte, React,
Rust headless, shared renderer, Node/GPUI routing, specimens, tests, and the
g16 evidence/register surfaces
Outcome: real repair confirmed; compiled as `g16.031` and `g16.032`

## Question

The g16 continuation register left all three controls `unknown`. The audit had
to distinguish an evidence-only gap from actual semantic/interface drift and
identify whether native interaction could reuse an existing seam without
entering the payload drag-and-drop programme.

## Authority

- `docs/architecture/008-audio-control-family.md`
- `docs/contracts/components/knob.md`
- `docs/contracts/components/fader.md`
- `docs/contracts/components/xy-pad.md`
- `docs/architecture/011-drag-and-drop-substrate.md` for the explicit
  continuous-value versus payload-drag boundary
- `docs/roadmaps/g16/parity-evidence-ledger.md`

The contracts already lock the public values, laws, pointer modes, fine mode,
detents, reset, keyboard, wheel, type-in, callback ordering, and accessibility
semantics. No operator product decision is missing.

## Findings

### TypeScript and web adapters

- `knobTransition`, `faderTransition`, and `xyPadTransition` accept a second
  `DRAG_BEGIN` while a drag is active. The Svelte and React adapters also
  overwrite their active pointer on a second pointer-down. Both facts violate
  the exactly-once begin/end contract.
- All six adapters handle pointer-up and pointer-cancel, but none handles lost
  pointer capture or teardown. A captured gesture can therefore remain open
  without its end callback.
- Svelte Knob/Fader focus the root after Enter or Escape while the entry blur
  always commits. Enter can commit twice; Escape can be followed by a commit.
  React already has a one-blur suppression guard.
- Focused core tests cover many pure transitions, but component tests cover
  mostly keyboard/reset. They do not prove pointer ownership, callback order,
  capture loss, or Knob/Fader type-in closure through both web adapters.

### Rust headless

The register's “visual-state-only” description is incorrect. Rust already has
`AudioValueContext`/`audio_value_transition` and
`XYPadContext`/`xy_pad_transition`. They are incomplete paired authorities:

- fine movement interpolates repeatedly from the current value toward each
  new pointer sample instead of using an anchored start with modifier rebase;
- the generic scalar machine does not carry Fader detents or distinguish Knob
  vertical and circular mapping;
- XYPad gesture begin carries no press position and therefore cannot match the
  web control's immediate coarse press update;
- normalized-set events can establish drag state without an accepted begin;
  and
- Rust tests prove only a shallow gesture pair and one atomic XY key change,
  not the contract surface or paired TypeScript results.

### Shared renderer and GPUI

- `KnobSpec`, `FaderSpec`, and `XYPadSpec` carry serializable visual and label
  data. Their poodle-render builders accept no handlers and attach no machine
  interaction. GPUI specimens convert static nodes only.
- Knob/Fader declare a Slider role but omit numeric value, bounds, value text,
  and Fader orientation even though `NodeA11y` already supports them.
- XYPad declares one Group with combined text. It does not expose the two
  Slider accessibility children required by its contract.
- The ledger is honest: all three have structural GPUI specimen routes and no
  named mounted regression.

### Reusable native seam

The native stack does not need a second drag framework:

- Node `on_scrub` and the GPUI backend already prove local normalized
  measurement, captured movement outside bounds, and press/move/release for
  Slider and RangeSlider.
- Generic Node `on_drag` reports only per-frame deltas. Under crates.io GPUI
  0.2.2 it emits no end and carries no modifiers or normalized local point, so
  it cannot satisfy Knob/XYPad lifecycle by itself.
- A bounded continuous-value event can combine normalized local x/y, logical
  pixel delta, modifiers, and press/move/release/cancel. It stays
  renderer-neutral and separate from subjects, payloads, targets, operations,
  cross-window transfer, files, and drag-out.

## Classification

This is a known repair, not an evidence-only gap and not a new product
decision. The three rows move from `unknown` to `known repair` in the
continuation register. No parity-ledger cell moves during planning.

The repair is split into two batches because one worktree would span paired
cores, six web adapters, Node vocabulary, GPUI backend routing, three native
mounts, accessibility, specimens, vectors, and regressions:

1. `g16.031` — paired TypeScript/Rust machines, shared vectors, web pointer
   lifetime, and Svelte entry closure;
2. `g16.032` — bounded Node continuous-value event, GPUI backend dispatch,
   handler-backed audio renderers, stateful specimens, accessibility, and
   three named mounted regressions.

`g16.031` waits for `g16.030` because all three component cards edit paired
core/headless exports and shared vector runners. `g16.032` follows `031` and
must not overlap `g16.025`, which edits the same Node/GPUI interaction seam for
payload drag-and-drop.

## Non-claims

- no component implementation changed;
- no ledger evidence changed;
- no broad native accessibility or visual comparison was claimed;
- no Jetstream behavior or admission was added;
- no payload drag/drop architecture was changed; and
- DragNumberField and the other audio controls were not audited.

## Validation

The audit used source inspection only; it changed no implementation or ledger
claim. `effigy docs:lint`, `effigy check:parity-evidence-ledger`, and
`effigy docs:check` passed after the cards and front doors were reconciled. One
final headless `effigy qa` also passed. The final diff check is recorded in the
orchestrator commit that adds this log and the two cards.
