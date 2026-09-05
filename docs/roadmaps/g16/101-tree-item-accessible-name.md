# g16.101 — Tree Item Accessible Name

Status: complete — merged in PR #205 at `1d8e6aeab`
Type: accessibility defect repair — Svelte and React, contract note
Opened: 2026-09-04
Depends on: none
Governing refs: `../../contracts/components/tree.md` (accessibility section),
`../../contracts/003-native-accessibility.md`,
`packages/svelte/components/src/tree-item/TreeItem.svelte`,
`packages/react/components/src/tree-item/TreeItem.tsx`
Consumer evidence: `figmatic/PAPERCUTS.md:8` — Longhorn's accessibility
snapshot returns `role=treeitem` nodes with no name, so rows cannot be found
by label; live tree proofs map sibling indexes instead
Operator decision 2026-09-04: compile as a ready card
Dispatch manifest: `../dispatch.md`

## Goal

Every Tree row exposes an explicit accessible name equal to its visible
label, in Svelte and React, so assistive technology and agent snapshots can
address rows by name. Hierarchy stays as the contract defines it: a windowed
flat list with `aria-level`.

## Fixed Boundary

- Set `aria-label={node.label}` on each `role="treeitem"` element in
  `TreeItem.svelte` and `TreeItem.tsx`, and on the loading and placeholder
  rows (`Tree.svelte:669`, `Tree.tsx:513`) with their visible text. While a
  row is being renamed, the treeitem keeps its label as name; the rename
  input already carries `Rename …`.
- If a consumer passes a `label` snippet or node-render override, the name
  falls back to `node.label`; never to an empty string.
- Contract: add one line to the accessibility section: treeitem accessible
  name is the node label, set explicitly, not derived from descendants.
- Do not restructure rows into nested DOM or add `aria-owns`; the windowed
  flat list is contract-approved. Record in the execution log that child
  discovery in snapshots is the consumer tool's job (Longhorn's own papercut
  names its half).
- Do not touch drag, selection, or keyboard behaviour.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Name is explicit | remove the label text child | `getByRole("treeitem", { name })` still resolves in Svelte and React tests |
| Name matches the visible label | rename a node | name updates with the label |
| Placeholders are named | loading row | accessible name is the visible loading text |
| axe stays green | run the Svelte a11y sweep | no new violations |

## Validation

Focused Svelte and React Tree tests with `getByRole` name queries, `effigy
test:a11y`, `effigy ci:web`, `effigy docs:check`, `git diff --check
origin/main...HEAD`.

## Owned Paths

`packages/svelte/components/src/tree-item/TreeItem.svelte`,
`packages/svelte/components/src/Tree.svelte` (placeholder rows only),
`packages/react/components/src/tree-item/TreeItem.tsx`,
`packages/react/components/src/Tree.tsx` (placeholder rows only), their
tests, `docs/contracts/components/tree.md` (one accessibility line),
execution log under `docs/logs/2026-09/`, root `PAPERCUTS.md` (append only).

Reserved for the coordinator at merge: `docs/roadmaps/g16/README.md`,
`docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`.

## Stop Conditions

Stop if the explicit name conflicts with an existing `aria-labelledby`
pattern in a consumer, or if axe reports a name collision with the rename
input. Escalation owner: Chatterbox.

## Continuation

The same name rule is the A1 accessibility claim for Tree in the Nucleus
programme; the GPUI side is a later A1 card, not this one.
