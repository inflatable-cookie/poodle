# Specs

Status: active
Updated: 2026-03-23

Specs hold normative repo-wide rules for contracts, artifacts, parity evidence, accessibility, packaging, and downstream adoption boundaries.

## Use

- use specs when implementation work needs repo-wide rules instead of per-component semantics
- use contracts for component-specific anatomy, props, behavior, and token semantics
- use roadmaps for sequencing and milestone execution

## Active Spec Set

The active spec set currently runs from `001` through `061`, including:

- token source and artifact contracts
- component contract template and parity rules
- accessibility and assistive-technology baselines
- product, workstation, and docs-surface rules
- packaging, release, migration, and deprecation policy
- GPUI baseline, parity, and acceptance-harness artifacts
- shared demo-app contract and audit baselines

The complete file list in this folder is the source of truth.

The active strict-planning layer is:

- `062-g10-strict-posture-and-next-boundary-gate.md`
- `batch-cards/README.md`

## Working Rule

When a roadmap task touches emitted artifacts, runtime parity, packaging, or cross-component behavior, read the relevant spec first and treat it as normative.
Before roadmap generation rollover, purge stale generation-specific
strict-planning artifacts from the active tree so the next generation does not
inherit dead planning debris. Repo-wide normative specs are not part of that
purge unless they are actually superseded.

## Next Task

Keep the spec index synchronized with the actual spec corpus and the active roadmap program, especially when roadmap consolidation changes which spec families are still current versus purely historical.
