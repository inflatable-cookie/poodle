# 022 poodle-codegen Emitter Core

Status: merged (`143c63a1` → `9dab52ac`)
Milestone: `g13.003`
Owner: Poodle core
Branch: `thread/g13-022-poodle-codegen-emitter-core`
Depends on: `g13-b011` (`4a22c8d8`), `g13-b012` (`911fdfd8`), `g13-b015`
(`7878c537`) — all merged
Governing refs: `docs/specs/063-rust-authored-component-and-scene-ir.md`
§"Generated Artifact Contract" (`IR-07`, `IR-11`),
`docs/roadmaps/g13/003-deterministic-codegen-and-drift-gate.md`,
`docs/research/value-tracks/tk-deterministic-emission-and-drift-gating.md`,
`docs/roadmaps/g13/batch-cards/003-crate-placement-ruling-and-schema-handoff.md`
(R1, R2)

## Goal

Build the compiler boundary: a `poodle-codegen` crate that turns a validated
`IrModel` into committed TypeScript, plus the `ir:build` / `ir:check` selector
pair that keeps the committed output honest.

This card delivers the **emitter machinery and one target**. The remaining four
targets (JSON schema, registry, conformance vectors, docs fragments) are a
follow-up card and are out of scope here — the machinery is the risk, not the
target count.

## Fixed By Ruling (do not re-decide, do not extend)

`g13-b015` gathered the evidence and made no recommendation, as instructed.
These are the orchestrator's rulings on its five questions. Each cites the
evidence that decided it.

### R1 — Invocation shape: standalone `lib` + `[[bin]]`

Not `build.rs`, not a proc-macro. `g13-b003` R1 already placed the crate at
`packages/codegen/` with this shape; `b015` confirms it is the only shape that
works.

The deciding property is **read-only capability**. `ir:check` must regenerate
in isolation and fail on drift without touching the worktree. A build script
is expected to write `OUT_DIR` and cannot fail read-only cleanly. A proc-macro
cannot write at compile time at all — `ts-rs`' own docs say so, which is why
its export path is a `#[cfg(test)]` test — and its output is never committed,
so drift gating degrades to "did it compile".

Supporting: standalone is the only shape in this repo (18 generation/drift
selectors, zero `build.rs` files), and the `--check` pattern is already proven
twice (`build-tokens.ts`, `build-default-icons.ts`).

### R2 — Formatting authority: the emitter owns every byte

The `45caae82` failure was two formatters disagreeing — the token emitter's
string templates versus rustfmt's wrapping. A drift gate with a second
formatter on either side is a flaky gate.

So: **no external formatter runs over emitted output.** The emitter owns every
whitespace decision, exactly as `formatF32` already owns number formatting in
the token build. This is cheap here because the target is TypeScript and the
repo has no JS formatter config at all (no prettier, dprint, biome, or
editorconfig — verified).

If a later target emits Rust, that target — not this card — runs a pinned
`rustfmt` as the emitter's **final step**, so that regenerated output equals
formatted output by construction, and the check compares post-format bytes.
Do not introduce a Rust target here.

### R3 — Drift gate: read-only, byte-exact, orphan-aware, and honestly composed

`ir:check` must:

- Regenerate into memory or a temporary location and compare **byte-exact**
  against committed files. Never write to the worktree, including on failure.
- Report **every** drifted path, not the first — matching the drift gates and
  `validate`'s all-findings-at-once shape.
- Classify whitespace-only differences separately from content differences.
  `b015` failure mode 3: formatting divergence masquerading as content drift
  turned a one-line fix into an investigation.
- Detect **stale orphans** — committed files under the output root that the
  emitter no longer produces. `build-default-icons.ts` already has this
  pattern; `build-tokens.ts` does not, and that is failure mode 5.
- Exit non-zero on any of the above, and leave `git status --porcelain` empty.

And the composition rule, which is what actually failed in `45caae82`:
**a gate composes only `*:check` / `audit:*` selectors, never a write-mode
generator.** `docs:check` today reaches `tokens:build` (write mode) through
`report:parity`, which is precisely why the drift went undetected. Wire
`ir:check` into the gate; never `ir:build`.

