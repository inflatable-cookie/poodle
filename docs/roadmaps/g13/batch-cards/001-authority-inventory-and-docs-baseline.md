# 001 Authority Inventory And Docs Baseline

Status: ready
Milestone: `g13.001`
Owner: Poodle core
Branch: `thread/g13-001-authority-inventory`
Governing refs: `docs/specs/063-rust-authored-component-and-scene-ir.md`
(`IR-01`–`IR-12`), `docs/architecture/001-poodle-system-shape.md`,
`docs/architecture/006-headless-core-and-machine-model.md`,
`docs/contracts/001-working-rules.md`

## Goal

Produce the evidence baseline needed for the orchestrator to freeze authority
and crate placement. Repair current documentation inventory drift without
starting IR implementation.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents or parallel research tasks. Read sources directly.
- Do not edit roadmap README files, roadmap milestone files, this card's
  status, `docs/roadmaps/dispatch.md`, architecture, specs, or working rules.
- Write only the scoped deliverables, batch log, and PAPERCUTS entries.
- Do not create crates/packages, component implementations, compatibility
  shims, or new public APIs.
- Do not hand-edit generated artifacts. Use their existing generator; if none
  exists or generation rewrites unrelated source, stop and report.
- Where sources conflict, stop with exact citations. Do not choose a new
  authority or invent an IR feature.
- Commit the finished batch on the branch above and push it. Do not merge.

## Scope

### In scope

- `docs/roadmaps/g13/authority-inventory.md` — evidence table and measured
  current-state map.
- Current files needed to repair these inherited docs failures:
  - AgentSubagent usage-doc coverage
  - Keyboard, ModMatrixGrid, and WaveformDisplay contract-index coverage
  - Keyboard, ModMatrixGrid, and WaveformDisplay Svelte preview coverage
  - stale `@inflatable-cookie/poodle-svelte` export/preview counts in the shared
    demo audit
- Existing registries/generators required to make those facts authoritative.
- `docs/logs/2026-08/11-g13-001-authority-inventory.md` — commands, exit states,
  counts, findings, and anything not verifiable.
- `PAPERCUTS.md` only for new, non-duplicate execution friction.

### Out of scope

- `docs/specs/`, `docs/architecture/`, component contracts, public component
  APIs, component implementation source, CSS, tokens, visual baselines, and
  Effigy configuration.
- Any crate/package creation or codegen implementation.
- Selecting the final crate location, schema, macro syntax, lowering design,
  or migration sequence.
- Fixing unrelated lint/test failures discovered during the batch.

## Steps

1. Capture the current failures from `effigy docs:check`, `effigy docs:lint`,
   and `effigy svelte:surface-audit` in the batch log.
2. Build `authority-inventory.md` from exact repository evidence. For each
   surface record: owner/path, authored vs generated, source of truth,
   consumers, drift check, and current duplication/bypass.
3. Cover at minimum:
   - component contracts and contract indexes
   - TypeScript machines/styles and Svelte/React shells
   - Svelte/React specimen definitions, registries, preview shells, and report
     generation
   - Rust specs/headless/render/node path
   - GPUI and Jetstream registries, specimens, adapters, and accessibility/
     visual reports
   - direct Jetstream `RenderComponent<Spec>` implementations and preview
     compatibility layers
4. Add focused file maps for Button, RangeSlider, TextInput, the preview
   header/theme selector, and size/density specimen axes across all four
   runtimes.
5. Record crate-placement evidence only: existing workspace boundaries,
   publication metadata, dependency direction, source-consumer constraints,
   and plausible locations. Provide a comparison table; make no recommendation.
6. Repair only the named docs-baseline failures using the existing authority
   and generators. Do not change component behavior.
7. Re-run validation. Record commands and exact exit states in the batch log.
8. Review the final diff for scope, then commit and push the worker branch.

## Acceptance Criteria

- [ ] `authority-inventory.md` maps every required web/native definition and
  evidence surface with exact paths.
- [ ] Direct native bypasses and compatibility layers are enumerated rather
  than summarized as generic debt.
- [ ] Pilot component/shell/axis file maps are complete across all four
  runtimes.
- [ ] Crate-placement evidence is sufficient for an orchestrator ruling and
  contains no worker decision.
- [ ] Named docs inventory drift is repaired without component behavior or API
  changes.
- [ ] `effigy docs:check` passes.
- [ ] `git diff --check` passes.
- [ ] Batch log contains command exit states and measured counts.
- [ ] No file outside scope changed.

## Evidence

- Full path/count tables in `authority-inventory.md`.
- Before/after docs-gate output summarized in the batch log.
- `git diff --stat` and final changed-file list in the batch log.
- No recommendation required; the orchestrator rules on placement.

## Stop Conditions

- A named docs failure requires changing component behavior, a public API, or
  a component contract.
- Existing authoritative surfaces disagree about a pilot component's public
  semantics or runtime ownership.
- A required generated artifact has no generator, or its generator mutates
  unrelated source.
- The inventory cannot distinguish an active path from dead compatibility
  code using repository evidence.
- Work expands into IR schema/codegen design or package creation.

Stop with paths, commands, and the smallest unresolved question. Do not patch
around the conflict.
