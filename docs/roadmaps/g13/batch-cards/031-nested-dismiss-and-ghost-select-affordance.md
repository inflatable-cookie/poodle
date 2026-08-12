# 031 Nested Dismiss Layers, And The Ghost Select Affordance

Status: merged (`4daf6993` → `ef12a3bf`)
Milestone: side-quest (shared primitives, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-031-nested-dismiss-and-select-affordance`
Depends on: `g13-b030` merged (`c6590823`)
Governing refs: `packages/core/src/dom/dismiss.ts`,
`docs/contracts/002-anchored-overlays.md`,
`docs/contracts/components/select.md`

## Goal

Two defects reported from a live HistoryCentre screenshot. Both live in shared
primitives, not in HistoryCentre, and both affect every consumer of those
primitives.

1. Clicking an option in a `Select` that sits inside a `Popover` **dismisses
   the Popover**.
2. A `variant="ghost"` `Select` renders **no dropdown affordance at all**.

## Defect 1 — A nested layer dismisses its own host

### What happens

`Select` portals its listbox to `body`. `Popover.contains` asks
`layerContains(target, rootElement, surfaceElement)` — its trigger and its own
portalled surface. The Select's listbox is in neither. So a click on a Select
option is "outside" the Popover, and the Popover dismisses.

### Why it started

This is a regression from the peer-dismissal change. `resolveDismiss` now
returns **every** layer that does not contain the target:

```ts
return layers.slice().reverse().filter(
  (layer) => layer.dismissOnOutsideInteract && !(target !== null && layer.contains(target)),
);
```

Before that change only the innermost layer dismissed, so the Popover survived
by accident. The peer behaviour is correct and wanted — one click closing N
open peers is what it was for. What it lacks is the difference between a
**peer** and a **host**.

### The ruling — layers record their parent

A layer stack is not flat. It is a tree, and the missing fact is ancestry.

`registerDismissLayer` records, at registration, the layer that was on top of
the stack. That is the layer inside which this one opened — its parent. A
portalled surface does not change that; registration order does.

`resolveDismiss` then spares, on outside interaction:

- any layer that contains the target, as today, and
- **any ancestor of a layer that contains the target.**

Everything else that does not contain the target still dismisses, so the peer
behaviour is unchanged.

**Do not solve this by widening `Popover.contains`.** Making a host reach into
whatever a child portalled would invert the ownership — the host would have to
know every component that might open inside it. Ancestry belongs to the layer
stack, which already sees every registration.

### Tests that must keep passing, unchanged

`packages/core/test/dismiss.test.ts` already pins the peer rules. These are the
ones that make a naive fix fail, so run them first and do not edit them:

- `outside interaction dismisses every peer it fell outside of`
- `a layer containing the target is spared while its peers dismiss`
- `a pinned layer is spared while its peers dismiss`
- `escape still unwinds one layer at a time when overlays nest`

### New tests

- A child layer's portalled surface is clicked: the child is spared **and its
  ancestor is spared**, while a peer of the ancestor dismisses.
- Ancestry survives portalling — the parent is the layer on top at
  registration, not a DOM ancestor.
- Three levels deep: clicking in the innermost spares all three.
- A true outside click still dismisses the whole chain in one interaction.

## Defect 2 — Ghost Select hides its own chevron

### What happens

`Select.svelte` gates the indicator on `{#if variant !== "ghost"}`. A ghost
Select therefore has no border and no chevron, so nothing says it opens a list.
In the screenshot the only glyph on that row is the disabled checkout button,
which reads as a status tick rather than a control.

### This is a `Select` defect, not a HistoryCentre one

Two components already work around it, independently, with a hardcoded
character rather than the icon system:

- `OrderBy.svelte:318` — `<span class="poodle-order-by__chevron">▾</span>`
- `FilterBuilder.svelte:346` — `<span class="poodle-filter-builder__chevron">▾</span>`

When two components reimplement the same missing affordance, the affordance is
missing from the component.

### The ruling — ghost keeps the chevron

Remove the `variant !== "ghost"` gate. Ghost drops the border and the fill; it
does not drop the signal that the control is a select. Then delete the two
local workarounds, which now double up.

Expect a visual change in every ghost `Select`. The in-repo users are
`FilterBuilder`, `LogList`, `BlockEditor`, `OrderBy`, `HistoryCenter`,
`RelationPicker`, `DataTable`. Enumerate the visual diffs and classify them;
refresh no baseline.

`DataTable` draws no workaround chevron today, so it gains an affordance it was
missing. That is the fix working, not a regression.

## Scope

### In scope

- `packages/core/src/dom/dismiss.ts` — parent recording and the ancestor rule.
- `packages/core/test/dismiss.test.ts` — new tests only; do not edit the four
  named above.
- `packages/svelte/components/src/Select.svelte` and its React counterpart —
  the chevron gate.
- `OrderBy` and `FilterBuilder` in both runtimes — remove the workaround
  chevrons and their CSS.
- `docs/contracts/002-anchored-overlays.md` — the ancestry rule.
- `docs/contracts/components/select.md` — ghost keeps the indicator.

### Out of scope — stop conditions if reached

- HistoryCentre. Both defects are in the primitives; if HistoryCentre needs a
  change to benefit, something is wrong with the fix.
- The focus-trap escape recorded in `PAPERCUTS.md` — the portalled listbox
  escaping `Popover`'s Tab trap. Related, but a keyboard concern with its own
  shape. Leave it; the dismiss fix may make it easier and the papercut says so.
- `dismissOnOutsideInteract` defaults. `b026` settled those.
- Refreshing visual baselines.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read `dismiss.ts` and its full test file before changing anything. The four
  named tests are the specification for the peer behaviour.
- Svelte first, then React mirrors exactly.
- **Run `effigy check:svelte`.** Not optional.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit the two defects **separately** so either can be reverted alone.
- Commit and push with
  `git push -u origin thread/g13-031-nested-dismiss-and-select-affordance`. Do
  not merge.

## Writable Paths

- `packages/core/src/dom/dismiss.ts`
- `packages/core/test/dismiss.test.ts`
- `packages/{svelte,react}/components/src/{Select,OrderBy,FilterBuilder}.*`
- Their tests
- `packages/core/src/styles/{select,order-by,filter-builder}.css`
- `docs/contracts/002-anchored-overlays.md`
- `docs/contracts/components/{select,order-by,filter-builder}.md`
- `docs/logs/2026-08/<DD>-g13-031-nested-dismiss-and-select-affordance.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy test:core`, `test:components`, `check:svelte`,
   `docs:lint`, `git diff --check`. Record exit states. All start green.
2. Read `dismiss.ts` and its tests.
3. Defect 1: parent recording, the ancestor rule, new tests. Commit.
4. Defect 2: the chevron gate, then delete both workarounds. Commit.
5. Visual enumeration in report mode; classify. Refresh nothing.
6. Validate:
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

- [ ] Clicking a Select option inside a Popover leaves the Popover open, proven
  by test at the `dismiss.ts` level.
- [ ] All four named peer tests pass unedited.
- [ ] Ancestry is recorded at registration and survives portalling.
- [ ] Ghost Select renders the chevron; both workaround chevrons and their CSS
  are gone.
- [ ] Contracts record the ancestry rule and the ghost indicator.
- [ ] The two defects are separate commits.
- [ ] All step-6 commands exit 0; no baseline refreshed.

## Stop Conditions

- Ancestry cannot be recorded without a registration-order assumption that
  portalling breaks. Say which case.
- A named peer test cannot pass alongside the ancestor rule. Give the two
  requirements that conflict.
- A ghost Select consumer depends on the absent chevron for layout.

Stop with exact paths, commands, and the smallest unresolved question.
