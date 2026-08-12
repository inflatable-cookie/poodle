# 034 HistoryCentre — Three Field Defects From Loophole

Status: ready
Milestone: side-quest (component behaviour, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-034-history-center-field-defects`
Depends on: `g13-b033` (`aa0350d2`), merged
Governing refs: `docs/contracts/components/history-center.md`,
`packages/core/src/history-center.ts`

## Goal

Three defects found by Loophole in live use. Two of them are one root cause.

Loophole did not work around any of them, which is correct — an adapter that
hides a contract gap makes the gap permanent.

## Fixed By Ruling (do not re-decide)

### R1 — The picker row's `disabled` governs the Select, not the actions menu.

`history-center.ts:342` sets the picker row's `disabled: forkCount <= 1`.
`HistoryCenter.svelte:824` passes it to the `Select` — **correct**: with one
auto-chosen fork there is nothing to choose between.

`HistoryCenter.svelte:867` also passes it into `pickerActions`, and line 401
folds it into Checkout's enablement. That is the defect. One alternative fork
is the common case, so Checkout greys out in exactly the situation
checkout-without-moving-HEAD exists to serve, with no reason shown.

Checkout's only correct gates are:

- `picked === undefined`
- `picked.preferred` (already the current line)
- a rename is open

The auto-chosen single fork **counts as picked**. `pickedEntryId` already falls
back to `level.chosen?.entryId` (`history-center.ts:341`), so verify
`pickedContinuation` resolves it and fix it there if it does not — do not paper
over it in the renderer.

Rename has the same problem in practice: check every item's enablement, not
just Checkout's.

### R2 — Defects 2 and 3 are one root cause: a stale open level.

**Loophole's diagnosis for defect 2 is wrong, and building to it would add
machinery Poodle does not need.** The report says "the machine never re-reads
props after OPEN … no `$effect` re-sending data". Both runtimes already re-read
props live:

- Svelte: `rows` is `$derived` (`HistoryCenter.svelte:160`) and
  `machineContext` is `$derived` (line 173).
- React: both are recomputed every render (`HistoryCenter.tsx:160`, `:171`).

Do **not** add a `SYNC` / `REFRESH` event on the reasoning that props are
snapshotted. They are not.

The real cause is that an open level caches `continuations`, `chosen`, `pick`
and `runPages` from when it loaded, and nothing invalidates them when the host
supplies root pages that already contain that run. `pushRun` then splices the
cached run **under a spine that now holds the same entries**.

That produces both reported symptoms at once:

- the duplicate Svelte keys (`each_key_duplicate … at indexes 7 and 8`), and
- the "list unchanged until reopen" reading — the list *did* change, it just
  grew a duplicate copy of the run under the old fork point.

Proven, not inferred. This test fails on `main` today:

```ts
// packages/core/test/history-center.test.ts
test("a fork run whose entries now sit on the spine emits no duplicate rows", () => {
  const level = {
    anchorEntryId: "c2",
    continuations: [continuation("f1", { branchId: "feature/alt" }),
                    continuation("g1", { branchId: "feature/lead", preferred: true })],
    pick: null,
    chosen: continuation("f1", { branchId: "feature/alt" }),
    runPages: [page([entry("f2"), entry("f1")])],
    inner: null,
  } as HistoryCenterOpenFork;

  // The host navigated into the fork: f1/f2 are the primary line now and
  // arrive in new root pages. The open level is untouched.
  const pagesAfter = [page([entry("f2"), entry("f1"), entry("c2", 2), entry("c1")])];
  const rows = historyCenterVisibleRows(pagesAfter, new Map([["c2", level]]));
  const ids = rows.filter((r) => r.kind === "entry").map((r) => r.entry.id);
  expect(ids.filter((id, i) => ids.indexOf(id) !== i)).toEqual([]);
});
```

Observed output today:

```
entry:c1  entry:c2  picker:c2  entry:f1  entry:f2  entry:f1  entry:f2
                              └─ stale level ─┘  └── new spine ──┘
```

**Required behaviour.** A level whose shown fork now sits on the spine is
stale. Poodle must:

1. Never render a spliced run that duplicates a spine entry.
2. Keep the level **open** — disclosure is UI state and persists (b028 R1);
   this is not a close.
3. Drop that level's loaded data and re-request it through the existing
   `loadContinuations` effect, so the picker re-reads its continuations and
   offers the line just left. Until it arrives the level renders the existing
   `not-yet-loaded` row — do not invent a new row kind.

Detect staleness from the **data**, not from array identity: the level is stale
when its `chosen`/`pick` fork's first entry id appears in the joined root
pages. A `pages`-identity `$effect` is the wrong trigger — a host that rebuilds
its pages array each render would loop.

### R3 — Everything v3 holds.

One loop over `historyCenterVisibleRows`. No `svelte:self`, no self-import.
`depth` drives padding only. No Longhorn import, no `fetch`. Svelte first,
React mirrors exactly.

## Scope

### In scope

- `packages/core/src/history-center.ts`: the menu-enablement inputs (R1) and
  the stale-level rule (R2).
- Both web components: the actions menu must stop inheriting `row.disabled`.
- Both test suites, core and component.
- Contract: the Select-vs-menu split, and the stale-level rule with its
  re-request.

### Out of scope — stop conditions if reached

- Any Longhorn or Loophole file.
- Native adapters.
- Refreshing visual baselines.
- Adding a `SYNC` / `REFRESH` event (see R2).

## Required Tests

Core:

- The duplicate-row test above, verbatim, passing.
- A stale level keeps `open` set for its anchor — it is not closed.
- A stale level re-requests through `loadContinuations` exactly once, not on
  every derivation.
- A level whose run does **not** appear on the spine is untouched — no
  spurious invalidation, no extra load.

Components, both runtimes:

- `forkCount === 1`: the Select is disabled AND Checkout is enabled, given a
  non-preferred auto-chosen fork.
- `forkCount === 1`: Checkout is still disabled when the single fork is the
  preferred one — the `picked.preferred` gate, not the row gate.
- Rename is enabled on the single-fork row.
- No duplicate `data-row-entry` on any row after the host supplies pages
  containing an open level's run.

Do not weaken an existing test to make one of these pass.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- R2's diagnosis is settled and evidenced. If you believe it is wrong, stop and
  say why — do not implement a different theory silently.
- **Run `effigy check:svelte`.** Not optional.
- `docs:callback-drift` is a new gate — run it.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-034-history-center-field-defects`. Do not
  merge.

## Writable Paths

- `packages/core/src/history-center.ts`
- `packages/core/test/history-center.test.ts`
- `packages/svelte/components/src/HistoryCenter.svelte`
- `packages/react/components/src/HistoryCenter.tsx`
- `packages/{svelte,react}/components/test/HistoryCenter.test.*`
- `packages/svelte/components/test/HistoryCenterHostHarness.svelte`
- `packages/{svelte,react}/preview/src/**/HistoryCenterSpecimen.*`
- `docs/contracts/components/history-center.md`
- `docs/logs/2026-08/<DD>-g13-034-history-center-field-defects.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy test:core`, `test:components`, `check:svelte`,
   `docs:lint`, `git diff --check`. All start green.
2. Land the R2 repro test first and watch it fail. Do not start the fix until
   you have seen it fail.
3. Core: the stale-level rule, then the menu-enablement inputs.
4. Svelte: stop passing `row.disabled` into `pickerActions`; keep it on the
   `Select`.
5. Mirror React exactly.
6. Contract.
7. Validate:
   ```sh
   effigy test:core
   effigy test:components
   effigy test:parity
   effigy check:svelte
   effigy docs:lint
   effigy docs:contract-drift
   effigy docs:callback-drift
   effigy svelte:surface-audit
   effigy drift:recipes
   git diff --check
   ```

## Acceptance Criteria

- [ ] Single-fork row: Select disabled, Checkout and Rename enabled on their
  own gates.
- [ ] The R2 repro test passes, and no derivation emits a duplicate entry id.
- [ ] A stale level stays open, re-requests once, and shows `not-yet-loaded`
  until its data lands.
- [ ] No `SYNC` / `REFRESH` event was added.
- [ ] All step-7 commands exit 0; no baseline refreshed.

## Stop Conditions

- The stale-level rule cannot distinguish "the run is on the spine now" from a
  legitimately repeated entry id. Say what you tried.
- Re-requesting loops — the load result re-triggers staleness.
- Enabling Checkout on the single-fork row breaks the `AlreadyAtTarget` rule.

Stop with exact paths, commands, and the smallest unresolved question.
