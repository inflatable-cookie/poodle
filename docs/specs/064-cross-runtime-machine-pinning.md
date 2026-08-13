# 064 Cross-Runtime Machine Pinning

Status: active
Updated: 2026-08-13
Owner: Poodle core
Depends on: `../architecture/006-headless-core-and-machine-model.md`,
`../architecture/001-poodle-system-shape.md`,
`../contracts/001-working-rules.md`

## Purpose

Close the cross-runtime drift holes the g13 pilot proved a vocabulary IR
cannot close (`../roadmaps/g13/pilot-verdict-evidence.md`). The two authority
pairs — `poodle-core` for Svelte + React, `poodle-render` for GPUI +
Jetstream — stay two, and are pinned to each other **by execution rather
than by description**. Every hole is a failing gate, not a sweep finding.

## Scope

**In scope — the five pinning mechanisms, and nothing else:**

1. **Generated machine interfaces** — one schema declares each machine's
   states, events, effects, and context types; TypeScript and Rust type
   declarations are generated from it. Interface only.
2. **Differential machine traces** — the TS and Rust implementations run
   identical event traces and must produce identical `(state, context,
   effects)`.
3. **Vector completeness gating** — a machine's conformance vector must
   exercise every state, transition, and effect on both implementations; a
   thin vector fails CI.
4. **Capability absence registry** — every runtime capability is either
   exercised in parity evidence or declared absent with a reason; no third
   state.
5. **Specimen evidence gates** — scene-authored specimens render in all four
   runtimes and specimen evidence becomes standing gates.

**Out of scope, permanently — the boundary a future card cannot read
around:** compiled or generated behaviour of any kind, any evaluator, and the
expression vocabulary. Generated machine **interfaces** are the only
generated surface. The g13 verdicts are the last word on behaviour:
`../roadmaps/g13/pilot-verdict-evidence.md` (revise) and
`../roadmaps/g13/020-consolidate-and-reassess.md` (retire the vocabulary
authority as a corpus mechanism). Spec `063`'s component half is retired.

## Mechanisms

### 1. Generated machine interfaces

A machine's contract shape — states, events, effects, context types — is
declared once in the machine-interface schema (`machines.json` or its
sibling) and generated into TypeScript and Rust declarations. An event
mismatch becomes a compile error in both languages.

- No transitions, guards, or derivation in the schema. Interface only.
- A machine whose shape cannot be expressed is a reported finding, never
  absorbed by widening the schema into behaviour.
- A gate fails when a machine's interface is not generated from the schema.
- Generated interfaces replace hand-written declarations, pilot machines
  first (`hover`, `menu`, `modal`, `popover`), without renaming any public
  export.

### 2. Differential machine traces

Machines are pure: `transition(state, context, event) → (state, context,
effects)`. Identical event traces through the TS and Rust machines must
produce identical results.

- Traces are written from the contract's Behavior Machine section, hand
  authored per machine, plus generated traces: bounded exhaustive over small
  event alphabets, seeded random beyond that.
- A normalization layer handles payload representation differences only. A
  divergence that cannot be normalized is a finding — likely a capability gap
  for mechanism 4 — not papered over.
- Invariant checks run at every step of every trace.
- A planted divergence in either implementation fails CI, naming the
  machine, the trace, and the diverging step.

### 3. Vector completeness gating

"Pinned" means covered. Being listed in the conformance-vector manifest is
not coverage.

- A machine's vector must exercise every state, transition, and effect. A
  thin vector fails CI.
- Both implementations run each vector.
- A vector that fails on first run is a divergence finding, not tuned away.
- Known-thin vectors close first: `menu`/`popover` dismissal events, `slider`
  two-thumb, `text` (absent entirely).

### 4. Capability absence registry

Capability absence is declared, never silent (working rules
`../contracts/001-working-rules.md` §Shared Implementation). A runtime that
lacks a declared capability is a failing gate until it is implemented or the
absence is declared with a reason and its observable consequence recorded.

- Every capability is either exercised in parity evidence or declared absent
  with a reason — no third state.
- A gate fails when a runtime silently lacks a declared capability or a
  capability is declared nowhere.
- Native holes found by the gate are routed to the closing card, not logged
  and forgotten.

### 5. Specimen evidence gates

Specimen pages are scene-authored once
(`../specs/065-scene-authoring-and-specimen-fixtures.md`) and rendered by all
four runtimes, so implementation differences are diagnosable instead of
confounded by fixture differences. Specimen evidence converts into standing
gates as coverage lands.

## Boundaries

- The pairs stay two. One cross-language source of truth does not return;
  this spec pins pairs, it does not merge them.
- The scene system carries fixtures only. Fixtures bind literals and declared
  axes; nothing executes.
- No component-surface codegen, no new framework targets, no evaluator, no
  expression vocabulary.
- Svelte is the reference implementation; a GPUI or Jetstream hole is a port,
  not an accepted delta.

## Stop Conditions

- The schema must grow transitions, guards, or derivation to express a
  machine — widen nothing; the finding is a report.
- A divergence between the implementations cannot be normalized and is not a
  capability absence — record it as a finding, not a tolerance.
- A vector passes only by trimming a state, transition, or effect from the
  contract — stop and fix the contract or the machine.
- The generated surface stops being smaller than the declarations it
  replaces — stop and reassess, do not keep emitting.

## Promotion

Promoted from the g14 roadmap (`../roadmaps/g14/README.md` fixed inputs:
"pinning mechanisms, and nothing else") and the recorded verdicts. Normative
here; execution sequencing lives in the roadmap, and implementation cards
(`g14.004`–`g14.009`) build the mechanisms behind these gates.
