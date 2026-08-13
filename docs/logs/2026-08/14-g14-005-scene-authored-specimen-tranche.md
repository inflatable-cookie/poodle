# 14 — g14.005 Scene-authored Specimen Tranche One (batch log)

Branch: `thread/g14-005-scene-authored-specimen-tranche`
Date: 2026-08-13
Card: `docs/roadmaps/g14/batch-cards/005-scene-authored-specimen-tranche-one.md`
Milestone: `g14.003` — the tranche that decides the specimen lane

## 1. Baseline (step 1)

Fresh branch from `origin/main` (`dc685efd`, after g14-b002 merged). Green:
`effigy ir:build` / `ir:check` / `test:components` (1112) / `ci:web` /
`docs:lint` / `git diff --check`.

## 2. The schema amendment — R2 amended (the finding the tranche was built to make)

**R2 said "no scene schema extension."** The tranche's first discovery was
that the scene vocabulary cannot carry the five specimens' *grouped*
sections — `ComponentInstance` had no group concept, and the static tier
is precisely grouped sections (states rows, prop demos). The user (the
orchestrator) ruled: a schema gap is a finding to **fix**, not a boundary
to defend — spec 063 Scene IR lists "groups" as scene content, and the
implementation had simply never carried it.

- `ComponentInstance.group: Option<String>` added to `scenes.rs` — the
  specimen-section heading; consecutive instances in the same group render
  under one heading. `#[serde(default)]`, so existing fixtures round-trip
  unchanged.
- Validation: empty groups are a finding (new `FindingKind::EmptyString`).
- The boundary that *does* hold: no behaviour, no evaluator, no expression
  vocabulary — the fixtures bind literals and declared axes only.

## 3. The authored model and the measurement — the lane is real

`packages/codegen/src/models/display_specimens.rs` (699 lines): five
fixture component definitions (props only — typed so `ir:check` validates
every binding), 14 shared types (tones, sizes, densities, roles), and five
scenes carrying 59 grouped instances. Per-specimen cost vs the hand-written
surface it replaces (baseline §5 numbers, re-measured on this SHA):

| Component | Svelte | React | GPUI | Jetstream | Total | Scene share |
|---|---|---|---|---|---|---|
| Callout | 84 | 85 | 210 | 126 | **505** | 9 instances |
| Pill | 83 | 63 | 375 | 261 | **782** | 21 instances |
| Spinner | 78 | 88 | 187 | 140 | **493** | 13 instances |
| Avatar | 40 | 34 | 157 | 136 | **367** | 9 instances |
| EmptyState | 56 | 42 | 80 | 100 | **278** | 4 instances |
| **Total** | 341 | 312 | 1009 | 763 | **2,425** | **699 authored** |

**699 authored lines replace 2,425 hand-written lines across four runtimes
— 0.29×, against the b052 bar of ~8.9×.** The verdict on rolling out the
next family is data: the lane wins by ~30×. One definition replaces four
copies; the four copies demonstrably drifted (Callout: Svelte showed five
tones, Jetstream six — the fixture carries the contract's full six, and
the web gains `pending`).

The cost is front-loaded and reusable: the emitters and the four
interpreters are family-agnostic — the next tranche's marginal cost is the
model + fixtures only.

## 4. Emitters and wiring (R3)

- `specimen-ts` target: one file per scene plus `specimen-scenes.ts` (slug
  registry) into `packages/{svelte,react}/preview/src/generated/specimens/`.
- `specimen-rust` target: one self-contained `specimens.rs` (structs +
  `pub static SPECIMEN_SCENES`, lifetime-generic so the interpreters can
  synthesize matrix instances) into `packages/{gpui,jetstream}/preview/src/
  generated/specimens/`.
- **Orphan-sweep lesson:** both shell and specimen targets share the
  `generated/` top level, and write mode sweeps each target's root — the
  first `ir:build` deleted the shell artifacts. The specimen targets own a
  *nested* root (`generated/specimens/`), which is the documented
  convention for sibling targets. Recorded in `PAPERCUTS.md`.
