# 025 poodle-codegen — Remaining Emitters

Status: merged
Milestone: `g13.003`
Owner: Poodle core
Branch: `thread/g13-025-codegen-remaining-emitters`
Depends on: `g13-b022` merged (`9dab52ac`)
Governing refs: `docs/specs/063-rust-authored-component-and-scene-ir.md`
§"Generated Artifact Contract" (`IR-07`, `IR-11`),
`docs/roadmaps/g13/003-deterministic-codegen-and-drift-gate.md`,
`docs/roadmaps/g13/batch-cards/022-poodle-codegen-emitter-core.md`

## Goal

`g13-b022` proved the machinery on one target. Add the other four the
milestone names — JSON/schema, registry, conformance vectors, docs fragments —
and close `g13.003`.

The machinery is settled and re-verified on merged main: two clean generations
are byte-identical, `ir:check` is read-only and reports content drift,
whitespace-only drift, and stale orphans distinctly. **Do not change it.** This
card writes four `EmitTarget` implementations and registers them.

## Fixed By Ruling (do not re-decide)

`g13-b022`'s R1–R5 remain in force. Read that card's "Fixed By Ruling" section
before starting. The three that will actually come up:

- **The emitter owns every byte.** No external formatter over generated output.
  **No Rust-emitting target** — that is what would drag a formatter in, and it
  is out of scope here.
- **No type-mirroring crate.** `ts-rs`, `schemars`, `typeshare`, `specta` all
  emit from Rust type *structure*; these targets emit from model *instances*.
  Adding any of the four is a stop condition. In particular, do not reach for
  `schemars` to produce the JSON Schema — hand-emit it like everything else.
- **Gates compose only checkers.** `ir:build` never enters a gate. `ir:check`
  already covers every registered target automatically; adding a target must
  not require touching `tasks/effigy.tasks.toml`. If it does, say so — that is
  a design flaw worth reporting.

## The Four Targets

Each is an `EmitTarget` with its own `output_root`, so the orphan scan stays
per-target. Content below is the intent; the exact shape is yours to design
within the abstraction.

| Target | `output_root` | Emits |
|---|---|---|
| JSON surface | `json` | One document per component describing its prop surface, shared-type references, permitted subsets, events and axes, plus a stable index. The machine-readable form bridges and validators consume. |
| JSON Schema | `schema` | A JSON Schema for the JSON-surface documents above, so a consumer can validate one without Rust. Hand-emitted. |
| Registry | `registry` | One document listing every component: id, capabilities required, axes supported, shared types referenced. The "what exists" index. |
| Conformance vectors | `conformance` | The corpus `CV` rows as input → expected-output pairs, in a form all four runtimes can execute against the same file. This is the artifact that makes cross-runtime agreement checkable rather than asserted. |
| Docs fragments | `docs` | Markdown prop-surface fragments suitable for inclusion in component contracts. Fragments only — do not rewrite any existing contract. |

That is five rows for four bullets because the milestone's "JSON/schema" is two
artifacts. Ship all five.

## Scope

### In scope

- Five `EmitTarget` implementations under `packages/codegen/src/targets/`,
  registered in `targets::all()` in stable order.
- Fixture extension **only if needed** to exercise a target — for example the
  synthetic model may need a capability or a conformance vector to have
  anything to emit. Keep it synthetic.
- Committed output under `packages/codegen/generated/<root>/` for each.
- Tests per target: byte-identical double generation, drift detected,
  orphan detected, malformed input handled without panic.
- A test proving the milestone's acceptance directly: **one fixture change
  updates every declared artifact in one `ir:build`.**
- A test proving emitted JSON validates against the emitted JSON Schema.

### Out of scope — stop conditions if reached

- Any change to `emit.rs`, `check.rs`, `write.rs`, or `error.rs` beyond what a
  new target strictly requires. The machinery is proven; if a target needs a
  machinery change, stop and report rather than reshaping it.
