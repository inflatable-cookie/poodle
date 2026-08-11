---
title: g13 — review and merge of batches 001 and 005
status: complete
owner: Poodle orchestrator
updated: 2026-08-11
tags: [log, g13, review, merge, workers, authority-inventory, corpus]
---

## Scope

Orchestrator review and merge of the two pushed-but-unmerged g13 worker
commits. No IR crate, schema, macro, compiler, or generator was created
(`IR-12` pause holds). No component behavior, public API, or contract changed.

| Batch | Card | Branch | Commit | Verdict |
|---|---|---|---|---|
| `g13-b001` | `g13/batch-cards/001-authority-inventory-and-docs-baseline.md` | `thread/g13-001-authority-inventory` | `251cc858` | accepted, merged |
| `g13-b005` | `g13/batch-cards/005-pilot-contract-expressiveness-corpus.md` | `thread/g13-pilot-expressiveness-corpus` | `2f8dc5db` | accepted, merged |

## Merge commands and exit states

Both branches started from ancestors of `main` (`535fcf22` and `c0e1eb01`), so
a three-way merge preserved current `main` roadmap and dispatch state; only
`PAPERCUTS.md` conflicted in each case.

| Command | Exit | Result |
|---|---|---|
| `git show --stat 251cc858` | 0 | 11 files, +1113/−12; scope matches the card |
| `git show --check 251cc858` | 0 | no whitespace defects |
| `git merge --no-ff thread/g13-001-authority-inventory` | conflict → resolved | `PAPERCUTS.md` only; merge commit `a0ca039d` |
| `git show --stat 2f8dc5db` | 0 | 3 files, +440; docs-only |
| `git show --check 2f8dc5db` | 0 | no whitespace defects |
| `git merge --no-ff thread/g13-pilot-expressiveness-corpus` | conflict → resolved | `PAPERCUTS.md` only; merge commit `bb3f79ef` |
| `git merge-base --is-ancestor 251cc858 main` | 0 | ancestry confirmed |
| `git merge-base --is-ancestor 2f8dc5db main` | 0 | ancestry confirmed |

### Conflict resolution

`PAPERCUTS.md` conflicted on both merges because each worker prepended a new
entry to the same position as `main`'s orchestration entry. Both resolutions
kept **all** entries; nothing was dropped or rewritten. The three `2026-08-11`
entries now on `main` are: the `docs:check` token-artifact rewrite (b001), the
React web-native prop omissions (b005), and the detached `omp -p` launch
failure (main).

## Consolidated validation gate (on merged `main`)

| Command | Exit | Result |
|---|---|---|
| `effigy svelte:surface-audit` | 0 | 163 public exports, 163 fully covered, 0 gaps |
| `effigy docs:lint` | 0 | 170 component contracts, 12 parity targets, 128 contract↔Svelte and 112 contract↔spec prop surfaces validated |
| `effigy docs:check` | 0 | full chain green incl. `vite build` (967 modules) |
| `git diff --check` | 0 | clean |
| `git status --porcelain` | 0 | clean |

The inherited `docs:lint` failure on `main` (missing `keyboard`,
`mod-matrix-grid`, `waveform-display` contract-index and preview-coverage
entries; stale shared-demo export/preview counts) is repaired by the merged
b001 commit, as predicted in the handoff.

`effigy docs:check` again rewrote `packages/tokens/artifacts/rust/*` via
`report:parity` → `tokens:build`. Those files were restored (`git checkout --`)
and are not part of either merge — the known papercut, unchanged in severity.

## Independent verification of b001 claims

Counts re-measured directly on `main`, not taken from the worker's log:

| Claim | Worker | Measured | Verdict |
|---|---|---|---|
| Component contracts | 170 | 170 | confirmed |
| Jetstream `impl RenderComponent<…Spec>` | 108 | 108 | confirmed |
| GPUI `impl RenderComponent<…Spec>` | 101 | 101 | confirmed |
| Jetstream `js_*` compat shims | 157 | 157 | confirmed |
| `compat.rs` / `nel.rs` / `node_compat.rs` lines | 1104 / 464 / 6470 | 1104 / 464 / 6470 | confirmed |
| Svelte / React component files | 164 / 165 | 164 / 165 | confirmed |
| `poodle-render` `pub fn` | 269 | 269 | confirmed |
| Historical parity audits | 139 | 141 files, minus `README.md` + `TEMPLATE.md` = 139 | confirmed |
| Jetstream registry claims generator derivation, none exists | claimed | header states "generated from … re-deriving from that file"; no generator script in repo | confirmed |

Generated-artifact integrity: `effigy docs:check` regenerated
`component-docs.json`, `parity-report.json`, and `accessibility-report.json`
for both web packages and produced **zero** diff against the merged tree. The
worker's committed artifacts are exactly generator output, not hand edits.

## Independent verification of b005 claims

