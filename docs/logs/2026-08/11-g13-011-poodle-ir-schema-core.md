---
title: g13 batch 011 — poodle-ir schema core
status: complete
milestone: g13.002
owner: Poodle core
updated: 2026-08-11
tags: [log, g13, IR, poodle-ir, schema, validation, shared-types, permitted-subsets]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/011-poodle-ir-schema-core.md` on branch
`thread/g13-011-poodle-ir-schema-core`: created the versioned, serializable
component and scene IR crate `packages/contracts/ir/` (crate `poodle-ir`,
library only, no `[[bin]]`), modeled the vocabulary the 129-requirement
expressiveness corpus proved needed, and implemented whole-model validation.

Placement and dependency direction follow `g13-b003` R1/R2 exactly:
`packages/contracts/ir/`, `poodle-tokens` as the only in-repo dependency, and
no other crate manifest references `poodle-ir`.

## Deliverables (only the scoped writes)

- `packages/contracts/ir/` — new crate: `Cargo.toml` (manifest posture copied
  from `packages/contracts/components/Cargo.toml`: `publish = false`,
  `[package.metadata.poodle]` public-intent true / channel `preview` /
  stability `pre-release`; `poodle-tokens` + serde only), `README.md`,
  `src/` (13 modules), `tests/roundtrip.rs`.
- `docs/logs/2026-08/11-g13-011-poodle-ir-schema-core.md` — this log.
- `PAPERCUTS.md` — one new entry (release-manifest coverage gap, see §Stop
  condition below).

Nothing outside those paths changed. `packages/release-manifest.json` was not
touched — see the stop condition.

## Module layout (one concern per module)

| Module | Serves |
|---|---|
| `id.rs` | identity — `Identifier`, `Layer`, `ContractRef` (`CROSS-01`, `CROSS-02`) |
| `shared.rs` | first-class shared enumerated types + `PermittedSubset` (g13-b003 R6.1/R6.2) |
| `props.rs` | `Prop`, `PropType`, `Value` — portable spec surface, web-only marking (`CROSS-02`, `CROSS-03`) |
| `state.rs` | `ControlledState`, do-not-mix rule (`CROSS-04`, `TXT-02`) |
| `events.rs` | `Event`, `EventPayload`, `EventTiming`, ordering constraints (`CROSS-05`, `CROSS-06`) |
| `parts.rs` | `Part` with parent/child, conditional/repeated kinds (`CROSS-12`) |
| `attributes.rs` | `StateAttribute` presence-only/valued, omit/always (`CROSS-13`; `RNG-17`/`TXT-16` custom properties) |
| `axes.rs` | size ladder + sizeRole, density adjustments, orientation (`CROSS-07/08/11`) |
| `tokens.rs` | `TokenRef` resolved against `poodle-tokens` semantic registry, `RecipeHookRef` chain (`CROSS-09`) |
| `accessibility.rs` | role, name rule, ARIA mapping, native attrs (`CROSS-15`) |
| `capabilities.rs` | adapter capability inventory (`CROSS-17`) |
| `keyboard.rs` | keyboard command tables with capability requirements (`CROSS-16`) |
| `visual.rs` | VisualState projection shape (`CROSS-14`, `RNG-16`) |
| `conformance.rs` | `ConformanceVector` steps, `RuntimeTarget`, `Extension` (`CROSS-18`, EXT rows) |
| `component.rs` | `ComponentDefinition` assembly (`CROSS-01`) |
| `scenes.rs` | `Scene`, axis matrix, layout/search/tabs/preview-state/parity/registry (`CROSS-21`, `SHELL-01`–`10`) |
| `validation.rs` | single entry point returning **all** findings; schema version; token/theme/size/density resolution |
| `lib.rs` | `IrModel`, `IR_SCHEMA_VERSION = 1`, re-exports |

## The two requirements this card exists for

- **Shared types are first-class** (g13-b003 R6.1): `SharedType` defines a
  named enumerated type once; props, defaults, and bindings reference it by
  id, and `validate` resolves every reference. The motivating fragmentation
  cases (`ButtonTone`, `OverlayPlacement`, the 8 unregistered enumerated
  types from `g13-b007`) are the documented rationale.
- **Permitted subsets** (g13-b003 R6.2): a prop carries a first-class
  `PermittedSubset { shared_type, members }`; `validate` rejects any default
  or scene binding whose member value is outside the subset with a
  `PermittedSubsetViolation` finding naming the binding, the value, and the
  permitted set. `UNKNOWN-02`'s superset question (`ButtonVariant::Danger`,
  `ButtonTone::Success`) resolves inside this model: the shared type may
  define the superset; each component permits its own subset.

## Validation rules (all findings at once, identifier + actionable message)

`validate(&IrModel) -> Vec<Finding>` covers, per card scope: schema-version
mismatch; duplicate ids (shared types, members, components, scenes, props,
controlled states, events, parts, attributes, size rungs, density
adjustments, capabilities, keyboard commands, visual states, extensions,
vectors, registry entries); invalid references (shared types, members,
subset members, parents, `when`/`over` props, attribute/aria/name sources,
token paths, theme/size/density names, component/scene/vector refs);
impossible prop bindings (unknown prop, duplicate binding, type mismatch,
valued attribute without source, do-not-mix controlled pair); unsupported
parent cycles; missing accessibility data (required name with no source);
undeclared capabilities (keyboard command `requires` not declared); and
permitted-subset violations.

## Tests (14, all passing)

- `valid_model_has_no_findings` — the synthetic fixture (shared types
  `control-tone`/`polarity`, component `sample-button`, conformance vector,
  scene with all five axes, layout, tabs, search, preview state, parity
  harness, registry) validates clean.
- `json_round_trip_preserves_meaning_and_ordering` — deterministic
  serialization (two encodings byte-identical), round-trip equality,
  `schema_version` present as first key, `BTreeSet` subset serializes sorted.
- One negative test per rule: duplicate ids, invalid reference, impossible
  binding, parent cycle, missing accessibility, undeclared capability,
  schema version, unresolvable token.
- **Permitted subset**: binding a member outside the subset is rejected;
  a default outside the subset is rejected.
- `reports_all_findings_at_once_not_first` — three independent violations
  all reported in one pass.
- Do-not-mix controlled pair rejected.

## Corpus coverage and findings

All 129 requirements have a representation (each public type/field doc-comments
the requirement ID or contract section). Rows that needed transcription-level
choices, not design decisions:

- `RNG-17` / `TXT-16` (computed custom properties from normalized visual
  state): `StateAttribute.source` resolves VisualState-field ids, so
  `--poodle-range-start/…` and `--poodle-text-input-control-padding-*` are
  declared as valued attributes sourced from projection fields.
- `CROSS-19` / `RNG-02` degenerate-range guard and step snapping: expressed as
  `ConformanceVector` steps (`VectorStepKind::Guard`), which is the spec 063
  "shared conformance vectors" slot.
- `CROSS-20` (`isUnavailable = disabled || loading`): the boolean composition
  has no first-class derivation-expression slot in the schema core; it is
  expressed declaratively as a VisualState field plus a vector step intent.
  **Finding (partial):** a general derived-state expression vocabulary
  (`||`, `&&`, comparisons) is not part of this card's module set; if the
  pilot needs it, it belongs in a later card, not a stop.
- `BTN-15` rest-attribute passthrough: web-native escape retained above the IR
  (`NEG-02`), declared intent only.
- `BTN-25` / `RNG-28` / `TXT-29` "Rust spec surface" rows: the IR expresses
  the data vocabulary; derived helper methods (`activation_allowed`,
  `effective_tone`, token resolvers) are `poodle-specs` API concerns, not IR
  data.
- `GTA` class rows (`CROSS-21`, `SHELL-10`): the declaration side is modeled
  (`Scene`, `SpecimenRegistry`); the emitted registry artifacts themselves are
  `poodle-codegen` output in a later card.

No corpus requirement required a framework, DOM, GPUI, Jetstream, or
`poodle-node` type; none required a dependency beyond `poodle-tokens`; none
required a macro.

## Validation

| Command | Exit state |
|---------|-----------|
| `cargo build --manifest-path packages/contracts/ir/Cargo.toml` | 0 |
| `cargo test --manifest-path packages/contracts/ir/Cargo.toml` | 0 — 14 passed |
| `cargo clippy --manifest-path packages/contracts/ir/Cargo.toml --all-targets -- -D warnings` | 0 |
| `cargo fmt --manifest-path packages/contracts/ir/Cargo.toml -- --check` | 0 — **after** apply; all diffs were inside `packages/contracts/ir/`, none in linked workspaces (the historical `PAPERCUTS.md` case did not recur) |
| `effigy docs:lint` | **1** — see stop condition |
| `git diff --check` | 0 |
| `git status --porcelain` | 0 — only `packages/contracts/ir/` untracked (plus log + papercuts after this write) |

## Stop condition

One card-internal conflict, reported rather than resolved, because resolving
it requires a change outside the writable paths:

- **`effigy docs:lint` fails at exit 1** with exactly one error:
  `packages/contracts/ir/Cargo.toml package "poodle-ir" is missing from
  packages/release-manifest.json` (raised by the reverse check in
  `packages/svelte/preview/scripts/lint-docs.ts`
  `validateReleaseOperations`, which walks every `Cargo.toml` under
  `packages/` and requires a release-manifest entry; no carve-out exists).
- Card 011's writable paths are `packages/contracts/ir/**`, the log, and
  `PAPERCUTS.md`, and its acceptance criteria state "no other manifest
  references the crate" — so adding the entry to
  `packages/release-manifest.json` would be a scope failure, and the two
  acceptance criteria ("all step-7 commands exit 0" and "no other manifest
  references the crate") cannot both hold.
- The ruling (`g13-b003` R1) already fixes the entry's shape: name
  `poodle-ir`, path `packages/contracts/ir`, kind `contract-crate`, language
  `rust`, public-intent true, channel `preview`. Precedent: g04-003 added its
  crate to the same file as part of its card.
- **Smallest unresolved question:** may this card (or a follow-up) add the
  `poodle-ir` entry to `packages/release-manifest.json` so `effigy docs:lint`
  exits 0, or is the release-manifest registration a separate card's scope?
  Until answered, `docs:lint` cannot pass with `poodle-ir` present.

## Not done

Per batch card and worker rules: no merge, no roadmap/status/dispatch edits,
no release-manifest edit (above), no codegen/emission, no macros, no real
Button/RangeSlider/TextInput definitions, no other crate manifest touched.
