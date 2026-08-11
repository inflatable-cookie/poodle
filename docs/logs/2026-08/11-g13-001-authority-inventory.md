---
title: g13.001 — authority inventory and docs-baseline repair
status: complete
owner: Poodle core
updated: 2026-08-11
tags: [log, g13, ir, authority-inventory, docs-baseline]
---

## Scope

Executed `docs/roadmaps/g13/batch-cards/001-authority-inventory-and-docs-baseline.md`
on `thread/g13-001-authority-inventory`. Produced the authority inventory,
repaired the four named docs-baseline failures, and recorded pre-existing
findings. No component behavior, public API, contract, architecture, spec,
working-rule, roadmap, card-status, or Effigy-config change.

## Before: gate failures (baseline, exit states)

| Command | Exit | Failure |
|---|---|---|
| `effigy svelte:surface-audit` | 1 | `AgentSubagent (agent-subagent): usage docs` — 162/163 components covered |
| `effigy docs:lint` | 1 | contract index missing `components/keyboard.md`, `components/mod-matrix-grid.md`, `components/waveform-display.md`; preview coverage missing `Keyboard`, `ModMatrixGrid`, `WaveformDisplay`; `shared-demo-app-audit.json` `exportCount`/`previewedCount` mismatch for `@inflatable-cookie/poodle-svelte` |
| `effigy docs:check` | 1 | stops at `svelte:surface-audit` (above) |

Baseline measured counts: 175 component files, 163 public component exports,
162 fully covered components, 1 gap (AgentSubagent).

## Repairs (existing authority + generators only)

1. `docs/contracts/README.md` — added the 3 missing contract index entries
   (`keyboard`, `mod-matrix-grid`, `waveform-display`); contracts themselves
   already existed; `docs/contracts/components/README.md` was already current.
2. `packages/svelte/preview/src/parity.ts` — added `Keyboard`,
   `ModMatrixGrid`, `WaveformDisplay` to the catalog-hub
   `packageSurfaceCoverage` group (specimens and registry entries already
   existed: `KeyboardSpecimen.svelte`/`ModMatrixGridSpecimen.svelte`/
   `WaveformDisplaySpecimen.svelte` + `registry.ts` slugs + `component-registry.ts`
   entries + `component-docs.ts` usage entries).
3. `packages/svelte/preview/src/component-docs.ts` — added the missing
   `"agent-subagent"` usage-docs entry (props/events per contract
   `docs/contracts/components/agent-subagent.md` + shell
   `AgentSubagent.svelte`); contract, component-registry, and specimen
   coverage already existed.
4. Regenerated artifacts with their generators:
   - `bun run --cwd packages/svelte/preview parity:report` → exit 0,
     `packages/svelte/preview/artifacts/parity-report.json` (200 exports)
   - `bun run --cwd packages/svelte/preview docs:export-json` → exit 0,
     `packages/svelte/preview/artifacts/component-docs.json`
   - `bun run --cwd packages/react/preview docs:export` → exit 0,
     `packages/react/preview/artifacts/component-docs.json`
   - `bun run --cwd packages/react/preview parity:report` → exit 0 (after
     `bun install`, see Finding 2)
5. `packages/shared-demo-app-audit.json` — rolled the hand-maintained
   `exportCount`/`previewedCount` (186 → 200) to match the regenerated parity
   report, per the audit's own `artifactsSource` contract and the precedent in
   `docs/logs/2026-08/08-text-input-caret-selection-focus.md`.

Regeneration also reconciled two pre-existing stale spots in the same
generated artifacts (both from source, not hand-edits): `MessageCenter`
entered `parity.ts`/`component-docs.ts` in `45b4733a` without artifact
regeneration, and the Popover usage string lagged the source.

## After: gate output (exit states)

| Command | Exit | Result |
|---|---|---|
| `effigy svelte:surface-audit` | 0 | 163/163 components covered, 0 gaps |
| `effigy docs:lint` | 0 | validated 170 component contracts, 42 operator guides, 11 docs sections, 4 families, 12 parity targets, 12 accessibility targets; no errors |
| `effigy docs:check` (with `bun install` completed) | 0 | drift:recipes clean (2427 files, 0 Treatment refs); surface-audit 0 gaps; docs:lint clean; react docs export; both parity reports (200 exports); both accessibility reports (12 targets); `vite build` ✓ (967 modules) |
| `git diff --check` | 0 | see Deliverables |

