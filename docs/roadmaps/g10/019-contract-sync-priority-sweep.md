# g10.019 Contract Sync Priority Sweep

Status: queued
Owner: Poodle core
Depends on: g10.017
Updated: 2026-04-17

## Purpose

Audit found 43 of 124 Svelte components (35%) have props or states present in
the Svelte implementation that are absent from their contract files in
`docs/contracts/components/`. Contracts are the source of truth for all
implementations — undocumented Svelte props create silent drift and make it
impossible to audit or implement other targets correctly.

This milestone works through the backlog in priority order.

---

## Tier 1 — High-impact feature gaps

These are non-trivial features that cross-platform implementations need to know
about.

**TextInput** (`docs/contracts/components/text-input.md`)
- Add `rows` (number | null) — controls textarea row height; enables multiline mode
- Add `resize` ("none" | "vertical" | "both") — textarea resize handle behaviour
- Add `list` (string | null) — links to `<datalist>` for suggestion lists
- Contract already mentions multiline mode but does not document the props that
  control it

**CodeInput** (`docs/contracts/components/code-input.md`)
- Contract is minimal; Svelte exposes 16 configurable props
- Audit the Svelte implementation in full and document all non-internal props

**NumberInput** (`docs/contracts/components/number-input.md`)
- Svelte has 24 props not in contract; needs a full audit pass

**DataTable** (`docs/contracts/components/data-table.md`)
- Svelte has 26 undocumented props; a significant configuration surface

**LogList** (`docs/contracts/components/log-list.md`)
- 23 undocumented Svelte props

**CommandPalette** (`docs/contracts/components/command-palette.md`)
- 11 undocumented Svelte props

**MarkdownEditor** (`docs/contracts/components/markdown-editor.md`)
- 11 undocumented Svelte props

**RelationPicker** (`docs/contracts/components/relation-picker.md`)
- 14 undocumented Svelte props

---

## Tier 2 — Standardised axis props not yet documented

`density`, `size`, and `sizeRole` were added as standardised props to ~35
components. Contracts should document them consistently. Affected components
include: ContextMenu, Menu, Menubar, NavigationMenu, and others identified in
the audit.

Approach: add a standard "Size and Density" section to each contract's Props
table where these are present in Svelte but absent from the contract.

---

## Tier 3 — PageHeader reversal

The PageHeader contract documents 12 props that the Svelte component does not
expose. Investigate whether:
- The Svelte component was refactored and the contract was never updated
- The contract is aspirational (planned but not yet implemented)

Resolve the discrepancy either by updating the Svelte component or updating the
contract to reflect current reality.

---

## Tier 4 — Orphan contract cleanup

11 contracts have no corresponding Svelte component (form-shell,
remediation-banner, and others). Confirm each is either:
- Retired (delete the contract or move to archive)
- Intentionally informational (add a note at the top)

---

## Execution checklist

- [ ] TextInput: add rows, resize, list to contract
- [ ] CodeInput: full contract audit and update
- [ ] NumberInput: full contract audit and update
- [ ] DataTable: full contract audit and update
- [ ] LogList: full contract audit and update
- [ ] CommandPalette: contract audit and update
- [ ] MarkdownEditor: contract audit and update
- [ ] RelationPicker: contract audit and update
- [ ] Tier 2 density/size batch (35 components)
- [ ] PageHeader: investigate and resolve reversal
- [ ] Orphan contract cleanup (11 contracts)

## Next task

Start with Tier 1: TextInput contract (smallest, highest cross-platform impact),
then CodeInput/NumberInput in a batch.