- `--author-specimens` bin mode (generalized the author/check helpers from
  the shell-only shape), fixture `specimens-model.json`, and four new emit
  steps in `ir:build`/`ir:check`.

## 5. Web consumption

`SceneSpecimen.svelte` + `SceneSpecimen.tsx` render a scene from the
generated fixture: groups → `SpecimenGroup`s, instances → components with
typed props (`content` projects to children), Sizes/Densities tabs iterate
the scene's first instance over the axis values (the projection convention
the natives share). Registries map the five slugs to the renderer; both
`ComponentPage`s pass the slug through. The five hand-written specimen
files deleted from each web preview.

Verified live in both previews — identical group labels, instance counts,
and tones:

| Slug | Groups | Instances | Notes |
|---|---|---|---|
| callout | Tones, Message prop, Without title, Dismissible | 9 | 6 tones incl. pending |
| pill | Tones, Code font, Muted, Badge, Inherited typography, Custom accent | 22 | |
| spinner | Ring, CLI grid, Context tones | 13 | grid tone pattern intact |
| avatar | Initials, Tone and shape, Image | 9 | |
| empty-state | Neutral, Search, First run, Compact custom visual | 4 | |

## 6. Native consumption

`scene_specimen.rs` interpreters in both native previews read the generated
fixture via `#[path]` and build each runtime's specs (the jetstream
`js_*` compat path, the GPUI `node_compat` path); the GPUI side uses the
existing `specimen_layout` tabs. The five hand-written files deleted from
each native preview.

Census (Jetstream, `--json`): the five project their prior surfaces
(callout `Alert`, spinner `Status`, avatar `Image`, ...) — the fixture
renders the same a11y tree. `test:jetstream-a11y` audit: clean. GPUI
compiles (two pre-existing dead-code warnings on `node_compat` methods the
deleted specimens used; `node_compat` is outside the card's writable
paths).

## 7. Classifications (R2 boundary, applied)

- **Callout's dismissal demo** — interactive harness (state + callback);
  the fixture carries the static `dismissible` instance, matching the
  natives' existing static demo.
- **Callout's action button, EmptyState's actions/visual snippets** —
  composition beyond the flat instance tier; excluded from the fixture and
  recorded here. The web loses the in-callout action demo; the four
  runtimes keep identical content.
- **Group chrome** (rows, chips, inverse chips) — renderer-owned
  presentation, like the shell's label projection (b035 R4); the fixture
  owns the text.

## 8. The gate (deliverable)

Planted a byte in `callout-specimen.ts` → `ir:check` exits 1 naming
`callout-specimen.ts (content drift)` → restored → green.

## 9. Validation

| Command | Exit |
|---|---|
| `effigy ir:build` / `ir:check` | 0 / 0 |
| `effigy ci:rust` | 0 (codegen incl. the new target unit tests) |
| `effigy test:components` | 0 (79 files, 1112 tests) |
| `effigy ci:web` | 0 |
| `effigy docs:lint` | 0 |
| `git diff --check` | 0 |
| `cargo check` jetstream + gpui previews (branch clone, warm target) | 0 |
| census + `test:jetstream-a11y` audit | 0 / clean |

`ci:native` runs at review in the main checkout, per the card.

## 10. Acceptance

- [x] Same specimen renders in all four runtimes from one definition —
  proven live in both web previews, via the census/registry path for the
  natives.
- [x] Per-specimen cost measured and reported: **0.29×**, decisively under
  the 9× stop-condition bar. The lane rolls out.
- [x] No scene schema *creep* — one sanctioned addition (the missing
  `group` vocabulary), nothing behavioural; the schema change is recorded
  as the tranche's finding.
- [x] No hand-written specimen file for the five remains (20 files
  deleted: 5 × 4 runtimes).
- [x] `ir:build`/`ir:check`, `test:components`, `ci:web`, `docs:lint`,
  `git diff --check` exit 0.

## 11. Papercuts

One entry appended: the shared-`generated`-root orphan sweep lesson.
Otherwise the b002 environment (branch clone + sources-only jetstream copy
+ `CARGO_TARGET_DIR`) carried the native checks without modification.
