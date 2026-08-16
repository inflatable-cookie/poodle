# g15.001 — Release-baseline roster inventory

Status: complete — PR #24 (review repairs in PR #24, second round)
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
the tree. The full reproducible count method for every surface lives in
`docs/roadmaps/g15/release-baseline-roster.md#count-method`; the surface list
below is the same measurement:

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
  audio family (`audio.rs`), `bx.rs` (Box), `shell_status_bar.rs` (StatusBar),
  and `time_field.rs` (TimeInput) accepted as naming discrepancies
- GPUI specimens from `packages/gpui/preview/src/specimens/` plus the batched
  `audio_controls.rs` (which has no `meter_surface` function; MeterSurface is
  not-applicable per spec 068); headless regression coverage from
  `tests/headless_regressions.rs` (Button, RangeSlider, Popover; two
  infrastructure proofs excluded)
- packed-install from `test/package-install/web-preview.ts` mounted-proof
  lists
- downstream use from read-only import scans (single- and multi-line
  statements resolved) across **all 16 canonical consumers** under
  `~/Dev/projects`: acowtancy, bovine-accelerator-desktop, compli-me,
  composer, contact-patch, figmatic, finch, longhorn, loophole,
  loophole-legacy, nucleus, songsprout, soundcheck, soundcheck-library,
  underlay, underlay-reference. Excluded: `poodle` (source repo), `jetstream`
  (program-deferred), worktree/absorbed duplicates (`soundcheck-wt`,
  `acowtancy/dairy-card011-worktree`), vendored/build/generated/fixture/
  example/archive/test paths. No canonical consumer imports `poodle-react`.

## Denominator and Key Counts

present / missing / not-applicable per surface:

| Surface | present | missing | not-applicable |
| --- | ---: | ---: | ---: |
| Denominator (full Svelte roster) | 175 | — | — |
| Contract / implementation / export / Svelte specimen | 175 / 175 / 175 / 175 | 0 | 0 |
| Focused Svelte test evidence | 61 | 114 | 0 |
| React implementation+export / gallery / focused test | 173 / 169 / 58 | 2 / 6 / 117 | 0 |
| Rust declaration | 163 | 11 | 1 |
| Rust render | 161 | 13 | 1 |
| GPUI specimen | 145 | 29 | 1 |
| Packed-install mounted proof (Svelte) | 9 | 166 (not exercised) | 0 |
| Downstream consumer use (16 canonical consumers) | 110 | 65 (no use found) | 0 |
| Jetstream | 0 (program-deferred) | — | — |

`not-applicable` is exactly `MeterSurface` on the Rust declaration, Rust
render, and GPUI axes (web-only by fixed decision, spec 068). GPUI count
correction vs the first round: `audio_controls.rs` covers 12 audio widgets and
does **not** cover MeterSurface, so GPUI present is 145, not 146.

## Gaps by Surface

| Surface | Gap count | Owner tranches |
| --- | ---: | --- |
| Svelte focused evidence | 114 | `g15.002`–`g15.005` |
| React implementation/export | 2 (AgentPlan, AgentPlanRecord) | `g15.006` |
| React gallery | 6 | `g15.006` |
| React focused tests | paired into `g15.002`–`g15.005`; scoped cases in `g15.006` | `g15.002`–`g15.006` |
| Rust declaration | 11 (+1 not-applicable) | `g15.007`–`g15.009` |
| Rust render | 13 (+1 not-applicable) | `g15.007`–`g15.009` |
| GPUI specimen | 29 (+1 not-applicable) | `g15.007`–`g15.010` |
| Release gate (`bun audit`) | 1 security advisory | `g15.014` |

Not gaps: Jetstream (program-deferred), 65 components without a found consumer
(absence is not a release failure), `effigy doctor` baseline findings.

## Generated Runway

Thirteen follow-on cards compiled from the measured register, listed in
dependency order in `docs/roadmaps/g15/README.md`:

- `g15.002`–`g15.005` — Svelte focused evidence by family with paired React
  evidence (the Svelte release blocker class; **`g15.002` is the proposed
  first executable tranche**)
- `g15.006` — React mirror implementation & gallery closure (2 implementations,
  6 gallery pages, scoped evidence)
- `g15.007` — Licence family native completion (carries `g14.017`)
- `g15.008` — Model-connection family native completion (carries `g14.020`)
- `g15.009` — Update, settings, Radio & context-provider native closure
- `g15.010` — Display, workstation & agent GPUI specimens
- `g15.011` — Human-centred specimen catalogue audit (carries `g14.026`)
- `g15.012` — Primitive-first visual conformance lane (per the `g14.022`
  decision; headless capture required)
