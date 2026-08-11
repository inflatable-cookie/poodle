# 11 — g13.019 HistoryCenter Web Reference (batch log)

Branch: `thread/g13-019-history-center-web` (dedicated worktree)
Date: 2026-08-11
Card: `docs/roadmaps/g13/batch-cards/019-history-center-web-reference.md`
Status: **DELIVERED** — headless machine + contract + Svelte + React at parity
+ specimens + zero audit gaps. Native parity is `020` (required, not this
card).

## 1. Headless machine — `packages/core/src/history-center.ts`

Framework-free machine following `tabs.ts`/`popover.ts` style (pure
`transition(state, context, event) → { state, context, effects[] }`, callbacks
as effects, adapter executes effect intents).

### State model

- `state`: `"closed" | "open"` — the popover open state (TOGGLE/OPEN/CLOSE;
  OPEN and CLOSE idempotent in their target state).
- `context`: `{ entries, branches, expandedBranchIds, focusIndex, rejection }`
  — entries/branches host-supplied every transition; `branches: null`
  disables all branch/checkpoint presentation (presence-driven, no `mode`
  prop).
- Rows: `historyCenterRows(entries, branches, expandedBranchIds)` flattens
  entries plus branch rows under expanded forks (branches inserted directly
  after their fork entry — stable index math). `isForkPoint` = card rule
  `branchCount > 1`; expansion additionally requires `branches !== null`.
- Events: `TOGGLE/OPEN/CLOSE`, `FOCUS_MOVE` (next/prev/first/last, wraps),
  `ACTIVATE_ROW` (entry → `emitSelectEntry`, branch → `emitCheckout`),
  `TOGGLE/EXPAND/COLLAPSE_BRANCHES`, `CHECKOUT`, `RENAME`,
  `SHOW_REJECTION`, `DISMISS_REJECTION`.
- Effects: `emitOpenChange`, `focusRow`, `emitSelectEntry`, `emitCheckout`,
  `emitRenameBranch`. Undo/redo/load-more are plain button commands the
  adapter forwards directly — the machine carries no state for them and no
  callback is ever invoked speculatively (the host owns what undo does).
- Guards: non-fork entries, missing `branches`, out-of-range activation, and
  empty lists are inert (stay, no effects). Collapsing a fork moves focus off
  its branch rows onto the fork entry; focus never survives on a removed row.
- Transient rejection: `SHOW_REJECTION` displays a message (idempotent for
  the same message), `DISMISS_REJECTION` clears it. The adapter diffs the
  prop (`lastRejectionProp`): a *new* non-null value displays, dismissal is
  local and never re-shows the same value, host clearing the prop clears the
  notice.
- Keydown mapper `historyCenterKeydownEvent`: ArrowDown/Up, Home/End,
  Enter/Space → machine events; other keys propagate (Tab reaches the focus
  trap, Escape the dismiss layer).

Unit tests: `packages/core/test/history-center.test.ts`, 25 tests / 55
expectations, `bun test` green — open state, row flattening, wrap navigation,
activation routing, expansion gating, collapse focus clamp, rejection
transience, command pass-through, keydown mapping.

## 2. Contract — `docs/contracts/components/history-center.md`

Fixed prop surface implemented verbatim (entries, totalEntries,
hasMoreEntries, branches, totalBranches, hasMoreBranches, canUndo, canRedo,
busy, status, statusMessage, rejection, maxBranchNameBytes=256, open
(bindable), defaultOpen, placement, undoLabel, redoLabel, listLabel, title,
emptyMessage, ariaLabel, size, sizeRole="chrome", density; callbacks onUndo,
onRedo, onOpenChange, onSelectEntry, onCheckout, onRenameBranch,
onLoadMoreEntries, onLoadMoreBranches). Records: no Longhorn dependency
(direction is Longhorn → Poodle; no type imports, no `ForkHistorySession`,
no `ForkHistoryPanel` composition), no protocol enforcement (constants live
in the Longhorn crate; `maxBranchNameBytes` is a client-side affordance
only), presence-driven linear degradation, create-checkpoint deferral
(known-deltas table), component-owned popover content, machine-backed
classification with full Behavior Machine section (context/states/events/
transitions/effects/part attributes/machinery dependencies), token table
with 17 recipe hooks, GPUI notes pointing at `020`.

