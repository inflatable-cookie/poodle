# 012 Detail Display, Card, Header, And Navigation Rules

Status: active
Updated: 2026-03-11
Depends on: `005-product-composite-composition-and-information-architecture-rules.md`, `011-browse-shell-filter-search-and-loading-rules.md`

## Purpose

Freeze the baseline composition rules for product-style detail surfaces so
headers, breadcrumbs, summary cards, and readonly detail groups all share one
documented information architecture.

## Local Identity Rule

Every detail surface needs one local identity region.

That region should:

- expose a clear primary heading
- allow subordinate eyebrow or subtitle copy
- optionally host local actions
- optionally host breadcrumb context above the title block

`PageHeader` owns this local identity.
It does not become app-shell chrome.

## Breadcrumb Rule

Breadcrumbs provide hierarchical location context, not tab navigation or shell
navigation.

They must:

- preserve one clear current location
- remain ordered ahead of the local page title
- stay meaningful when truncated

## Summary Versus Detail Rule

Detail surfaces usually need both:

- summary surfaces such as cards
- structured detail groups such as detail sections and detail rows

These roles are not interchangeable.

Use cards when:

- grouped status or highlight information benefits from stronger visual emphasis
- the content works as compact summary blocks

Use detail rows when:

- label/value relationships are primary
- readonly metadata needs stable scanning structure

## Section Hierarchy Rule

`DetailShell` owns page- or scope-level framing.
`DetailSection` owns related subgrouping within the body.
`DetailItem` owns individual readonly label/value relationships.

Hosts should not flatten all three levels into one unstructured card wall.

## State Rule

`DetailShell` may expose:

- `ready`
- `empty`
- `loading`
- `error`

Unlike browse shells, detail shells do not use `no-results` as a baseline
state. A detail destination either exists, is empty, is loading, or is in
error.

## Accessibility Rule

Both runtimes must preserve:

- heading hierarchy
- breadcrumb current-location semantics
- local action grouping near the header
- readable label/value relationships in detail rows
- sensible focus continuity when detail content swaps between ready, loading,
  empty, and error posture

Svelte should use semantic headings, navigation, and description-list patterns
where practical.
GPUI must explicitly recreate equivalent structure in the native accessibility
tree.

## Seed Evidence

- `docs/contracts/components/card.md`
- `docs/contracts/components/page-header.md`
- `docs/contracts/components/breadcrumbs.md`
- `docs/contracts/components/detail-shell.md`
- `docs/contracts/components/detail-section.md`
- `docs/contracts/components/detail-item.md`
- `packages/svelte/composites/src/Card.svelte`
- `packages/svelte/composites/src/PageHeader.svelte`
- `packages/svelte/composites/src/Breadcrumbs.svelte`
- `packages/svelte/composites/src/DetailShell.svelte`
- `packages/svelte/composites/src/DetailSection.svelte`
- `packages/svelte/composites/src/DetailItem.svelte`
- `packages/svelte/preview/src/App.svelte`

## Next Task

Carry this detail-display baseline into `g02.005` and later milestones so
pickers, relation flows, and media/detail surfaces compose onto stable local
identity and readonly-detail rules.
