# g15.013 — v0.2.0 Release Certification

Status: **ready** — prerequisites complete; dispatch after `g15.007` releases
shared roster/register rows
Depends on: `g15.002`–`g15.005` (Svelte focused evidence closure),
`g15.006` (React mirror closure), `g15.014` (release-gate remediation —
security prerequisite)
Governing refs: `release-baseline-roster.md`, `release-gap-register.md`,
`../../contracts/001-working-rules.md`

## Outcome

Certify and package Poodle v0.2.0 once every Svelte-denominator blocker is
closed **and every release gate is green**. The certification claim is the
frozen Svelte roster (175 components with contract, implementation, export,
specimen, and focused evidence). Missing React, Rust, and GPUI surfaces
remain explicit parity gaps under the working rules; experimental package
labels stay honest. This card does not claim active-cohort parity completion.
Effigy's release contract does not allow a red gate to be waived: `effigy qa`
must be fully green before certification, which requires the nanoid advisory
remediation to have landed first.

## Prerequisites

- [x] `g15.002`–`g15.006` landed: every Svelte-denominator surface complete
      and evidenced for all 175 components; React mirror rows closed.
- [x] `g15.014` (release-gate remediation) landed: the `bun audit` nanoid
      advisory (GHSA-2v37-7h3g-55p8) through the React preview's Vite
      dependency resolved, or the dependency replaced, with the advisory
      cleared from `effigy qa`.

## Execution Plan

- [ ] **Batch A — packed-consumer proof:** extend the packed-tarball proof
      across the roster (beyond the 9 mounted components) and confirm packed
      reachability per component.
- [ ] **Batch B — package, documentation, and release notes:** v0.2.0 package
      metadata, changelog, and documentation reflecting the honest state:
      Svelte certified; React/Rust/GPUI parity gaps named; Jetstream deferred.
- [ ] **Batch C — certification run:** full `effigy qa` green, register rows
      closed where owned by earlier cards, and the release claim handed to
      the operator.

## Goals

- [ ] Every Svelte-denominator surface is complete and evidenced for all 175
      components.
- [ ] `effigy qa` passes on every lane, including `bun audit`.
- [ ] v0.2.0 package, changelog, and documentation reflect the honest state:
      Svelte certified; React/Rust/GPUI parity gaps named; Jetstream deferred.
- [ ] Register rows updated to closed where owned by earlier cards; remaining
      rows renamed as post-release parity work, not v0.2.0 blockers.

## Operator Gate

- [ ] No tag, publish, or release mutation happens without explicit operator
      approval on the certification run. This card records the gate; the
      operator executes the release.

## Acceptance

- [ ] `effigy qa` passes fully green (every lane, including `bun audit`).
- [ ] The release claim matches the roster: no cross-runtime pass borrowed,
      no parity claim beyond the Svelte denominator.
- [ ] The experimental React/Rust/GPUI package labels remain honest.
- [ ] The operator has approved the release mutation explicitly.

## Stop Conditions

- The certification claims cross-runtime parity or active-cohort completion.
- A release blocker is waived by a declared absence.
- A red release gate is bypassed rather than remediated.
- Packaging proceeds while a Svelte-denominator surface is still open, or
  while the `bun audit` advisory is still open.

## Writable Scope

- package metadata and release notes for v0.2.0
- packed-consumer proof under `test/package-install/`
- roster and register status updates owned by completed cards
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- `effigy qa` (headless local release board — must be fully green)
- `effigy test:web-pack-install`
- `effigy docs:check`
- `git diff --check`

Never run a `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or
any Jetstream selector.