Prop surface was checked against the real authority shapes (read-only, in
`~/Dev/projects/longhorn`): `ForkEntryRecord`/`ForkBranchRecord`/path+branch
page snapshots confirm the bridge is a plain mapping (`branchCount`/
`checkpoint`/`entryCount` are HistoryCenter-side derivations the bridge
computes; `position` and branch `current`/`pinned`/`name: Option<String>`
carry over directly). `checkout(entry)` semantics observed in
`ForkHistoryPanel.svelte` (skip current, carry branch context) inform the
`onCheckout(branchId, entryId)` contract but are not enforced by the
component.

## 3. Svelte and React implementations

Both at parity: same props/defaults/semantics, same `open` bindable pattern,
same machine wiring (`send()` channel; Popover's `onOpenChange` re-enters as
OPEN/CLOSE). Trigger cluster is three ghost IconButtons — undo
(`arrow-left`), list (`list`), redo (`arrow-right`) — enablement
`canUndo/canRedo` AND NOT `busy`. Popover: anchored, dismiss-on-outside,
initial focus first-focusable, surface focus trap via `trapFocusKeydown`
while open, roving `tabindex` over rows, `historyCenterKeydownEvent` per
keydown. Paged list renders only supplied entries + load-more with supplied
count as offset; nothing assumes a complete history. Fork points expand to
branch rows (name-or-id, entry count, current badge) with inline rename
(Enter commits → `onRenameBranch`, Escape cancels, blur commits; `maxlength`
= `maxBranchNameBytes`); checkpoint pins and fork indicators render only
when `branches` is supplied. Rejection renders as a dismissible
`role="status"` notice; loading/failed status rows present; empty state
reuses `EmptyState`.

Glyph constraint: the 85-icon default set has no undo/redo/history/pin
glyphs (`corner-up-left`/`rotate-ccw`/`pin` absent), so the cluster uses
`arrow-left`/`list`/`arrow-right` and checkpoints use
`git-commit-horizontal` — recorded as a papercut (§8).

## 4. Audit surfaces (all four)

- contract: `docs/contracts/components/history-center.md` (+ both contract
  index READMEs)
- component registry: `entry("HistoryCenter", …)` in
  `packages/svelte/preview/src/component-registry.ts`
- specimen registry: `"history-center": HistoryCenterSpecimen` in
  `packages/svelte/preview/src/specimens/registry.ts` (+ React
  `gallery/specimen-map.ts`)
- usage docs: `"history-center"` entry in
  `packages/svelte/preview/src/component-docs.ts` (full prop table + usage
  snippet)

Specimens in both runtimes with identical labels: `linear`, `fork`,
`rejection`, `empty`, `loading`. No visual baselines exist for the new slug;
none were created (no `--update`).

## 5. Validation (step 8)

| Command | Exit | Notes |
|---|---|---|
| `bun install` | 0 | 234 packages |
| `effigy test:core` | 0 | 458 tests / 44 files, 0 fail |
| `effigy test:components` | 0 | 879 tests / 46 files (was 855; +20 HistoryCenter, +4 parity additions) |
| `effigy test:parity` | 0 | 164 tests / 2 files |
| `effigy svelte:surface-audit` | 0 | 164 exports, 0 gaps (was 163) |
| `effigy docs:lint` | 0 | 171 component contracts (was 170) |
| `effigy docs:contract-drift` | 0 | every documented public prop implemented in Svelte |
| `effigy docs:check` | 0 | rewrote `packages/tokens/artifacts/rust/*` (card warning) + preview artifact JSONs |
| `git checkout -- packages/tokens/artifacts/rust/` | 0 | restored; nothing from that directory committed |
| `git checkout -- packages/{react,svelte}/preview/artifacts/{component-docs,parity-report}.json` | 0 | restored regenerated build outputs; not in the writable set (b008 precedent) |
| `git diff --check` | 0 | clean |
| `git status --porcelain` | — | only the writable paths (§7) |

`effigy docs:spec-drift` (exit 0): history-center lands in the expected
"skipped (no contract/spec/props)" bucket — **no Spec exists for
history-center, which is expected until card `020`; no Spec added here.** The
skipped set is the recorded gap.

Extra (not gated by the card, run for confidence): `effigy test:a11y` exit 0
(166 tests, includes HistoryCenter via the component glob).

## 6. Acceptance criteria

- [x] Headless machine in `poodle-core` with unit tests, no framework import
  (§1; `bun:test`, `bun test` green; machine type-checks standalone under
  `tsc --strict`).
