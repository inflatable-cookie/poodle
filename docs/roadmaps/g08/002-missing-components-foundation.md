# g08.002 Implement Missing Components Batch 1 (Foundation Primitives)

Status: complete
Owner: Poodle Core
Depends on: g08.001

## Contract Check

Before implementing each component, read its contract end to end. Cross-reference
the Svelte implementation for visual reference. Verify the contract hasn't been
renamed or restructured since this milestone was written.

## Goals

16 foundation contracts have Svelte implementations but no GPUI component at all.
This milestone implements them from scratch to production quality — full token
resolution, focus rings, and ARIA from the start.

## Components (16)

### alert-dialog

- [ ] Read contract: `docs/contracts/components/alert-dialog.md`
- [ ] Read Svelte: `packages/svelte/primitives/src/AlertDialog.svelte`
- [ ] Create spec struct in `poodle-gpui-primitives`
- [ ] Implement component in `poodle-gpui-components`
- [ ] Write specimen in `poodle-gpui-preview`
- [ ] Pass 10-point quality checklist

### breadcrumbs

- [ ] Read contract: `docs/contracts/components/breadcrumbs.md`
- [ ] Read Svelte: `packages/svelte/primitives/src/Breadcrumbs.svelte`
- [ ] Create spec struct, implement component, write specimen
- [ ] Pass 10-point quality checklist

### bulk-action-bar

- [ ] Read contract: `docs/contracts/components/bulk-action-bar.md`
- [ ] Read Svelte: `packages/svelte/primitives/src/BulkActionBar.svelte`
- [ ] Create spec struct, implement component, write specimen
- [ ] Pass 10-point quality checklist

### card

- [ ] Read contract: `docs/contracts/components/card.md`
- [ ] Read Svelte: `packages/svelte/primitives/src/Card.svelte`
- [ ] Create spec struct, implement component, write specimen
- [ ] Pass 10-point quality checklist

### collapse-toggle

- [ ] Read contract: `docs/contracts/components/collapse-toggle.md`
- [ ] Read Svelte: `packages/svelte/primitives/src/CollapseToggle.svelte`
- [ ] Create spec struct, implement component, write specimen
- [ ] Pass 10-point quality checklist

### combobox

- [ ] Read contract: `docs/contracts/components/combobox.md`
- [ ] Read Svelte: `packages/svelte/primitives/src/Combobox.svelte`
- [ ] Create spec struct, implement component, write specimen
- [ ] Pass 10-point quality checklist

### detail-item

- [ ] Read contract: `docs/contracts/components/detail-item.md`
- [ ] Read Svelte: `packages/svelte/primitives/src/DetailItem.svelte`
- [ ] Create spec struct, implement component, write specimen
- [ ] Pass 10-point quality checklist

### list-card

- [ ] Read contract: `docs/contracts/components/list-card.md`
- [ ] Read Svelte: `packages/svelte/primitives/src/ListCard.svelte`
- [ ] Create spec struct, implement component, write specimen
- [ ] Pass 10-point quality checklist

### nav-card

- [ ] Read contract: `docs/contracts/components/nav-card.md`
- [ ] Read Svelte: `packages/svelte/primitives/src/NavCard.svelte`
- [ ] Create spec struct, implement component, write specimen
- [ ] Pass 10-point quality checklist

### nav-card-grid

- [ ] Read contract: `docs/contracts/components/nav-card-grid.md`
- [ ] Read Svelte: `packages/svelte/primitives/src/NavCardGrid.svelte`
- [ ] Create spec struct, implement component, write specimen
- [ ] Pass 10-point quality checklist

### order-by

- [ ] Read contract: `docs/contracts/components/order-by.md`
- [ ] Read Svelte: `packages/svelte/primitives/src/OrderBy.svelte`
- [ ] Create spec struct, implement component, write specimen
- [ ] Pass 10-point quality checklist

### pagination

- [ ] Read contract: `docs/contracts/components/pagination.md`
- [ ] Read Svelte: `packages/svelte/primitives/src/Pagination.svelte`
- [ ] Create spec struct, implement component, write specimen
- [ ] Pass 10-point quality checklist

### region

- [ ] Read contract: `docs/contracts/components/region.md`
- [ ] Read Svelte: `packages/svelte/primitives/src/Region.svelte`
- [ ] Create spec struct, implement component, write specimen
- [ ] Pass 10-point quality checklist

### resize-handle

- [ ] Read contract: `docs/contracts/components/resize-handle.md`
- [ ] Read Svelte: `packages/svelte/workstation/src/ResizeHandle.svelte`
- [ ] Create spec struct, implement component, write specimen
- [ ] Pass 10-point quality checklist

### status-bar

- [ ] Read contract: `docs/contracts/components/status-bar.md`
- [ ] Read Svelte: `packages/svelte/workstation/src/StatusBar.svelte`
- [ ] Create spec struct, implement component, write specimen
- [ ] Pass 10-point quality checklist

### table

- [ ] Read contract: `docs/contracts/components/table.md`
- [ ] Read Svelte: `packages/svelte/primitives/src/Table.svelte`
- [ ] Create spec struct, implement component, write specimen
- [ ] Pass 10-point quality checklist

## Acceptance Criteria

- [ ] All 16 components implemented with full token resolution
- [ ] All 16 pass the 10-point quality checklist (see README)
- [ ] All 16 have specimen pages in the preview app
- [ ] Zero hardcoded px values in any new component
- [ ] Clean compile with `cargo check -p poodle-gpui-preview`
