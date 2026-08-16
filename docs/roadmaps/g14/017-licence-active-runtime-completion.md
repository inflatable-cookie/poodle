# g14.017 — Licence Active-runtime Completion

Status: blocked — rewrite after `g14.021`; web reference approved
Depends on: `g14.016`, `g14.021`

Disposition: do not dispatch this version. `g14.008` rejected the pipeline it
assumes. Preserve its component requirements, then recompile the native work
after cleanup without the portable-interface/shared-corpus claim.

## Outcome

Move `LicenceStatus`, `LicenceActivation`, and `LicenceSeats` through the
adopted portable-interface, shared-case, normalized-observation, Rust renderer,
and GPUI pipeline. Preserve the approved curated specimens. Close the
intentional web-reference gap without weakening file-selection or interaction
requirements. Keep every Rust surface backend-neutral for later Jetstream
admission.

## Goals

- [ ] One portable interface and executable case corpus per component.
- [ ] Shared native composition in `poodle-render`; thin backend interpretation.
- [ ] Functional activation, inline seat rename, seat release, focus, keyboard,
      accessibility, and renderer evidence in Svelte, React, and GPUI.
- [ ] Downstream Longhorn adapter asserts Poodle mirror field maps against its
      generated field maps without adding a Poodle → Longhorn edge.

## Execution Plan

- [ ] Classify the three profiles: status=`display`, seats=`collection`,
      activation=`input/composite`.
- [ ] Keep the approved web specimens human-centred; share their ordered
      outline later through the specimen-catalogue lane rather than replacing
      them with exhaustive cases.
- [ ] Generate/check Rust portable declarations and add `poodle-render` specs.
- [ ] Port CodeInput's explicit `groups` partition and optional `separator`
      plus full-value completion validation and its tick/cross result before
      projecting grouped LicenceActivation; remove the native renderer's
      inferred 3+3 rule.
- [ ] Implement native components and preview projections.
- [ ] Preserve LicenceSeats composition: decorative monitor, controlled inline
      rename with blank-to-null emission, and ghost danger trash IconButton
      without ever rendering the machine ID.
- [ ] Treat file selection/base64 as a runtime-owned capability with equivalent
      result; missing GPUI plumbing remains red, not absent-pass.
- [ ] Run full observations, captures, and strict completion.
- [ ] Land or cite the Longhorn-owned bridge assertion comparing both public
      field-map surfaces.

## Surviving Native Infrastructure (recorded by `g14.021`)

Recorded, not designed. `g14.021` removed the pipeline this card's plan
assumes; what a rewrite can actually build on is:

- `poodle-render` — the shared native composition tier. One implementation per
  component, interpreted by the GPUI and Jetstream backends. Unchanged, and
  still where native component work belongs.
- `poodle-specs` — hand-written Rust declarations again. No codegen step, no
  interface JSON, no byte-exact authority check.
- `packages/gpui/preview/src/headless_driver.rs` — the in-memory GPUI test
  platform: mount a `poodle-node` tree, drive real pointer/key/drag input
  through the real dispatch tree, read real backend focus. No OS window, no
  focus theft, ~0.05s. Exercised by `tests/headless_regressions.rs` through
  `effigy regressions:native`.
- `effigy test:native-visual` — pixel compare/refresh with `--control-size`.
  Local-only; needs a window.
- The existing drift gates (`docs:spec-drift`, `drift:roles`, `drift:events`,
  `drift:handlers`) — each covers one projection, none proves completion.

What does **not** exist any more: portable interface modules, typed case
corpora, normalized observation, the primitive capability report, the
cross-runtime comparator, and `conformance:complete`. Every acceptance
criterion and plan step above that names one of them needs restating before
this card is dispatchable.

`g14.022` decides the replacement execution method. Do not design it here.

## Acceptance Criteria

- [ ] All g14.015 semantic/copy/privacy acceptance cases pass the active cohort.
- [ ] No runtime substitutes a static activation mock for real input/file
      plumbing.
- [ ] No runtime renders machine IDs or gates a feature.
- [ ] Rename and release emit exact machine IDs to the host while all visible
      and accessible identity remains label-only.
- [ ] `conformance:complete` passes all three components with no missing or
      declared-absence required active capability and reports Jetstream as
      program-deferred.
- [ ] The downstream drift assertion fails on a planted Longhorn or Poodle
      field change.
- [ ] Temporary web-only deltas and duplicate fixtures are removed.

## Stop Conditions

- An active native runtime cannot execute a required input path and the only
  proposed pass is declared absence.
- Component-specific behaviour is added to generic conformance runners.
- Longhorn types or policy enter Poodle.
- `LicenceCentre` is revived by symmetry rather than new shell evidence.
