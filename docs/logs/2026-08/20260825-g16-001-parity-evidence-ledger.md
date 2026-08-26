# g16.001 — Active-Cohort Parity Evidence Ledger

Date: 2026-08-25
Status: complete; operator review pending
Branch: `t3code/parity-evidence-ledger`

## Outcome

The worker replaced contradictory current parity claims with one checked-in
ledger at `docs/roadmaps/g16/parity-evidence-ledger.md`. The ledger derives
the public Svelte roster from `packages/svelte/components/src/index.ts`, the
portable catalogue from the generated preview authority, and checks every
component row plus its evidence references.

The live denominator remained 175 public components and 174 portable native
components. `MeterSurface` is the only approved native exclusion. Jetstream is
recorded once as program-level `deferred`.

## Inventory method

The checker derives the 175-row roster and 174-route native denominator, then
validates the evidence cells against live exports, registries, focused tests,
GPUI routes, named mounted regression tests, visual inventory paths, and
checked-in artifacts. Summary counts are computed from rows. Planted missing,
duplicate, extra, unresolved-path, and unresolved-fragment cases are covered
by focused checker tests.

Measured closeout:

- Svelte and React surface: 175 focused rows each.
- Shared Rust: 174 present rows; `MeterSurface` not applicable.
- GPUI construction: 174 focused routes; `MeterSurface` not applicable.
- GPUI mounted behaviour: 29 rows mounted across 33 named tests; 145 rows
  missing and one not applicable.
- Accessibility: 175 Svelte axe rows; React axe missing; 174 GPUI rows manual.
- Visual: one compared Button row for web and GPUI, with 18 accepted fixtures;
  five web manual skips and 173 non-Button GPUI comparison gaps remain.

## Artifact dispositions

- GPUI cross-runtime report: replaced the stale g09.018/96-component report
  with the g16.001 compact posture and 174-route construction denominator.
- Native accessibility proof: retained its layer/section evidence while
  adding the current g16.001 denominator, manual posture, mounted boundary,
  and missing assistive-technology proof.
- Jetstream report: replaced the stale g10.014/117-component result with a
  program-deferred report that preserves maintained shared Rust and adapter
  surfaces.
- Svelte and React parity report generators: embed the current compact GPUI
  report without borrowing Svelte accessibility evidence into React.
- Planning references: marked the old GPUI offscreen fork path superseded and
  aligned the Longhorn note with crates.io GPUI's non-activating windowed
  diagnostic boundary.

## Validation

Passed:

- `effigy test:parity-evidence-ledger` — 4 tests.
- `effigy check:parity-evidence-ledger`, `effigy report:parity`,
  `effigy report:accessibility`, `effigy docs:lint`, and `effigy docs:check`.
- `effigy probe:gpui-specimens`, `effigy regressions:native` — 70 tests,
  `effigy ci:web`, `effigy ci:native`, and one `effigy qa` board.
- `git diff --check`.

`effigy test:visual-fixtures` then passed after a bounded authority-path test
repair: the TypeScript test now reads and sanctions the live
`packages/gpui/preview/src/bin/window_capture/inventory.rs` loader. The fixture
inventory and thresholds were unchanged.

## Unresolved evidence gaps

The ledger does not claim broad GPUI mounted behaviour, native accessibility,
React axe coverage, or all-component visual parity. GPUI pixels remain outside
default QA/CI and require the operator-approved non-activating windowed
diagnostic. No `g16.002` was compiled; the operator chooses the next evidence
class after reviewing this ledger.
