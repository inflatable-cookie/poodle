# Poodle

Poodle is a generalized design system: tokens, primitives and reusable
composites, delivered as Svelte and React web packages and a shared Rust
renderer with GPUI and Jetstream backends, all bound to one set of
renderer-neutral contracts. It serves applications that need one UI language
across web and native. It must never become a home for one product's widgets
or a consumer-shaped adapter layer.

## Where things live

- Current state: `docs/README.md`
- Knowledge (one owner per fact): `docs/knowledge/README.md`
- Retired concepts, which must not come back: `docs/knowledge/retired.toml`
- Open questions: `docs/knowledge/questions.md`
- Component contracts (public API reference): `docs/contracts/`
- Generated evidence (receipts, ledgers, census): `docs/evidence/`

The plan (lanes, their documents and their order), leads, papercuts, brief
drafts, tasks and status live in Queue, never in this repository. Read what's
next with `plan.get` (see the `northstar` skill).

## Commands

Use the installed shared Effigy skill for task routing, resolved from
`~/.agents/skills/effigy/SKILL.md`; the `~/.codex`, `~/.claude` and `~/.cursor`
skill roots are aliases to that same canonical directory. Resolve symlinks
before comparing roots, and treat two distinct matches as ambiguous rather than
picking one. This repository does not vendor a copy: if no installed Effigy
skill is present, say so and run `npx skills add inflatable-cookie/effigy -g`,
then confirm discovery from a fresh agent context. The selectors and
guardrails below stay local.

- `effigy tasks` — list selectors; pick the narrow ones for the change
- `effigy docs:lint` — contract, docs and generated-evidence checks
- `effigy ci:web` / `effigy ci:rust` — the required PR CI lanes
- `effigy ci:fresh` — milestone validation on `main` (the planner runs it at
  release points; no per-task full QA): frozen install, then `ci`, both with
  the Bun pinned in `package.json` `packageManager`
- `effigy qa` — the broad headless repository board

## Product rules

- Keep Poodle to generalized tokens, primitives and reusable composites.
  App-specific DAW widgets stay in their owning products.
- Svelte, React, shared Rust composition and GPUI follow one documented
  contract. Parity means semantic inputs, states, behavior and token use first.
  Svelte is the parity authority. Jetstream follows its admission status in
  [working rules](docs/knowledge/contracts/working-rules.md).
- Underlay and its applications import Poodle's published packages directly;
  any translation lives in the consumer. Poodle carries no consumer-named
  directory or adapter (architecture 001, operator decision 2026-09-02).
- Bits Svelte is an implementation detail, not public contract authority.
- Before v1.0, add no compatibility shims, aliases or silent fallbacks. Stop and
  ask before a breaking migration.

## Guardrails

- Never run `*-windowed` conformance selectors locally without explicit
  operator approval; use the headless `effigy ci:conformance` path.
- Do not edit `.github/workflows/` or run release mutations without explicit
  operator approval. Release follows
  [release](docs/knowledge/contracts/release.md).
- Prefer harness-managed worktrees. Manual creation requires the
  operator-selected `AGENTS_WORKTREE_CONTAINER_DIR` from ignored
  `.agents.local.env`; never guess a temporary or repository-adjacent path. See
  [agent local paths](docs/knowledge/contracts/agent-local-paths.md).
- File small solvable friction as a Queue papercut and carry on; do not turn
  it into unplanned work. See
  [working rules](docs/knowledge/contracts/working-rules.md#papercuts).
- When a change alters what is true, update the owning knowledge file in the
  same PR.
- An operator ruling given in conversation goes into its owning file before
  the thread ends.

## Validate

Targeted checks per task, full QA at milestones (Tom, 2026-09-30).

- **Workers** run, once, the Effigy selectors for the code they changed, a
  compile or type check of what they touched, and `effigy docs:lint` if docs
  or evidence changed. Then they open the PR. No whole suites (`qa`,
  `ci:web`, `ci:rust`, `docs:check`, `ci:fresh`), no repeat passes.
- **Reviewers** read the diff, run the same targeted checks and exercise the
  behaviour. No suites.
- **The planner** runs milestone QA (`effigy ci:fresh`) on `main` through
  Queue's `project.qa.run` at release points and after a major chunk of work,
  and briefs fixes for what it finds.
- Briefs name the targeted checks as acceptance. Required PR CI still runs on
  GitHub; workers don't wait or poll for it.

Run validation through Effigy selectors, not raw `cargo`, `bun` or `vitest`.
Kill only processes you started, by PID; never `pkill -f` or `killall`. Run
`git diff --check` before opening a PR. The full rules are in
[working rules](docs/knowledge/contracts/working-rules.md#validation).
