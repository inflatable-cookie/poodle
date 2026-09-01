# g16.033 — HistoryCenter Rejection Surface

Status: accepted and merged in PR #120 as `df0c7acd9`
Date: 2026-09-01
PR: https://github.com/inflatable-cookie/poodle/pull/120
Implementation commit: `ffe357d50753a6eb94c38bc77a585a3fdac57266`
Card: `docs/roadmaps/g16/033-history-center-rejection-surface.md`
Handoff: `docs/handoffs/20260901-105037-g16-033-history-center-rejection-surface.md`
Governing refs: `docs/contracts/components/history-center.md`,
`docs/architecture/006-headless-core-and-machine-model.md`,
`docs/contracts/001-working-rules.md`,
`docs/triage/20260902-000956-history-portfolio-holds.md`
Branch: `papercuts/g16-033-history-center-rejection-surface`
Base: `main` at `9bdcf03e7` (rebased from `8cccdc65c` after PR #119 merged;
the two incoming research/handoff docs overlap nothing here)

## Outcome

A refused history deletion now says what actually happened. `HistoryCenter`
carries five renderer-neutral refusal meanings instead of two, and the three
new ones are the three that were being flattened: history moved under the
request, the authority protects the entry, deletion is not on offer at all.

| Code | Exact copy |
| --- | --- |
| `AlreadyAtTarget` | `Already at the requested target` |
| `UnknownEntry` | `Entry does not exist` |
| `StaleHistory` | `History changed; this entry was not deleted` |
| `ProtectedEntry` | `This history entry is protected` |
| `DeletionUnavailable` | `History deletion is unavailable` |

The copy stays in Poodle's exhaustive resolvers on both sides —
`historyCenterRejectionMessage`, `history_center_rejection_message`, and
`HistoryCenterSpec::rejection_message`. Adapters map codes; no adapter, host,
or protocol string carries a message. Nothing here adds a locale input, a
message catalogue, or a host escape hatch.

Separately, the packed Svelte package now proves its own v3 `HistoryEntry`
export to a real installed consumer.

## Decisions worth keeping

- **Current-line and pinned/checkpoint refusal share one code.** Both are
  entry-level protection. Which policy refused is the authority's business,
  and Poodle splitting it would have made the host describe its own rules
  through Poodle's vocabulary — exactly the leak the seam exists to stop.

- **No web-shell source changed.** Svelte and React take
  `HistoryCenterRejectionCode` straight from core, so widening the union
  widened both props with nothing to edit. That is the seam working: the
  shells mount what the machine resolved. Both shells gained proof, not code.

- **The renderer did not change either.** `poodle-render`'s notice takes the
  resolved message, so the five codes reach GPUI through the existing tree.
  The new `poodle-render` test is there because that tree is what natives
  paint — a category that collapses there collapses in GPUI.

- **Specimens were left alone.** The rejection notice already teaches the
  pattern with one representative code. Four more closed popovers would teach
  nothing the pattern does not already show, and the card's "only as needed"
  is a guard against exactly that churn. The categories are proved where they
  can fail: resolvers, mounted shells, the shared tree, and a mounted window.

- **A `.d.ts` read as text is not a type proof.** The packed proof compiles.
  A real consumer installs the tarballs, `tsc` typechecks `HistoryEntry` from
  the package root and from `@inflatable-cookie/poodle-svelte/types` with
  `continuationCount`, and one unsuppressed fixture per import path must fail
  with `TS2339: Property 'branchCount' does not exist on type 'HistoryEntry'`.
  The harness refuses a negative fixture that contains `@ts-expect-error`,
  `@ts-ignore`, or an `any` cast, because a suppressed expected failure passes
  for the wrong reason.

- **This is a source candidate, not a release.** npm `latest` is still
  `0.2.2`. No package version, tag, release note, publication workflow, or
  Loophole pin moved, and none may move without separate release authority.

