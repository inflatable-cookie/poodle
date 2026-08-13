# 14 — g14.001 Record The Verdict And Pinning Doctrine (batch log)

Branch: `t3code/record-verdict-pinning-doctrine`
Date: 2026-08-13
Card: `docs/roadmaps/g14/batch-cards/001-record-verdict-and-pinning-doctrine.md`
Closes: `g14.001` (milestone status flip belongs to the orchestrator's
front-door rollover — not in this card's writable paths)

Docs-only card: promote the g13 verdict's durable outcomes and the pinning
doctrine into stable architecture and working rules, write the normative
specs for the pinning line. No code, no gates, no behaviour.

## 1. Baseline (step 1)

Clean tree on the dispatch branch (carries the card file only). All green:

| Command | Exit |
|---|---|
| `effigy docs:lint` | 0 |
| `git diff --check` | 0 |

## 2. Architecture 006 — the machine model gains the pinning contract

`docs/architecture/006-headless-core-and-machine-model.md` gains a Pinning
Contract section under Cross-Runtime Contract. The dual-layer shape and the
pure-machine model are untouched; the machine model additionally holds the
three machine-level pins, each a sentence, each citing the verdicts:

- **Machine interfaces generated, not hand-mirrored** — one schema, TS +
  Rust declarations; interface only, no transitions/guards/derivation.
- **Differential traces as the cross-pair equivalence check** — identical
  `(state, context, effects)` from identical event traces; prose defines
  intent, traces define equivalence.
- **Vector completeness as a gate** — full-surface vectors, thin vector
  fails CI.

The pair-level mechanisms (capability absence registry, specimen evidence
gates) are deferred to spec 064 with a pointer, not restated here.

## 3. Architecture 001 — pair-wise authority is the durable shape

`docs/architecture/001-poodle-system-shape.md` gains an Authority Pairs
section after Contract Authority: `poodle-core` for the web pair,
`poodle-render` for the native pair. The cross-language authority ambition
is recorded **closed**, citing the verdict's measurement (≈31,400 lines of
machinery, +965 consumer lines, zero duplication removed); the drift-gate
estate is the corpus-wide authority; the pairs stay two, pinned by execution
(spec 064).

## 4. Working rules 001 — absence is declared, never silent

Two surgical amendments, both citing the verdict:

- §Shared Implementation gains the rule: capability absence is declared with
  a reason, never inferred from silence; a native hole is a **failing gate,
  not a sweep finding**.
- §Runtime Parity Authority: the "gap to port" bullet now reads "a failing
  gate until it is ported or declared absent with a reason" — no third
  state.

## 5. Spec 064 — cross-runtime machine pinning (new, normative)

`docs/specs/064-cross-runtime-machine-pinning.md` written from the g14
roadmap's mechanisms, not invented ones: the five pins (generated machine
interfaces, differential machine traces, vector completeness gating,
capability absence registry, specimen evidence gates), each with its gate
shape from `g14.004`–`g14.009`. Scope, boundaries, and stop conditions are
stated so a future card cannot read around them:

- **The boundary:** interfaces in, behaviour out, no evaluator, no
  expression vocabulary. Generated machine interfaces are the only
  generated surface; spec 063's component half is cited as retired.
- **Stop conditions** name the four failure modes that end a pin (schema
  growing behaviour, unnormalizable divergence, vector trimming the
  contract, generated surface not smaller than what it replaces).

## 6. Spec 065 + spec 063 retirement — the split

- `docs/specs/065-scene-authoring-and-specimen-fixtures.md` (new, active):
  the scene half promoted — scene IR as the one fixture authority, one
  authored scene rendered by all four runtimes, no evaluator (fixtures bind
  literals and declared axes), no application framework. Carries the
  measured-tranche migration pointer (`g14.003`).
- `docs/specs/063-...md` status → **retired — pilot record (g14.001)**; a
  Retirement section at the top records the split and points at both
  successors: 065 for the scene half, and for the component half the pinning
  doctrine (064 + architecture 001/006 + working rules 001). Body retained
  verbatim as the pilot's record — nothing below the notice was rewritten.

## 7. Specs index (step 6)

`docs/specs/README.md` updated: 063's entry reads retired pilot record with
both successors; 064 and 065 added to the current cross-cutting references.

## 8. Validation (step 7)

| Command | Exit |
|---|---|
| `effigy docs:lint` | 0 |
| `git diff --check` | 0 |

No code, task, or gate file touched — the diff is docs-only.

## 9. Acceptance

- [x] Architecture 001/006 and working rules 001 amended; nothing
  provisional promoted into them — every amendment cites the recorded
  verdict.
- [x] Spec 064 active; spec 065 active; spec 063 retired with the verdict
  cited (§5–6).
- [x] The boundary is stated in 064 so a future card cannot read around it:
  interfaces in, behaviour out, no evaluator, no expression vocabulary.
- [x] `effigy docs:lint` and `git diff --check` pass.
- [x] No code, task, or gate file touched.

## 10. Stop conditions / papercuts

No stop condition hit: no architecture doc contradicted a fixed input (001
had no cross-language authority claim to reconcile; 006's model sections
needed no rewrite), and nothing required inventing behaviour the verdict did
not record. No `PAPERCUTS.md` entry needed.
