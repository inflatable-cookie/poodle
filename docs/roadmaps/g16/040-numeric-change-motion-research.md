# g16.040 — Numeric Change Motion Research

Status: research-ready — `g16.034` merged in PR #124
Opened: 2026-09-01
Depends on: merged `g16.034` at `369a24f8c`; architecture 012; existing
Transitions.dev `Number pop-in` and `Spinning counter` findings
Governing refs: `../../architecture/012-semantic-motion-policy.md`,
`../../contracts/components/value-readout.md`,
`../../contracts/components/metric-tile.md`,
`../../research/value-tracks/transitions-dev-catalogue.md`
Intake: DesEngs candidate 4, merged in PR #126
Source leads: [NumberFlow](https://number-flow.barvian.me/),
[Calligraph](https://calligraph.raphaelsalaja.com/)

## Goal

Consolidate NumberFlow and Calligraph evidence with the existing
Transitions.dev numeric-motion findings. Decide whether display-only numeric
updates justify a bounded semantic motion role in Poodle. Editing controls are
out of scope.

This card authorizes research only. It does not add a role, component,
formatter, dependency, or animated default.

## Questions

- Which consumer change has semantic value beyond decorative emphasis?
- Can formatted sign, prefix, suffix, grouping, decimals, width change, and
  trend direction retain stable identity without owning number formatting?
- What full treatment fits architecture 012's property budget and native
  approximation rules?
- Must reduced and frozen always paint the latest formatted value immediately?
- How does visual digit segmentation remain one accessible value and one
  announcement?

## Required Evidence

- Reuse and cite the Transitions.dev dossier; do not repeat its catalogue
  audit as new research.
- Inspect NumberFlow and Calligraph from durable primary or pinned sources,
  including mechanism, licensing, interruption, and reduced-motion behavior.
- Compare ValueReadout, MetricTile, ListCardCounter, and representative
  consumers; explicitly exclude NumberInput and DragNumberField editing.
- Assess Svelte, React, shared Rust, and GPUI feasibility, rapid retargeting,
  tabular numerals, locale changes, forced colors, and capture mode.

## Deliverable And Promotion Gate

Write `docs/research/value-tracks/numeric-change-motion.md` as a consolidation
dossier. Recommend role, recipe-only guidance, static behavior, or rejection.
Promotion requires accepted `g16.034`, an operator-approved semantic consumer,
and explicit active-cohort fallback.

## Writable Scope

The dossier only, plus `PAPERCUTS.md` for new execution friction. Do not edit
architecture, contracts, source, packages, roadmaps, triage, or consumers.

## Validation

Run `effigy docs:lint` and `git diff --check`.
