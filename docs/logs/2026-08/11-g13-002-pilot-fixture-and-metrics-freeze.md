# 11 — g13.002 Pilot Fixture And Metrics Freeze (batch log)

Branch: `thread/g13-002-pilot-fixture-metrics` (worktree, pushed with
`git push -u origin thread/g13-002-pilot-fixture-metrics`)
Date: 2026-08-11
Card: `docs/roadmaps/g13/batch-cards/002-pilot-fixture-and-metrics-freeze.md`

## 1. Bootstrap and baseline gates (step 1)

| Command | Exit | Notes |
|---|---|---|
| `bun install` | 0 | 234 packages installed |
| `effigy svelte:surface-audit` | 0 | 163 exports, 0 coverage gaps |
| `effigy docs:lint` | 0 | validated 170 contracts, 12 parity targets, … |
| `git diff --check` | 0 | clean |

`main` was green at merge `bb3f79ef`; all gates green on the clean checkout —
no stop triggered.

## 2. Fixture freeze (step 2)

Manifest written to `docs/roadmaps/g13/pilot-baseline-manifest.md`:

- `FIX-BTN-01`…`FIX-BTN-08` (8 rows; `FIX-BTN-08` blocked on `UNKNOWN-02`)
- `FIX-RNG-01`…`FIX-RNG-06` (6 rows; `FIX-RNG-04/05` blocked on `UNKNOWN-01`)
- `FIX-TXT-01`…`FIX-TXT-04` (4 rows)
- `FIX-SHELL-01`…`FIX-SHELL-10` (10 rows, covering `SHELL-01`–`SHELL-10`)
- `FIX-AXIS-01`…`FIX-AXIS-04` (4 rows, one per runtime axis mechanism;
  Jetstream row records `absent` shared `SpecimenLayout` helper per inventory §6)

Total 32 fixtures, every one bound to an existing surface and at least one
corpus requirement ID. `UNKNOWN-01`/`UNKNOWN-02` recorded as blocked fixtures
with no assumed answer. `GAP-01`–`GAP-07` each mapped to a fixture row or an
explicit no-fixture statement (manifest §3).

## 3. Quantitative baseline (step 3)

Machine: darwin 25.5.0, arm64, Apple M5 Max, 18 cores, macOS 26.5.2.

### Authored LOC per pilot component (inventory §5 files, `wc -l`)

Command shape: `wc -l <file>` per file.

- Button: contract 575 · `styles/button.css` 317 · `Button.svelte` 220 ·
  `Button.tsx` 164 · spec `button.rs` 273 · render `button.rs` 621 ·
  GPUI specimen 522 · Jetstream specimen 245
- RangeSlider: contract 549 · `slider.ts` 344 + `styles/range-slider.css` 227 ·
  `RangeSlider.svelte` 176 · `RangeSlider.tsx` 185 · spec `range_slider.rs` 162 ·
  headless `slider.rs` 607 · render `range_slider.rs` 497 · GPUI specimen 387 ·
  Jetstream specimen 192
- TextInput: contract 697 · `input.ts` 76 + `styles/text-input.css` 252 ·
  `TextInput.svelte` 617 · `TextInput.tsx` 530 · spec `text_input.rs` 459 ·
  headless `text_input.rs` 837 · render `text_input.rs` 625 · GPUI specimen 459 ·
  Jetstream specimen 267

### Authored LOC surface totals (inventory §1 globs)

| Glob | Files | LOC |
|---|---|---|
| `docs/contracts/components/*.md` | 171 | 53,645 |
| `packages/svelte/components/src/*.svelte` | 164 | 28,835 |
| `packages/react/components/src/*.tsx` | 165 | 26,178 |
| `packages/core/src/*.ts` | 41 | 6,642 |
| `packages/core/src/styles/*.css` | 159 | 21,532 |
| `packages/contracts/components/src/*.rs` | 157 | 33,611 |
| `packages/contracts/headless/src/*.rs` | 24 | 6,864 |
| `packages/render/src/*.rs` | 160 | 40,797 |
| `packages/gpui/preview/src/specimens/*.rs` | 146 | 32,299 |
| `packages/jetstream/preview/src/specimens/*.rs` | 151 | 24,231 |
| `packages/gpui/adapter/src/render_*.rs` | 9 | 2,831 |
| `packages/jetstream/adapter/src/render_*.rs` | 7 | 3,328 |
| `packages/svelte/preview/src/specimens/*` | 162 | 15,155 |
| `packages/react/preview/src/gallery/specimens/*` | 156 | 13,339 |
| `packages/svelte/preview/src/component-registry.ts` | 1 | 278 |
| `docs/parity/*.md` | 141 | 9,592 |

