# g16 — Published Consumer Adoption

Status: active — `v0.2.2` foundation, first product wave, and coupled Underlay
product wave complete; remaining consumer lanes in review
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
- Acowtancy, Composer, Contact Patch, and Songsprout move
  every active web and Rust Underlay dependency to tag `v0.9.2`
  (`ddba26400f480638829917cf72eecc62be4b978d`). Web packages use the tagged
  Git source; Rust crates use the same Git repository and tag while preserving
  their feature sets. Active sibling Underlay `file:`/`path` sources are
  removed. Underlay `v0.9.2` pins Poodle Svelte 0.2.2, while application-owned
  direct Poodle dependencies remain exact 0.2.2.
- Compli Me advanced independently to tagged Underlay `v0.9.4` at `7004af5b`
  before its tag-boundary correction merged. That later release is accepted:
  all active web and Rust surfaces name the same tag and direct Poodle remains
  exact 0.2.2.
- Underlay Reference also advanced to `v0.9.4` on `main` before its sibling-path
  follow-up was reviewed. Card `025` preserves that newer release while moving
  every active web and Rust dependency across the same tag boundary.

## Inventory

The frozen baseline is recorded in
[g16.001](001-consumer-adoption-inventory.md). Seventeen authoritative
repositories consume Poodle directly. Poodle itself, temporary consolidation
trees, and in-repository archived applications are excluded.

| Lane | Repositories | State |
| --- | --- | --- |
| Release recovery | Poodle | complete; `v0.2.2` published from exact candidate `d5607def` |
| Foundation | Longhorn, Underlay, Soundcheck Library | complete at 0.2.2 |
| Adapter follow-up | Jetstream | deferred behind product adoption; paired Rust integration remains local |
| First product wave | Nucleus, Soundcheck, Underlay Reference | Poodle 0.2.2 complete |
| Coupled Underlay product wave | Acowtancy, Compli Me, Composer, Contact Patch, Songsprout | complete |
| Underlay Reference follow-up | Underlay Reference | `025` implementation approved; stale PR metadata must be corrected before merge |
| Longhorn product wave | Finch, Figmatic, Bovine Accelerator Desktop, Loophole | Figmatic and Loophole complete; Finch changes requested for stale sibling bootstrap/authority; Bovine lane remains in flight |
| Adapter follow-up | Jetstream | `023` ready; dispatch after product lanes |
| Legacy product | Loophole Legacy | `024` ready as final authoritative product lane |

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
   complete; PR 9 merged at `c94f72e9`, with public web packages, Rust tag,
   and one crates.io-GPUI identity proved
9. [009 — Underlay 0.2.2 follow-up](009-underlay-poodle-v022-follow-up.md) —
   complete; PR 5 merged at `d6fe7b9b`, exact registry bump with no migration
10. [010 — Soundcheck Library 0.2.2 follow-up](010-soundcheck-library-poodle-v022-follow-up.md) —
    complete; PR 6 merged at `7f5ff0b9`, exact development pin, peer-line bump,
    and dual-lock validation
11. [011 — Nucleus Poodle 0.2.2 adoption](011-nucleus-poodle-v022-adoption.md) —
    complete; PR 1 merged at `9b3f67c9`, with one public Poodle identity over
    the retained local Longhorn packages
12. [012 — Soundcheck Poodle 0.2.2 adoption](012-soundcheck-poodle-v022-adoption.md) —
    complete; PR 11 merged at `b1c5937d`, with one public Poodle identity
    across the app, Longhorn adapter, and Soundcheck Library
13. [013 — Underlay Reference Poodle 0.2.2 adoption](013-underlay-reference-poodle-v022-adoption.md) —
    complete; PR 1 merged at `f5ea7d72`, with registry adoption clean and the
    pre-existing Effigy test-routing baseline exposed rather than hidden
14. [014 — Acowtancy Underlay 0.9.2 and Poodle 0.2.2 adoption](014-acowtancy-poodle-v022-adoption.md) —
    complete; PR 56 merged at `1ab977f8`, removing active sibling Underlay
    compilation aliases while retaining explicit cross-repository QA
