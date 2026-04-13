# 001 - Working Rules

Status: active
Owner: Poodle core
Depends on: `docs/architecture/product-guardrails.md`

## Contract

- Treat `docs/roadmaps/`, `docs/specs/`, and `docs/logs/` as the execution
  authority chain for active Poodle work.
- Use `docs/specs/` as the strict planning and execution-control layer when the
  roadmap alone is not enough to keep the next owner honest.
- In a strict lane, a bare `continue` should resolve through the previous
  closeout's `Next Task`, which should point at the current ready card or an
  explicit planning gate.
- If there is no ready card, the lane is in planning. Do not improvise from a
  dirty worktree or the most recent chat summary.
- When multiple plausible next seams exist inside `g10`, freeze the active
  posture first, then choose the next owner deliberately.
- Keep currentness surfaces aligned so completed cards do not remain advertised
  as ready.

## Current Posture

Poodle is in a strict-paused `g10` posture until the next active owner is
chosen cleanly between the Jetstream lane and the GPUI follow-on queue.

## Next Task

Execute the strict planning gate in `docs/specs/062-g10-strict-posture-and-next-boundary-gate.md`.
