# 015 Deterministic Emission And Drift-Gating Patterns

Status: ready
Milestone: `g13.003` research precursor (codegen)
Owner: Poodle core
Branch: `thread/g13-015-emission-drift-patterns`
Supersedes: `004-rust-ir-prior-art-and-failure-audit.md` (retired)
Governing refs: `docs/specs/063-rust-authored-component-and-scene-ir.md`
§"Generated Artifact Contract", `IR-07`, `IR-11`,
`docs/roadmaps/g13/batch-cards/003-crate-placement-ruling-and-schema-handoff.md`
(R1, R2)

## Why This Replaces 004

Card `004` was written to expose failure modes *before* `g13.002` chose
libraries or schema mechanics. That window has closed: `g13-b011` chose serde
and delivered tagged unions, stable identifiers, defaults, optionality,
`IR_SCHEMA_VERSION`, deterministic `BTreeSet` ordering and JSON round-trip;
`g13-b012` closed the expression boundary. Researching those decisions now
would produce confirmation or churn.

What remains open is the **codegen fork**, and it is narrower than `004`'s
candidate list implies. `ts-rs`, `typeshare` and `specta` all solve *mirror my
Rust types as TypeScript types*. Poodle does not want that: we are not
exporting `poodle-ir`'s structs, we are emitting component prop surfaces,
registries and evidence **from IR instances**. Type-mirroring a schema crate
would yield a TypeScript definition of `PermittedSubset` and `Expr`, which no
consumer wants.

So this card asks the questions the emitter actually faces, and it mines the
strongest available evidence — **this repository already does one-source,
many-target generation with a drift gate, in production.**

## Goal

Produce the evidence the `poodle-codegen` design needs on three decisions:
emitter invocation shape, deterministic output, and drift gating.

Research only. No recommendation, no implementation, no dependency selection.

## Decision Questions (state without assuming the answer)

1. **Invocation shape.** Standalone CLI binary, Cargo build script (`build.rs`),
   or proc-macro? What does each cost in build coupling, incremental rebuild
   behaviour, CI reproducibility, and ability to run read-only?
2. **Determinism.** What makes emitted output byte-stable across machines and
   runs — ordering, formatting, float and string escaping, line endings,
   timestamps, absolute paths, map iteration order?
3. **Drift gating.** How are committed generated artifacts kept honest? What
   does a `--check` mode need to report to be actionable, and how does it fail?
4. **Diagnostics.** How does an emitter point a failure back at authored source
   rather than at generated output?
5. **Role, if any, for type-mirroring tools.** Given the data-driven emission
   above, is there any part of the job `ts-rs` or `schemars` genuinely serves —
   for example emitting a JSON Schema for the IR artifact itself, as distinct
   from the component surfaces?

## Primary Sources — available offline

**In-repo (the strongest evidence; treat as the main case study):**

- `packages/tokens/scripts/build-tokens.ts` — one DTCG source emitting CSS, TS
  and Rust into `packages/tokens/artifacts/{css,ts,rust}`, with a `--check`
  mode (line 10, `checkOnly`) that compares against committed output instead of
  overwriting. Its own header cites `scripts/build-default-icons.ts --check` as
  the pattern it mirrors.
- `scripts/build-default-icons.ts` — the second `--check` generator.
- Report generators: `packages/svelte/preview/scripts/parity-report.ts`,
  `export-component-docs.ts`, `build-accessibility-report.ts`.
- Drift gates: `contract-prop-drift.ts`, `contract-spec-drift.ts`,
  `contract-role-drift.ts`, `adapter-manifest-drift.ts`, and the new
  `contract-value-domain-drift.ts`. Note their shared conventions —
  `DRIFT_REPORT=1` / `VALUE_DOMAIN_ENFORCE=1` escapes, all-findings-at-once
  reporting, exit codes.
- The Effigy selector surface: 18 generation and drift selectors in
  `tasks/effigy.tasks.toml` (`audit:*`, `report:*`, `drift:*`, `docs:*`).
