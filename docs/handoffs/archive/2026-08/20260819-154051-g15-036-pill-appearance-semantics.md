---
title: g15.036 Pill appearance semantics worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-19
updated: 2026-08-19
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260819-154051-g15-036-pill-appearance-semantics.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, pill]
---

## What This Thread Is Doing

Correct the temporary Pill API that landed in PR #44. Pill already had an
appearance axis. Adding a second `fill` axis created duplicate concepts and an
unhelpful precedence matrix.

Execute only:

`docs/roadmaps/g15/036-pill-appearance-semantics.md`

Remove Pill `fill`, add `appearance="tint"`, make tint the visual-preserving
default, and make the existing `appearance="solid"` use the opaque solid-tone
recipe. Callout and RemediationBanner keep the `ToneFill` API landed by PR
#44.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Required base commit:** `c24b19f41f3e735a4fe83cdd02552e96f3363d16`
- **Base provenance:** squash merge of PR #44; local `HEAD` and `origin/main`
  matched this SHA before planning began
- **Planning checkout:** orchestrator-owned; do not edit it
- **Worker mode:** implementation worker in a clean, dedicated, non-`main`
  harness-managed worktree
- **Suggested branch:** `t3code/g15-036-pill-appearance-semantics`
- **Allowed runway:** `g15.036` only
- **Card budget:** one card, one PR, then stop
- **Parallel lane:** `g15.022`; it owns audio/music specimen curation and has
  no Pill files
- **PR base:** `main`
- **Merge authority:** absent; push the PR and stop for orchestrator review
- **Tool restriction:** headless only. Never run windowed, native-visual,
  conformance, Jetstream, or release selectors.

## Required Reading

Read completely before implementation:

- `AGENTS.md`
- `.agents/skills/effigy/SKILL.md`
- `docs/roadmaps/g15/README.md`
- `docs/roadmaps/g15/036-pill-appearance-semantics.md`
- `docs/contracts/001-working-rules.md`
- `docs/contracts/004-shared-control-types.md`
- `docs/contracts/components/pill.md`
- `docs/specs/022-packaging-versioning-and-release-channel-rules.md`
- `docs/logs/2026-08/20260819-g15-035-solid-tone-surfaces.md`

PR #44 and `g15.035` are historical evidence for the temporary API. The
`g15.036` card is the current authority for Pill.

## Worktree Preflight

Before broad repository reads, run only:

1. `git rev-parse --show-toplevel`
2. `git branch --show-current`
3. `git status --porcelain`
4. `git worktree list --porcelain`

Accept the launcher-provided worktree when it is registered, clean, and not
`main`, even if its path or branch differs from the placeholders. Do not make a
second worktree. If the supplied context is dirty, on `main`, or unregistered,
stop and report it rather than cleaning or resetting somebody else's state.

Then fetch and prove:

- `HEAD` equals current `origin/main`;
- `git merge-base --is-ancestor
  c24b19f41f3e735a4fe83cdd02552e96f3363d16 HEAD` succeeds;
- this handoff and the `g15.036` card exist in `HEAD`.

Use the harness-managed worktree. Manual creation is only a fallback through
the validated `AGENTS_WORKTREE_CONTAINER_DIR` in `.agents.local.env`; never
guess `/tmp` or a repository-adjacent path.

## Contract Boundary

The end state is exact:

```ts
type PillAppearance = "tint" | "solid" | "subtle" | "badge";
```

- default: `tint`;
- default visuals: unchanged from the pre-PR #44 ordinary Pill shell;
- solid: PR #44's contrast-safe opaque 45/55 tone recipe;
- subtle and badge: current behavior unchanged;
- Pill has no `fill`, `ToneFill`, `with_fill`, `data-fill`, deprecated alias,
  precedence rule, or silent fallback;
- Callout and RemediationBanner retain `ToneFill` and `fill` unchanged.

This is an approved pre-v0.2 breaking correction. Do not preserve the
temporary API. Do not use the change to redesign Pill states, global tone
types, tokens, or the shared solid color resolver.

## Implementation Order

1. Correct the Pill contract and shared-type consumer wording.
2. Correct paired TS props/types/defaults and shared CSS.
3. Correct `PillAppearance`, `PillSpec`, and `poodle-render::pill` in Rust.
4. Add focused evidence for default visual preservation, all four
   appearances, tones/custom accent, dot/remove, and absence of Pill `fill`.
5. Remove the authored specimen `fill` axis, add tint to appearances,
   regenerate every derived artifact, and inspect the generated diff.
6. Align the Svelte, React, and GPUI Pill examples on one compact appearances
   group. Tests own the matrix.
7. Write one August correction log with public-entry-point impact and exact
   validation results.

Generated files are never edited by hand. Use the repository generator and
`effigy ir:check`.

## Writable Boundary

Use the card's exact writable scope. In particular:

- Pill contracts, types, shells, CSS, tests, render/spec code, and specimens;
- Pill authored/generated display-specimen artifacts;
- shared control-type wording only to remove Pill as a `ToneFill` consumer;
- one unique August correction log;
- append-only `PAPERCUTS.md` only for newly encountered execution friction.

Do not change Callout or RemediationBanner implementation, Button, token
schema, other component APIs, catalogue shells, Jetstream, conformance
architecture, release automation, `.github/workflows/`, roadmap state, or the
dispatch ledger.

## Validation

Orient with `effigy tasks` and `effigy doctor`, then run one coherent final
headless round:

- focused Svelte/React Pill component tests;
- focused Rust Pill spec/render and contrast tests;
- focused generated-specimen and parity evidence;
- `effigy ir:check`;
- `effigy test:components`;
- `effigy check:svelte`;
- `effigy react:build`;
- `effigy test:parity`;
- `effigy check:gpui`;
- `effigy regressions:native`;
- `effigy test:web-pack-install`;
- `effigy docs:check`;
- `git diff --check origin/main...HEAD`.

Record warnings and known `effigy doctor` baseline findings honestly. Do not
turn unrelated baseline cleanup into this PR.

## Stop Conditions

Stop and report evidence if:

- removing Pill `fill` would alter Callout or RemediationBanner's public API;
- visual preservation needs an alias or silent fallback;
- the four appearances cannot remain mutually exclusive across web and Rust;
- the existing solid recipe fails current contrast evidence;
- generated specimen support would need behavior or callbacks;
- another component, token-schema change, or release mutation becomes
  necessary.

## PR Handoff

Before opening the PR:

1. Inspect the complete diff and `git status`.
2. Confirm no Pill `fill` residue with targeted `rg` searches, excluding the
   historical `g15.035` card/log and this correction record.
3. Run the validation round above and record exact results in the batch log.
4. Commit intentionally and push the worker branch.
5. Open a draft PR against `main` with the card outcome, public API correction,
   validation, generated artifacts, and unresolved findings.
6. Report branch, commit SHA, PR URL, changed-file summary, validation, and
   blockers to the operator.
7. Do not merge, change roadmap/ledger status, delete the worktree, or begin
   another card.

## Suggested First Move

Run the four-command worktree preflight, read the required authorities, then
trace `fill`, `ToneFill`, `PillAppearance`, and generated Pill specimen fields
with `rg`. Write the exact removal/addition inventory before editing. Start at
the contract and type authorities, not CSS.
