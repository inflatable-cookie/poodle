# g16.101 — Tree Item Accessible Name

Status: complete
Date: 2026-09-04
Card: `docs/roadmaps/g16/101-tree-item-accessible-name.md`
Handoff: `docs/handoffs/20260904-170000-g16-101-tree-item-accessible-name.md`
Governing refs: `docs/contracts/components/tree.md` (§6 Accessibility),
`docs/contracts/003-native-accessibility.md`
Branch: `feature/g16-101-tree-item-accessible-name`
Base: `origin/main` at `ce60eb7cbf2b4b53e17df3d55c5b4336a56731e8`

## Outcome

Every Tree row now exposes an explicit accessible name equal to its visible
label, in Svelte and React, so assistive technology and agent snapshots can
address rows by name. The windowed flat-list hierarchy is unchanged; no drag,
selection, or keyboard behavior was touched.

### Changes

1. **Svelte `TreeItem.svelte`:** added `aria-label={node.label}` to the
   `role="treeitem"` element. The rename input keeps its own
   `Rename ${node.label}` label, so the row retains its label as name while
   being renamed with no name collision. Because the attribute sits on the
   treeitem itself, a consumer `row` snippet or node-render override cannot
   blank the name — it always falls back to `node.label`, never an empty
   string.
2. **React `TreeItem.tsx`:** added `aria-label={node.label}` to the
   `role="treeitem"` element (placed after the drag/drop prop spread; the
   drag props inject no `aria-*` attributes, so there is no precedence fight).
3. **Loading placeholders:** `aria-label="Loading…"` on the loading
   `role="treeitem"` row in `Tree.svelte` (`loadingRow` snippet) and
   `Tree.tsx` (`loadingRow` function), matching their visible text.
4. **Contract:** added one line to §6 Accessibility → Semantics in
   `docs/contracts/components/tree.md`: treeitem accessible name is the node
   label, set explicitly (`aria-label`; loading placeholders use their visible
   text), never derived from descendants.
5. **Tests:** added a `Tree accessible names` describe to
   `packages/svelte/components/test/Tree.test.ts` and
   `Tree accessible names (react)` to
   `packages/react/components/test/Tree.test.tsx` (4 tests each): explicit
   name over rendered contents, name retained during rename, name updates
   after a rename commit, and the named loading placeholder.

### Snapshot Child Discovery Note

Per the card's fixed boundary: accessibility-snapshot child discovery remains
the consumer tool's job. The flat windowed row list with `aria-level` is the
contract-approved hierarchy; Poodle intentionally does not add nested DOM or
`aria-owns` to make snapshot tooling infer parent/child edges. Longhorn's own
papercut (`figmatic/PAPERCUTS.md:8`) covers naming its half — finding rows by
name is now possible; expanding rows into children in snapshots is the
consumer's responsibility.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Name is explicit | remove the label text child | `getByRole("treeitem", { name })` still resolves in Svelte and React tests (rename flow removes the label span; exact-match query over expanded descendants fails any content-derived name) |
| Name matches the visible label | rename a node | F2 rename + commit + host rerender: `getByRole("treeitem", { name: "source" })` resolves, old name gone |
| Placeholders are named | loading row | `getByRole("treeitem", { name: "Loading…" })` resolves in both suites |
| axe stays green | run the Svelte a11y sweep | `effigy test:a11y` passes 179/179 with no new violations |

## Validation

- **Focused Svelte Tests:** `bunx vitest run --project svelte-components
  packages/svelte/components/test/Tree.test.ts` — 51/51 passed.
- **Focused React Tests:** `bunx vitest run --project react-components
  packages/react/components/test/Tree.test.tsx` — 51/51 passed.
- **A11y Sweep:** `effigy test:a11y` — 179/179 passed (pre-existing
  TextInput/HistoryCenter compile warnings only; no axe violations).
- **Web CI:** `effigy ci:web` — full board green (first run caught a
  `rerender({ props: … })` type error in the new Svelte test; fixed to
  positional props and re-run clean).
- **Docs Check:** `effigy docs:check` — exit 0.
- **Clean Diff:** `git diff --check origin/main...HEAD` — clean.

## Limits

- No drag, selection, keyboard, or expansion behavior changed.
- No GPUI work (that is a later A1 card under the Nucleus programme).
- Stop conditions did not trigger: no consumer `aria-labelledby` conflict
  exists (the treeitem element is internal to the components), and the rename
  input's `Rename ${node.label}` name never collides with the row label.
- Reserved coordinator paths (`docs/roadmaps/g16/README.md`,
  `docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`)
  untouched; root `PAPERCUTS.md` untouched (nothing to append).
