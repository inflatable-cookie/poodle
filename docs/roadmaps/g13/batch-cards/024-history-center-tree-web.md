# 024 HistoryCenter v2 — Web Rendering And Lanes

Status: merged (`bd341ad2` → `3580399d`)
Milestone: side-quest (component architecture, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-024-history-center-tree-web`
Depends on: `g13-b023` — **branch from
`thread/g13-023-history-center-tree-core`, not from `main`.**
Governing refs: `docs/contracts/components/history-center.md`,
`docs/architecture/006-headless-core-and-machine-model.md`,
`docs/requests/longhorn-fork-tree-page.md`

## Goal

Rebind the Svelte and React components onto the v2 tree machine and draw the
tree. `g13-b023` delivered the stitcher, the row model, and the lane metadata;
this card renders them.

## Base — read before starting

**You are branching from `g13-b023`, not `main`.** `main` still has the v1
machine. Confirm before you write anything:

```sh
git log --oneline -3          # d0ac9b05 and 0e1bb49a must be present
rg -c 'historyCenterRows' packages/core/src/history-center.ts
effigy test:core              # must be 466 pass / 0 fail
effigy test:components        # EXPECTED RED: 8 fail / 890 pass
```

That red is the work. The two HistoryCenter suites test the retired v1 API.
If `test:components` is green at the start, you are on the wrong base — stop.

b023 is deliberately unmerged so `main` never carries the broken state.

## What b023 Already Gives You

Do not re-derive any of this.

```ts
historyCenterRows(branches, paths): HistoryCenterRow[]

type HistoryCenterRow =
  | { kind: "entry"; index; branchId; entry; depth; lane: HistoryRowLane }
  | { kind: "caption"; index; branch; depth }

interface HistoryRowLane {
  branchId: string;
  parentBranchId: string | null;  // null for the spine and root-attached runs
  start: boolean;                 // run's first entry row — the elbow
  continue: boolean;              // any row that is not the run's first
  end: boolean;                   // run's last entry row
}
```

`index` always equals the row's array position — keyboard traversal is linear
over the array in visual order. Lane structure is always true; only `depth`
saturates at 3.

## Fixed By Ruling (do not re-decide)

- **Rendering only.** The stitcher and machine are settled. If rendering seems
  to need a machine change, stop and report — that is a design gap worth
  knowing about, not something to patch around.
- **No clocks.** `HistoryEntry.recordedAtMs` is optional and authority-supplied.
  A run caption derives its relative time from **its own run's most recent
  entry**, and renders no time at all when the field is absent. `Date.now()`
  anywhere is a stop condition.
- **Captions rename, never navigate.** Independently required upstream:
  `ForkNavigationTarget::Checkout` cannot express an empty branch head, so
  there is no target to navigate to even if the UI offered one.
- **One navigation callback.** `onNavigateEntry(branchId, entryId)`, always the
  entry actually clicked. No checkout concept anywhere in the surface.
- **Lane styling is recipe-themable**, following the established
  `--poodle-recipe-history-center-*` convention already in the stylesheet.
  Light git-graph: a vertical lane plus an elbow into the run. Restrained —
  this is a popover, not a graph viewer.

## Scope

### In scope

- `packages/svelte/components/src/HistoryCenter.svelte` and
  `packages/react/components/src/HistoryCenter.tsx` rebound onto the v2
  machine. Identical prop names, identical behaviour, per the parity rules.
- `packages/core/src/styles/history-center.css`: lane and indentation
  rendering, depth-driven inset, caption rows, recipe hooks for lane colour
  and thickness.
- Both component test suites rewritten against v2 — the 8 currently-failing
  tests are v1 API tests and should be replaced, not patched.
- Specimens in both web runtimes, covering exactly the acceptance list:
  linear-only; one fork mid-spine; fork-off-a-fork; many shallow forks (the
  six-branch field case); rejection notice; rename within a caption.
  Svelte and React labels identical.
- Contract: the rendering, specimen, and recipe-hook sections b023 marked as
  `024`-owned, plus the lane hooks in the token table.
- Accessibility: list semantics and keyboard traversal across spine and runs.
  Depth must reach assistive tech — `aria-level` on rows, or an equivalent
  the contract records and justifies.

### Out of scope — stop conditions if reached

- `packages/core/src/history-center.ts` — the machine is b023's. Test files
  are yours; the machine is not.
- Native adapters (GPUI, Jetstream). The follow-up card, after this merges.
- Any Longhorn or Loophole file.
- Fetching or paging strategy. The component takes `branches` and `paths` as
  data.
- Refreshing visual baselines. Enumerate and classify; HistoryCenter diffs are
  expected, anything else is a stop condition.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read the v2 contract and `packages/core/src/history-center.ts` in full first.
  The doc comments state the row and lane semantics precisely; follow them
  rather than inferring from the tests.
- Svelte is the reference implementation (`001-working-rules.md`, Runtime
  Parity Authority). Build it first, then mirror React.
- Do not weaken a test to make it pass. If a v2 behaviour looks wrong, stop.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-024-history-center-tree-web`. Do not merge.

## Writable Paths

- `packages/svelte/components/src/HistoryCenter.svelte`
- `packages/react/components/src/HistoryCenter.tsx`
- `packages/svelte/components/test/HistoryCenter.test.ts`
- `packages/react/components/test/HistoryCenter.test.tsx`
- `packages/core/src/styles/history-center.css`
- `packages/{svelte,react}/preview/src/**/HistoryCenterSpecimen.*`
- `docs/contracts/components/history-center.md`
- `docs/logs/2026-08/<DD>-g13-024-history-center-tree-web.md`
- `PAPERCUTS.md` (new, non-duplicate friction only)

## Steps

1. Verify the base per "Base — read before starting". Record exit states,
   including the expected 8 failures.
2. Read the v2 contract and machine.
3. Svelte first: rows, lanes, captions, rename, rejection, keyboard.
4. Mirror React exactly.
5. Lane CSS with recipe hooks and depth-driven inset.
6. Rewrite both test suites against v2.
7. Specimens in both runtimes, all six cases.
8. Contract: rendering sections, recipe-hook table, the a11y depth decision.
9. Visual enumeration in report mode; classify. Refresh nothing.
10. Validate:
    ```sh
    effigy test:core
    effigy test:components
    effigy test:parity
    effigy docs:lint
    effigy docs:contract-drift
    effigy svelte:surface-audit
    git diff --check
    git status --porcelain
    ```

## Acceptance Criteria

- [ ] `effigy test:components` is **green** — the 8 v1 failures are gone,
  replaced by v2 coverage rather than deleted.
- [ ] Both runtimes render spine and runs with lanes, indentation saturating
  at depth 3 while `branchId` stays true.
- [ ] Clicking any entry emits `onNavigateEntry(branchId, entryId)` for that
  exact entry. Captions rename and do not navigate.
- [ ] Keyboard traverses spine and runs linearly in visual order.
- [ ] All six specimen cases present in both runtimes, labels identical.
- [ ] No `Date.now()`; captions render a time only when entries supply one.
- [ ] Contract records rendering, lane recipe hooks, and the a11y depth
  decision.
- [ ] All step-10 commands exit 0; no baseline refreshed.

## Stop Conditions

- The base is wrong (`test:components` green at the start).
- Rendering appears to require a machine or stitcher change.
- Lane metadata is insufficient to draw the graph — say exactly which case.
- A visual diff appears on a component other than HistoryCenter.

Stop with exact paths, commands, and the smallest unresolved question.