### R4 — No type-mirroring dependency

Neither `ts-rs` nor `schemars` is adopted. Both are **type-mirroring derives**:
they emit from Rust type *structure*. The IR emits from *instances* — a
component's permitted subset of a shared type, a registry, a conformance vector
are all values, not types. `PermittedSubset<ButtonTone>` would have to be
authored as a type before `ts-rs` could see it, which inverts the design.

`typeshare` and `specta` remain **unevaluated** — not vendored on this machine,
so `b015` correctly refused to describe them from memory. Unevaluated is not a
rejection, but it does mean: do not adopt either sight-unseen in this card.

Adding any of these four crates to the manifest is a stop condition.

### R5 — Report-artifact gating stays out of scope

The three report generators have no check mode. Whether they become gated
artifacts or stay non-gated derived docs is a real question and is **not this
card's**. Do not touch them.

## Scope

### In scope

- The `poodle-codegen` crate at `packages/codegen/`, `lib` + `[[bin]]`, with
  the manifest posture ruled in `b003`: `publish = false`, public-intent
  **false**, channel `internal`, stability `internal-tooling`,
  release-manifest kind `tooling`.
- Dependencies: `poodle-ir` plus serialization/emit deps only. Nothing depends
  on `poodle-codegen` — it is a tool.
- An emission core: a target abstraction, byte-exact writing, the generated
  header, deterministic ordering, and the check/write mode split.
- **The generated header**, per `IR-07`: authored source path, IR schema
  version, generator version. No timestamp, no absolute path, no machine or
  user identifier, no environment-dependent value — those are the
  nondeterminism this whole card exists to exclude.
- **One target: TypeScript.** Types for a component's prop surface, emitted
  from an `IrModel` instance. Must type-check standalone with no framework
  dependency — no Svelte, React, or DOM types.
- Effigy selectors `ir:build` (write) and `ir:check` (read-only), following the
  `tokens:build` / `audit:tokens` shape in `tasks/effigy.tasks.toml`, with the
  same style of comment explaining why the artifacts are committed.
- Tests: two clean generations byte-identical; check-mode drift detection;
  check mode leaves the tree unchanged; stale-orphan detection; malformed and
  invalid-IR input handled without panic; the generated TypeScript
  type-checks.

### Out of scope — stop conditions if reached

- The other four emitters (JSON schema, registry, conformance vectors, docs
  fragments). Follow-up card.
- Any Rust-emitting target, and therefore any formatter integration.
- Authoring real component definitions. `Button`, `TextInput` and
  `RangeSlider` belong to `g13.005`–`g13.007`. Fixtures stay synthetic.
- Replacing, rewiring, or regenerating any existing generated artifact —
  tokens, icons, reports, contracts, registries. This card adds a new
  generator; it migrates nothing.
- Wiring `ir:check` into `docs:check` or `ci-web`. The gate is added when there
  is a real artifact to gate; adding it now would gate an empty set and give
  false assurance. Add the selectors only.
- Macros. `IR-12` still says ordinary Rust first.
- Changing `poodle-ir`, beyond additive accessors if emission genuinely needs
  one — and if it does, say so in the log.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read the `b015` value-track document before designing anything. Its failure
  table is the test list: each of failure modes 1–10 either has a test here or
  a one-line note in your log saying which later card owns it.
- Do not re-decide R1–R5. If the implementation appears to require breaking
  one, stop and report — that is a ruling error worth knowing about, not
  something to work around.
- Do not add `ts-rs`, `schemars`, `typeshare`, or `specta`.
- Do not add `poodle-codegen` to any other crate's manifest.
- Determinism is the acceptance criterion, so prove it rather than asserting
  it: generate twice and compare bytes, in a test.
- `cargo fmt --check` first. `cargo fmt` has previously reformatted unrelated
  linked workspaces (`PAPERCUTS.md`, 2026-08-10); if `--check` reports files
  outside `packages/codegen/`, stop and report rather than formatting.
