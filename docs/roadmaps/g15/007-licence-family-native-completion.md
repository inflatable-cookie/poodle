# g15.007 — Licence Family Native Completion

Status: **blocked** — orchestration hold; `g15.006` then `g15.005` are next
Depends on: `g15.001` (measured gaps); carries `g14.017` requirements with
approved web references (`g14.015`/`g14.016`) intact
Governing refs: `release-baseline-roster.md`, `release-gap-register.md`,
`../../roadmaps/g14/017-licence-active-runtime-completion.md`,
`../../roadmaps/g14/conformance-estate.md`, `../../contracts/001-working-rules.md`

## Outcome

Complete the native surface for the Licence family — LicenceActivation,
LicenceSeats, LicenceStatus — which the inventory measured as web-complete but
missing Rust declaration, Rust render, and GPUI specimen. This recompiles the
approved `g14.017` requirements against the post-reject system: hand-written
`<Name>Spec` declarations, `poodle-render` composition, and focused
owner-local evidence. It does not revive the portable-interface or shared-corpus
pipeline.

## Scope

- LicenceActivation, LicenceSeats, LicenceStatus
- new Rust declarations in `poodle_specs`
- new `poodle-render` implementations
- new GPUI specimens and focused headless tests

## Execution Plan

- [ ] **Batch A — declarations:** hand-written `LicenceActivationSpec`,
      `LicenceSeatsSpec`, `LicenceStatusSpec` in `poodle_specs` matching the
      contract props tables (web-native props stay out of the portable spec).
- [ ] **Batch B — render:** `poodle-render` implementations for all three; no
      component-specific behaviour in generic runners.
- [ ] **Batch C — GPUI and evidence:** GPUI specimens plus focused tests
      (activation input path, inline seat rename, release, status display);
      headless regression cases via `effigy regressions:native` where a
      mounted window is required, otherwise focused `#[test]` cases in the
      render crate.

## Goals

- [ ] Hand-written `LicenceActivationSpec`, `LicenceSeatsSpec`,
      `LicenceStatusSpec` matching the contract props tables (web-native props
      stay out of the portable spec).
- [ ] `poodle-render` implementations for all three; no component-specific
      behaviour in generic runners.
- [ ] GPUI specimens and focused tests: activation input path, inline seat
      rename, release, status display; never render machine IDs; label-only
      visible identity.
- [ ] Headless regression cases via `effigy regressions:native` where a
      mounted window is required; otherwise focused `#[test]` cases in the
      render crate.
- [ ] Preserve approved curated specimens unchanged.

## Acceptance

- [ ] Every active-cohort surface (spec, render, GPUI) has evidence named in
      the card log; one runtime does not borrow another's pass.
- [ ] `cargo test -p poodle-render`, `effigy check:gpui`, and
      `effigy regressions:native` pass.
- [ ] Jetstream reported as program-deferred, not as an accepted absence.
- [ ] Longhorn's adapter assertions stay Longhorn-owned; no Poodle → Longhorn
      edge added.

## Stop Conditions

- A portable interface, shared corpus, or comparator reappears under a new
  name.
- File selection/base64 is silently substituted by a static mock instead of
  being recorded as a runtime-owned capability delta.
- `LicenceCentre` is revived by symmetry.

## Writable Scope

- Rust declarations, render modules, GPUI specimens, focused tests
- bounded contract-first fixes to scoped defects the new evidence exposes
- `release-baseline-roster.md` and `release-gap-register.md` (native rows only,
  no status lines)
- one August batch log under `docs/logs/2026-08/`
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- `cargo test -p poodle-render`
- `effigy check:gpui`
- `effigy regressions:native`
- `effigy docs:check`
- `git diff --check`

Never run a `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or
any Jetstream selector.
