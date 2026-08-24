# g16 — Published Consumer Adoption

Status: active — `v0.2.2` published from the accepted crates.io-GPUI
candidate; consumer adoption resumes with a fresh Longhorn lane
Posture: adoption-first with one Poodle patch release gate
Opened: 2026-08-23
Governing refs: `../../../README.md`, `../../README.md`,
`../../contracts/001-working-rules.md`,
`../../logs/2026-08/20260823-g15-013-v021-release-certification.md`

## Generation Goal

Move every authoritative Poodle consumer under `~/Dev/projects` onto the
corrected published `0.2.2` release and prove each repository against what it
actually installs. Remove committed Poodle `file:` overrides from active
consumers so a green checkout tests the registry package rather than an
adjacent source tree.

This is a consumer rollout, not a new parity or component programme. Longhorn
proved that `v0.2.1` leaks Poodle's GPUI fork into the public Rust graph and is
type-incompatible with crates.io GPUI 0.2.2. Cards `005`–`007` own the one
blocking Poodle patch recovery and operator release gate; they do not reopen
component work or add a compatibility shim.

## Adoption Policy

- Applications pin `@inflatable-cookie/poodle-core` and
  `@inflatable-cookie/poodle-svelte` to exact `0.2.2` after `g16.007`. Poodle is pre-1.0;
  `latest` is resolved and recorded, not left as a moving range.
- Libraries move their peer requirement to the narrow `0.2.x` shape already
  used by that library: exact peers stay exact; caret peers become `^0.2.2`.
- Active committed Poodle `file:` dependencies and overrides are removed.
  Local unpublished development may use ignored machine-local tooling, never a
  committed override that defeats registry evidence.
- Longhorn's Rust Poodle git dependencies move together to version `0.2.2`
  and tag `v0.2.2`. Jetstream's explicitly paired local Rust paths remain
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
| Release recovery | Poodle | complete; `v0.2.2` published from exact candidate `d5607def` |
| Foundation | Longhorn, Underlay, Soundcheck Library | Underlay and Soundcheck Library complete at 0.2.2; `008` remains active |
| Adapter follow-up | Jetstream | waits for Longhorn |
| Product applications | Acowtancy, Bovine Accelerator Desktop, Compli Me, Composer, Contact Patch, Figmatic, Finch, Loophole, Nucleus, Songsprout, Soundcheck, Underlay Reference | Underlay-dependent cards may compile; Longhorn-dependent apps still wait |
| Legacy product | Loophole Legacy | final wave; verify that it remains supported before merge |

## Runway

1. [001 — Consumer adoption inventory](001-consumer-adoption-inventory.md) —
   complete; scope, policy, exclusions, and dependency order frozen
2. [002 — Longhorn Poodle 0.2.1 adoption](002-longhorn-poodle-v021-adoption.md) —
   paused after exposing the released GPUI source-identity defect; superseded
   after `006`
3. [003 — Underlay Poodle 0.2.1 adoption](003-underlay-poodle-v021-adoption.md) —
   complete; PR 4 merged at `750005eb`
4. [004 — Soundcheck Library Poodle 0.2.1 adoption](004-soundcheck-library-poodle-v021-adoption.md) —
   complete; PR 5 merged at `a720f22`
5. [005 — GPUI crates.io recovery](005-gpui-cratesio-recovery.md) —
   complete; PR 73 merged at `8dfdfa3c`
6. [006 — v0.2.2 release candidate](006-v022-release-candidate.md) —
   complete; candidate `d5607def` accepted and PR 74 merged at `6ea561be`
7. [007 — v0.2.2 release certification](007-v022-release-certification.md) —
   complete; tag `v0.2.2`, npm core/Svelte publication, artifact, and clean
   public-registry consumer install verified
8. [008 — fresh Longhorn v0.2.2 adoption](008-longhorn-poodle-v022-adoption.md) —
   ready; web registry, Rust tag, and single crates.io-GPUI identity proof
9. [009 — Underlay 0.2.2 follow-up](009-underlay-poodle-v022-follow-up.md) —
   complete; PR 5 merged at `d6fe7b9b`, exact registry bump with no migration
10. [010 — Soundcheck Library 0.2.2 follow-up](010-soundcheck-library-poodle-v022-follow-up.md) —
    complete; PR 6 merged at `7f5ff0b9`, exact development pin, peer-line bump,
    and dual-lock validation
11. `011` — Jetstream adoption — compile after Longhorn lands
12. `012+` — exact product-repository cards — compile after their foundation
    dependencies land and expose any migration requirements

## Current Task And Parallel Lanes

The release-recovery lane is complete. Tag `v0.2.2` points to exact candidate
`d5607def`; run `32756610293` published core and Svelte 0.2.2 to npm `latest`,
retained React as source-only, and uploaded the packed artifacts. A clean
public-registry consumer install passed.

Underlay `009` and Soundcheck Library `010` are complete. Longhorn `008` is the
remaining active foundation lane and must stay on its fresh 0.2.2 branch rather
than recover the stopped 0.2.1 worktree. Underlay-dependent product cards can
compile in parallel; Jetstream, Soundcheck, and other Longhorn-dependent
products still wait for `008`.

The crates.io GPUI boundary and non-activating evidence transport are accepted
and merged in `g16.005`. PR 74 removed the stale `bzip2` / `libbz2-rs-sys`
notice-policy claims, re-pinned the candidate, and reran the complete board.
Longhorn `g16.002` stopped without a PR and its worktree was removed rather
than propagate the GPUI fork. Underlay `g16.003` and Soundcheck Library
`g16.004` remain valid evidence for removing local overrides. Their 0.2.2
follow-ups are now both merged. No product card advances on 0.2.1.

## Completion

The generation closes only when every authoritative repository has:

- no active Poodle `0.1.0` or `0.2.1` manifest or lockfile resolution;
- no committed active Poodle `file:` override;
- exact npm 0.2.2 or Rust tag `v0.2.2`, with crates.io GPUI identity;
- a clean install from its declared sources;
- its own relevant headless QA evidence and merged PR.