- `g15.013` — v0.2.0 release certification (requires green `effigy qa` and an
  operator gate)
- `g15.014` — Release-gate remediation: nanoid advisory prerequisite

None dispatched or implemented. No roadmap status line or `dispatch.md` was
changed; the g15 README runway records dependency order only.

## Validation

| Command | Result |
| --- | --- |
| inventory consistency check (175 = export set; roster ↔ register lists exact; present+missing+NA = 175 per surface) | pass |
| `effigy docs:check` | pass |
| `effigy check:svelte` | pass |
| `effigy react:build` | pass |
| `effigy test:components` | pass |
| `effigy test:web-pack-install` | pass |
| `effigy qa` (headless local release board) | all lanes pass except the pre-existing `bun audit` nanoid advisory through the React preview's Vite dependency (GHSA-2v37-7h3g-55p8) — recorded, not fixed; owned by `g15.014` |
| `effigy doctor` | baseline findings recorded without fixing: generated-in-src, god-file, stale-suppression, comment-ratio warning |
| `git diff --check` | pass |

No `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or Jetstream
selector ran.

## Review Repairs (second round)

Orchestrator findings on the first round and how this log, the roster, the
register, and the runway were repaired:

1. **GPUI count inconsistency** — corrected to present 145 / missing 29 /
   not-applicable 1 (MeterSurface) with the reproducible count method in the
   roster. All surfaces now report present/missing/not-applicable separately
   and sum to 175.
2. **Downstream sampling** — replaced the 5-consumer sample with full
   discovery across all 16 canonical consumers (worktree duplicates and
   deferred Jetstream excluded); downstream use rose from 93 to 110
   components; 65 have no use found. No `poodle-react` imports exist anywhere
   in the canonical consumer set.
3. **Turnkey evidence cards** — `g15.002`–`g15.005` now carry a minimum
   evidence threshold (load-bearing observable contract behaviour, not
   anatomy), 3–4 named execution batches each, bounded contract-first fixes,
   a stop condition for public-contract ambiguity, and roster/register +
   batch-log writable scope. React evidence is paired into the same batches so
   `g15.006` shrinks to implementations and gallery pages with the right
   dependency shape.
4. **Visual lane** — `g15.012` rewritten to the operator-approved objective
   (`g14.022` decision lane): bounded variant inventory, same named fixtures
   per runtime, renderer-aware geometry/token/pixel comparison with
   antialiasing tolerance; fixtures never become an API/behaviour authority or
   universal representation; headless capture that cannot steal desktop focus
   is required, and the first batch stops with a finding if GPUI cannot
   provide one.
5. **Certification gate** — `g15.013` now requires a fully green `effigy qa`
   (no waived security gates), depends on the new remediation card `g15.014`
   (nanoid advisory), and carries an explicit operator gate before any
   tag/publish/release mutation.

## Uncertainties and Judgment Calls

- **Focused-evidence threshold**: the anatomy smoke generates one named mount
  case per component; it was counted as board health, not focused evidence.
  The 61/114 split is a posture judgment, not a code fact — a component in the
  114 may have a case inside a family test that imports it transitively.
  The register names the split for review; the evidence cards now define the
  minimum threshold workers must meet.
- **Rust naming discrepancies accepted**: `CallOutSpec`, `ShellStatusBarSpec`,
  and `TimeFieldSpec` are documented in their declaration headers as renames;
  they were counted as present. `bx.rs` (Box), the batched `audio.rs` /
  `audio_controls.rs` (12 audio widgets, no standalone files),
  `shell_status_bar.rs`, and `time_field.rs` were counted the same way.
- **IconProvider/UiPresentationProvider render posture**: no render module
  exists; their contracts document native host ownership. The register leaves
  them to `g15.009` as implement-passthrough-or-declare, rather than
  pre-judging them as not-applicable.
- **MeterSurface**: treated as not-applicable for Rust/GPUI per the fixed
  decision in spec 068, and web-only in the canonical catalogue.
- **Downstream direction**: consumer scans are read-only contextual evidence;
  a component with no found use was not marked as a failure. Consumer import
  lists include type-only names that resolve to the `types` block; those are
  excluded from component usage counts.
- **Pack-install**: the standing proof covers 9 Svelte components; extending
  the mounted proof is folded into `g15.013`, not treated as a per-component
  blocker.

## Change Footprint

Docs only: `release-baseline-roster.md`, `release-gap-register.md`, thirteen
new roadmap cards, the g15 README runway, this log. No component API, runtime
code, specimen, package export, workflow, or downstream repository changed.