(Counts include files the glob matches; inventory §1 excludes README/lib
files — see manifest §4.2 notes.)

### Generated LOC

| Glob | Files | LOC |
|---|---|---|
| `packages/svelte/preview/artifacts/*.json` | 4 | 19,197 |
| `packages/react/preview/artifacts/*.json` | 3 | 16,771 |
| `packages/core/src/tokens/generated/**/*` | 25 | 1,813 |
| `packages/core/src/icons/generated.ts` | 1 | 309 |
| `packages/tokens/artifacts/**/*` | 31 | 2,407 |

Measured with bash `globstar`; the card's literal globs
`packages/core/src/tokens/generated/*` and `packages/tokens/artifacts/**`
match only directories without it (0 lines) — papercut recorded.

### Duplicated definition count

4 per component (Svelte shell + React shell + poodle-specs + poodle-render),
paths in manifest §4.4 (inventory §8 item 1).

### Runtime extension count

Row-level `EXT` marks in the corpus: Button 3 (BTN-26/27/29), RangeSlider 2
(RNG-26/27), TextInput 1 (TXT-31), total 6. Corpus §8 counts table states
5/3/2 = 10 — discrepancy recorded, not resolved (manifest §4.5).

### Clean build time (cold cache, 1 run each)

| Build | real | user | sys | exit |
|---|---|---|---|---|
| `cargo build -p poodle-specs --manifest-path packages/contracts/components/Cargo.toml` | 2.13 s | 3.00 s | 0.55 s | 0 |
| `cargo build -p poodle-render --manifest-path packages/render/Cargo.toml` | 3.18 s | 6.85 s | 1.18 s | 0 |
| `bun run --cwd packages/svelte/preview build` | 2.13 s | 3.02 s | 0.46 s | 0 |

`cargo clean --manifest-path …` run before each build; logs show full
recompilation (cold). Sibling jetstream checkout present at `../../jetstream`
(no Jetstream build required by the measure set).

### Diagnostic quality

Six gate failure-message constructions quoted verbatim from source with
file:line in manifest §4.7 (`contract-prop-drift.ts:139-141/161-162/172`,
`contract-spec-drift.ts:308-312/320/326`, `contract-role-drift.ts:98-99/264-271`,
`adapter-manifest-drift.ts:70-73/82`, `surface-audit.ts:116-121`,
`lint-docs.ts:3263-3266/3271-3274/3281-3283`). No failures induced.

### Four-runtime drift count

Manifest §4.8: Button 14, RangeSlider 16, TextInput 13 (status-line basis) or
26 (enumerated-bullet basis for text-input). Sources reported separately:
parity docs (opens + accepted deltas), GPUI report 0 (no pilot entries —
deltaRegister is suite-scoped), Jetstream report 0 (no pilot mentions),
corpus EXT + GAP rows.

## 4. Validation and commit (step 4)

| Command | Exit | Notes |
|---|---|---|
| `effigy docs:lint` | 0 | — |
| `effigy docs:check` | 0 | full gate: drift:recipes, surface-audit, docs:lint, react:docs, report:parity, report:accessibility, docs:build |
| `git checkout -- packages/tokens/artifacts/rust/` | 0 | restored the docs:check rewrite (card ruling 6); nothing from that directory committed |
| `git diff --check` | 0 | — |
| `git status --porcelain` | — | only `?? docs/roadmaps/g13/pilot-baseline-manifest.md` before log/PAPERCUTS were written |

Final changed paths (must be exactly the three writable):

```
 PAPERCUTS.md                                       |  25 ++
 docs/logs/2026-08/11-g13-002-pilot-fixture-and-metrics-freeze.md | 167 +++++++++++
 docs/roadmaps/g13/pilot-baseline-manifest.md       | 316 +++++++++++++++++++++
 3 files changed, 508 insertions(+)
```
(from `git diff --cached --stat` before commit; only the three writable paths
changed)

## 5. Findings (recorded, not resolved)

- Corpus §8 EXT counts (10) ≠ row-level EXT marks (6).
- Jetstream RangeSlider densities specimen uses the standard variant, not the
  embedded bipolar the contract §13 / `RNG-25` describes.
- Card's generated-LOC globs silently measure 0 without `globstar`.
- `docs/parity/text-input.md` status line (`gpui=2 jetstream=2`) lags its own
  enumerated open bullets (8 GPUI / 9 Jetstream) — historical-file count drift
  (cf. `OBS-04`).

None triggered a stop condition (no public-semantics disagreement beyond the
accepted `OBS-*` rows; no required evidence surface missing; no measurement
required changing source or refreshing a baseline; no schema/codegen/package
work).
