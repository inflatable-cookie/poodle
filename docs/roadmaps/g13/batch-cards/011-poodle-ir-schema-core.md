# 011 poodle-ir Schema Core

Status: ready
Milestone: `g13.002`
Owner: Poodle core
Branch: `thread/g13-011-poodle-ir-schema-core`
Depends on: `g13-b003` rulings (placement, dependency direction, schema
requirements)
Governing refs: `docs/specs/063-rust-authored-component-and-scene-ir.md`
(`IR-01`–`IR-12`), `docs/roadmaps/g13/002-rust-ir-schema-and-validation-core.md`,
`docs/roadmaps/g13/batch-cards/003-crate-placement-ruling-and-schema-handoff.md`,
`docs/roadmaps/g13/pilot-expressiveness-corpus.md`,
`docs/roadmaps/g13/pilot-baseline-manifest.md`

## Goal

Create the versioned Rust model for component and scene definitions. Data and
validation only — no renderer, no framework behaviour, no code generation.

This is the first card that creates an implementation package. `IR-12` is
satisfied: `g13-b003` recorded the placement ruling.

## Fixed By Ruling (do not re-decide)

From `g13-b003`:

- **Location:** `packages/contracts/ir/`, crate name `poodle-ir`.
- **Shape:** library only. **No `[[bin]]`.** The emitter is a separate crate in
  a later card; do not create it here.
- **Manifest posture:** `publish = false`;
  `[package.metadata.poodle]` public-intent true, channel `preview`, stability
  `pre-release`; release-manifest kind `contract-crate`. Copy the exact shape
  from `packages/contracts/components/Cargo.toml`.
- **Dependencies:** `poodle-tokens` only, plus serialization. Nothing else from
  this repo. Depending on `poodle-node`, `poodle-specs`, `poodle-headless`,
  `poodle-render`, `poodle-adapter`, either adapter, or either preview is a
  stop condition — the IR sits *above* `poodle-node` (`IR-04`).
- **Nothing may depend on `poodle-ir`.** Do not add it to any existing
  manifest.

## Scope

### In scope

The serializable model plus its validation, covering the vocabulary the merged
corpus proved is needed:

- **Identity** — stable component/scene/part/prop/event identifiers
  (`CROSS-01`, `CROSS-02`).
- **Shared types** — a first-class shared-type layer: named enumerated types
  defined once and referenced by many components (`g13-b003` R6.1). This is not
  optional sugar; it is the requirement that `ButtonTone` and
  `OverlayPlacement` fragmentation produced.
- **Per-component permitted subsets** — a component may permit a subset of a
  shared type's members, expressed as a first-class constraint (`g13-b003`
  R6.2).
- **Props** — name, type, default, required, web-only marking (`CROSS-02`,
  `CROSS-03`).
- **Controlled state** — controlled/uncontrolled pairs and the do-not-mix rule
  (`CROSS-04`).
- **Events** — name, payload, firing condition (`CROSS-05`).
- **Parts / anatomy** — parent-child structure, conditional and repeated nodes
  (`CROSS-12`).
- **State-derived attributes** — presence-only vs valued, omitted vs always
  emitted (`CROSS-13`).
- **Axes** — size, density, orientation, theme, contrast (`CROSS-07`–`CROSS-11`).
- **Token and recipe references** — typed, resolved against `poodle-tokens`
  (`CROSS-09`).
- **Accessibility intent** — role, name rule, ARIA state mapping (`CROSS-15`).
- **Capabilities** — declared adapter capabilities a component requires
  (`CROSS-17`).
- **VisualState** — serializable projection shape (`CROSS-14`).
- **Scenes** — specimen/scene definitions with axis matrices (`CROSS-21`,
  `SHELL-*`).
- **Validation** — duplicate IDs, invalid references, impossible prop bindings,
  unsupported cycles, missing accessibility data, undeclared capabilities, and
  a value outside a component's permitted subset of a shared type.
- **Serialization** — stable ordering, schema version, JSON round-trip.

### Out of scope — stop conditions if reached

- Any code generation, emission, or TypeScript output. That is `poodle-codegen`
  in a later card.
- Macros. `IR-12` and the milestone both say ordinary Rust constructors first;
  macros only after measured authoring pain. Do not write one.
- Migrating, rewriting, or referencing any existing component. Do not touch
  `poodle-specs`, `poodle-render`, adapters, previews, or web packages.
