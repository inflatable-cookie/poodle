# g14.016 — Licence Reference Review

Status: complete — reference approved
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

- [x] Review all three pages in Svelte and React under at least one dark and
      one light theme.
- [x] Review hierarchy, spacing, tone, copy, activation-model selection,
      external and embedded account journeys, account/offline flow, seat
      honesty, empty state, pending state, and destructive confirmation.
- [x] Record each feedback item as approve, change now, contract change, or
      later native/conformance work.
- [x] Apply one bounded Svelte/React refinement batch when feedback requires
      code changes.
- [x] Freeze the reviewed web reference for `g14.017` shared cases.

## Execution Plan

## Review Decisions — Active

| Feedback | Disposition | Reference outcome |
| --- | --- | --- |
| React preview chrome diverged from Svelte | change now | React uses the shared theme picker; Svelte carries the matching runtime badge |
| Three peer activation tabs do not match real product choices | contract change | key is one product mode; account is the other, with offline file as its direct fallback |
| Account activation may be browser-owned or embedded | contract change | injected token provider owns acquisition; optional host content supplies embedded login fields |
| Machine naming should be optional and inline | contract change | omitted hides it; string seeds it; null uses honest unnamed placeholder copy; inline editor shares the submit row |
| Offline switch read as an external link and dominated the header | change now | top-right `xs` ghost Button with `cloud-off`/`user` icon and secondary text colour |
| Header, route, and footer controls were crowded | change now | explicit shared spacing at both boundaries |
| Licence keys may need fixed-length segmented entry | contract change | optional `keyCodeInput={{ length, groups, separator }}`; default remains free-form TextInput |
| CodeInput inferred one 3+3 divider from length | contract change | web grouping is an explicit full partition with an optional presentation-only separator and supports any number of groups; native port is a named g14.017 delta |
| Segmented keys need immediate format feedback | contract change | CodeInput validates only at full length and renders a value-bound success tick or failure cross; LicenceActivation delegates the check to the injected parser |
| Seat rows need a faster visual identity cue | change now | every LicenceSeats row starts with the same decorative computer icon; labels remain the only identity claim |
| Machine names need correction in place | contract change | every row composes EditableLabel and emits host-owned rename requests by machine ID; blank commits restore the honest unnamed state |
| Release text actions dominate a compact machine list | change now | release uses a row-named ghost danger IconButton with `trash-2`; confirmation remains intact |

### Review bundle

- [x] Open the three exact routes in both web previews.
- [x] Confirm the page fixtures expose every usability state, coverage-window
      combination, trust basis, both activation models, the account/offline
      switch, external and host-owned embedded account content,
      pending/disabled posture, labelled/unnamed seat shape, and empty seat
      authority.
- [x] Capture matching review images when the operator cannot inspect both
      previews live.

### Operator decision

- [x] Collect concrete feedback against the component contracts rather than a
      replacement settings-screen design.
- [x] Change a contract first when feedback alters observable semantics, copy,
      events, accessibility, or layout intent.
- [x] Keep application enforcement, Longhorn behaviour, and `LicenceCentre`
      outside the review.

### Bounded refinement

- [x] Update shared core derivation/CSS first, then keep Svelte and React shells
      structurally aligned.
- [x] Update both specimen pages and focused tests for every accepted change.
- [x] Regenerate only the existing web reports owned by the touched surface.

## Acceptance Criteria

- [x] The operator can inspect all three components without Rust or Jetstream
      workspace setup.
- [x] Svelte and React show the same review cases and interaction affordances.
- [x] Feedback has an explicit disposition; no unresolved visual or semantic
      question is silently handed to native implementation.
- [x] Accepted changes pass focused component/parity/package checks and
      `git diff --check`.
- [x] The completion record states whether the landed g14.015 reference was
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

## Completion Record

The operator approved the refined Svelte and React reference after the bounded
review batch landed through commits `d5f2bba1` and `5180ac16`. g14.017 owns the
remaining shared-case, Rust, and GPUI work.
