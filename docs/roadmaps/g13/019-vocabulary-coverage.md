# g13.019 Vocabulary Coverage Across The Corpus

Status: planned
Owner: Poodle core
Depends on: `g13.018`

## Objective

Bring the rest of the corpus into the single vocabulary authority, family by
family, without rewriting components.

## Deliverables

- Vocabulary definitions for each family: part names, `data-*` names, value
  domains, axes, recipe hooks, capability declarations.
- `ir:check` coverage for each family as it lands.
- A running count of components covered, and of drift the coverage catches.

## Constraints

- **Additive only.** This is not component migration; that is closed
  (`g13.009`–`016`). No component is rewritten to consume a definition unless
  doing so is smaller than not doing so, measured per component.
- The pilot measured +965 lines across nine files for full consumption. If a
  family's consumption cost looks similar, declare the vocabulary and stop
  there — the authority and the drift gate are the deliverable, not the wiring.

## Acceptance

- Coverage is reported as a number, not a claim.
- Every family's vocabulary is drift-gated.
- Corpus LOC does not grow materially. If it does, stop and report.

## Next

`g13.020` consolidates and reassesses from the single authority.
