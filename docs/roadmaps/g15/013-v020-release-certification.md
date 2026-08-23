# g15.013 — v0.2.0 Release Certification

Status: **operator gate — `g15.050` accepted and landed; awaiting explicit tag
and publication authorisation**
Depends on: every earlier g15 implementation/specimen/conformance card,
`g15.042`–`g15.052` as sequenced by the runway, and one clean accepted v0.2.0
candidate SHA
Governing refs: `release-baseline-roster.md`, `release-gap-register.md`,
`../../contracts/001-working-rules.md`

## Outcome

Authorise the exact v0.2.0 candidate prepared by `g15.050`, then perform the
human-owned tag/publication operation. This card contains no implementation
batch and is never dispatched to a worker. The frozen Svelte roster remains
the release denominator: 175 components with contract, implementation, export,
specimen, and focused evidence. React, shared Rust, GPUI, visual evidence, and
deferred Jetstream are reported independently and honestly.

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
- [x] `g15.011` landed: the human-centred specimen catalogue audit is complete.
- [x] `g15.012` landed: the primitive-first visual conformance lane is complete.
- [x] `g15.038` landed: SegmentedControl's contracted icon and icon-only option
      surface exists in shared Rust and GPUI without a compatibility twin.
- [x] `g15.042` landed: Stepper selection and re-run are live in GPUI.
- [x] `g15.043` landed: UiPresentationProvider has a real native cascade.
- [x] `g15.048` landed: every public web component is reachable from clean
      packed roots.
- [x] `g15.049` landed: release and native pre-tag automation are truthful.
- [x] `g15.052` landed: Button and Stepper use the reusable native focus-ring
      channel, and Stepper accepts keyboard entry without a pointer prelude.
- [x] `g15.050` landed: one clean v0.2.0 candidate, artifacts, notes, and QA
      receipt are pinned to an exact SHA.

## Compiled Release Children

- [`g15.048`](048-packed-roster-reachability.md) owns packed public-root proof.
- [`g15.049`](049-release-automation-truthfulness.md) owns stale/vacuous
  automation repair and requires explicit operator approval before workflow
  edits.
- [`g15.050`](050-v020-release-candidate.md) owns versions, notes, dry-run
  artifacts, full headless QA, and the exact candidate receipt.
- [`g15.052`](052-native-focus-ring-parity.md) owns the native focus-ring and
  Stepper keyboard-entry blocker found by `g15.042`/`g15.047`.

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

- [ ] Operator reviews the `g15.050` receipt and exact SHA.
- [ ] Operator explicitly authorises the tag and publication mutation.
- [ ] The release is dispatched against that tag; no failed tag is reused.
- [ ] Registry/GitHub results are checked and the generation closeout records
      the actual published set.

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

## Validation

Use only the pinned `g15.050` evidence and the repaired release automation.
Do not rerun a windowed local harness to manufacture late evidence. Release
mutation remains operator-owned.
