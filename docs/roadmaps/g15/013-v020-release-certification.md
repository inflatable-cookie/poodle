# g15.013 — v0.2.0 Release Certification

Status: **blocked** — pending orchestrator review of `g15.001`
Depends on: `g15.002`–`g15.005` (Svelte focused evidence closure),
`g15.006` (React mirror closure)
Governing refs: `release-baseline-roster.md`, `release-gap-register.md`,
`../../contracts/001-working-rules.md`

## Outcome

Certify and package Poodle v0.2.0 once every Svelte-denominator blocker is
closed. The certification claim is the frozen Svelte roster (175 components
with contract, implementation, export, specimen, and focused evidence).
Missing React, Rust, and GPUI surfaces remain explicit parity gaps under the
working rules; experimental package labels stay honest. This card does not
claim active-cohort parity completion.

## Scope

- release certification against the frozen roster and closed register rows
- packed-consumer proof across the roster
- package, documentation, and release notes for v0.2.0

## Goals

- [ ] Every Svelte-denominator surface is complete and evidenced for all 175
      components.
- [ ] Packed reachability proven across the roster, not a sample.
- [ ] v0.2.0 package, changelog, and documentation reflect the honest state:
      Svelte certified; React/Rust/GPUI parity gaps named; Jetstream deferred.
- [ ] Register rows updated to closed where owned by earlier cards; remaining
      rows renamed as post-release parity work, not v0.2.0 blockers.

## Acceptance

- [ ] `effigy qa` passes (except the known `bun audit` advisory, if still
      present, recorded with its upstream owner).
- [ ] The release claim matches the roster: no cross-runtime pass borrowed,
      no parity claim beyond the Svelte denominator.
- [ ] The experimental React/Rust/GPUI package labels remain honest.

## Stop Conditions

- The certification claims cross-runtime parity or active-cohort completion.
- A release blocker is waived by a declared absence.
- Packaging proceeds while a Svelte-denominator surface is still open.

## Writable Scope

- package metadata and release notes for v0.2.0
- roster and register status updates owned by completed cards
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- `effigy qa` (headless local release board)
- `effigy test:web-pack-install`
- `effigy docs:check`
- `git diff --check`

Never run a `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or
any Jetstream selector.