- **Known failure to analyse, not just cite:** `effigy docs:check` rewrites
  committed `packages/tokens/artifacts/rust/*` because `report:parity` →
  `tokens:build` runs the generator in write mode inside a read-only gate, and
  `audit:tokens` fails at HEAD because `45caae82` hand-formatted generated
  output. Both are in `PAPERCUTS.md`. This is a live, local instance of exactly
  the failure mode this card exists to characterise — give it a full section.

**Vendored crates (read the actual source, cite version + path):**

- `ts-rs` / `ts-rs-macros` — `~/.cargo/registry/src/*/ts-rs-macros-12.0.1`
- `schemars` — `~/.cargo/registry/src/*/schemars-1.2.2`

`typeshare` and `specta` are **not vendored**. Do not describe them from
memory. Record them as unevaluated with the reason.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents.
- **Primary sources only.** Read the actual files and crate sources. Cite
  path, and version where applicable. Do not fill gaps from memory — an
  unevaluated candidate is a finding, not a blank to fill.
- Label every statement as verified fact, source-author claim, or worker
  inference.
- **Make no recommendation.** Present evidence and bounded tradeoffs; the
  orchestrator decides.
- Do not edit specs, architecture, contracts, roadmaps, dispatch state, code,
  manifests, lockfiles, or generated artifacts.
- Do not add a dependency, write a proof of concept, or benchmark anything.
- Two other workers hold all Tabs and AppHeader files. Touch neither.
- Never `git add -A`; stage only your writable paths by explicit path.
- Commit and push with
  `git push -u origin thread/g13-015-emission-drift-patterns`. Do not merge.

## Writable Paths

- `docs/research/value-tracks/tk-deterministic-emission-and-drift-gating.md`
- `docs/logs/2026-08/<DD>-g13-015-emission-drift-patterns.md`
- `PAPERCUTS.md` (new, non-duplicate friction only)

Any other changed path is a scope failure.

## Steps

1. State the five decision questions without presupposing answers.
2. **Case-study the in-repo generators first.** For each: invocation shape,
   what guarantees determinism, whether it has a check mode, what its failure
   output looks like, and where it writes. Quote real code with file:line.
3. Analyse the `docs:check` / `audit:tokens` failure in full — mechanism, why
   it went unnoticed, and what an emitter design would need to prevent it.
4. Read the vendored `ts-rs-macros` and `schemars` sources for what they
   actually generate and what they assume about the input types. Answer
   question 5 from that evidence.
5. Build one comparison table across invocation shapes (CLI / build script /
   proc-macro) on: build coupling, incremental behaviour, read-only capability,
   CI reproducibility, diagnostic quality, and observed in-repo precedent.
6. Extract concrete failure modes. For each, name the **smallest check** that
   would catch it in Poodle — reusing an existing gate where one fits.
7. Record unresolved questions and unevaluated candidates explicitly.
8. `git diff --check`, `git status --porcelain`, and any docs formatting check
   that does not rewrite generated data. Record exit states.

## Acceptance Criteria

- [ ] All five decision questions are addressed with primary-source evidence.
- [ ] Every in-repo generator and drift gate listed above is characterised with
  file:line citations.
- [ ] The `docs:check` / `audit:tokens` failure has its own section covering
  mechanism, detection gap, and prevention.
- [ ] `ts-rs` and `schemars` are assessed from vendored source with versions;
  `typeshare` and `specta` are recorded unevaluated with the reason.
- [ ] One comparison table covers CLI vs build script vs proc-macro on all six
  listed dimensions.
- [ ] Each failure mode maps to the smallest check that would catch it.
- [ ] Verified fact, source-author claim, and worker inference are
  distinguishable throughout.
- [ ] No recommendation, no dependency choice, no code, no benchmark.
- [ ] Only the three writable paths changed; `git diff --check` exits 0.

## Stop Conditions

- A claimed capability cannot be established from an available primary source.
- The work requires choosing the emitter shape, a dependency, or schema
  mechanics.
- Research pressure expands into a proof of concept or a dependency spike.
- Characterising a failure requires modifying a generator or its artifacts.

Stop with the missing evidence and the exact question. Do not fill gaps from
memory.

## Promotion Target

The orchestrator synthesises findings into the `poodle-codegen` card's fixed
rulings — emitter shape, determinism requirements, and the drift gate — before
`g13.003` becomes executable.
