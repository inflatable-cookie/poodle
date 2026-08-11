# g13.008 Pilot Verdict And Architecture Promotion

Status: planned
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

## Next

On adopt, `g13.009`. Otherwise follow the revised runway recorded here.
