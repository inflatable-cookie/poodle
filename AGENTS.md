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
- What's next: `docs/plan.md`
- Unresolved leads: `docs/triage/`
- Component contracts (public API reference): `docs/contracts/`
- Generated evidence (receipts, ledgers, census): `docs/evidence/`
- Small recurring friction: `PAPERCUTS.md`

Tasks, briefs and status live in Queue, never in this repository.

## Commands

Use the repo-local `.agents/skills/effigy/SKILL.md` for task routing.

- `effigy tasks` — list selectors; pick the narrow ones for the change
- `effigy docs:lint` — contract, docs and generated-evidence checks
- `effigy ci:web` / `effigy ci:rust` — the required PR CI lanes
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
- Record small solvable friction in `PAPERCUTS.md`; do not turn it into
  unplanned work.
- When a change alters what is true, update the owning knowledge file in the
  same PR.
- An operator ruling given in conversation goes into its owning file before
  the thread ends.

## Validate

Every brief names a bounded validation budget: the focused selectors allowed
during implementation and at most one final broad selector. Do not stack
overlapping boards (`docs:check`, `ci:web`, `qa`, release gates) or rerun a
green broad selector after narrower checks. Aggregate runs must show live child
progress; stop and report a named over-budget child. Workers do not wait or
poll for GitHub checks. The full rules are in
[working rules](docs/knowledge/contracts/working-rules.md#validation).

Run `git diff --check` and the brief's selectors before opening a PR.
