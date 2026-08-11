# 003 Crate Placement Ruling And Schema Handoff

Status: complete — rulings recorded 2026-08-11
Milestone: `g13.001`
Owner: Poodle maintainer (orchestrator)
Depends on: `g13-b001` (`251cc858`) and `g13-b002` (`89debbcb`), both merged
Governing refs: `docs/specs/063-rust-authored-component-and-scene-ir.md`
(`IR-01`–`IR-12`), `docs/architecture/001-poodle-system-shape.md`,
`docs/roadmaps/g13/authority-inventory.md`,
`docs/roadmaps/g13/pilot-baseline-manifest.md`

## Goal

Make the judgment calls workers must not make, close `g13.001`, and leave
`g13.002` executable without discovering authority or package boundaries in
code.

This card is the ruling record. It creates no crate — `IR-12` is satisfied by
the ruling existing, not by implementation. The first implementation card is
`011`.

---

## R1 — Crate placement and publication posture

| Crate | Location | Shape | `publish` | public-intent / channel / stability | Release kind |
|---|---|---|---|---|---|
| `poodle-ir` | `packages/contracts/ir/` | **lib only, no `[[bin]]`** | false | true / preview / pre-release | `contract-crate` |
| `poodle-codegen` | `packages/codegen/` | lib + `[[bin]]` | false | false / internal / internal-tooling | `tooling` |

**Why split.** `poodle-ir` is pure serializable data with validation — exactly
what `packages/contracts/` holds (9 such crates today, none with a `[[bin]]`).
The emitter is a build tool: putting it in the contract tree would be the first
bin there and would contradict the tree's own posture, which `g13-b001` already
flagged eroding (`poodle-render` dev-depends on `poodle-jetstream` for the token
resolver, recorded in that manifest as migration debt).

`packages/codegen/` is a new top-level directory. There is no precedent for it,
and that is the honest cost of this ruling — but the release manifest already
has a `tooling` kind for exactly this class, so it slots in without inventing a
category.

**Rejected: single crate with a `[[bin]]`.** Fewer moving parts, and splitting
later is mechanical. Rejected because contract-tree purity is a stated
architecture boundary and the split costs one directory.

**Generated output location.** Generated TypeScript lands inside the consuming
web packages under a `generated/` directory, mirroring
`packages/core/src/tokens/generated/`. That shape already exists, consumers
already accept it, and `audit:*` gates already police hand-edits to it.

**Precedent this follows.** `packages/tokens/` is the repo's existing
one-source-many-targets generator: a TS package, no `Cargo.toml`, emitting into
`artifacts/{css,ts,rust}`, with the Rust artifacts pulled into the
`poodle-tokens` crate via `#[path = "../../../tokens/artifacts/rust/mod.rs"]`.
g13 inverts the direction (Rust source, TS output) but keeps the shape:
generated code lives outside the authoring crate and is pulled in by the
consumer.

---

## R2 — Dependency direction (frozen)

Permitted:

```
poodle-ir       -> poodle-tokens          (only)
poodle-codegen  -> poodle-ir              (+ serialization/emit deps)
```

Forbidden, and enforced by review until a gate exists:

- `poodle-ir` must not depend on `poodle-node`, `poodle-specs`,
  `poodle-headless`, `poodle-render`, `poodle-adapter`, either adapter, either
  preview, or any framework crate. The IR sits **above** `poodle-node`
  (`IR-04`); depending on it would invert that.
- `poodle-specs`, `poodle-headless`, `poodle-render`, both adapters, and both
  previews must not depend on `poodle-ir` **during the pilot**. The pilot emits
  artifacts alongside the existing tiers; it does not invert existing
  dependencies. That inversion is a `g13.008`-gated decision.
- `poodle-codegen` must not be depended on by anything. It is a tool.

`poodle-ir -> poodle-tokens` is allowed because `poodle-specs` already does it,
it keeps token references compile-time validated rather than stringly typed,
and `poodle-tokens` has no runtime deps of its own.

**Path reachability.** Both crates must stay path-reachable from
`packages/jetstream/preview/Cargo.toml`, which sits beside sibling-repo path
deps. Both locations satisfy this.

**No root `Cargo.toml` exists.** Each new crate is an independent manifest. No
workspace member list to update — and no shared lockfile or target directory,
which will inflate clean-build times against the `g13-b002` baseline. Measure
like for like.

---

## R3 — Jetstream paths and compatibility layers

Each enumerated surface from `authority-inventory.md` §4, ruled:

