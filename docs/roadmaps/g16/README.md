# g16 — Published Consumer Adoption

Status: active — the exact consumer inventory and adoption policy are frozen;
three independent foundation repositories are ready for parallel upgrade
Posture: adoption-first; no Poodle API or architecture changes
Opened: 2026-08-23
Governing refs: `../../../README.md`, `../../README.md`,
`../../contracts/001-working-rules.md`,
`../../logs/2026-08/20260823-g15-013-v021-release-certification.md`

## Generation Goal

Move every authoritative Poodle consumer under `~/Dev/projects` onto the
published `0.2.1` release and prove each repository against what it actually
installs. Remove committed Poodle `file:` overrides from active consumers so a
green checkout tests the registry package rather than an adjacent source tree.

This is a consumer rollout, not a new parity or component programme. Any real
`0.1.0` to `0.2.1` compatibility break is repaired in the owning consumer
against Poodle's published API. It does not reopen the release candidate or add
a Poodle compatibility shim.

## Adoption Policy

- Applications pin `@inflatable-cookie/poodle-core` and
  `@inflatable-cookie/poodle-svelte` to exact `0.2.1`. Poodle is pre-1.0;
  `latest` is resolved and recorded, not left as a moving range.
- Libraries move their peer requirement to the narrow `0.2.x` shape already
  used by that library: exact peers stay exact; caret peers become `^0.2.1`.
- Active committed Poodle `file:` dependencies and overrides are removed.
  Local unpublished development may use ignored machine-local tooling, never a
  committed override that defeats registry evidence.
- Longhorn's Rust Poodle git dependencies move together to version `0.2.1`
  and tag `v0.2.1`. Jetstream's explicitly paired local Rust paths remain
  local under its existing integration contract.
- Regenerate the owning repository's lockfiles without opportunistic unrelated
  dependency upgrades. Review every lockfile diff.
- Each repository gets its own branch, validation evidence, and PR. Workers do
  not make cross-repository commits from one branch.
- Historical snapshots are not consumers: exclude
  `acowtancy-consolidation.*` and `finch/archive/app-electron`. The
  `loophole-legacy` repository is authoritative enough to receive its own
  final-wave upgrade unless its owner explicitly retires it.

## Inventory

The frozen baseline is recorded in
[g16.001](001-consumer-adoption-inventory.md). Seventeen authoritative
repositories consume Poodle directly. Poodle itself, temporary consolidation
trees, and in-repository archived applications are excluded.

| Lane | Repositories | State |
| --- | --- | --- |
| Foundation | Longhorn, Underlay, Soundcheck Library | ready in parallel |
| Adapter follow-up | Jetstream | waits for Longhorn |
| Product applications | Acowtancy, Bovine Accelerator Desktop, Compli Me, Composer, Contact Patch, Figmatic, Finch, Loophole, Nucleus, Songsprout, Soundcheck, Underlay Reference | waits for the foundation it consumes |
| Legacy product | Loophole Legacy | final wave; verify that it remains supported before merge |

## Runway

1. [001 — Consumer adoption inventory](001-consumer-adoption-inventory.md) —
   complete; scope, policy, exclusions, and dependency order frozen
2. [002 — Longhorn Poodle 0.2.1 adoption](002-longhorn-poodle-v021-adoption.md) —
   ready; independent foundation lane
3. [003 — Underlay Poodle 0.2.1 adoption](003-underlay-poodle-v021-adoption.md) —
   ready; independent foundation lane
4. [004 — Soundcheck Library Poodle 0.2.1 adoption](004-soundcheck-library-poodle-v021-adoption.md) —
   ready; independent foundation lane
5. `005` — Jetstream adoption — compile after `002` lands
6. `006+` — exact product-repository cards — compile after their foundation
   dependencies land and expose any migration requirements

## Current Task And Parallel Lanes

Dispatch `g16.002`, `g16.003`, and `g16.004` in parallel to separate
repository worktrees. They share no mutable files or release authority.
Do not dispatch Jetstream or product applications until the relevant foundation
PR is merged and its installed peer/lockfile shape is known.

## Completion

The generation closes only when every authoritative repository has:

- no active Poodle `0.1.0` manifest or lockfile resolution;
- no committed active Poodle `file:` override;
- the correct npm or Rust tag dependency shape;
- a clean install from its declared sources;
- its own relevant headless QA evidence and merged PR.
