---
title: g13 batch 022 — poodle-codegen emitter core
status: complete
milestone: g13.003
owner: Poodle core
updated: 2026-08-11
tags: [log, g13, IR, codegen, emission, drift-gating, typescript, spec-063]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/022-poodle-codegen-emitter-core.md` on branch
`thread/g13-022-poodle-codegen-emitter-core`: built the compiler boundary —
a `poodle-codegen` crate (`packages/codegen/`, `lib` + `[[bin]]`) that turns
a validated `IrModel` into committed TypeScript, plus the `ir:build` /
`ir:check` selector pair. One target (TypeScript) ships; the other four
(JSON schema, registry, conformance vectors, docs fragments) are the
follow-up card. The five orchestrator rulings (R1–R5) were followed as
fixed; none needed revisiting.

Read before designing, per the card: the `b015` value track in full, every
module of `packages/contracts/ir/src/`, the token emitter precedent
(`packages/tokens/scripts/build-tokens.ts`, particularly its `compare` /
`writeFile` / check-mode paths), and `scripts/build-default-icons.ts` (the
repo's only stale-orphan pattern). No sub-agents were used.

## Deliverables (only the scoped writes)

- `packages/codegen/` — new crate: `Cargo.toml` (lib + `[[bin]]`, `publish =
  false`, `[package.metadata.poodle]` public-intent false / channel internal
  / stability internal-tooling), `README.md`, `src/lib.rs`, `src/emit.rs`
  (target abstraction, generated header, deterministic ordering),
  `src/check.rs` (read-only drift gate), `src/write.rs` (write mode),
  `src/model.rs` (load + validate), `src/targets/{mod,ts}.rs` (registry +
  the TypeScript target), `src/bin/poodle-codegen.rs` (CLI),
  `fixtures/synthetic-model.json` (synthetic fixture exercising every
  `PropType`/`Value` variant), `tests/emission.rs` (14 integration tests),
  `generated/ts/*` (committed output: `shared-types.ts`, three component
  files, `index.ts`).
- `tasks/effigy.tasks.toml` — the two selectors only: `ir:build` (write) and
  `ir:check` (read-only), with the same style of comment as `audit:tokens`
  explaining why the artifacts are committed.
- `packages/release-manifest.json` — `poodle-codegen` entry (kind
  `tooling`, language `rust`, public-intent false, channel `internal`).
- `packages/release-operations.json` — matching package entry
  (`changeControl` tooling-owned, no release-notes/deprecation requirement,
  removal gate owner-directed-with-doc-update), mirroring the other
  internal tooling packages.
- `docs/release-notes/0.1.0.md` — internal-tooling registration paragraph.
- `docs/logs/2026-08/11-g13-022-poodle-codegen-emitter-core.md` — this log.
- `PAPERCUTS.md` — one new entry (docs:check parity-report count staleness,
  §Validation).

Nothing outside those paths changed. `packages/contracts/ir/` was not
touched: emission needed no `poodle-ir` change, additive or otherwise (the
card's stop condition on non-additive changes did not arise; the accessors
`IrModel::shared_type`, `PropType::is_shared`, `Value::as_member`, and
`PermittedSubset::permits` from b011/b012 sufficed).

## Design (as ruled)

- **R1 — invocation shape.** Standalone `lib` + `[[bin]]` at
  `packages/codegen/`. The CLI is the only entry point; nothing depends on
  the crate.
- **R2 — the emitter owns every byte.** No formatter runs over the output.
  The TypeScript target emits every whitespace decision from its own
  templates: fixed two-space indentation, leading-pipe union members, a
  `format_number` mirroring `formatF32` (integral values print without a
  fractional part, else shortest round-trip via Rust's Display — never
  exponent notation, never locale-dependent), and JSON-escaped string
  literals. No Rust-emitting target exists, so no rustfmt integration
  (out of scope by ruling).
- **R3 — drift gate.** `ir:check` renders into memory and compares
  byte-exact against the committed files, reporting every drifted path with
  a classification (`content drift` / `whitespace-only difference` /
  `missing`), plus every stale orphan under the output root. It leaves the
  worktree untouched — proven by test. **Structural read-only:** the check
  module contains no filesystem write call, and the bin's `--check` branch
  never reaches the writer (`write_outputs` is a separate module only the
  write branch calls). The composition rule is respected: `ir:check` is a
  read-only selector, `ir:build` is write mode, and neither is wired into
  any gate (the card forbids wiring `ir:check` into `docs:check` until a
  real artifact exists to gate).
- **R4 — no type-mirroring dependency.** No `ts-rs`, `schemars`,
  `typeshare`, or `specta`. Dependencies are `poodle-ir` + `serde_json`
  only — serialization/emit deps, per the ruled posture. (The card's stop
  condition "adding any of these four crates" did not arise.)
- **R5 — report artifacts untouched.** The three report generators and
  their artifacts were not touched (see the docs:check finding below).

### Emission core

- `EmitTarget` trait (`id`, `output_root`, pure `render`) — one registered
  target (`typescript`), the registry being the follow-up card's extension
  point.
- Header (`IR-07`): exactly four lines — generator name + version
  (`env!("CARGO_PKG_VERSION")`, compile-time), authored source path (the
  fixture argument verbatim, repo-relative when invoked via the selector),
  IR schema version (`poodle_ir::IR_SCHEMA_VERSION`), and the
  regenerate/gate hint. No timestamp, no absolute path, no machine or user
  identifier; a pure function of the source path. Tested byte-level.
- Deterministic ordering: shared types and components sort by id before
  emission; prop and member order stay as authored (the contract's own
  order, preserved by serialization the way `build-tokens.ts` preserves
  token path order). `PermittedSubset.members` is already a `BTreeSet`.
  Proven by the double-generation tests (lib and CLI) and by an
  environment-variance test (generation under altered `USER`/`HOME` lands
  byte-identical).

### The TypeScript target

Per model: `shared-types.ts` (one union type per shared enumerated type),
`<component-id>.ts` (one `export type <Name>Props = Readonly<{…}>` per
component, importing only the shared types it references), `index.ts`
(re-exports). Prop mapping: `String`→`string`, `Number`→`number`,
`Bool`→`boolean`, `Pair(t)`→`[t, t]`, `List(t)`→`Array<t>`, `Opaque`→
`unknown`. A `Shared` prop with a permitted subset emits the subset union
inline (the R6.2 constraint survives into the artifact — the exact
ButtonTone case from `g13-b003`); without a subset it references the shared
type by name. Required props are non-optional; defaults, expression-derived
defaults, and web-only marks (CROSS-03) land in the prop doc comment. The
output is framework-free: `tsc --noEmit --strict --lib es2020` (no DOM)
passes over it, asserted by test.

Web-only props are **included** in the TS prop surface and marked in the
doc comment, because the TypeScript target is the web surface (spec 063
"generate neutral TypeScript artifacts for the web"); CROSS-03's exclusion
is a portable-spec-surface rule aimed at the Rust spec target, which is not
this card's emitter. Recorded here so the follow-up card can override if
the intended reading was different.

### The synthetic fixture

`packages/codegen/fixtures/synthetic-model.json` — three synthetic
components (`badge`, `gauge`, `search-field`; deliberately not
Button/RangeSlider/TextInput, which belong to g13.005–g13.007) and four
shared types (`tone`, `placement`, `orientation`, `validation-state`)
covering every `PropType` variant (String, Number, Bool, Shared, Pair,
List, Opaque via the null-default pair), every `Value` variant used in
defaults (String with quotes/backslash escaping, Number integral and
fractional, Bool, Member, Pair, List, Null), web-only marks, one-member
subsets, a `default_expr` (a `coalesce` over a declared prop — valid per
b012's type-checking), a `from-prop` name rule, and an aria mapping. The
fixture validates clean against `poodle-ir`'s whole-model `validate`.

## Tests (17 total, all passing)

`packages/codegen/tests/emission.rs` (14) plus `src/check.rs` unit tests
(3):

- `double_generation_is_byte_identical`, `double_generation_via_cli_is_
  byte_identical` — the determinism acceptance criterion, proven twice
  (lib and CLI).
- `generation_is_independent_of_environment` — altered `USER`/`HOME`
  produce identical bytes.
- `header_carries_source_ir_and_generator_versions_only`,
  `generated_files_contain_no_absolute_path` — header shape; no timestamp
  slot, no absolute path.
- `check_detects_content_drift_and_reports_every_path` — two drifted files
  both reported.
- `check_classifies_whitespace_only_drift_separately` — the `45caae82`
  failure class is labeled `whitespace-only difference`.
- `check_reports_missing_committed_file_as_drift`, `check_detects_stale_
  orphan`, `write_mode_deletes_stale_orphan` — missing files are drift
  (the tokens `compare` ENOENT path, already caught); orphans are detected
  in check mode and deleted in write mode (the icons pattern).
- `check_leaves_the_tree_unchanged_even_on_drift` — content + whitespace +
  missing + orphan drift all planted; both the lib check and the CLI check
  leave the tree byte-identical to the drifted baseline, and the CLI exits
  1 with the classified report.
- `malformed_json_is_a_clean_error_not_a_panic`,
  `invalid_model_is_a_clean_error_not_a_panic` — garbage JSON and a
  `schema_version: 99` model are clean `CodegenError`s (lib) and clean
  exit-1 CLI failures; stderr never contains `panicked`.
- `emitted_typescript_type_checks_with_no_framework_dependency` — `bunx
  --no-install tsc --noEmit --strict --lib es2020` over the generated
  `index.ts` exits 0.
- `whitespace_classification_*`, `report_message_lists_every_finding_
  classified` — classification unit tests.

## b015 failure-mode coverage (each of the ten modes: test or owner)

| # | Failure mode | Covered by | Owner |
|---|---|---|---|
| 1 | Hand-edited generated artifact diverges from emitter | `check_detects_content_drift_and_reports_every_path` — byte-exact `ir:check` | this card |
| 2 | Gate composes write-mode generator, drift invisible | Composition rule: `ir:build` never composes into a gate; `ir:check` is the only gate-shaped selector (R3). Not mechanically testable here — enforced by selector wiring and by `ir:check`'s structural read-only property | this card (design) |
| 3 | Formatting divergence masquerades as content drift | `check_classifies_whitespace_only_drift_separately` — whitespace-only differences classified | this card |
| 4 | Emitter and formatter disagree, regenerate ≠ committed | Emitter owns every byte (R2); double-generation byte identity | this card |
| 5 | Stale committed artifact silently survives | `check_detects_stale_orphan`, `write_mode_deletes_stale_orphan` — orphan scan on the output root | this card |
| 6 | Committed artifact missing entirely | `check_reports_missing_committed_file_as_drift` (ENOENT = `Missing` drift) | this card |
| 7 | Source catalogue version drifts from the artifacts | Header pins IR schema version + generator version (`header_carries_*` test); the fixture's `schema_version` is validated before emission (`invalid_model_*` test) | this card |
| 8 | Report artifact goes stale because nothing compares it | Observed live: `docs:check` fails at HEAD because the committed parity report (200 exports) lags the current tree (201, `HistoryCenter` from g13-b020/b021) — see §Validation. No check mode exists for reports and this card may not add one | follow-up card (R5: report-artifact gating is out of scope) |
| 9 | Drift escapes a gate via silent skip | `check_detects_content_drift_and_reports_every_path` — every generated file compared, all findings reported at once; orphan scan covers the whole root | this card |
| 10 | Emitted output embeds nondeterminism | `generation_is_independent_of_environment`, `double_generation_*`, header tests — no timestamp/absolute path/machine value | this card |

## Validation

| Command | Exit state |
|---------|-----------|
| `cargo build --manifest-path packages/codegen/Cargo.toml` | 0 |
| `cargo test --manifest-path packages/codegen/Cargo.toml` | 0 — 17 passed (14 integration + 3 unit) |
| `cargo clippy --manifest-path packages/codegen/Cargo.toml -- -D warnings` | 0 |
| `cargo fmt --manifest-path packages/codegen/Cargo.toml -- --check` | 0 — baseline check clean; `cargo fmt` touched only `packages/codegen/` files, no out-of-crate churn |
| `cargo test --manifest-path packages/contracts/ir/Cargo.toml` | 0 — 30 passed |
| `effigy ir:build` | 0 — `Generated 5 files (target: typescript, IR schema 1).` |
| `git status --porcelain` | only the writable paths (manifest/operations/release-notes/tasks modified, `packages/codegen/` untracked) |
| `effigy ir:check` | 0 — `Verified 5 files … All generated artifacts are current.` |
| `git status --porcelain` | byte-identical to the line above — check mode left the tree untouched |
| `effigy docs:lint` | 0 — passes with the `poodle-codegen` registrations |
| `effigy docs:check` | **1 — pre-existing, reproduced at clean HEAD (commit `bdbb906a`) in a fresh worktree.** Two distinct causes, neither in this card's writable paths: (a) `bun run --cwd packages/react/preview parity:report` fails with `Cannot find module '@inflatable-cookie/poodle-core/tokens'` until `bun install` refreshes the linking (stale worktree node_modules); after `bun install`, (b) the regenerated `packages/svelte/preview/artifacts/parity-report.json` reports 201 exports / 164 components (`HistoryCenter`, added by g13-b020/b021, merged into this branch's base) while the committed report and `packages/shared-demo-app-audit.json` say 200/163, and `docs:lint`'s audit-json consistency check then fails. Both are b015 failure mode 8 — report artifacts with no check mode. The card forbids regenerating or rewiring existing report artifacts, and their paths are outside the writable set, so this card records rather than fixes. The dirty artifacts `docs:check` produces (`packages/tokens/artifacts/rust/*`, both parity/component-docs reports) were restored with `git checkout --` per the step-8 sequence |
| `git checkout -- packages/tokens/artifacts/rust/` (plus the two report artifacts above) | 0 — worktree back to the writable-paths-only state |
| `git diff --check` | 0 |

## Not done

Per batch card and worker rules: no merge, no roadmap/status/dispatch edits,
no other four emitters, no Rust-emitting target or formatter integration, no
real component definitions (fixtures synthetic), no existing generated
artifact touched, no `ir:check` wiring into `docs:check`/`ci-web`, no
macros, no `poodle-ir` change (no additive accessor needed either), no
`git add -A` (staged by explicit path). No stop condition was reached; the
rulings held.