| Surface | Size | Ruling |
|---|---|---|
| `packages/jetstream/adapter/src/render_*.rs` — 108 `impl RenderComponent<Spec>` | 7 files | **Active runtime capability.** They delegate to `poodle-render` and are the adapter's test/evidence surface. Not debt, not a fork. Unchanged by the pilot. |
| `packages/jetstream/preview/src/compat.rs` — 157 `js_*` shims | 1104 ln | **Migration debt.** Preserves the retired `poodle-jetstream-components` call shapes so specimen sources did not have to change when that tier was deleted (`ee704699`). A scene-IR subsume target. Keep stable until then (`IR-09`). |
| `packages/jetstream/preview/src/nel.rs` — `El` fluent builder | 464 ln | **Migration debt.** Same class: preserves the old `ui_element` chrome surface over `poodle_node::Node`. Subsume target. |
| `packages/gpui/preview/src/node_compat.rs` — `from_spec` facades | 6470 ln | **Migration debt.** The GPUI analogue. Preview-only, ships to no consumer. Subsume target. |
| `packages/jetstream/preview/src/component_registry.rs` | 1115 ln | **Codegen target — the pilot's first real one.** Its header declares it generated from the Svelte registry ("do not hand-edit entries") and no generator exists in this repo. It is the clearest case of a file that should be emitted rather than maintained. |

None of these is dead code. None ships to consumers — all are
`internal-tooling` preview crates.

---

## R4 — Fixture manifest and quantitative baseline

`docs/roadmaps/g13/pilot-baseline-manifest.md` is **approved** as the pilot
before-state: 32 `FIX-*` fixtures and the full measurement set.

One correction already applied: the runtime-extension baseline is **6**
(`BTN-26`, `BTN-27`, `BTN-29`, `RNG-26`, `RNG-27`, `TXT-31`), not the 10 the
corpus §8 table claimed. Row-level marks won; the corpus table was recomputed
with an amendment note.

Blocked fixtures `FIX-RNG-04`, `FIX-RNG-05` (`UNKNOWN-01`) and `FIX-BTN-08`
(`UNKNOWN-02`) are unblocked by R5 below.

---

## R5 — Contract questions carried out of `g13-b005`

Both ruled, and both already implemented:

- **`UNKNOWN-01`** — `range-slider.md` §6's `aria-orientation` prohibition was
  written before the embedded variant existed and described only the native
  range inputs. Amended: required on the embedded `role="slider"` stops, where
  ARIA defaults to horizontal. `FIX-RNG-04`/`FIX-RNG-05` unblocked.
- **`UNKNOWN-02`** — not a Rust superset. `ButtonTone` is shared across the
  button family, and the union of `button.md` and `icon-button.md` is exactly
  the four Rust members. Ruled: one canonical `ButtonTone`
  (`docs/contracts/004-shared-control-types.md`), all four tones for all three
  components, delivered in every runtime. `FIX-BTN-08` unblocked.

---

## R6 — Schema requirements discovered after spec 063 was written

Two requirements this session's evidence produced that the spec does not yet
carry. Both are `g13.002` inputs, recorded here and in spec 063's planning
notes:

1. **Shared types are first-class.** `ButtonTone` fragmented across three
   contracts because nothing defined it once. `g13-b007` then found
   `OverlayPlacement` fragmented the same way across three more, plus **8
   enumerated shared types with no definition anywhere in `docs/`**
   (`DockEdge`, `DockSizing`, `DockCollapsedPosture`, `DockEmphasis`,
   `ColorInputMode`, `AudioAutomationState` ×3). The IR needs a shared-type
   layer, not just per-component prop lists.
2. **Per-component permitted subsets of a shared type.** `ButtonSpec` accepts
   `ButtonTone::Success` while `button.md` permitted only three tones; the
   inverse held for IconButton. The IR must express "this component permits
   this subset of this shared type" as a first-class constraint, and the
   generated artifacts must carry it. This is the single most direct piece of
   evidence that a shared source would have prevented real drift.

`g13-b007`'s inventory (21 disagreements, 8 unresolved types across 16
components) is **IR requirements evidence, not a remediation backlog.** Do not
open a manual parity programme against it. It is the measure of what one source
must eliminate.

---

## Exit

- `g13.001` acceptance is evidenced; the milestone can move to complete.
- `g13.002` has exact package paths (R1), dependency constraints (R2), inputs
  (R4, R6), and stop conditions.
- Its first worker-ready card is
  [`011-poodle-ir-schema-core.md`](011-poodle-ir-schema-core.md).
- No unresolved design question remains open for `g13.002`'s first card. The
  ones that surfaced later — declarative focus intent (deferred from `b009`),
  and whether the pilot inverts existing dependencies — are `g13.003+` and
  `g13.008` respectively, and are named rather than delegated.
