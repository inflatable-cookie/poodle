# 14 — g14.003 Frozen Baseline And Inventories (batch log)

Branch: `thread/g14-003-frozen-baseline-and-inventories`
Date: 2026-08-13
SHA: `847a8652`
Card: `docs/roadmaps/g14/batch-cards/003-frozen-baseline-and-inventories.md`
Milestone: `g14.002` (measurement only; parallel to `g14-b002`)

Reset per Thread Reuse Protocol: `git fetch origin --prune`, branch created
from `origin/main` (this worktree cannot check out `main`; it is locked in
the primary checkout).

## Measurements

All numbers live in `docs/roadmaps/g14/g14-baseline-manifest.md`. Commands
and exit states:

| Command | Exit | Notes |
|---|---|---|
| Native-gap `bun` script from `native-registration-gap.md` | 0 | GPUI 16 missing, Jetstream 17, union 18 |
| `bun packages/svelte/preview/scripts/capability-drift.ts` | 0 | 36 rows |
| `bun packages/svelte/preview/scripts/machine-shape-drift.ts` | 0 | 21 pinned; `rs:text_input` baselined |
| `bun packages/svelte/preview/scripts/contract-role-drift.ts` (main checkout, same SHA) | 1 | `range-slider` / `slider` — known-red for `g14-b002` |
| same script in this worktree | 1 | Cargo lockfile collision; no census (PAPERCUTS) |
| `git diff --check` | 0 | |

`effigy docs:contract-role-drift` is not a defined selector (finding). The
script is `drift:roles`.

## 053 vs this inventory

Machine modules, vectors, and harnesses are untouched by 053. Capability
declarations moved to headless (36 rows still). Native gap grew by two
(`UpdateCenter`, `UpdateStatus`) relative to the 2026-08-12 table. Slider
vector still 3 cases with no two-thumb vocabulary.

## Hole class

Behaviour divergence between the two machine implementations on events the
shared vector never fires. Named from the slider/menu re-count, not from the
card's example list. No surface gate compares TS vs Rust transition output.

## Findings (not patched)

- `range-slider` role → `g14-b002`
- Union 16→18 → `g14.008`
- Vector thinness → `g14.006`
- Four specimen copies + native filename aliases → `g14.003`
- Missing `docs:contract-role-drift` selector → orchestrator wording

## Writable paths

Manifest, this log, PAPERCUTS (one new entry: worktree Cargo lockfile
collision). Nothing executable changed.
