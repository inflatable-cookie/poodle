# 004 Rust IR Prior-art And Failure Audit

Status: ready
Milestone: `g13.002` research precursor
Owner: Poodle core
Branch: `thread/g13-rust-ir-prior-art`
Governing refs: `docs/specs/063-rust-authored-component-and-scene-ir.md`
(`IR-01`–`IR-08`, `IR-11`–`IR-12`),
`docs/research/research-to-implementation-playbook.md`

## Goal

Gather primary-source evidence on Rust-authored declarative IR and deterministic
TypeScript/schema generation. Expose failure modes before g13.002 chooses
libraries or schema mechanics.

This is a research batch. It makes no Poodle architecture recommendation and
does not modify implementation or authority documents.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents or parallel research tasks.
- Use primary sources: official project documentation, repositories, source,
  issue trackers, and language/tool documentation. Label inference explicitly.
- Do not edit specs, architecture, contracts, roadmaps, dispatch state, code,
  package manifests, lockfiles, or generated artifacts.
- Write only the research value track, batch log, and valid PAPERCUTS entries.
- Do not recommend a winner. Present evidence and bounded tradeoffs; the
  orchestrator decides after the local authority inventory returns.
- Commit and push the worker branch. Do not merge.

## Scope

### In scope

- `docs/research/value-tracks/tk-rust-authored-ui-ir.md`
- `docs/logs/2026-08/11-g13-rust-ir-prior-art.md`
- Evidence for Rust type/serialization and generation approaches relevant to:
  - tagged unions, recursive trees, stable IDs, defaults, optionality, and
    schema versioning
  - Rust-to-TypeScript and Rust-to-JSON-Schema emission
  - deterministic ordering/output and source-linked diagnostics
  - build-script, proc-macro, library, and standalone-CLI boundaries
  - committed generated artifacts and read-only drift checking
  - limits around generics, lifetimes, traits, custom serde, executable
    behavior, framework lifecycle, and cross-language compatibility
- At minimum inspect `serde`, `schemars`, `ts-rs`, `typeshare`, and `specta`.
  Add another project only when it contributes a distinct proven pattern.

### Out of scope

- A Poodle schema proposal, crate placement, dependency selection, macro
  syntax, proof-of-concept code, benchmarks, or implementation.
- General UI-framework comparisons unrelated to compiler/IR boundaries.
- Secondary-source popularity summaries or unsourced claims.

## Steps

1. State the Poodle decision questions from IR-01–IR-08 without assuming the
   answer.
2. For each candidate/pattern, cite version/date and primary sources.
3. Build a comparison table covering model fidelity, customization,
   deterministic generation, diagnostics, versioning, dependency/build shape,
   maintenance signal, and known limitations.
4. Extract concrete failure cases and the smallest pilot test that would expose
   each one in Poodle.
5. Separate verified facts, source-author claims, and worker inference.
6. Record unresolved questions for local evidence; make no recommendation.
7. Run docs formatting/link checks available without changing generated data,
   then commit and push.

## Acceptance Criteria

- [ ] Every material claim has a primary-source citation and access date.
- [ ] All five required projects/patterns are compared on the same dimensions.
- [ ] Failure modes cover type fidelity, serialization evolution, diagnostics,
  deterministic output, and executable-behavior boundaries.
- [ ] Each failure mode maps to a bounded Button/RangeSlider/TextInput or
  preview-shell pilot test.
- [ ] Facts, claims, and inference are distinguishable.
- [ ] No recommendation or Poodle authority change appears in the worker
  output.
- [ ] `git diff --check` passes and the batch log records command exit states.

## Stop Conditions

- Primary sources cannot establish a claimed capability or limitation.
- The work requires choosing Poodle's crate placement or schema.
- Research pressure expands into implementation or a dependency spike.

Stop with the missing evidence and exact question. Do not fill gaps from
memory.

## Promotion Target

After review, the orchestrator synthesizes durable findings into spec 063
planning notes or a translation memo before g13.002 becomes executable.
