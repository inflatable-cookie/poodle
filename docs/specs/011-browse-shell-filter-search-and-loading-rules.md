# 011 Browse Shell, Filter, Search, Pagination, And Progressive Loading Rules

Status: active
Updated: 2026-03-11
Depends on: `005-product-composite-composition-and-information-architecture-rules.md`, `009-form-shell-validation-and-action-row-rules.md`, `010-data-table-selection-bulk-action-and-virtualization-rules.md`

## Purpose

Freeze the composition rules for browse-heavy surfaces so filter toolbars,
query entry, pagination, and progressive loading all share one documented
posture.

## Search And Filter Composition Rule

Search and filter controls should be grouped ahead of browse results through a
single visible control region.

This region must:

- expose a stable label
- keep control order logical
- present result summary text when helpful
- avoid collapsing status into icon-only affordances

## Empty Versus No-Results Rule

Browse shells must distinguish:

- `empty`: the underlying collection has no content yet
- `no-results`: the active query/filter scope matched nothing

These are not interchangeable visual synonyms.
They describe different user situations and must stay distinguishable in both
runtimes.

## Browse Shell Neutrality Rule

Browse shells own browse framing, not the underlying domain model.

Hosts still own:

- query execution
- filter meaning
- fetch policy
- pagination policy
- progressive-loading policy
- remediation actions

## Pagination Versus Progressive Loading Rule

Both pagination and progressive loading are valid browse postures.

Use pagination when:

- explicit range awareness matters
- stable page positions matter
- the result set needs deterministic page-to-page movement

Use progressive loading when:

- contextual continuity matters more than exact page numbers
- appending results beneath the current context is the better experience
- hosts can load more without breaking focus continuity

Flint does not force one policy globally.
It requires the chosen policy to be explicit and coherent.

## Focus Continuity Rule

Changing query, filters, page, or progressive-load window must not strand
keyboard or assistive-technology users.

That means:

- filter controls remain first in focus order
- browse regions remain addressable
- page changes or appended results preserve a reasonable focus target
- state changes do not silently dump focus into non-interactive containers

## Accessibility Rule

Both runtimes must preserve:

- labeled filter/control grouping
- explicit browse-region semantics
- clear empty/no-results distinction
- pagination range and boundary semantics where pagination exists
- load-more affordance semantics where progressive loading exists

Svelte should use native semantics first.
GPUI must recreate equivalent grouping, region, and loading-state meaning in
the native accessibility tree.

## Seed Evidence

- `docs/contracts/composites/filter-toolbar.md`
- `docs/contracts/composites/browse-search-shell.md`
- `packages/svelte/composites/src/FilterToolbar.svelte`
- `packages/svelte/preview/src/App.svelte`

## Next Task

Carry this browse-shell baseline into `g02.004` and beyond so detail, card,
and navigation work composes onto a stable search-and-results posture.
