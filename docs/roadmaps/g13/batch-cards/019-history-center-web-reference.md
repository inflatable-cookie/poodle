# 019 HistoryCenter — Web Reference

Status: ready
Milestone: side-quest (new component, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-019-history-center-web`
Followed by: `020` HistoryCenter native parity (required, not optional)
Governing refs: `docs/contracts/components/message-center.md` (the pattern),
`docs/architecture/006-headless-core-and-machine-model.md`,
`docs/architecture/007-appearance-recipe-contract.md`,
`docs/contracts/001-working-rules.md` §Runtime Parity Authority

## Goal

Build `HistoryCenter`: the history counterpart to `MessageCenter`. A compact
titlebar-grade trigger cluster plus a popover listing history, with fork
branches when the host supplies them.

This card delivers the **web reference** — contract, headless machine, Svelte,
React, specimens. Native parity follows in `020` and is required by the
parity rule; this split exists because Svelte is the reference implementation,
so the web surface must be settled before the native port.

## Maintainer Rulings (already decided — do not re-litigate)

1. **It lives in Poodle and is authority-agnostic.** Data in via props,
   commands out via callbacks. `MessageCenter` is the precedent: `items` in,
   `onItemSelect`/`onRemove`/`onMarkAllRead` out, no knowledge of any store.
2. **No Longhorn dependency, and none is possible.** The dependency runs
   Longhorn → Poodle; `longhorn-poodle-svelte` imports Poodle, not the reverse.
   Do not import Longhorn types, do not reference `ForkHistorySession`, and do
   not compose `ForkHistoryPanel` — that panel is a Longhorn-side dev panel
   (Buttons, Stack, InlineListSection, Spinner, Callout taking `session`
   directly). HistoryCenter is what replaces it; the Longhorn bridge maps
   session → props.
3. **US spelling: `HistoryCenter`**, matching `MessageCenter`.
4. **Protocol constants are not imported.** `MAXIMUM_FORK_BRANCH_NAME_BYTES`
   (256) and `MAXIMUM_FORK_PROJECTION_PAGE_SIZE` (256) live in the Longhorn
   crate. The component enforces no protocol rule: it emits a command, the host
   validates, and a rejection comes back for display. `maxBranchNameBytes` is an
   optional client-side affordance prop only.
5. **Linear degradation is presence-driven.** When `branches` is not supplied,
   all branch and checkpoint UI is hidden and the popover is a plain list. No
   `mode` prop — same reasoning as AppHeader's `center`.
6. **Create-checkpoint is deferred.** Checkpoints render as pins; creating one
   is not in v1. Record the deferral in the contract.
7. **Naming the region:** `center`-style snippets are not used here. The
   popover content is component-owned.

## Prop Surface (fixed — derived from the real authority shapes)

Generic, Longhorn-free, but structurally compatible with
`ForkEntryRecord`/`ForkBranchRecord` so the bridge is a plain mapping.

```ts
type HistoryEntryPosition = "past" | "current" | "future";

interface HistoryEntry {
  id: string;
  label: string;
  position: HistoryEntryPosition;
  checkpoint?: boolean;      // renders as a named pin
  branchCount?: number;      // > 1 marks a fork point, expandable
  groupId?: string | null;
}

interface HistoryBranch {
  id: string;
  name: string | null;       // auto-named by the authority; null → show id
  annotation?: string | null;
  entryCount?: number;
  current?: boolean;
  pinned?: boolean;
}
```

Props: `entries`, `totalEntries`, `hasMoreEntries`, `branches`,
`totalBranches`, `hasMoreBranches`, `canUndo`, `canRedo`, `busy`,
`status` (`"idle" | "loading" | "failed"`), `statusMessage`,
`rejection` (`string | null`), `maxBranchNameBytes` (default 256),
`open` (bindable), `defaultOpen`, `placement`, `undoLabel`, `redoLabel`,
`listLabel`, `title`, `emptyMessage`, `ariaLabel`, `size`, `sizeRole`,
`density`.

Callbacks: `onUndo`, `onRedo`, `onOpenChange`, `onSelectEntry(id)`,
`onCheckout(branchId, entryId)`, `onRenameBranch(branchId, name)`,
`onLoadMoreEntries(offset)`, `onLoadMoreBranches(offset)`.

**No callback may be invoked speculatively.** The host owns what undo does.

## Scope

### In scope

- `packages/core/src/history-center.ts` — headless machine for popover open
  state, list keyboard navigation, branch expansion, and transient rejection
  display. Framework-free, unit-tested (architecture 006).
- Contract `docs/contracts/components/history-center.md`.
- Svelte and React components at parity.
- Trigger cluster: three IconButtons — undo, list, redo. Undo/redo enabled from
  `canUndo`/`canRedo` and disabled while `busy`. Nothing else; titlebar space
  is premium.
- Popover: anchored, dismiss-on-outside, focus-trapped, list keyboard-navigable.
- Paged list — render what is supplied and offer load-more. **Never assume the
  full history is present.**
- Current-position marker; entry click → `onSelectEntry`.
- Fork points: an expandable indicator where `branchCount > 1`; branch rows show
  name and entry count; click → `onCheckout`.
- Inline rename on a branch row → `onRenameBranch`.
- Checkpoint pins.
- `rejection` shown as a transient inline notice — never silent.
- Full recipe-hook coverage (architecture 007).
- Registry, specimen and usage-doc wiring so `svelte:surface-audit` passes:
  contract, component registry, specimen registry, usage docs.
- Specimens: linear, fork, rejection, empty, loading.

### Out of scope — stop conditions if reached

- Any Longhorn import, or any reference to its session/protocol types.
- Native runtimes — that is `020`.
- Virtualisation. Paging only; Tree's virtual scroll is Svelte-only precedent
  and would break the native port.
- Creating checkpoints.
- Consumer repositories, including the Longhorn bridge.
- `poodle-ir`.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- **Read `MessageCenter` first** — Svelte, React, contract, and its specimen.
  Match its shape: data in, callbacks out, `open` bindable with `defaultOpen`.
  Note it has no core machine; HistoryCenter does, so follow an existing core
  machine (`packages/core/src/tabs.ts` or `slider.ts`) for that file's style.
- Svelte and React must be at parity — same props, defaults, semantics.
- Adding a component means all four audit surfaces, not just the component
  file. `svelte:surface-audit` will fail otherwise.
- Do not refresh visual baselines; a new specimen has no baseline, so expect
  no diff and treat any as a stop condition.
- `effigy docs:check` rewrites `packages/tokens/artifacts/rust/*`; restore with
  `git checkout -- packages/tokens/artifacts/rust/` and never commit it.
- Other workers hold Tabs, NavigationMenu, AppHeader and
  `004-shared-control-types.md`. Touch none of them.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-019-history-center-web`. Do not merge.

## Writable Paths

- `packages/core/src/history-center.ts` and its test
- `packages/core/src/styles/history-center.css`
- `packages/core/src/index.ts` (export the machine)
- `packages/svelte/components/src/{HistoryCenter.svelte,index.ts,types.ts}`
- `packages/react/components/src/{HistoryCenter.tsx,index.ts,types.ts}`
- `packages/{svelte,react}/components/test/HistoryCenter.*`
- `docs/contracts/components/history-center.md`
- `docs/contracts/README.md`, `docs/contracts/components/README.md` (index)
- `packages/svelte/preview/src/{component-docs.ts,component-registry.ts,parity.ts}`
- `packages/svelte/preview/src/specimens/{HistoryCenterSpecimen.svelte,registry.ts}`
- `packages/react/preview/src/gallery/{specimen-map.ts,specimens/HistoryCenterSpecimen.tsx}`
- `docs/logs/2026-08/<DD>-g13-019-history-center-web.md`
- `PAPERCUTS.md` (new, non-duplicate friction only)

## Steps

1. Baseline: `bun install`, `effigy test:components`, `effigy test:parity`,
   `effigy svelte:surface-audit`, `effigy docs:lint`, `git diff --check`.
2. Read `MessageCenter` end to end, then a core machine for style.
3. Write the headless machine and its unit tests first — popover state, list
   navigation, branch expansion, transient rejection.
4. Contract, then Svelte, then React.
5. Styles with full recipe-hook coverage.
6. Registry/specimen/usage-doc wiring; confirm `svelte:surface-audit` reports
   no gap for `history-center`.
7. Specimens in both web runtimes, identical labels: linear, fork, rejection,
   empty, loading.
8. Validate:
   ```sh
   effigy test:core
   effigy test:components
   effigy test:parity
   effigy svelte:surface-audit
   effigy docs:lint
   effigy docs:contract-drift
   effigy docs:check
   git checkout -- packages/tokens/artifacts/rust/
   git diff --check
   git status --porcelain
   ```
   `docs:spec-drift` will report `history-center` as having no Spec — that is
   expected until `020`. Record it; do not add a Spec here.

## Acceptance Criteria

- [ ] Headless machine exists in `poodle-core` with unit tests, no framework
  import.
- [ ] Svelte and React at parity, matching the fixed prop surface exactly.
- [ ] Trigger cluster is undo / list / redo, enablement from `canUndo`,
  `canRedo`, `busy`.
- [ ] Popover is anchored, dismiss-on-outside, focus-trapped, keyboard
  navigable.
- [ ] List renders only supplied entries and offers load-more; nothing assumes
  a complete history.
- [ ] Fork points expand; branch rows show name and count; rename is inline and
  emits `onRenameBranch`.
- [ ] Omitting `branches` hides all branch and checkpoint UI.
- [ ] `rejection` renders as a visible transient notice.
- [ ] No Longhorn import anywhere.
- [ ] `svelte:surface-audit` reports zero gaps.
- [ ] All step-8 commands exit 0 except the recorded `docs:spec-drift` gap.
- [ ] Batch log records commands, exit states, and the machine's state model.

## Stop Conditions

- A required behaviour cannot be expressed without importing a Longhorn type.
- The popover needs a capability `AnchoredSurface`/`MenuSurface` does not have.
- Paging cannot be expressed without virtualisation.
- `svelte:surface-audit` cannot pass without touching a file outside the
  writable set.

Stop with exact paths, commands, and the smallest unresolved question.