- Any Rust-emitting target, and therefore any formatter integration.
- Authoring real component definitions. `Button`, `TextInput` and
  `RangeSlider` belong to `g13.005`–`g13.007`.
- Replacing or regenerating any existing artifact outside
  `packages/codegen/generated/`. Do not touch tokens, icons, reports,
  contracts, or registries elsewhere in the repo.
- Wiring `ir:check` into `docs:check` or `ci:web`. Still deferred until real
  component definitions exist — gating an empty synthetic set gives false
  assurance.
- `poodle-ir` changes beyond additive accessors. Say so in the log if you add
  one.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read `packages/codegen/src/emit.rs`, `check.rs`, and `targets/ts.rs` first.
  `ts.rs` is the reference implementation — match its structure, its
  doc-comment convention, and its determinism discipline.
- Deterministic ordering everywhere: sort by a stable key, never rely on map
  iteration order. This is the single most likely way to break the gate.
- Do not add `ts-rs`, `schemars`, `typeshare`, or `specta`.
- Do not add `poodle-codegen` to any other crate's manifest.
- `cargo fmt --check` first; if it reports files outside `packages/codegen/`,
  stop and report rather than formatting. Note that generated token artifacts
  are **not** rustfmt-formatted by design (`5854634c`) — leave them alone.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-025-codegen-remaining-emitters`. Do not merge.

## Writable Paths

- `packages/codegen/**`
- `docs/logs/2026-08/<DD>-g13-025-codegen-remaining-emitters.md`
- `PAPERCUTS.md` (new, non-duplicate friction only)

Any other changed path is a scope failure.

## Steps

1. Baseline: `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`
   on `packages/codegen`; `effigy ir:check`; `effigy docs:lint`;
   `git diff --check`; `git status --porcelain`. Record exit states.
2. Read `emit.rs`, `check.rs`, `targets/ts.rs`, then `poodle-ir`'s model.
3. Implement the targets one at a time, each with its tests, regenerating and
   committing its output as you go. Do not write all five then debug.
4. The cross-cutting tests: one fixture change updates every artifact; emitted
   JSON validates against the emitted schema.
5. Validate:
   ```sh
   cargo build --manifest-path packages/codegen/Cargo.toml
   cargo test --manifest-path packages/codegen/Cargo.toml
   cargo clippy --manifest-path packages/codegen/Cargo.toml -- -D warnings
   cargo fmt --manifest-path packages/codegen/Cargo.toml -- --check
   effigy ir:build
   git status --porcelain     # only intended generated output
   effigy ir:check            # exit 0
   git status --porcelain     # byte-identical to the line above
   effigy docs:lint
   effigy gate:clean
   git diff --check
   ```

## Acceptance Criteria

- [ ] Five targets registered, each with its own `output_root` and committed
  output.
- [ ] Two consecutive clean generations are byte-identical across all five,
  proven by test.
- [ ] `ir:check` detects content drift, whitespace-only drift, and stale
  orphans **per target**, and still never writes.
- [ ] One fixture change updates every declared artifact in one `ir:build`,
  proven by test — the milestone's acceptance criterion.
- [ ] Emitted JSON validates against the emitted JSON Schema, proven by test.
- [ ] No type-mirroring crate; no machinery reshaping; no artifact outside
  `packages/codegen/generated/` touched.
- [ ] All step-5 commands exit 0.
- [ ] Batch log records commands, exit states, and — for any `b015` failure
  mode still uncovered — which card owns it.

## Stop Conditions

- A target needs a change to the shared machinery.
- Byte-identical double generation cannot be achieved. Report the source of
  nondeterminism; do not normalise it away in the comparison.
- Registering a target requires editing `tasks/effigy.tasks.toml`.
- The conformance-vector shape cannot express a corpus `CV` row. Give the row
  and what is missing.

Stop with exact paths, commands, and the smallest unresolved question.
