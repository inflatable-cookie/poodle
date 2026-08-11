---
title: g13 batch 025 — codegen remaining emitters
status: complete
milestone: g13.003
owner: Poodle core
updated: 2026-08-12
tags: [log, g13, IR, codegen, emission, drift-gating, json-schema, registry, conformance, docs-fragments, spec-063]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/025-codegen-remaining-emitters.md` on branch
`thread/g13-025-codegen-remaining-emitters`: added the four targets the
milestone names — JSON surface, JSON Schema, registry, conformance vectors,
docs fragments — to the `poodle-codegen` crate, registered them, extended
the synthetic fixture, and closed the milestone's artifact set. The shared
machinery (`emit.rs`, `check.rs`, `write.rs`, `error.rs`) is **unchanged**:
none of the five targets needed it, so the card's stop condition never
arose. `b022`'s rulings R1–R5 held; nothing needed re-deciding.

Per the card's worker rules: no sub-agents; sources read directly; targets
implemented one at a time, each with its tests, regenerating and committing
output as it landed (8 commits). No `git add -A` anywhere; every stage was
by explicit path. No merge, no push of other branches.

## Deliverables (only the scoped writes — every path under `packages/codegen/`)

- `packages/codegen/src/targets/json.rs` — **JSON surface** (`output_root`
  `json`): one document per component (`<id>.json`) with prop surface
  (tagged `kind` types and values, permitted subsets, defaults and
  expression defaults, required/web-only marks), shared-type references,
  events (payload + timing), axes (size/density/orientation), plus a stable
  id-sorted `index.json`.
- `packages/codegen/src/targets/schema.rs` — **JSON Schema** (`schema`):
  one hand-emitted draft 2020-12 `schema.json` validating both JSON-surface
  document kinds; `$defs` carry the closed vocabularies and the
  model-derived `component-id` / `shared-type-id` enums, so the schema
  moves with the model like every other declared artifact. No `schemars`
  (ruling R4 — its type-mirroring derive could not see instance facts
  anyway).
- `packages/codegen/src/targets/registry.rs` — **registry** (`registry`):
  one `registry.json` listing every component — id, name, capabilities
  required, axes supported, shared types referenced — the "what exists"
  index.
- `packages/codegen/src/targets/conformance.rs` — **conformance vectors**
  (`conformance`): one `vectors.json` — the corpus `CV` rows as a shared
  vector file in the repo's `machines.json` convention, with `declared_by`
  cross-references.
- `packages/codegen/src/targets/docs.rs` — **docs fragments** (`docs`): one
  `<id>.md` per component, a `### Public Props` table in the exact shape
  `contract-prop-drift.ts` parses (backticked prop names starting each
  row), includable in component contracts. No existing contract touched.
- `packages/codegen/src/targets/json_common.rs` — shared `generated`
  header object + pretty-JSON file builder for the four JSON targets
  (the `//` comment header is not valid JSON, so `IR-07` content rides as a
  JSON object field).
- `packages/codegen/src/targets/mod.rs` — registry extended to all six
  targets in stable id order (conformance, docs, json, registry, schema,
  typescript). `ir:build` / `ir:check` iterate `targets::all()`, so **no
  edit to `tasks/effigy.tasks.toml` was needed** — the card's "gates
  compose only checkers" rule holds as designed.
- `packages/codegen/fixtures/synthetic-model.json` — synthetic extension
  only: one conformance vector (`gauge-bounds`, three steps exercising the
  invariant/transition/effect-intent kinds plus a bounded boolean guard),
  gauge events (value-change/value-commit with pair payloads and an
  ordering constraint), capabilities (pointer-capture, scrub-fraction),
  size + density axes (ladder rung, density adjustment), search-field's
  `clear` event, and gauge's conformance reference. The guard uses
  `and`/`not` of literals because vector guards live in the model-only
  expression scope, where `eq`'s left side (a state reference) is a
  finding.
- `packages/codegen/tests/targets.rs` (new, 14 tests) and
  `packages/codegen/tests/emission.rs` (one test updated, see below).
- `packages/codegen/generated/{json,schema,registry,conformance,docs}/**`
  — committed output for all five new targets.
