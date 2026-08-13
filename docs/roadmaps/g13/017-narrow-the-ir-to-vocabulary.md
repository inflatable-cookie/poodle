# g13.017 Narrow The IR To Vocabulary

Status: ready
Owner: Poodle core
Depends on: `g13.008` (verdict **revise**, recorded 2026-08-13)

## Objective

Cut the IR to the scope the pilot actually demonstrated, and make the boundary
hard enough that it cannot creep back.

## Deliverables

- Remove `expr.rs` and the expression vocabulary. It is 331 lines with no
  evaluator; one emitter target mentions `Expr` twice. It exists to describe
  behaviour the verdict says the IR does not describe.
- Remove emitter surface that only served the generative ambition. Measure what
  is left rather than estimating it.
- State the scope boundary in `docs/specs/063-*.md` in terms a future card
  cannot read around: vocabulary in, behaviour out, no evaluator.
- Keep every current artifact byte-identical, or explain each byte that moves.

## Acceptance

- `ir:build` / `ir:check` still pass; every generated artifact is unchanged or
  its change is justified in the log.
- The four-runtime propagation proof still runs.
- Removed LOC is reported against the ≈31,400-line pilot total.
- No component behaviour changes. This is subtraction only.

## Next

`g13.018` applies the two amendments the pilot named.
