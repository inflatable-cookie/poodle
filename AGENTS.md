# AGENTS

Scope: whole `poodle/` repository.

## Hard Rules

- Treat `docs/` as the authority for Poodle vision, architecture, roadmap, and execution status.
- Keep Poodle focused on generalized tokens, primitives, and reusable composites; do not fold app-specific DAW widgets into the core library.
- Keep Svelte and GPUI implementations aligned to one documented contract; parity means semantic inputs, states, behavior, and token usage first.
- Keep Underlay-facing integration behind Underlay-owned adapters and token bridges; app code should not need to know Poodle exists.
- Treat Bits Svelte as an implementation detail where useful, not as Poodle's public contract.
- Keep AGENTS content lean: scope, hard rules, validation, links.

## Validate

- `git diff --check`

## References

- `README.md`
- `docs/README.md`
- `docs/vision/001-poodle-vision.md`
- `docs/architecture/001-poodle-system-shape.md`
- `docs/architecture/product-guardrails.md`
- `docs/contracts/001-working-rules.md`
- `docs/roadmaps/README.md`
- `docs/specs/062-g10-strict-posture-and-next-boundary-gate.md`
- `docs/specs/batch-cards/README.md`

## Task Runner

Poodle uses [Effigy](https://github.com/inflatable-cookie/effigy) as its task runner.

Common commands:
- `effigy tasks` — list all available tasks
- `effigy tokens:build` — build design tokens
- `effigy docs:dev` — start documentation dev server
- `effigy docs:check` — run all documentation checks
- `effigy health` — full health check
- `effigy doctor` — environment and configuration health
- `effigy scan attention-markers` — scan for TODO/FIXME markers

Legacy npm scripts in `package.json` are deprecated; use `effigy <task>` instead.

## Internal Writing Style

Use the repo-local style reference for internal work and normal replies:

- `docs/policy/internal-writing-style.md`
