# g15.012 — Primitive-First Visual Conformance Lane

Status: **blocked** — pending orchestrator review of `g15.001`; seam recorded,
harness not designed
Depends on: `g15.001` (measured roster)
Governing refs: `release-baseline-roster.md`,
`../../roadmaps/g14/conformance-estate.md`, `../../contracts/001-working-rules.md`

## Outcome

Establish a bounded, primitive-first visual-conformance lane that reuses the
retained headless/native capture foundation (web snapshot tools under
`test/visual/`, `effigy test:native-visual` with `--control-size`, the retained
headless driver). It is a diagnostic aid for human review, not a new component
authority, not a release prerequisite invented by this card, and not a parity
completion gate. It must not smuggle back the rejected g13/g14 mechanisms.

## Scope

- a primitive-first sequence (foundation display and shell primitives before
  composites)
- one shared capture plan at outline level; runtime capture stays local
- focused visual evidence per component, recorded separately from functional
  evidence

## Goals

- [ ] Define the capture seam against the retained estate and the frozen
      roster; do not design a universal corpus or comparator.
- [ ] Land the first primitive batch of visual baselines with named cases.
- [ ] Record the lane as diagnostic: green baselines never count as component
      completion, and every capture is reviewed by a human.

## Acceptance

- [ ] The lane is bounded to primitives first and named components per batch.
- [ ] No component authority, completion gate, or cross-runtime comparator is
      introduced.
- [ ] Headless capture evidence exists for the first batch; native visual
      compare remains local-only and operator-owned.

## Stop Conditions

- The lane grows a shared corpus, normalized observations, or a completion
  board.
- A capture replaces focused functional evidence.
- Work expands beyond the named batch without a new card.

## Writable Scope

- capture fixtures, baselines, and batch cards
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- web snapshot tooling for the batch
- `effigy docs:check`
- `git diff --check`

Never run a `*-windowed` selector or `test:native-visual` on a local desktop
without explicit operator approval; never run `qa:jetstream` or any Jetstream
selector.