- [x] Svelte and React at parity, fixed prop surface verbatim (§2, §3).
- [x] Trigger cluster undo/list/redo; enablement from `canUndo`/`canRedo`/
  `busy` (§3; covered by tests).
- [x] Popover anchored, dismiss-on-outside, focus-trapped, keyboard
  navigable (§3; trap + roving focus + wrap navigation).
- [x] List renders only supplied entries; load-more with supplied count as
  offset; nothing assumes completeness (§3).
- [x] Fork points expand; branch rows show name and count; rename inline
  emits `onRenameBranch` (§3; covered by tests).
- [x] Omitting `branches` hides all branch and checkpoint UI (§3; covered by
  tests — fork indicator, branch rows, and checkpoint pin all absent).
- [x] `rejection` renders as a visible transient notice (§3; covered by
  tests).
- [x] No Longhorn import anywhere (grep over all new files: only the
  contract's prose explaining the boundary; no type or component reference).
- [x] `svelte:surface-audit` reports zero gaps (164/164).
- [x] All step-8 commands exit 0 except the recorded spec-drift skip (§5).
- [x] Batch log records commands, exit states, and the machine's state model
  (§1, §5).

## 7. Changed paths (writable set only)

```
 docs/contracts/README.md                                  | index + history-center.md
 docs/contracts/components/README.md                       | index + history-center.md
 docs/contracts/components/history-center.md               | new contract
 docs/logs/2026-08/11-g13-019-history-center-web.md         | this log
 packages/core/src/history-center.ts                       | new headless machine
 packages/core/src/styles/history-center.css               | new styles (17 recipe hooks)
 packages/core/src/index.ts                                | machine exports
 packages/core/test/history-center.test.ts                 | new unit tests
 packages/react/components/src/HistoryCenter.tsx           | new component
 packages/react/components/src/index.ts                    | export
 packages/react/components/src/types.ts                    | HistoryEntry/HistoryBranch/HistoryStatus
 packages/react/components/test/HistoryCenter.test.tsx     | new tests
 packages/react/preview/src/gallery/specimen-map.ts        | slug + import
 packages/react/preview/src/gallery/specimens/HistoryCenterSpecimen.tsx | new specimen
 packages/svelte/components/src/HistoryCenter.svelte       | new component
 packages/svelte/components/src/index.ts                   | export + types
 packages/svelte/components/src/types.ts                   | HistoryEntry/HistoryBranch/HistoryStatus
 packages/svelte/components/test/HistoryCenter.test.ts     | new tests
 packages/svelte/preview/src/component-docs.ts             | usage docs entry
 packages/svelte/preview/src/component-registry.ts         | registry entry
 packages/svelte/preview/src/parity.ts                     | HistoryCenter export coverage
 packages/svelte/preview/src/specimens/HistoryCenterSpecimen.svelte | new specimen
 packages/svelte/preview/src/specimens/registry.ts         | slug + import
 PAPERCUTS.md                                              | friction entry (§8)
```

No roadmap/milestone/card status files touched; no `dispatch.md`; no
`git add -A`; no visual baselines refreshed; Tabs / NavigationMenu /
AppHeader / `004-shared-control-types.md` untouched.

## 8. Notes

- Papercut: the 85-icon default icon set has no undo/redo/history/pin
  glyphs, so HistoryCenter's titlebar-grade cluster falls back to
  `arrow-left`/`list`/`arrow-right` and checkpoint pins to
  `git-commit-horizontal`. Tooltips/labels carry the semantics, but adding
  the Lucide `corner-up-left`/`corner-up-right` (or `rotate-ccw`/`rotate-cw`)
  and `pin` glyphs would let a future host use canonical undo/redo/pin
  affordances. Requires an icon-set change (outside this card's writable
  set).
- `packages/svelte/preview/artifacts/recipe-inventory.json` not regenerated:
  same reasoning as b008 — its generator is not wired into any gate and the
  inventory is already stale vs earlier hooks.
- Pre-existing `tsc -p` failures in `packages/core` (`src/icons/index.ts`,
  `test/audio-envelope.test.ts`, `test/icons.test.ts`,
  `test/model-picker.test.ts`) are untouched by this card; the new machine
  type-checks clean standalone (`tsc --strict`, exit 0). No step-8 command
  runs bare `tsc`.
