---
title: Plan the Nucleus GPUI parity programme
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: planning
dispatch_authority: orchestrator
review_authority: orchestrator
merge_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-02
updated: 2026-09-02
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260902-021251-nucleus-gpui-parity-programme.md
base_required: pushed-main
tags: [coordination, handoff, planning, gpui, parity, nucleus]
---

## Goal

Produce one evidence-backed programme packet that makes GPUI parity measurable
and reachable using Nucleus as the first switch target. The operator chose
Nucleus verbatim: its 29 distinct Poodle components define the parity bar, and
shipping Nucleus on GPUI is the proof. Do not reopen that choice or ask the
operator questions in this lane.

## Authority

- Read `AGENTS.md`, `docs/triage/20260901-233708-holistic-posture-assessment.md`,
  the active parity ledger/contracts, the GPUI developer guide, and current g16
  runway.
- Read Nucleus only to inventory its real Poodle component use and product
  interaction requirements. Do not mutate Nucleus or any sibling repository.
- GPUI parity remains the product goal. Do not slow or de-prioritise native
  work, and do not substitute structural presence or a name map for execution.
- Jetstream's direct 108-component adapter remains held and cannot count as
  the Poodle render/node/GPUI proof.
- This is planning only. Do not change Poodle production source, ledger cells,
  tests, workflows, contracts, Nucleus, Jetstream, or release state.

## Output

Create exactly one new triage packet under `docs/triage/`. It must:

- pin the exact 29-component Nucleus inventory and identify any composition or
  version ambiguity;
- define per-component proof levels for mounted execution, accessibility, and
  visual parity, with executable evidence rather than test-name presence;
- identify current evidence and the shortest honest gap for each component;
- route paired-machine divergences, including nested HistoryCenter deletion,
  Slider negative-half rounding, and native Tabs tooltips, into bounded source
  repairs rather than hiding them in ledger prose;
- define the first shippable Nucleus-on-GPUI acceptance journey and stop
  conditions;
- separate Poodle work from Nucleus adoption, dedicated conformance-lab work,
  GPUI accessibility authority, and Jetstream quarantine/admission;
- propose a dependency graph and small implementation cards ordered by Nucleus
  leverage, not alphabetical component order;
- state how the existing ledger becomes execution-backed without discarding
  useful expected-test-manifest data.

Record recommendations as delegate proposals pending orchestrator/operator
review. Where external lab or accessibility authority is missing, mark the
edge explicitly; do not waive it or block unrelated source-ready cards.

## Validation And Completion

Run `effigy docs:lint` and `git diff --check origin/main...HEAD`. Keep the PR to
the single packet. Push and open one PR against current `main`; report URL and
exact head. Do not merge or dispatch implementation.