- `packages/codegen/Cargo.toml` — `serde` added to `[dependencies]` (the
  crate's posture stays poodle-ir + serialization/emit deps; serde is the
  same serialization family as serde_json and was already in the lock as a
  transitive dep) and `jsonschema` as a `[dev-dependency]` — a test oracle
  only, not a type-mirroring emitter (R4), and it renders nothing, so the
  crate's dependency posture is unchanged.

`packages/contracts/ir/` untouched: emission needed no `poodle-ir` change,
additive or otherwise. `emit.rs`, `check.rs`, `write.rs`, `error.rs`
untouched (verified by `git diff 623b26b4..HEAD --name-only`).

## Design (as built, within the card's "exact shape is yours to design")

- **Ordering discipline (the gate's single most likely breaker).**
  Components, shared types, and vectors sort by id before emission; props,
  events, and vector steps keep authoring order — props and events because
  they are the contract's own order, **steps because step order is
  execution order** (`machines.json`: "Effects are order-significant"),
  so it is never re-sorted. Capabilities sort by the corpus inventory
  order (`Capability`'s `Ord`); axes by fixed struct order
  (size/density/orientation); `declared_by` and registry shared-type lists
  sort by id. Subset members are already a `BTreeSet`. serde_json maps
  sort their keys, so every JSON artifact's key order is deterministic.
  Proven per-target by `every_target_double_generation_is_byte_identical`
  and by the whole-dir CLI double-generation test (which now spans all six
  targets).
- **The `IR-07` header in each medium.** TypeScript keeps `emit::header`
  (`//` comments). JSON targets carry the same content — generator name +
  version, authored source path, IR schema version — as a `generated`
  object field (the `//` form is not valid JSON). Docs fragments carry it
  as `<!-- -->` HTML comments (the `//` form is not markdown). No
  timestamp, no absolute path, no machine identifier in any of the three.
- **JSON surface.** Per-component documents with tagged `kind` objects for
  prop types (`string`/`number`/`boolean`/`shared`+`shared_type`/
  `pair`+`of`/`list`+`of`/`opaque`) and default values
  (`string`/`number`/`boolean`/`member`/`pair`/`list`/`null`) — string vs
  number vs member is unambiguous to a non-Rust consumer, unlike the
  externally-tagged serde form. The R6.2 permitted subset is the prop's own
  `permitted_subset` array. Shared types referenced by a component's props
  are listed with their members and the referencing prop ids. Events carry
  kind, payload, and full timing (phase, debounce, flush-on-blur,
  ordering). Axes carry the full IR content.
- **JSON Schema.** Draft 2020-12, root `anyOf` over `$defs.component-
  document` / `$defs.index-document`, `additionalProperties: false`
  throughout, and model-derived `component-id` / `shared-type-id` enums.
  The emitted keyword set is exactly what the emitted documents need;
  the emitted schema compiles under `jsonschema` (draft 2020-12) and every
  emitted document validates against it — and tampered documents fail
  (proven both ways).
- **Registry.** One entry per component: `capabilities` (sorted inventory
  order), `axes` (declared axes, fixed struct order), `shared_types`
  (sorted ids of shared types its props reference).
- **Conformance.** Each model `ConformanceVector` is a vector entry with
  `applies_to`, `description`, `declared_by` (components whose
  `conformance` names it, sorted — added because it makes the vector file
  the cross-reference for who relies on each vector), and `steps` in
  authoring order. **The "input → expected-output pairs" requirement is
  served by the step vocabulary itself**: each step pins
  (guard = input condition, kind + description = expected machine
  behavior), e.g. the fixture's transition step "INPUT outside [min, max]
  clamps to the nearest bound" with its guard. Corpus `CV` rows (RNG-11,
  RNG-12, CROSS-19, …) are expressible as such vectors — the card's
  conformance stop condition ("the conformance-vector shape cannot express
  a corpus `CV` row") did not arise; the shape is the IR's declarative
  intents in the shared-vector-file convention, exactly the `machines.json`
  evidence `ConformanceVector`'s own docs cite.
- **Docs fragments.** The `### Public Props` table mirrors the real
  contracts: `| Prop | Type | Default | Required | Notes |`; backticked
  prop names (what `contract-prop-drift.ts` matches); subset unions in the
  `button.md` style (`"danger" \| "default" \| "success"`); shared type
  names by reference (`Orientation`); `format_number` integers (`0`, never
  `0.0`); `—` for no default, `derived` for expression defaults; web-only
  props **included and marked** in Notes, matching the TypeScript target's
  include-and-mark decision (recorded in the b022 log; CROSS-03's exclusion
  targets the Rust spec surface, which no emitter here produces).
- **Milestone acceptance test.** One fixture change — a single added
  component definition (`status-light`: shared-type prop with subset, an
  event, a capability, a conformance reference) — and every declared
  artifact moves: ts gains the prop file and index entry, json gains the
  document and index entry, schema's `component-id` enum gains the id,
  registry gains the row (capability + shared refs), conformance's
  `declared_by` gains the component, docs gains the fragment. Asserted
  per-target with `assert_ne!` on the full rendered file sets, in one
  `ir:build`-equivalent pass.
- **`tests/emission.rs` update.** `check_leaves_the_tree_unchanged_even_on_
  drift` previously planted drift in a scratch containing only the ts root;
  the CLI now checks every registered target, so a scratch missing the json
  root reported `Missing` drift for json before the ts report. The test now
  writes a complete generation (all targets) into the scratch before
  planting the ts drift — the assertion set is unchanged.

## Tests (31 total, all passing)

`tests/emission.rs` (14) + `tests/targets.rs` (14) + `src/check.rs` unit
tests (3). The b022 suite is untouched except the one update above; the
machinery-level proofs (double generation via lib and CLI, environment
independence, whitespace classification, read-only check, malformed/invalid
input, framework-free tsc) now automatically span all six targets because
they drive `targets::all()` / the CLI.

New per-target tests (`tests/targets.rs`):

- `every_target_renders_the_fixture_without_error`,
  `every_target_double_generation_is_byte_identical` — each of the five new
  targets renders, and two renders are byte-identical.
- `every_target_detects_content_drift_and_stale_orphans`,
  `every_target_write_mode_deletes_stale_orphans`,
  `every_target_check_leaves_the_tree_unchanged` — per-target drift
  detection, orphan deletion, and read-only check (the card's "drift
  detected, orphan detected, check never writes" per target).
- `json_surface_index_lists_every_component_document_sorted`,
  `json_surface_document_carries_prop_subsets_events_and_axes`,
  `json_surface_documents_carry_the_ir07_generated_object` — the surface
  content contract (subset, events, axes, generated object).
- `every_emitted_json_document_validates_against_the_emitted_schema`,
  `emitted_schema_rejects_tampered_documents` — the JSON round trip, both
  directions (validity and teeth).
- `registry_lists_every_component_with_capabilities_axes_and_shared_types`
  — the "what exists" content.
- `conformance_vectors_carry_steps_guards_and_declared_by` — vector
  content, guard survival, declared_by.
- `docs_fragments_render_contract_style_props_tables` — the contract-shape
  table, subset unions, `format_number`, derived defaults, web-only marks.
- `one_fixture_change_updates_every_declared_artifact` — the milestone
  acceptance, plus assertions that the new component lands in the JSON
  surface and in the vector's `declared_by`.

## b015 failure-mode coverage (each of the ten modes: test or owner)

| # | Failure mode | Covered by | Owner |
|---|---|---|---|
| 1 | Hand-edited generated artifact diverges from emitter | `every_target_detects_content_drift_and_stale_orphans` + b022's content-drift test — byte-exact `ir:check` per target | this card |
| 2 | Gate composes write-mode generator, drift invisible | Composition rule unchanged (R3): `ir:build` never composes into a gate; `ir:check` is the only gate-shaped selector. `ir:check` covers every registered target automatically — no selector wiring change needed | this card (design) |
| 3 | Formatting divergence masquerades as content drift | b022's `check_classifies_whitespace_only_drift_separately` (machinery-level, applies to every target) | this card |
| 4 | Emitter and formatter disagree, regenerate ≠ committed | Emitter owns every byte (R2); `every_target_double_generation_is_byte_identical` + CLI double-gen | this card |
| 5 | Stale committed artifact silently survives | `every_target_detect*`/`every_target_write_mode_deletes_stale_orphans` — orphan scan per output root | this card |
| 6 | Committed artifact missing entirely | b022's missing-file test (ENOENT = `Missing` drift), machinery-level | this card |
| 7 | Source catalogue version drifts from the artifacts | Header pins generator + IR schema version (`json_surface_documents_carry_the_ir07_generated_object`, ts header test); fixture `schema_version` validated before emission | this card |
| 8 | Report artifact goes stale because nothing compares it | No check mode exists for the report generators and this card may not add one (R5) | follow-up card (same as b022) |
| 9 | Drift escapes a gate via silent skip | Every generated file compared, all findings reported at once; orphan scan covers each whole root | this card |
| 10 | Emitted output embeds nondeterminism | `every_target_double_generation_is_byte_identical`, environment-independence test, header/generated-object tests — no timestamp/absolute path/machine value in any medium | this card |

## Validation

All step-5 commands exit 0; two `git status --porcelain` snapshots around
`effigy ir:check` are byte-identical (check mode never writes).

| Command | Exit state |
|---------|-----------|
| `cargo build --manifest-path packages/codegen/Cargo.toml` | 0 |
| `cargo test --manifest-path packages/codegen/Cargo.toml` | 0 — 31 passed (14 emission + 14 targets + 3 unit) |
| `cargo clippy --manifest-path packages/codegen/Cargo.toml -- -D warnings` | 0 — one unused-import fix-up commit during the docs phase; final state clean |
| `cargo fmt --manifest-path packages/codegen/Cargo.toml -- --check` | 0 — `cargo fmt` touched only `packages/codegen/` files (papercut-aware; verified by `git status` after each run) |
| `effigy ir:build` | 0 — `Generated 1 files (target: conformance) / 3 (docs) / 4 (json) / 1 (registry) / 1 (schema) / 5 (typescript)`, IR schema 1; tree clean afterwards (output byte-identical to committed) |
| `git status --porcelain` | empty — only intended generated output, and it matched the committed bytes |
| `effigy ir:check` | 0 — `Verified 1/3/4/1/1/5 files … All generated artifacts are current.` |
| `git status --porcelain` | byte-identical to the line above — check left the tree untouched |
| `effigy docs:lint` | 0 |
| `effigy gate:clean` | 0 |
| `git diff --check` | 0 |

Baseline (before any change): `cargo test` 0 (17 passed), `cargo clippy -D
warnings` 0, `cargo fmt --check` 0, `effigy ir:check` 0, `effigy docs:lint`
0, `git diff --check` 0, `git status --porcelain` clean.

## Acceptance criteria

- [x] Five targets registered, each with its own `output_root` and
  committed output (json/schema/registry/conformance/docs under
  `packages/codegen/generated/`).
- [x] Two consecutive clean generations byte-identical across all targets,
  proven by test (`every_target_double_generation_is_byte_identical`,
  CLI double-gen).
- [x] `ir:check` detects content drift, whitespace-only drift, and stale
  orphans per target and never writes (b022 machinery tests + the three
  per-target tests).
- [x] One fixture change updates every declared artifact in one `ir:build`,
  proven by test (`one_fixture_change_updates_every_declared_artifact`) —
  the milestone's acceptance criterion.
- [x] Emitted JSON validates against the emitted JSON Schema, proven by
  test (both directions).
- [x] No type-mirroring crate (`serde` is serialization; `jsonschema` is a
  dev-only test oracle); no machinery reshaping (`emit.rs`, `check.rs`,
  `write.rs`, `error.rs` untouched); no artifact outside
  `packages/codegen/generated/` touched (all other repo paths unchanged by
  this card's commits — verified by `git diff 623b26b4..HEAD --name-only`).
- [x] All step-5 commands exit 0.
- [x] This log records commands, exit states, and b015 mode-8's owner.

## Not done

Per batch card and worker rules: no merge (pushed only the thread branch),
no roadmap/status/dispatch edits, no real component definitions (fixtures
stay synthetic; `Button`/`TextInput`/`RangeSlider` belong to g13.005–007),
no `ir:check` wiring into `docs:check`/`ci:web` (deferred until real
definitions exist), no Rust-emitting target or formatter integration, no
`git add -A`, no `tasks/effigy.tasks.toml` edit (none needed — new targets
are covered by the existing selectors through `targets::all()`). No stop
condition was reached: the machinery needed no change, double generation is
byte-identical across all six targets, registering a target required no
selector edit, and the conformance-vector shape expresses corpus `CV` rows
(see Design). Every source change is inside the writable
`packages/codegen/**`; the only other writable path touched is this log
(`PAPERCUTS.md` carries no new entry — no non-duplicate friction was hit).
