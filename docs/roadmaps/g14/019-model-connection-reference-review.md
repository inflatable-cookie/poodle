# g14.019 — Model Connection Reference Review

Status: blocked pending `g14.018`
Depends on: `g14.018`
Governing ref: `../../specs/067-model-connection-management.md`

## Outcome

Review the landed Svelte and React model-connection specimens with the operator
before shared cases and native rendering freeze the hierarchy. Apply at most
one bounded web-reference refinement batch from concrete feedback.

## Review Surface

- `#components/model-connection-picker`
- `#components/model-connection-setup`
- `#components/model-connection-card`
- `#components/model-catalogue-editor`

Run `effigy svelte:preview` and `effigy react:preview`.

## Goals

- [ ] Review all four pages in both web runtimes under dark and light themes.
- [ ] Review provider/route hierarchy, setup variability, card density,
      closed controls, expanded content, and model curation.
- [ ] Exercise keyboard selection, stage movement, disclosure, enable switch,
      UpdateCenter, reorder, hide, and restore.
- [ ] Record every item as approve, change now, contract change, or later
      native/conformance work.
- [ ] Freeze the accepted reference for g14.020.

## Acceptance Criteria

- [ ] Both previews expose the same cases and interaction affordances.
- [ ] Auto-detected, credential, OAuth, endpoint, unavailable, and negotiated
      postures are understandable without backend behavior.
- [ ] No visual decision implies Poodle owns auth, route selection, support, or
      persistence.
- [ ] Accepted changes pass focused web/parity/package checks and
      `git diff --check`.
- [ ] The completion record names the approved merge/refinement commits.

## Stop Conditions

- Feedback requires backend policy, credentials, discovery, persistence, or an
  external-repository change.
- Review starts shared cases, Rust, GPUI, or Jetstream before g14.008 adopt.
- A second unbounded redesign pass is proposed instead of revisiting the spec.

## Completion Protocol

Keep review and any bounded refinement in the orchestrator/operator thread so
the operator can inspect both live previews. Do not dispatch a review worker.