Note: `effigy docs:check` runs `report:parity` → `tokens:build`, which rewrites
the committed `packages/tokens/artifacts/rust/*` with generator-formatted
output. Those files were restored after the gate; they are not part of this
batch (see Finding 1).

## Findings (pre-existing, out of repair scope)

1. **`audit:tokens` is red at HEAD.** `45caae82` ("Format agent subagent
   contracts and generated Rust tokens") rustfmt-formatted the committed
   `packages/tokens/artifacts/rust/*` (8-space override arrays) without
   updating `packages/tokens/scripts/build-tokens.ts`, which emits 4-space
   arrays. `build-tokens.ts --check` therefore fails on the base commit with
   any change absent. `docs:check`'s `tokens:build` step silently rewrites the
   artifacts, dirtying the worktree. Same class as the 2026-08-10 papercut
   (doctor); distinct trigger (`docs:check`). Recorded in `PAPERCUTS.md`.
2. **React `parity:report` failed module resolution until `bun install`.**
   `bun run --cwd packages/react/preview parity:report` could not resolve
   `@inflatable-cookie/poodle-core/tokens` from the shared
   `packages/svelte/preview/src/parity.ts` with no `node_modules` present
   (verified identical failure with this batch's changes stashed). `bun
   install` (the repo's own `bootstrap:deps` step) fixed it; `docs:check` now
   passes. Environment bootstrap, not a repo defect.
3. **Jetstream `component_registry.rs` claims to be generated from the Svelte
   registry but has no generator in this repo.** Header says "Keep in sync by
   re-deriving from that file — do not hand-edit entries"; no script exists.
   Recorded in the inventory (§3.3, §8), not repaired (out of scope).
4. **`docs/parity/*.md` (139 files) describe deleted native tiers** and are
   explicitly non-authoritative per `docs/parity/README.md`. Recorded in the
   inventory (§8), untouched.

## Deliverables

- `docs/roadmaps/g13/authority-inventory.md` — evidence table, measured counts,
  four-runtime file maps (contracts, machines, shells, specimens, registries,
  preview shells, reports), preview header/theme selector and size/density
  axis maps, direct Jetstream `RenderComponent<Spec>` + compat-layer
  enumeration, crate-placement evidence with comparison table (no
  recommendation).
- `docs/logs/2026-08/11-g13-001-authority-inventory.md` — this log.
- `PAPERCUTS.md` — one new entry (Finding 1).
- Repaired/regenerated files (below).

## Changed files

```
git diff --stat (final batch):
 PAPERCUTS.md                                       |   9 +
 docs/contracts/README.md                           |   3 +
 docs/logs/2026-08/11-g13-001-authority-inventory.md | 135 ++++++++
 docs/roadmaps/g13/authority-inventory.md           | 368 +++++++++++++++++++++
 packages/react/preview/artifacts/component-docs.json    | 196 ++++++++++-
 packages/react/preview/artifacts/parity-report.json     |  60 +++-
 packages/shared-demo-app-audit.json                |   4 +-
 packages/svelte/preview/artifacts/component-docs.json   | 244 +++++++++++++-
 packages/svelte/preview/artifacts/parity-report.json    |  60 +++-
 packages/svelte/preview/src/component-docs.ts      |  30 ++
 packages/svelte/preview/src/parity.ts              |   3 +
 11 files changed, 1100 insertions(+), 12 deletions(-)
```

Final changed-file list: `docs/roadmaps/g13/authority-inventory.md`,
`docs/logs/2026-08/11-g13-001-authority-inventory.md`, `PAPERCUTS.md`,
`docs/contracts/README.md`, `packages/svelte/preview/src/parity.ts`,
`packages/svelte/preview/src/component-docs.ts`,
`packages/{svelte,react}/preview/artifacts/parity-report.json`,
`packages/{svelte,react}/preview/artifacts/component-docs.json`,
`packages/shared-demo-app-audit.json`.

No file outside scope changed: no roadmap/card status files,
`docs/roadmaps/dispatch.md`, architecture, specs, working rules, Effigy
configuration, crates, packages, or component source were touched.

Not verifiable in this environment: GPUI/Jetstream visual and AX gates need a
window server / sibling jetstream checkout and were not run; their scope is
unchanged by this batch. `test:native-visual`, `test:jetstream-visual`,
`test:jetstream-ax`, `test:jetstream-a11y` are local-only by design
(`effigy.tasks.toml`).