- Register the crate in the release manifest, operations doc, and release
  notes. `b011`'s card omitted this and the orchestrator had to do it after
  the fact — do not repeat that.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-022-poodle-codegen-emitter-core`. Do not merge.

## Writable Paths

- `packages/codegen/**` (new crate)
- `tasks/effigy.tasks.toml` (the two new selectors only)
- Release manifest, operations doc, and release notes — crate registration
  entries only
- `docs/logs/2026-08/<DD>-g13-022-poodle-codegen-emitter-core.md`
- `PAPERCUTS.md` (new, non-duplicate friction only)

Any other changed path is a scope failure. In particular: no `packages/svelte`,
no `packages/react`, no `packages/contracts/ir` beyond additive accessors, no
existing generated artifact.

## Steps

1. Baseline. `cargo test` and `cargo clippy -- -D warnings` on `poodle-ir`,
   `effigy docs:lint`, `git diff --check`, `git status --porcelain`. Record
   exit states.
2. Read the `b015` value track in full, then `packages/contracts/ir/src/`, then
   `packages/tokens/scripts/build-tokens.ts` (the primary in-repo precedent —
   particularly its `compare`, `writeFile`, and check-mode paths) and
   `scripts/build-default-icons.ts` (for stale-orphan detection).
3. Create the crate with the ruled posture and dependency set.
4. Build the emission core: target abstraction, header, deterministic ordering,
   byte-exact compare, write/check split. Check mode must be structurally
   incapable of writing — make it a type-level or API-level property, not a
   discipline.
5. Implement the TypeScript target against a synthetic fixture.
6. Add the two Effigy selectors.
7. Tests, covering at minimum: double-generation byte identity; drift detected;
   check leaves the tree clean; stale orphan detected; malformed input does not
   panic; whitespace-only drift classified as such; emitted TypeScript
   type-checks with no framework dependency.
8. Validate:
   ```sh
   cargo build --manifest-path packages/codegen/Cargo.toml
   cargo test --manifest-path packages/codegen/Cargo.toml
   cargo clippy --manifest-path packages/codegen/Cargo.toml -- -D warnings
   cargo fmt --manifest-path packages/codegen/Cargo.toml -- --check
   cargo test --manifest-path packages/contracts/ir/Cargo.toml
   effigy ir:build
   git status --porcelain          # must be empty or show only intended output
   effigy ir:check
   git status --porcelain          # must be byte-identical to the line above
   effigy docs:lint
   effigy docs:check
   git checkout -- packages/tokens/artifacts/rust/
   git diff --check
   ```

## Acceptance Criteria

- [ ] `packages/codegen/` is a `lib` + `[[bin]]` crate with the ruled manifest
  posture, depending on `poodle-ir` and serialization only, and depended on by
  nothing.
- [ ] Two consecutive clean generations are byte-identical, proven by test.
- [ ] `ir:check` fails on drift, reports every drifted path, classifies
  whitespace-only drift, detects stale orphans, and leaves the worktree
  unchanged — each proven by test.
- [ ] The generated header carries source path, IR version, and generator
  version, and carries no timestamp, absolute path, or machine identifier.
- [ ] Emitted TypeScript type-checks with no framework dependency.
- [ ] No type-mirroring crate added. No existing generated artifact touched.
- [ ] Crate registered in the release manifest, operations doc, and release
  notes.
- [ ] All step-8 commands exit 0.
- [ ] Batch log records commands, exit states, and — for each of `b015`'s ten
  failure modes — either the test covering it or the card that owns it.

## Stop Conditions

- Emission appears to require a non-additive change to `poodle-ir`.
- Byte-identical double generation cannot be achieved. Report the source of
  nondeterminism; do not paper over it by normalising in the comparison.
- Check mode cannot be made structurally unable to write.
- Any ruling R1–R5 appears to be wrong. Say which, and why, with evidence.

Stop with exact paths, commands, and the smallest unresolved question.
