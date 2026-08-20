# g15.013 — v0.2.0 Release Certification

Status: **blocked** — final generation gate after `g15.008`–`g15.012` and
`g15.038`
Depends on: `g15.002`–`g15.012` (release-baseline implementation, specimens,
and evidence), `g15.014` (release-gate remediation — security prerequisite),
`g15.038` (SegmentedControl native option parity)
Governing refs: `release-baseline-roster.md`, `release-gap-register.md`,
`../../contracts/001-working-rules.md`

## Outcome

Certify and package Poodle v0.2.0 only after every earlier g15 implementation,
specimen, audit, and conformance card is complete **and every release gate is
green**. The frozen Svelte roster remains the release denominator: 175
components with contract, implementation, export, specimen, and focused
evidence. The completed active-cohort cards must also be reported honestly;
experimental package labels stay explicit and Jetstream remains
program-deferred. Effigy's release contract does not allow a red gate or an
unfinished earlier card to be waived.

## Prerequisites

- [x] `g15.002`–`g15.006` landed: every Svelte-denominator surface complete
      and evidenced for all 175 components; React mirror rows closed.
- [x] `g15.014` (release-gate remediation) landed: the `bun audit` nanoid
      advisory (GHSA-2v37-7h3g-55p8) through the React preview's Vite
      dependency resolved, or the dependency replaced, with the advisory
      cleared from `effigy qa`.
- [x] `g15.007` landed: Licence native rows and the shared release roster and
      gap register are current before certification begins.
- [x] `g15.008`–`g15.010` landed: remaining native declarations, shared Rust
      rendering, and GPUI specimen gaps are closed.
- [ ] `g15.011` landed: the human-centred specimen catalogue audit is complete.
- [ ] `g15.012` landed: the primitive-first visual conformance lane is complete.
- [x] `g15.038` landed: SegmentedControl's contracted icon and icon-only option
      surface exists in shared Rust and GPUI without a compatibility twin.

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
      the full Svelte denominator and completed active-cohort evidence are
      named; experimental labels stay explicit; Jetstream is deferred.
- [ ] Register rows owned by `g15.002`–`g15.012` are closed; any remaining rows
      are explicitly outside the active cohort or release denominator.

## Operator Gate

- [ ] No tag, publish, or release mutation happens without explicit operator
      approval on the certification run. This card records the gate; the
      operator executes the release.

## Acceptance

- [ ] `effigy qa` passes fully green (every lane, including `bun audit`).
- [ ] The release claim matches the roster and completed active-cohort cards;
      no runtime borrows another runtime's pass.
- [ ] The experimental React/Rust/GPUI package labels remain honest.
- [ ] The operator has approved the release mutation explicitly.

## Stop Conditions

- The certification claims Jetstream completion or visual parity beyond the
  evidence landed by `g15.012`.
- A release blocker is waived by a declared absence.
- A red release gate is bypassed rather than remediated.
- Packaging proceeds while any `g15.008`–`g15.012` card, Svelte-denominator
  surface, or release gate remains open.

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
