# 007 Value-Domain Drift Inventory

Status: ready
Milestone: `g13.001` (drift-gate work, not IR implementation)
Owner: Poodle core
Branch: `thread/g13-007-value-domain-drift`
Depends on: contract amendments in `282ce489`
Governing refs: `docs/contracts/004-shared-control-types.md` (`T1`–`T3`),
`docs/contracts/001-working-rules.md`

## Goal

Poodle's drift gates check that a prop **exists** on both sides. They do not
check that its **permitted values** agree. That blind spot let `ButtonTone`
fragment across three contracts and three stylesheets undetected until a manual
review found it.

Build the value-domain check, run it, and produce the complete violation
inventory. **Report only — fix nothing.** Enforcement is a separate decision
once the size of the backlog is known.

## Why report-only

`docs:contract-drift` and `docs:spec-drift` both already support a
`DRIFT_REPORT=1` escape that suppresses `process.exit(1)`
(`contract-prop-drift.ts:172`, `contract-spec-drift.ts:326`). Follow that
existing pattern. A gate that fails the build on day one, across an unknown
backlog, blocks everyone; an inventory lets the orchestrator triage.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- **Fix no violations.** Finding a contract and an implementation disagreeing
  is the deliverable, not a task.
- Do not edit contracts, component source, CSS, Rust, specs, architecture,
  roadmap/milestone/card status, or `docs/roadmaps/dispatch.md`.
- Do not wire the new check into `docs:check` or CI. Add the selector only.
- Run `bun install` before any script run.
- Commit on the branch above and push with
  `git push -u origin thread/g13-007-value-domain-drift`. Do not merge.

## Writable Paths

- `packages/svelte/preview/scripts/contract-value-domain-drift.ts` (new)
- `effigy.tasks.toml` — one new selector only, not wired into `docs:check`
- `docs/roadmaps/g13/value-domain-drift-inventory.md` (the inventory)
- `docs/logs/2026-08/<DD>-g13-007-value-domain-drift.md`
- `PAPERCUTS.md` (new, non-duplicate friction only)

Any other changed path is a scope failure.

## Scope

### In scope

Comparing, per component, the permitted value set of each enumerated prop:

- **Contract side** — the union in the component contract's §3 props table,
  whether written inline (`"a" \| "b"`) or as a named type reference
  (`ButtonTone`, `StatusTone`, `ControlSize`, …).
- **TypeScript side** — the corresponding type in
  `packages/svelte/components/src/types.ts`.
- **Rust side** — the corresponding enum in
  `packages/contracts/components/src/types.rs` or the component's spec module.

Named types must be resolved before comparison. `ButtonTone` now resolves via
`docs/contracts/004-shared-control-types.md`; record any named type that cannot
be resolved from the docs rather than guessing it.

### Out of scope

- Fixing any violation.
- CSS. This card compares declared value domains, not visual delivery. (CSS
  delivery for the button family is `g13-b006`.)
- Non-enumerated props (strings, numbers, booleans, callbacks, objects).
- Enforcement, CI wiring, or `docs:check` inclusion.

## Steps

1. Baseline: `bun install`, `effigy docs:lint`, `git diff --check`. Record exit
   states.
2. Read `contract-prop-drift.ts` and `contract-spec-drift.ts` first and reuse
   their contract-parsing helpers rather than writing a third parser.
3. Build `contract-value-domain-drift.ts`. For every component contract with at
   least one enumerated prop, emit one finding per disagreement, classified:
   - `contract-wider` — contract permits a value the implementation lacks
   - `impl-wider` — implementation permits a value the contract does not
   - `unresolved-type` — a named type could not be resolved from the docs
   Each finding: component slug, prop name, side, the exact value set from each
   side, and the symmetric difference.
4. Exit 0 by default. Exit 1 only when `VALUE_DOMAIN_ENFORCE=1` is set, mirroring
   the existing `DRIFT_REPORT` pattern inverted.
5. Add one Effigy selector (`docs:value-domain-drift`) that runs it. Do not add
   it to `docs:check`.
6. Run it and write `docs/roadmaps/g13/value-domain-drift-inventory.md`:
   total counts by classification, a full findings table, and a per-component
   summary. Call out the button family explicitly — after `282ce489` those three
   should be clean, and if they are not, that is a finding about the amendment.
7. Validate: `effigy docs:lint`, `effigy docs:value-domain-drift`,
   `git diff --check`, `git status --porcelain`. Record exit states.

## Acceptance Criteria

- [ ] `contract-value-domain-drift.ts` resolves named types and inline unions on
  all three sides.
- [ ] The script exits 0 by default and 1 only under `VALUE_DOMAIN_ENFORCE=1`.
- [ ] One Effigy selector added; `docs:check` unchanged.
- [ ] The inventory lists every finding with both value sets and the symmetric
  difference, classified into the three categories.
- [ ] The button family's `tone` prop is reported explicitly (clean or not).
- [ ] No violation fixed; no contract, component, CSS, or Rust source edited.
- [ ] `effigy docs:lint` and `git diff --check` exit 0.
- [ ] Batch log records commands, exit states, and finding counts.

## Stop Conditions

- Contract §3 props tables are too irregular to parse without per-component
  special-casing beyond a handful of documented exceptions.
- A named type has no resolvable definition anywhere in `docs/`.
- Producing the inventory would require editing a contract or implementation.
- The work expands into fixing findings or enforcing the gate.

Stop with component slugs, prop names, exact paths, and the smallest
unresolved question.
