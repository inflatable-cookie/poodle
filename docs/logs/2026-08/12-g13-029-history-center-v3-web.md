# 12 — g13.029 HistoryCentre v3 — Rendering, Picker, Nested List (batch log)

Branch: `thread/g13-029-history-center-v3-web`
Date: 2026-08-12
Card: `docs/roadmaps/g13/batch-cards/029-history-center-v3-web.md`
Status: **BLOCKED AT VALIDATION** — rendering, picker, nested list, both
suites, specimens, CSS and the contract are done; `effigy check:svelte`
cannot exit 0 because `packages/core/src/history-center.ts` (b028's file,
frozen by this card) does not type-check. The defect and the smallest
unresolved question are reported at the end.

## 1. Base verification (step 1)

- `git log --oneline -2` → `d38abfc1` on `thread/g13-029-history-center-v3-web`;
  `680f3c64` (g13-028) present. ✓
- `effigy test:core` → 482 pass / 0 fail. ✓
- `effigy test:components` → **30 fail / 944 pass** (4 files: both
  HistoryCenter suites, the React smoke row, the parity row) — the expected
  red, all from the retired v2 surface (`historyCenterRows is not a
  function`). ✓ Base is the base the card promises.

## 2. Svelte — `packages/svelte/components/src/HistoryCenter.svelte`

Rebound onto v3 with ONE `{#each}` over `historyCenterVisibleRows(pages,
openForks)`, keyed by row identity (`kind + entry id`). No `svelte:self`, no
recursion, no nested list component (R1); `depth` drives a padding-left CSS
variable (`--poodle-history-center-depth`) and nothing else.

- Machine wiring: context `{ pages, open, focusRow, rejection }`; `send()`
  applies `result.context` back into local state and honours all seven
  effects (`emitOpenChange`, `focusRow` by identity, `emitNavigateEntry`,
  `emitRenameBranch`, `loadContinuations`, `loadContinuationRun`,
  `preferContinuation`) (R2 — the three host operations are callback props;
  no Longhorn import, no fetch).
- Result feeds: `continuationsResult` / `runResult` props diffed by reference
  in `$effect`s, dispatching `CONTINUATIONS_LOADED` / `RUN_LOADED` — the same
  reference-diff pattern v2's `rejection` prop used. `rejection` now carries
  the machine's `HistoryCenterRejectionCode` (`AlreadyAtTarget` /
  `UnknownEntry`); the machine owns the display copy.
- Rows: entry rows carry `data-fork-count`/`data-parent-entry`/`data-fork-id`
  plus `data-open`; the fork disclosure (icon + badge when `forkCount > 1` +
  chevron, `aria-expanded`, `Show|Hide N continuation(s)`) renders only when
  `forkCount > 0` — no badge or picker logic beyond the row's own `forkCount`
  (R3; the picker row appears only because core emits it). The picker offers
  each fork (label + branch name + Preferred badge) and a Choose confirm
  disabled while no pick is set or the pick is already `preferred` (R4).
  The not-yet-loaded row renders a spinner + "Loading…".
- Opened region (R6): a run's first entry row (`entry.id === forkId`) carries
  the chosen fork's name, run entry count and a derived relative time (from
  supplied `recordedAtMs` only — no clock, D2) plus the inline rename
  (Enter commits `onRenameBranch`, Escape cancels, blur commits; focus
  returns to the rename button).
- Keyboard: roving focus over the visible rows by identity (arrows wrap,
  Home/End); disclosure/picker/rename buttons keep native Enter/Space
  activation (the list keydown guard returns for them); the rename input owns
  its keys; the surface traps Tab. Depth reaches assistive tech via
  `aria-level` (depth + 1) — the contract records the decision (a `tree` role
  was rejected: traversal is linear over the flat array, so list semantics +
  `aria-level` are more honest).

## 3. React — `packages/react/components/src/HistoryCenter.tsx`

Exact mirror (Runtime Parity Authority, R5): identical props, defaults,
labels, anatomy classes, part attributes and result-feed semantics. The
`sendRef` pattern follows v2; the focusedRow effect only moves DOM focus when
the machine emitted a `focusRow` effect (a `focusRequestedRef` flag) so a
clampFocus identity change after DISCLOSE never steals focus from the
disclosure button the user clicked.

## 4. CSS — `packages/core/src/styles/history-center.css`

Lane rendering deleted (`.poodle-history-center__lanes`, `__lane`, the lane
recipe hooks, caption and branch-current-badge styles, load-more rows). Added:
the depth inset (`calc(inset-step × --poodle-history-center-depth)`, no clamp
anywhere), the fork disclosure (icon, chevron rotation, counter badge), the
picker row (options, preferred badge, confirm actions), the opened region
(run header + rename) and the not-yet-loaded row. New recipe hooks follow the
`--poodle-recipe-history-center-*` convention: `fork-color`,
`fork-badge-fill`/`-text`, `preferred-badge-fill`/`-text`; the retired lane
and current-badge hooks are gone from the contract table.

## 5. Tests — both suites rewritten against v3 (the 30 replaced, not patched)

18 tests per runtime (36 total → `test:components` 980 pass / 0 fail). The
Svelte flow tests drive a `HistoryCenterHostHarness.svelte` (a minimal host
simulation resolving the three ops synchronously from fixture maps); the React
flow tests use `rerender` with stable module-scope result references (the
reference-diff contract). Coverage includes every card item:

- badge reads 2 + one picker with two options at `forkCount 2`, asserted on
  the rendered rows (`data-row-kind`/`data-row-entry`/`data-depth` sequence),
  never confusable with a fork off a fork (depths 1 vs 2, `data-parent-entry`
  and `data-fork-id` asserted);
