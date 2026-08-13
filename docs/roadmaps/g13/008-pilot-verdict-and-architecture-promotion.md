# g13.008 Pilot Verdict And Architecture Promotion

Status: complete — verdict **revise**, recorded 2026-08-13
Owner: Poodle maintainer
Depends on: `g13.004`, `g13.005`, `g13.006`, `g13.007`

## Objective

Make the compiler decision from evidence before broad migration.

## Deliverables

- Compare pilot results with the frozen g13.001 baseline.
- Record authored/generated LOC, exception count, diagnostics, build cost,
  semantic fidelity, and four-runtime drift found or removed.
- Choose **adopt**, **revise**, or **reject** using spec 063's criteria.
- On adopt, promote stable authority/lowering rules into architecture 001/006,
  working rules, package docs, and AGENTS references where required.
- On revise or reject, recompile or close `009`–`016`; do not leave them
  appearing executable.

## Acceptance

- The verdict names tradeoffs and failed assumptions, not only green checks.
- Stable architecture never points at a provisional compiler model.
- Broad migration remains locked until the maintainer records **adopt**.

## Verdict

**REVISE.** Evidence and reasoning:
`docs/roadmaps/g13/pilot-verdict-evidence.md`.

Adopt was not available — spec 063's pass condition 5 ("generated code is
smaller and easier to inspect than the duplicated source it replaces") fails
outright, because nothing was replaced: the nine pilot files grew from 3,672 to
4,637 lines against ~31,400 lines of new machinery, removing zero duplication.

Reject was defensible and declined. The propagation and drift machinery works,
was proven live in all four runtimes three times, and is cheap per component
once the scope is vocabulary rather than behaviour.

The IR is kept and narrowed to **one source for cross-runtime vocabulary with
drift gating**. The behavioural ambition is dropped.

## Next

The revised runway is `g13.017`–`g13.020`
(`docs/roadmaps/g13/pilot-verdict-evidence.md` §8), ending in the
consolidate-and-reassess step.

`g13.009`–`g13.016` are **closed, not deferred** — they describe family-by-
family migration to a generative model this verdict declines. They are retained
as evidence and are not executable.
