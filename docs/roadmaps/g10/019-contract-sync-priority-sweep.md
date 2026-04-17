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

- [x] TextInput: add rows, resize, list to contract
- [x] Tier 2 density batch — menu, context-menu, menubar, navigation-menu (density added); list-card, table (size + sizeRole + density added)
- [x] Accurate gap scan — replaced rough backtick-grep with structured prop-row extraction; confirmed CodeInput/NumberInput/DataTable/LogList/CommandPalette/MarkdownEditor/RelationPicker/PageHeader are all fully documented (earlier "43/124" count was inflated by CSS property names and Svelte 5 interface Props pattern)
- [x] Confirmed 5 real gaps and patched all: icon-button (defaultPressed), media-thumbnail (frameWidth), pagination (chrome + deprecated standalone note), status-bar (chrome/size/sizeRole/density + removed incorrect out-of-scope note), icon-provider (registry → icons rename)
- [x] Tier 2 density/size sweep — full scan run; all remaining components are covered (table-format contracts have explicit rows; "active"-status contracts list them as standard control props bullet)
- [x] PageHeader: audit note added — Svelte 5 interface Props, contract is accurate
- [x] Orphan contract cleanup (8 confirmed orphans): format-display-date and format-file-size noted as utility function contracts; form-shell and tab-strip noted as cross-platform specs without Svelte components; inline-remediation, remediation-banner, state-tile, and validation-summary noted as pending Svelte implementations

## Outcome

All items complete. No undocumented drift remains. Next regular sync can be
re-run with the structured Python scan in this roadmap's notes.