15. [015 — Compli Me tagged Underlay and Poodle 0.2.2 adoption](015-compli-me-poodle-v022-adoption.md) —
    complete; PR 3 merged at `db5741f6`, preserving the newer Underlay `v0.9.4`
    release while aligning all four web manifests with Rust's tag boundary
16. [016 — Composer Underlay 0.9.2 and Poodle 0.2.2 adoption](016-composer-poodle-v022-adoption.md) —
    complete; PR 1 merged at `74e5a7a7`, with tagged Underlay, registry Poodle
    0.2.2, and bounded web and Rust lock changes
17. [017 — Contact Patch Underlay 0.9.2 and Poodle 0.2.2 adoption](017-contact-patch-poodle-v022-adoption.md) —
    complete; PR 1 merged at `c497547b`, with one tagged Underlay revision and
    registry Poodle 0.2.2 proved across the web and Rust graph
18. [018 — Songsprout Underlay 0.9.2 and Poodle 0.2.2 adoption](018-songsprout-poodle-v022-adoption.md) —
    complete; PR 1 merged at `22f8ae78`, with tagged Underlay and registry
    Poodle identities proved across Bloom, Greenhouse, Stem, and Nursery
19. [019 — Finch Poodle 0.2.2 adoption](019-finch-poodle-v022-adoption.md) —
    changes requested on PR 1; peer convergence is fixed, but unused Poodle
    sibling bootstrap and current authority still need alignment
20. [020 — Figmatic Poodle 0.2.2 adoption](020-figmatic-poodle-v022-adoption.md) —
    complete; PR 17 merged at `a6286e88`, with one registry Poodle identity
    and converged Longhorn peer
21. [021 — Bovine Accelerator Desktop Poodle 0.2.2 adoption](021-bovine-accelerator-desktop-poodle-v022-adoption.md) —
    handoff ready; isolated worktree required from refreshed target base
22. [022 — Loophole Poodle 0.2.2 adoption](022-loophole-poodle-v022-adoption.md) —
    complete; PR 8 merged at `92e01577`, with one registry Poodle identity and
    the renderer-test baseline kept separate
23. [023 — Jetstream Poodle 0.2.2 adoption](023-jetstream-poodle-v022-adoption.md) —
    ready after product dispatch; registry web plus retained paired Rust paths
24. [024 — Loophole Legacy Poodle 0.2.2 adoption](024-loophole-legacy-poodle-v022-adoption.md) —
    ready as final authoritative product lane; active Aura only
25. [025 — Underlay Reference tagged Underlay adoption](025-underlay-reference-v092-adoption.md) —
    implementation approved on PR 2 at `25818824`; correct stale v0.9.2 PR
    metadata before merge

## Current Task And Parallel Lanes

The release-recovery, foundation, and first product wave are complete. Tag
`v0.2.2` points to candidate `d5607def`; run `32756610293` published core and
Svelte to npm `latest`. Longhorn `008`, Underlay `009`, Soundcheck Library
`010`, Nucleus `011`, Soundcheck `012`, and Underlay Reference `013` are merged.

Cards `014`-`018` cover five independent coupled Underlay/Poodle repositories;
all are complete. `025` is the equivalent Underlay-only follow-up for the
already-migrated Underlay Reference; its implementation is approved at
Underlay v0.9.4, with only stale PR metadata left to correct. Cards `019`-`022`
cover four independent
Longhorn-shaped products. Figmatic `020` and Loophole `022` are complete;
Finch `019` needs a bounded bootstrap and current-authority correction. `021`
must not use Bovine Accelerator Desktop's operator checkout.

Jetstream remains off the critical path. Card `023` upgrades its public web
surface but preserves the paired local Rust contract. Dispatch it after product
lanes are in flight. Loophole Legacy `024` is the final active product lane and
must not touch its reference archives.

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
- for the six Underlay consumer repositories, one matching released Underlay
  tag across web and Rust with no active sibling Underlay source (`v0.9.2` for
  the generation baseline; Compli Me and Underlay Reference have advanced to
  `v0.9.4`);
- a clean install from its declared sources;
- its own relevant headless QA evidence and merged PR.

## Next Task

Review the bounded Finch `g16.019` update and merge Underlay Reference
`g16.025` after its PR metadata correction and explicit operator authority.
Continue the in-flight Bovine `g16.021` lane; hold `023` and `024` for the final
wave.
