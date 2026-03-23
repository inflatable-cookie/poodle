# g08.003 Implement Missing Components Batch 2 (Composites + Remaining)

Status: complete
Owner: Flint Core
Depends on: g08.001

## Contract Check

Before implementing each component, read its contract end to end. Cross-reference
the Svelte implementation for visual reference. These composites may depend on
foundation components from 002 — verify those are available first.

## Goals

4 composite contracts have Svelte implementations but no GPUI component. This
milestone implements them from scratch to production quality.

## Components (4)

### detail-section

- [ ] Read contract: `docs/contracts/composites/detail-section.md`
- [ ] Read Svelte implementation
- [ ] Create spec struct in `flint-gpui-composites`
- [ ] Implement component in `flint-gpui-components`
- [ ] Write specimen in `flint-gpui-preview`
- [ ] Pass 10-point quality checklist

### metric-tile

- [ ] Read contract: `docs/contracts/composites/metric-tile.md`
- [ ] Read Svelte implementation
- [ ] Create spec struct, implement component, write specimen
- [ ] Pass 10-point quality checklist

### page-header

- [ ] Read contract: `docs/contracts/composites/page-header.md`
- [ ] Read Svelte implementation
- [ ] Create spec struct, implement component, write specimen
- [ ] Pass 10-point quality checklist

### toast-stack

- [ ] Read contract: `docs/contracts/composites/toast-stack.md`
- [ ] Read Svelte implementation
- [ ] Create spec struct, implement component, write specimen
- [ ] Pass 10-point quality checklist

## Acceptance Criteria

- [ ] All 4 components implemented with full token resolution
- [ ] All 4 pass the 10-point quality checklist (see README)
- [ ] All 4 have specimen pages in the preview app
- [ ] Zero hardcoded px values in any new component
- [ ] Clean compile with `cargo check -p flint-gpui-preview`
