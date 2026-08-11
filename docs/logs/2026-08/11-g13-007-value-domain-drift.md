---
title: g13.007 — value-domain drift inventory
status: complete
owner: Poodle core
updated: 2026-08-11
tags: [log, g13, drift, value-domain, report-only]
---

## Scope

Executed `docs/roadmaps/g13/batch-cards/007-value-domain-drift-inventory.md`
on `thread/g13-007-value-domain-drift`. Built the value-domain check
(`contract-value-domain-drift.ts`), added the `docs:value-domain-drift` Effigy
selector (not wired into `docs:check`), ran it, and produced the complete
violation inventory. **Report only — nothing fixed.** No contract, component
source, CSS, Rust, spec, architecture, roadmap/card-status, dispatch, or
`docs:check` change.

## Baseline (exit states)

| Command | Exit |
|---|---|
| `bun install` | 0 |
| `effigy docs:lint` | 0 |
| `git diff --check` | 0 |

## Deliverables

- `packages/svelte/preview/scripts/contract-value-domain-drift.ts` (new) —
  per-component comparison of each enumerated prop's permitted value set across
  contract §3 ("Public Props" table), Svelte `Props` (resolved through
  `types.ts` + component locals + poodle-core), and the `<Name>Spec` enum.
  Reuses the contract-parsing helpers of `contract-prop-drift.ts` /
  `contract-spec-drift.ts` (same "### Public Props" table slice, same escaped-
  pipe row splitting, same `snake` field mapping) rather than a third parser.
- `tasks/effigy.tasks.toml` — one new selector `docs:value-domain-drift`; not
  added to `docs:check`.
- `docs/roadmaps/g13/value-domain-drift-inventory.md` (new) — the inventory.
- `docs/logs/2026-08/11-g13-007-value-domain-drift.md` — this log.
- `PAPERCUTS.md` — one new entry (stale `ButtonTone` in
  `docs/guides/svelte-developer-guide.md`).

## Behavior

- Exits 0 by default; exits 1 only when `VALUE_DOMAIN_ENFORCE=1` (the
  `DRIFT_REPORT=1` pattern inverted). Verified both ways.
- Classifications: `contract-wider`, `impl-wider`, `unresolved-type`; each
  finding carries component slug, prop, side, both value sets, and the
  symmetric difference.
- Documented exception: `ButtonVariant::Danger` dropped from the Rust side per
  `docs/contracts/004-shared-control-types.md` (backward-compat retention, not
  authored vocabulary). Applied to button/icon-button/split-button `variant`.
- Named types resolve from docs: 004 → own contract → other contracts
  (deterministic) → guides. Unresolvable named types are `unresolved-type`
  findings; named types that resolve to function/object shapes on the TS side
  are non-enumerated props (out of scope) and are skipped. `Snippet`-typed
  props excluded as framework idiom (same as `contract-prop-drift.ts`).
- Rust variants project to literals via kebab-case; spelling divergences
  (`alertdialog` vs `alert-dialog`, `firstRun` vs `first-run`, `between` vs
  `space-between`, `label`/`body` vs `default`) are reported, with the spelling
  reading called out in the inventory for triage.

## Findings Counts

- Checked: 128 components / 447 enumerated props (skipped 1055 non-enumerated,
  33 no TS side, 76 no Rust side, 5 non-comparable TS types).
- **21 value-domain findings across 16 components** — contract-wider: 11,
  impl-wider: 10.
- **8 unresolved-type** — `ColorInputMode` (1), `DockEdge`/`DockSizing`/
  `DockCollapsedPosture`/`DockEmphasis` (4), `AudioAutomationState` (3).

Button family: `tone` clean on all three sides for button/icon-button/
split-button after `282ce489` — the amendment holds; reported explicitly in
the inventory. `variant` clean after the documented `ButtonVariant::Danger`
exception.

## Validation (exit states)

| Command | Exit |
|---|---|
| `effigy docs:lint` | 0 |
| `effigy docs:value-domain-drift` | 0 |
| `bun packages/svelte/preview/scripts/contract-value-domain-drift.ts` (direct) | 0 |
| `VALUE_DOMAIN_ENFORCE=1 …` (direct) | 1 (expected) |
| `git diff --check` | 0 |
| `git status --porcelain` | 5 changed paths, all in scope |

## Stop Conditions

None triggered. Props tables parsed with general rules and the documented
hybrid forms (`Name: "a" | "b"` restatement cells, `Name | "literal"`, array
wrapped literal unions); no per-component special-casing was required. Every
named type either resolved from docs or was recorded as an `unresolved-type`
finding (the card defines that as a finding class, not a stop); producing the
inventory required no contract or implementation edits.

## Changed Files

```
docs/logs/2026-08/11-g13-007-value-domain-drift.md                (this log)
docs/roadmaps/g13/value-domain-drift-inventory.md
packages/svelte/preview/scripts/contract-value-domain-drift.ts
PAPERCUTS.md
tasks/effigy.tasks.toml
```

No other path changed. Not verifiable here: nothing outside this batch's scope
was exercised.
