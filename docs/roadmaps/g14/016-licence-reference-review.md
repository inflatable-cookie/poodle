# g14.016 — Licence Reference Review

Status: active — orchestrator/operator checkpoint
Depends on: `g14.015`
Governing refs: `../../contracts/components/licence-status.md`,
`../../contracts/components/licence-activation.md`,
`../../contracts/components/licence-seats.md`

## Outcome

Review the landed Svelte and React licence specimens before native work fixes
their visual hierarchy in a second substrate. Record approval or make one
bounded web-reference refinement pass from concrete operator feedback.

The basic specimen pages already exist at these matching routes:

- `#components/licence-status`
- `#components/licence-activation`
- `#components/licence-seats`

Run them with `effigy svelte:preview` and `effigy react:preview`.

## Goals

- [ ] Review all three pages in Svelte and React under at least one dark and
      one light theme.
- [ ] Review hierarchy, spacing, tone, copy, route equality, form flow, seat
      honesty, empty state, pending state, and destructive confirmation.
- [ ] Record each feedback item as approve, change now, contract change, or
      later native/conformance work.
- [ ] Apply one bounded Svelte/React refinement batch when feedback requires
      code changes.
- [ ] Freeze the reviewed web reference for `g14.017` shared cases.

## Execution Plan

### Review bundle

- [ ] Open the three exact routes in both web previews.
- [ ] Confirm the page fixtures expose every usability state, coverage-window
      combination, trust basis, activation route, pending/disabled posture,
      labelled/unnamed seat shape, and empty seat authority.
- [ ] Capture matching review images when the operator cannot inspect both
      previews live.

### Operator decision

- [ ] Collect concrete feedback against the component contracts rather than a
      replacement settings-screen design.
- [ ] Change a contract first when feedback alters observable semantics, copy,
      events, accessibility, or layout intent.
- [ ] Keep application enforcement, Longhorn behaviour, and `LicenceCentre`
      outside the review.

### Bounded refinement

- [ ] Update shared core derivation/CSS first, then keep Svelte and React shells
      structurally aligned.
- [ ] Update both specimen pages and focused tests for every accepted change.
- [ ] Regenerate only the existing web reports owned by the touched surface.

## Acceptance Criteria

- [ ] The operator can inspect all three components without Rust or Jetstream
      workspace setup.
- [ ] Svelte and React show the same review cases and interaction affordances.
- [ ] Feedback has an explicit disposition; no unresolved visual or semantic
      question is silently handed to native implementation.
- [ ] Accepted changes pass focused component/parity/package checks and
      `git diff --check`.
- [ ] The completion record states whether the landed g14.015 reference was
      approved unchanged or names the refinement commit.

## Stop Conditions

- Feedback requires new licence policy, enforcement, authority data, or a
  Longhorn dependency.
- The review starts Rust declarations, shared conformance cases, GPUI, or
  Jetstream work before `g14.008` adopts the pipeline.
- A static native mock is proposed only to create an early screenshot.
- Svelte and React need different component semantics to satisfy the review.

## Completion Protocol

The review and any bounded refinement stay in the current orchestrator thread
so the operator can inspect each change in the collaborative preview. If no
code changes are required, record approval and close without a PR. If changes
are required, keep them in one review batch with exact captures and focused
validation; do not dispatch a separate worker.
