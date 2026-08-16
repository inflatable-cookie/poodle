# g14.022 — Generation closeout

Status: delivered — pending orchestrator review
Date: 2026-08-16
Card: `docs/roadmaps/g14/022-generation-closeout.md`
Governing refs: `docs/roadmaps/g14/008-pilot-verdict.md`,
`docs/roadmaps/g14/conformance-estate.md`,
`docs/logs/2026-08/16-g14-021-experimental-cleanup-and-gate-consolidation.md`,
`docs/contracts/001-working-rules.md`

## Result

g14 closes as a rejected architecture pilot that still improved components,
native substrate behaviour, headless testing, and specimen ownership. The
closeout delivered:

- every live g14 status resolved: `017` and `020` are superseded execution
  plans with approved web references preserved; `026` carries forward into the
  next runway with its rubric intact; `022` itself is delivered pending review
- a next-generation front door (`docs/roadmaps/g15/README.md`) and exactly one
  release-baseline roster inventory card (`g15.001`), blocked pending this
  closeout's merge
- a stale-reference sweep that left standing surfaces consistent with the
  post-reject system
- a CI recommendation for the stale `ci-conformance.yml` workflow, recorded
  for operator action (workflow unchanged here)

No component API, runtime code, curated specimen, package export, workflow, or
downstream repository changed. No windowed, native-visual, or Jetstream
validation ran.

## Evidence

### Pilot cost (from `g14.008`)

| Measure | Value |
| --- | ---: |
| Mechanism source LOC (corrected cost inventory) | 22,746 |
| Replaced hand-written source | 472 |
| Corpora cases (six) | 104 |
| HistoryCenter comparator correction | 1,205 cross-runtime differences |
| Admission rules failed | 4 of 5, including the cost stop condition |

### Cleanup delta (`g14.021`, merge `59d93d2f` against `52039d6f`)

Raw total: 164 files, +2,705 / -34,285. Split honestly — generated deletion is
not implementation cost saved:

| Surface | Added | Deleted |
| --- | ---: | ---: |
| Hand-written source, tests, and config (core, render, gpui, codegen source, Svelte/React, contracts, task/CI config) | +1,783 | -21,048 |
| Generated artifacts (codegen conformance fixtures JSON, generated Rust declarations) | 0 | -12,475 |
| Docs (verdict, cleanup log, retained-estate ledger) | +922 | -762 |
| **Total** | **+2,705** | **-34,285** |

The rejected-plane paths alone account for -27,950 deleted with zero additions:
`packages/core/src/conformance/**`, `test/conformance/**`,
`packages/core/scripts/conformance-*`, `packages/codegen/src/conformance*`,
`packages/codegen/fixtures/conformance/**`, generated Rust declarations, and
the GPUI corpus adapters. The six retained headless regressions replace the
pilot's completion gate; they are 6 `#[test]` cases in
`packages/gpui/preview/tests/headless_regressions.rs`, run by
`effigy regressions:native` (~0.05s each, in-memory, no window).

### Retained estate confirmed live

- `packages/gpui/preview/src/headless_driver.rs` — in-memory test platform
- `packages/gpui/preview/tests/headless_regressions.rs` — six regressions
- hand-written Rust declarations in `poodle_specs`
- every pilot-caught defect has a named owner in the `g14.021` ledger; the
  retained web/render tests those owners point to exist in the tree
- `effigy test:native-visual` — local pixel compare/refresh with
  `--control-size` (windowed; recorded, not run)
- web snapshot tools and the Svelte↔React visual tiers under `test/visual/`
- `effigy ci:conformance` — retained alias for `regressions:native`

### Audit findings

- The rejected pilot plane is absent from source, package exports, and task
  selectors. Historical surfaces (architecture 009, spec 066, delivery logs,
  `g14.001`–`007` cards) remain readable and explicitly rejected.
- Standing docs were already consistent after `g14.021`; the sweep found one
  stale front-door claim (`docs/specs/README.md` still pointed specimen work
  at `g14.026` as current) and repaired it to the carry-forward.
- The g13 machine-vector codegen (`packages/codegen/src/targets/conformance.rs`)
  is untouched — different mechanism, same word.

## Residual Gaps

These remain open after closeout and are explicitly not closed:

- native/runtime completion for the approved Licence (`g14.017` requirements)
  and model-connection (`g14.020` requirements) web suites
- the human-centred specimen catalogue audit (`g14.026`) — rubric intact,
  unexecuted
- full Rust roster completion and an honest certified GPUI subset name
- a primitive-first visual conformance lane (seam recorded in
  `conformance-estate.md` and the g15 card; harness not designed)
- a v0.2.0 release baseline: the complete Svelte roster inventory and frozen
  denominator (`g15.001`, blocked pending this closeout)

## Known Limits

- The 1,205 HistoryCenter differences are not "fixed" — the comparator that
  produced them is deleted. The evidence stays in `g14.008`.
- Two of the six regressions are the platform's own infrastructure proofs
  (driver mounts, pointer reach); four pin pilot-caught product defects. The
  board does not and cannot claim cross-runtime parity.
- `effigy doctor` reports the known baseline: errors on generated-in-src,
  god-files, and stale-suppressions, warning on comment-ratio (15 ok). The
  graph-index baseline named in the g14.022 handoff did not appear in this
  run. Recorded as pre-existing; out of scope for this closeout.
- No windowed or Jetstream validation was possible in this lane by design;
  native visual evidence remains local-only.

## CI Workflow Recommendation

`.github/workflows/ci-conformance.yml` is stale and redundant:

- its name, header comment, path filters, cache key, job name, and step label
  all describe the removed pilot (filters reference deleted directories such
  as `packages/core/src/conformance/**` and `test/conformance/**`)
- its only command runs `effigy ci:conformance` = `regressions:native`, which
  `ci:native` already includes
- main has no branch protection, so no required check name depends on it

**Recommendation: delete the workflow.** If a PR-triggered native regression
gate is wanted later, wire `regressions:native` into `ci-native.yml` under an
honestly named job instead. Keep `effigy ci:conformance` as a compatibility
alias until the deletion lands, then retire it. Workflow mutation needs
explicit operator approval; nothing in `.github/workflows/` was changed here.

## Next-Program Question

After the release-baseline inventory exists and v0.2.0 ships from a frozen
Svelte roster, what execution method will complete native parity for the
approved Licence and model-connection components — and does the evidence
from the first inventory tranche justify reopening a shared native test
authority, or do focused owner-local regressions remain the durable shape?

## Validation

- `effigy qa` — headless local release board: all lanes pass except
  `bun audit`, which fails on a pre-existing high-severity nanoid advisory
  reached through the React preview's vite dependency
  (GHSA-2v37-7h3g-55p8). Dependency-level, unrelated to this closeout's
  docs-only change; recorded for a dependency-update lane.
- `effigy docs:check`
- `effigy regressions:native`
- `effigy doctor` — baseline recorded, not fixed
- `git diff --check`

No `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or Jetstream
selector ran.
