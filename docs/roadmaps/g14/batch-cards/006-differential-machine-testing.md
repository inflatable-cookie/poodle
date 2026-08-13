# 006 Differential Machine Testing

Status: ready
Milestone: `g14.005`
Owner: Poodle core
Branch: `thread/g14-006-differential-machine-testing`
Depends on: `g14.004` (merged)
Governing refs: `docs/specs/064-cross-runtime-machine-pinning.md` (mechanism
2), `docs/roadmaps/g14/005-differential-machine-testing.md`,
`docs/architecture/006-headless-core-and-machine-model.md`,
`docs/roadmaps/g14/g14-baseline-manifest.md` (the named hole class: TS↔Rust
behaviour divergence on events the shared vector never fires)

## Goal

Replace "each implementation against prose" with "the two implementations
against each other." Machines are pure:
`transition(state, context, event) → (state, context, effects)`. Identical
event traces through the TS and Rust machines must produce identical
results. This is the mechanism that closes the baseline's named hole class.

## Fixed By Ruling (recorded — do not re-decide)

- **R1 — The pilots are the four generated-interface machines:** `hover`,
  `menu`, `modal`, `popover`. The differential harness starts there; the
  other 17 machines are follow-on cards, not this one.
- **R2 — The harness is a new Effigy task, not new product code.** It runs
  both implementations as tests (bun for TS, cargo for Rust) against
  shared trace files, and a runner diffs the outcomes. The traces live in
  `packages/contracts/headless/differential/` so both harnesses read one
  corpus.
- **R3 — Divergence is a finding first.** A trace that diverges is
  reported with the exact step, event, and both outcomes; a genuine
  pre-existing divergence (the two implementations already differ) is
  reported and its trace marked `known-divergence` with the reason — do
  not tune the trace to hide it, do not "fix" the machine in this card
  unless the fix is a typo-class error. Real divergences become findings
  for the maintainer to route.
- **R4 — Traces come from the contract, plus generation.** Hand-authored
  traces written from each machine's Behavior Machine section; generated
  traces bounded-exhaustive over the event alphabet (depth-capped) and
  seeded random beyond. Invariants (e.g. emitOpenChange matches open
  state) are checked at every step.

## Deliverables

- The shared trace corpus format (JSON, declarative) and the four pilot
  machines' hand-authored traces.
- The TS runner (bun test over `packages/core/test/differential/`) and the
  Rust runner (cargo test over `packages/contracts/headless/tests/differential/`),
  both executing the same trace files and emitting a comparable outcome
  log.
- The differ: compares TS and Rust outcome logs per trace, exits 1 on
  divergence, names the machine, the trace, and the diverging step.
- Generated traces for the four pilots (bounded exhaustive + seeded
  random).
- An `effigy docs:differential` selector wired into CI (runs both runners
  + the differ).
- A planted divergence caught in both directions (TS-only and Rust-only)
  as the proof.

## Acceptance

- [ ] Planted divergence fails in both directions, naming the machine,
  trace, and step.
- [ ] All four pilot machines run the harness with zero unmarked
  divergences.
- [ ] Every genuine divergence found is recorded as a finding with the
  exact diff, not silently marked known.
- [ ] `effigy ci:rust`, `effigy test:core`, `effigy docs:differential`,
  `git diff --check` exit 0.

## Stop Conditions

- A machine's outcome cannot be serialized comparably across the boundary
  (e.g. an effect payload shape has no normalization) — report it as a
  finding (likely a `g14.007` capability-absence class) and leave that
  machine's traces marked; do not weaken the comparison.

## Writable Paths

- `packages/contracts/headless/differential/**` (new: traces + runner)
- `packages/core/test/differential/**` (new: TS runner)
- `packages/contracts/headless/tests/differential/**` (new: Rust runner)
- `tasks/effigy.tasks.toml` (new selector; append only)
- `docs/logs/2026-08/14-g14-006-differential-machine-testing.md`
- `PAPERCUTS.md` (append only)

## Steps

1. Reset per the Thread Reuse Protocol; baseline
   `effigy test:core`, `effigy ci:rust`, `git diff --check`.
2. Read spec 064 mechanism 2, the baseline manifest's hole-class section,
   and the four machines' Behavior Machine sections in their contracts.
3. Define the trace format; author hand traces for the four pilots from
   the contracts.
4. Build both runners against the shared corpus; wire the differ; add the
   Effigy selector.
5. Add generated traces (bounded exhaustive, then seeded random); run
   until the four pilots are divergence-free or every divergence is a
   recorded finding.
6. Plant a divergence in each direction and prove the differ catches both.
7. Validate the acceptance list; write the batch log; push with
   `git push -u origin thread/g14-006-differential-machine-testing`. Do
   not merge.
