# 029 HistoryCentre v3 — Rendering, Picker, Nested List

Status: merged (`16b68e61` → `2a6d3af9`)
Milestone: side-quest (component architecture, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-029-history-center-v3-web`
Depends on: `g13-b028` — **branch from
`thread/g13-028-history-center-v3-core`, not from `main`.**
Governing refs: `docs/contracts/components/history-center.md`,
`docs/contracts/001-working-rules.md` §Runtime Parity Authority,
`packages/svelte/components/src/Tree.svelte` (the rendering precedent)

## Goal

Rebind both web runtimes onto the v3 core and render the flat list with
node-owned forks. `g13-b028` delivered the record types, the visible-row
derivation and the machine. This card draws them.

## Base — read before starting

**You branch from `g13-b028`, not `main`.** Confirm before writing anything:

```sh
git log --oneline -2          # 680f3c64 must be present
effigy test:core              # 482 pass / 0 fail
effigy test:components        # EXPECTED RED: 30 fail / 944 pass
```

That red is the work. The failures are both HistoryCenter suites plus
`smoke.test.tsx` and `parity.test.tsx`, which are glob-driven over every
component and fail because the prop surface changed. If `test:components` is
green at the start, you are on the wrong base — stop.

b028 stays unmerged so `main` never carries the broken state.

## What Core Already Gives You

Do not re-derive any of it. Do not reshape it.

```ts
historyCenterVisibleRows(pages, open): HistoryCenterRow[]   // flat, display order
historyCenterForkCount(continuationCount): number            // R4, floored at 0
historyCenterJoinPages(pages): HistoryEntry[]                // older page first
historyCenterForksAt(...)                                    // filtered forks
historyCenterRejectionMessage(code)                          // AlreadyAtTarget | UnknownEntry
```

Row kinds: `entry`, `picker`, `not-yet-loaded`. Every row carries `depth`,
`parentEntryId` and `forkId`. Entry rows also carry `branchId` and `forkCount`.

Events you dispatch: `DISCLOSE`, `CONTINUATIONS_LOADED`, `PICK_CONTINUATION`,
`CONFIRM`, `RUN_LOADED`, `RENAME`, `SHOW_REJECTION`, `DISMISS_REJECTION`, plus
the existing open/close/focus/activate.

Effects you honour: `emitOpenChange`, `focusRow`, `emitNavigateEntry`,
`emitRenameBranch`, `loadContinuations`, `loadContinuationRun`,
`preferContinuation`.

## Fixed By Ruling (do not re-decide)

### R1 — One loop. No recursion.

Render `historyCenterVisibleRows` with a single `{#each}` / `.map()`. No
`svelte:self`, no self-import, no recursive React component. `depth` drives
indentation and nothing else — it is a number for padding, never a source of
truth about structure.

`Tree.svelte` is the precedent: it renders `flattenVisibleTreeRows` output in
one loop. Follow it.

A "nested list" is not a nested component. It is rows at a greater depth in the
same loop, and core already emits them in the right place.

### R2 — The three host operations are callbacks.

Poodle cannot call a Longhorn controller. The effects `loadContinuations`,
`loadContinuationRun` and `preferContinuation` become props the host supplies:
`onLoadContinuations`, `onLoadContinuationRun`, `onPreferContinuation`. The host
resolves each and feeds the result back as `CONTINUATIONS_LOADED` /
`RUN_LOADED`, or as `SHOW_REJECTION`.

Do not import a Longhorn type. Do not add `@inflatable-cookie/longhorn` to any
manifest. Do not call `fetch`.

### R3 — Affordances follow `forkCount`, exactly.

- `forkCount === 0` — no fork icon, no badge, no chevron. The entry is inert.
- `forkCount === 1` — fork icon and chevron. **No picker.** One continuation is
  nothing to choose between.
- `forkCount > 1` — fork icon, a counter badge reading `forkCount`, and a
  chevron. Opening yields a picker row.

Core already emits the picker row only when `forkCount > 1`. Do not add a
second condition in the renderer.

### R4 — Confirm does not move the document.

Activating a fork emits `preferContinuation`. It applies no delta. Disable
confirm when the picked continuation already has `preferred: true` — that is
what makes `AlreadyAtTarget` a race rather than a normal path.

### R5 — Svelte first, then React mirrors it exactly.

Runtime Parity Authority. Identical prop names, identical defaults, identical
labels in specimens.

## Scope

### In scope

- `HistoryCenter.svelte` and `HistoryCenter.tsx` rebound onto v3.
- The three callback props, plus the v2 props that survive. Remove `branches`
  and anything else v3 does not read; the props table is yours to update.
- `history-center.css`: **delete the lane rendering**. v3 has no lanes. Add
  depth-driven inset, the fork affordance, the badge, the picker row, and the
  opened region. Recipe hooks follow the existing
  `--poodle-recipe-history-center-*` convention.
- Both test suites rewritten against v3 — the 30 failures are v2-API tests and
  should be replaced, not patched.
- Specimens in both runtimes covering exactly:
  - two forks at one entry (badge reads 2, picker has two options)
  - a fork off a fork
  - `continuationCount: 1` — no badge, no picker
  - a run's last entry — no fork affordance
  - a rejection notice
  - `recordedAtMs: null` — a caption with no time
- Contract: the sections `b028` marked `029`-owned, the props table, the recipe
  hooks, and the accessibility decision.
- Accessibility: list semantics, keyboard traversal over the visible rows, and
  depth exposed to assistive tech (`aria-level` or an equivalent the contract
  records and justifies).

### Out of scope — stop conditions if reached

- `packages/core/src/history-center.ts` — core is `b028`'s. Test files are
  yours; the machine is not.
- Native adapters. A later card.
- Any Longhorn or Loophole file.
- Refreshing visual baselines. Enumerate and classify; HistoryCenter diffs are
  expected, anything else is a stop condition.

## Tests Worth Having

Beyond replacing the 30:

- Two forks at one entry render one badge reading 2 and one picker with two
  options, and are **not** confusable with a fork off a fork. Assert the
  rendered rows, not just the row data.
- Opening an entry, picking the non-preferred fork, confirming: emits
  `onPreferContinuation` with that entry id, and emits no navigation.
- Confirm is disabled when the picked continuation is already `preferred`.
- `continuationCount: 1` renders no badge and no picker.
- A run's last entry renders no fork affordance.
- `recordedAtMs: null` renders a caption with no time and shows no
  "Invalid Date".
- A row at depth 3 and a row at depth 5 both render, and neither is clamped —
  v2's depth cap is gone and must not return in CSS either.
- Both runtimes render the same anatomy for the same props (the parity suite
  covers this; make sure it does).

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read the v3 contract, `packages/core/src/history-center.ts`, and
  `Tree.svelte` in full before writing. The core doc comments state the row
  semantics precisely; follow them rather than inferring from tests.
- Build Svelte first, then mirror React.
- Do not weaken a test to make it pass. If a v3 behaviour looks wrong, stop.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-029-history-center-v3-web`. Do not merge.

## Writable Paths

- `packages/svelte/components/src/HistoryCenter.svelte`
- `packages/react/components/src/HistoryCenter.tsx`
- `packages/svelte/components/test/HistoryCenter.test.ts`
- `packages/react/components/test/HistoryCenter.test.tsx`
- `packages/core/src/styles/history-center.css`
- `packages/{svelte,react}/preview/src/**/HistoryCenterSpecimen.*`
- `docs/contracts/components/history-center.md`
- `docs/logs/2026-08/<DD>-g13-029-history-center-v3-web.md`
- `PAPERCUTS.md`

## Steps

1. Verify the base per "Base — read before starting". Record exit states.
2. Read the contract, the core module, and `Tree.svelte`.
3. Svelte: rows, affordances, picker, confirm, opened region, rename, keyboard.
4. Mirror React exactly.
5. CSS: delete lanes, add depth inset and the new affordances.
6. Rewrite both suites against v3.
7. Specimens in both runtimes, all six cases.
8. Contract: the `029`-owned sections, props table, recipe hooks, a11y.
9. Visual enumeration in report mode; classify. Refresh nothing.
10. Validate:
    ```sh
    effigy test:core
    effigy test:components
    effigy test:parity
    effigy check:svelte
    effigy docs:lint
    effigy docs:contract-drift
    effigy svelte:surface-audit
    git diff --check
    ```

## Acceptance Criteria

- [ ] `effigy test:components` is **green** — the 30 v2 failures are replaced by
  v3 coverage, not deleted.
- [ ] One loop per runtime. No recursive component anywhere.
- [ ] Affordances follow `forkCount` exactly, per R3.
- [ ] Confirm emits `onPreferContinuation` and never navigates.
- [ ] No lane CSS remains; no depth clamp in CSS.
- [ ] No Longhorn import, no Longhorn manifest entry, no `fetch`.
- [ ] All six specimen cases in both runtimes, labels identical.
- [ ] All step-10 commands exit 0; no baseline refreshed.

## Stop Conditions

- The base is wrong (`test:components` green at the start).
- Rendering appears to need a core change.
- A row kind cannot be rendered from the data core supplies. Name the row kind
  and the missing field.
- A visual diff appears on a component other than HistoryCenter.

Stop with exact paths, commands, and the smallest unresolved question.