- picking the non-preferred fork and confirming → `onPreferContinuation(l1)`,
  no navigation, run revealed;
- confirm disabled when the picked continuation is already `preferred`;
- `continuationCount 1` → no badge and no picker (inert row);
- a run's last entry → no fork affordance;
- `recordedAtMs` absent → the run header reads "2 entries" with no time and no
  "Invalid Date" (and the timed variant derives "2 entries · 20m ago" from
  supplied data);
- depth 3 and depth 5 rows render, neither clamped (data-depth 3/5,
  aria-level 4/6, inset variable 3/5);
- keyboard traversal over visible rows incl. the picker stop, wrapping and
  Home/End;
- rename commit/cancel in the opened region; rejection copy + dismiss;
  empty/loading/failed states; `aria-level` on every row.

## 6. Specimens — both runtimes, six cases, identical labels

`linear`, `two-forks` (badge 2 + picker with two options; opens by default so
the capture shows the flat list), `fork-off-fork` (nested runs at depth 2),
`single-continuation` (`continuationCount 1` → inert), `run-tail` (a run's
last entry → no affordance), `rejection` (`AlreadyAtTarget` → the machine's
copy), `no-timestamp` (caption with count and no time), `rename`. Each group
wires the three host ops to resolve synchronously from fixture maps, exactly
as a real host would.

## 7. Contract — `docs/contracts/components/history-center.md`

All `029`-owned sections rewritten: §1 Purpose, §2 Anatomy, §3 Public Props
table (v3 surface: `pages`, the two result feeds, the three host-op
callbacks; `branches`/`paths`/totals/load-more removed; `onNavigateEntry`
branch is `string | null`), §4 Visual States and Part Attribute Output,
§5 Events, §6 Accessibility (semantics + keyboard incl. the disclosure/picker
native-activation rule), §7 Depth Inset (no cap), §8 Token Usage recipe hooks,
§9 Svelte Notes. `docs:lint` structure preserved.

## 8. Validation

| Command | Exit | Result |
|---------|------|--------|
| `effigy test:core` | 0 | 482 pass / 0 fail |
| `effigy test:components` | 0 | **980 pass / 0 fail (69 files)** — the 30 v2 failures replaced by 36 v3 tests |
| `effigy test:parity` | 0 | Svelte↔React anatomy parity green (HistoryCenter row now passes) |
| `effigy docs:lint` | 0 | 171 component contracts |
| `effigy docs:contract-drift` | 0 | contract props table ↔ Svelte `$props()` destructure, both directions |
| `effigy svelte:surface-audit` | 0 | — |
| `bunx svelte-check --workspace packages/svelte/components --tsconfig ./tsconfig.json --threshold error` | 1 | **18 errors, all from `packages/core/src/history-center.ts`** (17) plus one cascade line in `HistoryCenter.svelte`; zero errors in my files |
| `bunx tsc --noEmit --skipLibCheck -p packages/react/preview` | 1 | 22 errors, all core's; none in `HistoryCenter.tsx` / the specimen |
| `bunx svelte-check --workspace packages/svelte/preview --tsconfig ./tsconfig.json --threshold error` | 1 | my `HistoryCenterSpecimen.svelte`: 0 errors; remaining errors pre-existing (core + unrelated specimens) |
| `git diff --check` | 0 | — |

## 9. Stop condition — `effigy check:svelte` is blocked by b028's core type defect

`packages/core/src/history-center.ts` (this card's frozen file) does not
type-check as committed. `HistoryCenterOpenFork.inner` is declared a single
level at line 198 (`inner: HistoryCenterOpenFork | null`) but every use site
treats it as a `ReadonlyMap<string, HistoryCenterOpenFork> | null`
(`withAddedLevel(candidate.inner, …)`, `new Map(level.inner)`,
`findLevel(level.inner, …)`, `{ ...level, inner: withAddedLevel(…) }`,
`walkLevels(level.inner)`), plus strict-null gaps in the machine (`confirm`'s
`pick` possibly null at ~859, `anchor`/`entry` possibly undefined, several
`| undefined` → `| null` assignments). Type-only: `effigy test:core` passes
482/0, so runtime semantics are proven; but no type gate can pass while the
declaration contradicts the code. Verified pre-existing: the same errors
appear with my component stashed (36 errors at base = 18 core + 18 v2
component; my rewrite removes the v2 18 and adds zero). No fix exists on any
branch (`git log --all` — only `680f3c64` and unrelated t3 checkpoints touch
the file); core has no type gate in effigy, so the error shipped silently and
the card's premise that only `test:components` is red is off by this.

Commands that expose it:

```sh
bunx svelte-check --workspace packages/svelte/components --tsconfig ./tsconfig.json --threshold error   # 18 errors
bunx tsc --noEmit --skipLibCheck -p packages/core                                                        # 40 errors
```

Smallest unresolved question: **may I apply the type-only fixes to
`packages/core/src/history-center.ts`** — `inner` → `ReadonlyMap<string,
HistoryCenterOpenFork> | null` plus the strict-null guards (`pick`,
`anchor`, `entry`) — so `effigy check:svelte` can exit 0? Runtime semantics
are untouched; the 482 core tests stay green as the proof. Until answered,
the card is not committed or pushed (the branch carries only the writable
files, uncommitted).

## 10. Notes for the authority

- `component-docs.ts` (preview doc data) still tables v1/v2 HistoryCenter
  props (`onSelectEntry`, `onCheckout`, `entries`, `branchCount`) — outside
  this card's writable paths, ungated by any check that failed. A follow-up
  should regenerate it from the contract.
- The PAPERCUTS head-fork entry stands: a single fork off a run's last entry
  still reads `forkCount 0` and stays unreachable through the disclosure
  model; the specimens and tests place forks mid-run for that reason.