| Claim | Verdict |
|---|---|
| 129 requirements (`CROSS` 21, `BTN` 29, `RNG` 29, `TXT` 32, `SHELL` 10, `NEG` 8) | confirmed — 129 table rows, 129 unique IDs, per-set counts exact |
| Registers: `UNKNOWN` 2, `OBS` 6, `GAP` 7 | confirmed |
| No schema/representation recommendation | confirmed — no Rust type, JSON shape, macro, compiler API, or crate placement appears; a recommendation-language sweep returned nothing |
| `UNKNOWN-01` is a real open question | confirmed — `range-slider.md` §6 forbids `aria-orientation` on the range inputs and is silent on the embedded `role="slider"` stops; `RangeSlider.svelte:173-174` emits it on both stops |
| `UNKNOWN-02` is a real open question | confirmed — `ButtonVariant::Danger` and `ButtonTone::Success` exist in `packages/contracts/components/src/types.rs`; `button.md` §3 unions are `primary\|secondary\|ghost` and `default\|danger\|warning` |
| `GAP-01` (no range/text conformance vectors) | confirmed — `vectors/machines.json` top-level keys are checkbox, popover, modal, hover, singleSelect, slider, menu, disclosure, toggleGroup, tabs |

## Rulings on worker findings

### b001

1. **`audit:tokens` red at HEAD; `docs:check` dirties token artifacts** —
   accepted as a pre-existing papercut, not a b001 regression. Not repaired in
   `g13.001`. Standing rule for later batches: restore
   `packages/tokens/artifacts/rust/*` after any `docs:check` run; never commit
   its rewrite as part of an IR batch.
2. **React `parity:report` needs `bun install` first** — accepted as
   environment bootstrap, not a repo defect. Later cards must run the repo's
   own dependency bootstrap before web generators.
3. **Jetstream `component_registry.rs` claims generator derivation with no
   generator** — accepted as recorded drift debt. Deliberately *not* repaired:
   writing that generator is a codegen decision reserved for `g13-b003`'s
   placement ruling and the `g13.002` schema card. Carried forward as an
   explicit input to `g13-b003`.
4. **`docs/parity/*.md` (139 files) describe deleted native tiers** — accepted
   as recorded. These remain non-authoritative per `docs/parity/README.md` and
   may be cited only as historical evidence, never as authority. Corpus
   `OBS-04` records the same staleness independently.

No b001 claim was rejected. The crate-placement section correctly stops at
evidence with no recommendation, as the card required.

### b005

1. **`UNKNOWN-01` (embedded RangeSlider `aria-orientation` contract scope)** —
   **left open, deliberately.** Not resolved during merge. This is a contract
   question under `IR-09`: resolving it means either amending
   `docs/contracts/components/range-slider.md` §6 to scope the embedded stops,
   or changing Svelte/React. Owner: maintainer, via `g13-b003`. No fixture in
   `g13-b002` may bless either reading.
2. **`UNKNOWN-02` (Rust `ButtonVariant::Danger` / `ButtonTone::Success` beyond
   the contract union)** — **left open, deliberately.** Same treatment: either
   the contract union widens or the Rust enums narrow. Owner: maintainer, via
   `g13-b003`. The pilot IR may not author either value until it is settled.
3. **Contradiction register empty** — accepted. `OBS-01`/`OBS-02` were
   correctly classified as mechanism/attribute nuance rather than public-
   semantic divergence; spot-checks agree.
4. **`OBS-03` (React omits `formenctype`/`formmethod`/`autocorrect`)** —
   accepted as a papercut, merged as such. Not fixed here: it is component
   source work outside `g13.001`.
5. **`GAP-01`–`GAP-07`** — accepted as named evidence gaps. They are inputs to
   `g13-b002`'s fixture freeze: a gap may be recorded as a baseline zero, but
   may not be closed by inventing evidence.

No b005 claim was rejected.

## Carried-forward inputs (not settled by this merge)

These pass to `g13-b002` (fixtures/metrics) and `g13-b003` (placement ruling):

- `UNKNOWN-01`, `UNKNOWN-02` — maintainer contract decisions, `g13-b003`.
- Jetstream registry with no generator — candidate codegen target, `g13-b003`.
- `GAP-01` missing range/text conformance vectors; `GAP-02` no executed native
  AT traces; `GAP-03` native vertical RangeSlider; `GAP-04` GPUI per-thumb
  focus; `GAP-05` Jetstream TextInput typing/key events; `GAP-06` contract-
  silent Button density values; `GAP-07` `truncate`/`fit`/`maxWidth` absent
  from `ButtonSpec` — all `g13-b002` baseline inputs.
- Crate placement remains unruled; `poodle-ir`/`poodle-codegen` do not exist.

## State updates in this closeout

- `g13-b001` and `g13-b005` set to `merged` in their cards, the batch-card
  index, and `docs/roadmaps/dispatch.md`.
- `g13.001` execution plan: `g13-b001` checked; milestone stays **in progress**
  pending `g13-b002` and `g13-b003`.
- `g13-b002` promoted to `ready` with worker rules, writable paths, commands,
  and stop conditions filled in.
- Worker worktrees removed after ancestry and cleanliness checks.

## Not done

No `g13.002` schema implementation. No IR crate or package. No family
migration — that stays gated on the `g13.008` adopt verdict.
