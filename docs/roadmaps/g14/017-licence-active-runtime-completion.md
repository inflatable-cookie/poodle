# g14.017 — Licence Active-runtime Completion

Status: blocked pending `g14.008` adopt and `g14.016`
Depends on: `g14.008`, `g14.016`

## Outcome

Move `LicenceStatus`, `LicenceActivation`, and `LicenceSeats` through the
adopted portable-interface, shared-case/specimen, normalized-observation, Rust
renderer, and GPUI pipeline. Close the intentional web-reference gap without
weakening file-selection or interaction requirements. Keep every Rust surface
backend-neutral for later Jetstream admission.

## Goals

- [ ] One portable interface and case/specimen corpus per component.
- [ ] Shared native composition in `poodle-render`; thin backend interpretation.
- [ ] Functional activation, inline seat rename, seat release, focus, keyboard,
      accessibility, and renderer evidence in Svelte, React, and GPUI.
- [ ] Downstream Longhorn adapter asserts Poodle mirror field maps against its
      generated field maps without adding a Poodle → Longhorn edge.

## Execution Plan

- [ ] Classify the three profiles: status=`display`, seats=`collection`,
      activation=`input/composite`.
- [ ] Replace the temporary duplicate web specimens with shared cases.
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
