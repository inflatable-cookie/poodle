# AGENTS

Scope: whole `poodle/` repository.

## Hard Rules

- Treat `docs/` as the authority for Poodle vision, architecture, roadmap, and execution status.
- Keep Poodle focused on generalized tokens, primitives, and reusable composites; do not fold app-specific DAW widgets into the core library.
- Keep Svelte, React, shared Rust composition, and GPUI aligned to one documented contract; parity means semantic inputs, states, behavior, and token usage first. Jetstream remains deferred until its admission runway.
- Keep Underlay-facing integration behind Underlay-owned adapters and token bridges; app code should not need to know Poodle exists.
- Treat Bits Svelte as an implementation detail where useful, not as Poodle's public contract.
- Never run `*-windowed` conformance selectors on a local desktop without explicit operator approval; use `effigy ci:conformance`.
- Keep AGENTS content lean: scope, hard rules, validation, links.

## Northstar Orchestration

- Use Northstar orchestrator mode for question-led planning, runway maintenance, worker dispatch, PR review, and closeout.
- The orchestrator owns the `main` planning checkout and does not become the implementation worker after dispatch. Publish the planning base and one committed handoff under `docs/handoffs/` per independent lane before launch.
- Give each worker only its repository-relative handoff path. Workers use clean, dedicated, non-`main` worktrees, push reviewable PRs, and never merge.
- Parallelise only independent ready cards with no shared mutable surfaces or unresolved authority. Merge only after independent review and explicit operator authorisation.

## Local Agent Paths and Worktrees

- `.agents.local.env.example` documents supported local path keys. `.agents.local.env` is ignored, path-only local state; never commit it or put credentials, secrets, or commands in it.
- Prefer a harness-managed worktree when one exists. Before creating a worktree manually, read `.agents.local.env` and require an absolute `AGENTS_WORKTREE_CONTAINER_DIR`.
- If the file or key is absent, stop and ask which absolute directory to use. Do not guess `/tmp`, `TMPDIR`, or a repository-adjacent path.
- After the operator answers, store the value locally and use `<container>/<repository-slug>-<lane-slug>`. A worker or subagent must not create a nested worktree when the harness or parent orchestrator already owns the lane.

The durable rules live in `docs/contracts/005-agent-local-paths.md`.

## Papercuts Loop

During execution, append small solvable friction to root `PAPERCUTS.md` before continuing. Record the impact, plausible fix, and affected surface; skip one-off failures, external blockers, sensitive data, and duplicates. Do not turn the note into unplanned work.

## Validate

- `git diff --check`

## References

- `README.md`
- `docs/README.md`
- `docs/vision/001-poodle-vision.md`
- `docs/architecture/001-poodle-system-shape.md`
- `docs/architecture/product-guardrails.md`
- `docs/contracts/001-working-rules.md`
- `docs/contracts/005-agent-local-paths.md`
- `docs/roadmaps/README.md`
- `docs/roadmaps/g15/README.md`
- `docs/specs/README.md`

## Task Runner

Poodle uses [Effigy](https://github.com/inflatable-cookie/effigy) as its task runner.

Common commands:
- `effigy tasks` — list all available tasks
- `effigy tokens:build` — build design tokens
- `effigy docs:dev` — start documentation dev server
- `effigy docs:check` — run all documentation checks
- `effigy qa` — full local release board (headless / CPU-only)
- `effigy qa:jetstream` — opt-in paired Jetstream integration board
- `effigy doctor` — orientation (built-ins + cheap `tasks.health`)
- `effigy scan attention-markers` — scan for TODO/FIXME markers

Legacy npm scripts in `package.json` are deprecated; use `effigy <task>` instead.

## Internal Writing Style

Use the repo-local style reference for internal work and normal replies:

- `docs/policy/internal-writing-style.md`

<!-- BEGIN EFFIGY AGENT CONTRACT -->
## Effigy Agent Contract

Use Effigy as the default command surface for supported project work.

Route by job, not by startup ritual:
- use `effigy graph` for code understanding
- use `effigy tasks` for selector inventory
- use `effigy doctor` for routing ambiguity or repo health
- use `effigy test --plan` when test execution shape matters

Use `effigy graph` when the job is code understanding: ownership, flow,
implementation, or changed-file impact. Do not insert graph into unrelated
deployment, state, docs, release, or direct task-execution work.

Prefer `effigy <task>`, `effigy test`, and the matching built-in surface over
raw package-manager or shell commands when Effigy covers the path. Use
`effigy --json <command>` whenever another agent or tool will consume output.

This repo's local `.agents/skills/effigy` copy is authoritative for this
project. When an agent supports both project-local and global skills, prefer
the project-local copy over any globally installed Effigy skill.

Do not add `--repo .` while already inside the target repo. Do not edit
`.github/workflows/` or run release mutations unless the user explicitly asks.

Reference docs:
- Effigy agent adoption: `docs/guides/047-agent-and-cross-repo-adoption.md`
- Graph workflows: `docs/guides/076-code-graph-and-agent-workflows.md`
- JSON contracts: `docs/guides/017-json-output-contracts.md`
<!-- END EFFIGY AGENT CONTRACT -->
