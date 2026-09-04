# AGENTS

Scope: whole `poodle/` repository.

## Hard Rules

- Treat `docs/` as the authority for Poodle vision, architecture, roadmap, and execution status.
- Keep Poodle focused on generalized tokens, primitives, and reusable composites; app-specific DAW widgets stay in their owning products.
- Keep Svelte, React, shared Rust composition, and GPUI aligned to one documented contract. Parity means semantic inputs, states, behavior, and token usage first. Jetstream follows its current admission status in the working rules.
- Underlay and its applications import Poodle's published packages directly; any translation lives in the consumer. Poodle carries no consumer-named directory or adapter (architecture 001, operator decision 2026-09-02).
- Treat Bits Svelte as an implementation detail, not public contract authority.
- Never run `*-windowed` conformance selectors locally without explicit operator approval; use the headless `effigy ci:conformance` path.
- Before v1.0, do not add compatibility shims, aliases, or silent fallbacks. Stop and ask before a breaking migration.

## Agent Workflow

- Use Northstar for planning, runway maintenance, worker dispatch, PR review, and closeout. The orchestrator owns the `main` planning checkout; workers use dedicated non-`main` worktrees, push PRs, and never merge.
- Use the repo-local `.agents/skills/effigy/SKILL.md` for task routing and supported execution.
- Prefer harness-managed worktrees. Manual creation requires the operator-selected `AGENTS_WORKTREE_CONTAINER_DIR` from ignored `.agents.local.env`; never guess a temporary or repository-adjacent path. See `docs/contracts/005-agent-local-paths.md`.
- Record small solvable execution friction in root `PAPERCUTS.md`; do not turn it into unplanned work.

## Validate

- Use `effigy tasks` to find selectors and run the narrow checks relevant to the batch.
- Use `effigy qa` for the broad headless repository board.
- Run `git diff --check` before handoff.
- Do not edit `.github/workflows/` or run release mutations without explicit operator approval.

## Canonical References

- `README.md`, `docs/README.md`, `docs/vision/001-poodle-vision.md`
- `docs/architecture/001-poodle-system-shape.md`, `docs/architecture/product-guardrails.md`
- `docs/contracts/001-working-rules.md`, `docs/contracts/005-agent-local-paths.md`
- `docs/roadmaps/README.md`, `docs/specs/README.md`
- `docs/policy/internal-writing-style.md`
