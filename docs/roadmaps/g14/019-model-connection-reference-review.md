# g14.019 — Model Connection Reference Review

Status: complete — reference approved
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

- [x] Review all four pages in both web runtimes under dark and light themes.
- [x] Review provider/route hierarchy, setup variability, card density,
      closed controls, expanded content, and model curation.
- [x] Exercise keyboard selection, stage movement, disclosure, enable switch,
      UpdateCenter, reorder, hide, and restore.
- [x] Record every item as approve, change now, contract change, or later
      native/conformance work.
- [x] Freeze the accepted reference for g14.020.

## Review Decisions

| Surface | Disposition | Reference outcome |
| --- | --- | --- |
| Model catalogue controls and density | change now | Reorder and visibility controls are live; titles are compact; provider labels are secondary; descriptions are optional; badges sit with optional info actions |
| Connection-card summary | change now | Status and access collapse into one right-side indicator; disabled cards dim; narrow cards move status below the summary while keeping controls right-aligned; the provider icon sits immediately left of the title |
| Connection-picker density and selection | change now | Redundant group badges and descriptions are removed; status copy is shorter; the selected tick replaces the provider icon |
| Setup routes without credentials | contract change | Options declare whether configuration is required; direct routes submit from selection while credential and host-configuration routes retain the configure stage |

## Acceptance Criteria

- [x] Both previews expose the same cases and interaction affordances.
- [x] Auto-detected, credential, OAuth, endpoint, unavailable, and negotiated
      postures are understandable without backend behavior.
- [x] No visual decision implies Poodle owns auth, route selection, support, or
      persistence.
- [x] Accepted changes pass focused web/parity/package checks and
      `git diff --check`.
- [x] The completion record names the approved merge/refinement commits.

## Stop Conditions

- Feedback requires backend policy, credentials, discovery, persistence, or an
  external-repository change.
- Review starts shared cases, Rust, GPUI, or Jetstream before g14.008 adopt.
- A second unbounded redesign pass is proposed instead of revisiting the spec.

## Completion Protocol

Keep review and any bounded refinement in the orchestrator/operator thread so
the operator can inspect both live previews. Do not dispatch a review worker.

## Completion Record

The operator approved the Svelte and React reference delivered by merge
`48c6ec37` after the bounded refinement in `ed6fd412`. g14.020 owns the
remaining shared-case, Rust, and GPUI work and stays blocked until g14.008
records **adopt**.