- **A falsification restore silently reverted two proofs, and review caught
  it.** Planting a pre-fix behaviour and restoring with `git checkout --` reads
  from the index, so on an unstaged tree it restores `HEAD`, not the working
  state. That wiped the `poodle-headless` and `poodle-specs` rejection tests
  along with the plant. `ci:rust` stayed green because it counts passes, not
  absences, and the card claimed proofs the head no longer carried. Both test
  surfaces are restored and re-falsified, and the working rule is to commit
  before planting.

- **Rust cannot assert the `SHOW_REJECTION` short-circuit, so it does not
  claim to.** The guard returns the very context it was handed, which is what
  lets the web adapters skip their write-back. A moved `HistoryCenterContext`
  has no identity to compare, so a plant that rebuilds an equal context is
  invisible there. The headless proof therefore asserts what the result does
  expose — a repeat does not stack and emits no second effect, and a
  replacement leaves no residue of the notice it displaced — and the doc
  comment says which of the two it is proving. The TypeScript proof, where
  reference identity is observable, keeps the stricter `toBe` assertion.

## Changed surfaces

| Surface | Change |
| --- | --- |
| `packages/core/src/history-center.ts` | five-code `HistoryCenterRejectionCode`; exhaustive resolver |
| `packages/core/test/history-center.test.ts` | copy table, distinctness, display / replacement / idempotence |
| `packages/contracts/headless/src/history_center.rs` | five-code enum; exhaustive resolver; two new machine tests |
| `packages/contracts/components/src/history_center.rs` | five-variant `HistoryCenterRejection`; exhaustive `rejection_message`; two tests |
| `packages/render/src/history_center.rs` | two shared-tree tests (no renderer change) |
| `packages/gpui/preview/tests/headless_regressions.rs` | `every_history_center_rejection_mounts_its_own_native_copy` |
| `packages/svelte/components/test/HistoryCenter.test.ts` | five-code mount proof; replacement / idempotence / clear |
| `packages/react/components/test/HistoryCenter.test.tsx` | the same two proofs |
| `test/package-install/fixture/packed-types/` | positive + two unsuppressed negative type fixtures and their tsconfigs |
| `test/package-install/web-preview.ts` | pinned consumer `typescript`, the three compiles, suppression guard, `packedHistoryEntryProof` evidence |
| `docs/contracts/components/history-center.md` | `Updated` date and approvers (the five-code table was already promoted) |

## Evidence

`effigy test:svelte-pack-install` records, per run:

```
"packedHistoryEntryProof": {
  "positive":         { "exitCode": 0, "diagnostics": [] },
  "expectedFailures": [
    { "importPath": "@inflatable-cookie/poodle-svelte",
      "diagnostic": "…error TS2339: Property 'branchCount' does not exist on type 'HistoryEntry'." },
    { "importPath": "@inflatable-cookie/poodle-svelte/types",
      "diagnostic": "…error TS2339: Property 'branchCount' does not exist on type 'HistoryEntry'." }
  ],
  "sourceImports": false, "workspaceAliases": false, "declarationTextSubstitute": false
}
```

alongside the existing per-tarball `sha256`, the installed `realpath`, and the
sibling-source / workspace-dependency refusals the harness already enforced.

## Validation

`ci:web`, `ci:rust`, `ci:native`, `docs:check`, `test:core`,
`test:components`, `test:contracts`, `test:svelte-pack-install`, the five
HistoryCenter-relevant drift checks, and `git diff --check origin/main...HEAD`
all pass on the rebased head.

`effigy qa` exits 1 on the known `main` baseline alone: `audit:security` reads
`sk-plus-translated-hi…` inside the English phrase `mask-plus-translated-highlight`
in `PAPERCUTS.md`, this card's own dispatch handoff, and
`docs/triage/20260901-080641-post-g16-research-queue.md`. All three carry that
phrase on `origin/main` already; this branch adds no `sk-` match. The
unanchored matcher is recorded in `PAPERCUTS.md` and is not this card's to fix.

## Ledger

No parity-ledger cell moved. This is a semantic correction and a
regression-proof lane, not a new mounted-evidence claim.