- Framework, DOM, GPUI, Jetstream, or `poodle-node` types anywhere in the
  schema. The milestone's acceptance is explicit about this.
- Authoring real component definitions for Button/RangeSlider/TextInput. Model
  the vocabulary; fixtures may be minimal and synthetic.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read `docs/roadmaps/g13/pilot-expressiveness-corpus.md` first. It is the
  requirements list — 129 requirements with IDs. Your types exist to express
  its `SDD` and `CV` rows. Cite requirement IDs in doc comments.
- Prefer ordinary structs, enums, and builders. No macros, no derive magic
  beyond serde and the standard derives.
- Every public type and field carries a doc comment naming the corpus
  requirement or contract section it serves.
- Do not edit roadmap/milestone/card status files or `docs/roadmaps/dispatch.md`.
- Do not add `poodle-ir` to any other crate's manifest.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-011-poodle-ir-schema-core`. Do not merge.

## Writable Paths

- `packages/contracts/ir/**` (new crate: `Cargo.toml`, `src/`, `tests/`)
- `docs/logs/2026-08/<DD>-g13-011-poodle-ir-schema-core.md`
- `PAPERCUTS.md` (new, non-duplicate friction only)

Any other changed path is a scope failure.

## Steps

1. **Read first.** The corpus (all 129 rows), spec 063 §"Component IR",
   §"Scene IR", §"Hard Boundary", and `packages/contracts/components/Cargo.toml`
   for the manifest shape. Do not start typing until the corpus is read.
2. **Create the crate** at `packages/contracts/ir/` with the ruled manifest
   posture and dependency set.
3. **Model the vocabulary** in focused modules — identity, shared types, props,
   events, parts, attributes, axes, tokens, accessibility, capabilities,
   visual state, scenes. One concern per module.
4. **Implement validation** as a single entry point returning *all* findings,
   not the first. Each finding carries the offending identifier and a message
   that names what to fix. Cover every case listed in scope.
5. **Serialization.** Serde with deterministic field and collection ordering.
   Include a schema version constant.
6. **Tests.** Round-trip fixtures proving JSON preserves meaning and ordering;
   one negative test per validation rule; a test asserting a value outside a
   permitted subset is rejected.
7. **Validate.**
   ```sh
   cargo build --manifest-path packages/contracts/ir/Cargo.toml
   cargo test --manifest-path packages/contracts/ir/Cargo.toml
   cargo clippy --manifest-path packages/contracts/ir/Cargo.toml -- -D warnings
   cargo fmt --manifest-path packages/contracts/ir/Cargo.toml -- --check
   effigy docs:lint
   git diff --check
   git status --porcelain
   ```
   `cargo fmt` has historically reformatted unrelated linked workspaces
   (`PAPERCUTS.md`, 2026-08-10). Use `--check` first; if it reports files
   outside `packages/contracts/ir/`, stop and report rather than formatting.

## Acceptance Criteria

- [ ] `packages/contracts/ir/` exists as a lib-only crate with the ruled
  manifest posture and `poodle-tokens` as its only in-repo dependency.
- [ ] Shared types are first-class, and a component can declare a permitted
  subset of one.
- [ ] Every scope bullet has a representation, each doc-commented with the
  corpus requirement ID or contract section it serves.
- [ ] Validation reports all findings at once, with identifier and actionable
  message, covering every listed rule including subset violation.
- [ ] JSON round-trips preserve meaning and ordering; a schema version is
  present.
- [ ] No framework, DOM, GPUI, Jetstream, or `poodle-node` type appears in the
  schema.
- [ ] No macro, no codegen, no emission.
- [ ] Nothing outside `packages/contracts/ir/` changed except the log and
  papercuts; no other manifest references the crate.
- [ ] All commands in step 7 exit 0.
- [ ] Batch log records commands, exit states, module layout, and every corpus
  requirement the model cannot yet express.

## Stop Conditions

- Expressing a corpus requirement needs a framework, DOM, or `poodle-node`
  type.
- Expressing a requirement needs a dependency beyond `poodle-tokens`.
- The vocabulary cannot be modelled without a macro.
- `cargo fmt --check` reports files outside the new crate.
- A corpus requirement is ambiguous enough that modelling it is a design
  decision rather than a transcription.

Stop with requirement IDs, exact paths, commands, and the smallest unresolved
question. A requirement the model genuinely cannot express is a **finding, not
a failure** — record it in the log and continue with the rest.
