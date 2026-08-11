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

## Addendum — `g13-b002` dispatch, review, and merge (same day)

`g13-b002` was dispatched after the closeout above and completed within the
session. Reviewed and merged here rather than in a separate log, since its
findings amend `g13-b005`.

| Batch | Card | Branch | Commit | Merge | Verdict |
|---|---|---|---|---|---|
| `g13-b002` | `g13/batch-cards/002-pilot-fixture-and-metrics-freeze.md` | `thread/g13-002-pilot-fixture-metrics` | `89debbcb` | `2368f436` | accepted, merged |

Dispatch: `deepseek-v4-flash` `xhigh`, isolated worktree, `--max-time 90m`,
detached `omp -p` with an event-driven pid watcher — no polling model monitor.
Merged with no conflicts. Gate on merged `main`: `effigy docs:lint`,
`effigy svelte:surface-audit`, `git diff --check`, `git status --porcelain` all
exit 0.

### Independent verification

| Claim | Verdict |
|---|---|
| 32 fixtures (`FIX-BTN` 8, `FIX-RNG` 6, `FIX-TXT` 4, `FIX-SHELL` 10, `FIX-AXIS` 4) | confirmed — 32 rows |
| Per-file authored LOC | confirmed — `Button.svelte` 220, `Button.tsx` 164, `render/src/button.rs` 621, `headless/src/text_input.rs` 837, `core/src/input.ts` 76 all exact |
| Glob totals | confirmed — `packages/svelte/components/src/*.svelte` = 28,835 exact |
| Verbatim drift-gate quotes | confirmed — `surface-audit.ts:116-121` matches the quoted source exactly |
| Jetstream RangeSlider densities finding | confirmed — `range_slider.rs:139` builds `RangeSliderSpec::new(25.0, 75.0)` with no `with_embedded_control` |
| `PAPERCUTS.md` restored additively after the worker's self-caught clobber | confirmed — diff against `main` shows four additions and zero removals |
| `UNKNOWN-01`/`UNKNOWN-02` left open | confirmed — `FIX-RNG-04`, `FIX-RNG-05` `blocked:UNKNOWN-01`; `FIX-BTN-08` `blocked:UNKNOWN-02`; no answer assumed |
| No package, schema, generator, or baseline refresh | confirmed — only the three writable paths changed |

### Correction to the `g13-b005` review above

`g13-b002` found that the corpus §8 classification table did not match the
corpus's own row marks. It is right, and the defect is wider than it reported.
The `b005` review recorded above verified requirement counts, ID uniqueness,
register counts, the two unknowns, and the absence of a schema recommendation —
it did **not** verify the classification column totals, and that gap let the
error through the merge gate.

Audited across all five columns, row marks versus the merged §8 table:

| Column | §8 as merged | Row marks | Status |
|---|---|---|---|
| SDD | 80 | 94 | wrong |
| GTA | 2 | 2 | correct |
| AC | 22 | 14 | wrong |
| CV | 39 | 28 | wrong |
| EXT | 10 | 6 | wrong |

The requirement counts themselves (129; `CROSS` 21, `BTN` 29, `RNG` 29, `TXT`
32, `SHELL` 10, `NEG` 8) were verified correct at merge and remain correct.

**Ruling: row-level marks win.** Each row carries its own contract citation and
evidence path; §8 was derived arithmetic. The corpus §8 table has been
recomputed from the rows and carries an amendment note preserving the
superseded figures. The baseline runtime-extension count is **6** — `BTN-26`,
`BTN-27`, `BTN-29`, `RNG-26`, `RNG-27`, `TXT-31`. Manifest §4.5 records the
same ruling. The `b005` worker's batch log is left unedited as historical
worker evidence.

### Other b002 findings

1. **Jetstream RangeSlider densities specimen renders the standard variant**
   where contract §13 / `RNG-25` specify embedded bipolar (Svelte and React
   comply). Accepted as a real cross-runtime specimen divergence; recorded as a
   papercut, not fixed here — it is specimen source work outside `g13.001`.
2. **Generated-LOC globs measure 0 without bash `globstar`.** Accepted; a
   measurement trap in the card I wrote, now a papercut. The manifest records
   the correct recursive counts.
3. **`docs/parity/text-input.md` status line (`gpui=2 jetstream=2`) lags its
   own enumerated bullets (8 GPUI / 9 Jetstream).** Accepted as historical-file
   drift, distinct from `OBS-04`'s stale paths. `docs/parity/` stays
   non-authoritative; not repaired.

No b002 claim was rejected. The manifest correctly reports the four-runtime
drift count with both the status-line and enumerated-bullet bases rather than
silently picking one.

## Not done

No `g13.002` schema implementation. No IR crate or package. No family
migration — that stays gated on the `g13.008` adopt verdict. `g13-b003` is
unblocked on dependencies but waits on two maintainer rulings (`UNKNOWN-01`,
`UNKNOWN-02`) plus the crate-placement decision.
