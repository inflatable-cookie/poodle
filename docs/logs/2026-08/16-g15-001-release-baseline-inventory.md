# g15.001 — Release-baseline roster inventory

Status: complete — PR #24
Date: 2026-08-16
Card: `docs/roadmaps/g15/001-release-baseline-roster-inventory.md`
Governing refs: `docs/roadmaps/g15/001-release-baseline-roster-inventory.md`,
`docs/roadmaps/g14/022-generation-closeout.md`,
`docs/roadmaps/g14/conformance-estate.md`,
`docs/contracts/001-working-rules.md`,
`docs/logs/2026-08/16-142919-g15-001-release-baseline-inventory-handoff.md`

## Method

The denominator was derived mechanically, not sampled: every
`export { default as <Name> } from "./<Name>.svelte"` in
`packages/svelte/components/src/index.ts` (175 matches) was checked one-to-one
against the component files (175/175 present) and against the packed package
`exports` map (`.` index, `./*.svelte` per-file wildcard, `./types`; `files`
ships `src`). Per-surface evidence was then measured by direct inspection of
the tree:

- contract docs vs kebab-case component names (`docs/contracts/components/`)
- Svelte preview specimens via the `specimenMap` in
  `packages/svelte/preview/src/specimens/registry.ts` (175 entries: 168
  dedicated files, 7 shared — 5 through the generated `SceneSpecimen`, 2
  composed inside hosting specimens)
- focused Svelte test evidence from component imports across all 66 files in
  `packages/svelte/components/test/` (harness `.svelte` files resolved as
  evidence for their test)
- React implementation/export from `packages/react/components/src/index.ts`,
  gallery from `packages/react/preview/src/gallery/specimen-map.ts`, tests
  from the React test directory
- Rust declarations from `pub struct <Name>Spec` across
  `packages/contracts/components/src` (recursive), with three documented
  naming discrepancies accepted (`CallOutSpec`, `ShellStatusBarSpec`,
  `TimeFieldSpec`)
- Rust render modules from `packages/render/src/lib.rs`, with the batched
  audio family (`audio.rs`) and `bx.rs` (Box) accepted as naming discrepancies
- GPUI specimens from `packages/gpui/preview/src/specimens/` plus the batched
  `audio_controls.rs`; headless regression coverage from
  `tests/headless_regressions.rs` (Button, RangeSlider, Popover; two
  infrastructure proofs excluded)
- packed-install from `test/package-install/web-preview.ts` mounted-proof
  lists
- downstream use from read-only import scans of the five known consumers
  (longhorn, underlay, soundcheck, soundcheck-library,
  bovine-accelerator-desktop)

## Denominator and Key Counts

| Measure | Count |
| --- | ---: |
| Denominator (full Svelte roster) | 175 |
| Contract / implementation / export / Svelte specimen | 175 / 175 / 175 / 175 |
| Focused Svelte test evidence | 61 |
| React implementation+export / gallery / focused test | 173 / 169 / 58 |
| Rust declaration / render | 163 / 161 |
| GPUI specimen | 146 |
| Packed-install mounted proof (Svelte) | 9 |
| Downstream consumer use (union of 5) | 93 |
| Jetstream | 0 (program-deferred) |

## Gaps by Surface

| Surface | Gap count | Owner tranches |
| --- | ---: | --- |
| Svelte focused evidence | 114 | `g15.002`–`g15.005` |
| React implementation/export | 2 (AgentPlan, AgentPlanRecord) | `g15.006` |
| React gallery | 6 | `g15.006` |
| React focused tests | mirror of the Svelte tranches | `g15.006` |
| Rust declaration | 12 (incl. MeterSurface not-applicable web-only) | `g15.007`–`g15.009` |
| Rust render | 14 (incl. MeterSurface not-applicable; IconProvider/UiPresentationProvider as declared-absence candidates) | `g15.007`–`g15.009` |
| GPUI specimen | 29 (incl. MeterSurface not-applicable) | `g15.007`–`g15.010` |

Not gaps: Jetstream (program-deferred), 82 components without a found consumer
(absence is not a release failure), `bun audit` nanoid advisory, `effigy
doctor` baseline findings.

## Generated Runway

Twelve follow-on cards compiled from the measured register, listed in
dependency order in `docs/roadmaps/g15/README.md`:

- `g15.002`–`g15.005` — Svelte focused evidence by family (the Svelte release
  blocker class; **`g15.002` is the proposed first executable tranche**)
- `g15.006` — React mirror closure
- `g15.007` — Licence family native completion (carries `g14.017`)
- `g15.008` — Model-connection family native completion (carries `g14.020`)
- `g15.009` — Update, settings, Radio & context-provider native closure
- `g15.010` — Display, workstation & agent GPUI specimens
- `g15.011` — Human-centred specimen catalogue audit (carries `g14.026`)
- `g15.012` — Primitive-first visual conformance lane (seam only)
- `g15.013` — v0.2.0 release certification

All blocked pending orchestrator review; none dispatched or implemented. No
roadmap status line or `dispatch.md` was changed.

## Validation

| Command | Result |
| --- | --- |
| inventory consistency check (175 = export set; roster ↔ register lists exact) | pass |
| `effigy docs:check` | pass |
| `effigy check:svelte` | pass |
| `effigy react:build` | pass |
| `effigy test:components` | pass |
| `effigy test:web-pack-install` | pass |
| `effigy qa` (headless local release board) | all lanes pass except the pre-existing `bun audit` nanoid advisory through the React preview's Vite dependency (GHSA-2v37-7h3g-55p8) — recorded, not fixed |
| `effigy doctor` | baseline findings recorded without fixing: generated-in-src, god-file, stale-suppression, comment-ratio warning |
| `git diff --check` | pass |

No `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or Jetstream
selector ran.

## Uncertainties and Judgment Calls

- **Focused-evidence threshold**: the anatomy smoke generates one named mount
  case per component; it was counted as board health, not focused evidence.
  The 61/114 split is a posture judgment, not a code fact — a component in the
  114 may have a case inside a family test that imports it transitively.
  The register names the split for review.
- **Rust naming discrepancies accepted**: `CallOutSpec`, `ShellStatusBarSpec`,
  and `TimeFieldSpec` are documented in their declaration headers as renames;
  they were counted as present. `bx.rs` (Box) and the batched `audio.rs`/
  `audio_controls.rs` (12 audio widgets, no standalone files) were counted the
  same way.
- **IconProvider/UiPresentationProvider render posture**: no render module
  exists; their contracts document native host ownership. The register leaves
  them to `g15.009` as implement-passthrough-or-declare, rather than
  pre-judging them as not-applicable.
- **MeterSurface**: treated as not-applicable for Rust/GPUI per the fixed
  decision in spec 068, and web-only in the canonical catalogue.
- **Downstream direction**: consumer scans are read-only contextual evidence;
  a component with no found use was not marked as a failure.
- **Pack-install**: the standing proof covers 9 Svelte components; extending
  the mounted proof is folded into `g15.013`, not treated as a per-component
  blocker.

## Change Footprint

Docs only: `release-baseline-roster.md`, `release-gap-register.md`, twelve new
roadmap cards, the g15 README runway, this log. No component API, runtime
code, specimen, package export, workflow, or downstream repository changed.
