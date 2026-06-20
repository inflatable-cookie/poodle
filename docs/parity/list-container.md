<!-- parity consv=gap gpui=6 jetstream=6 specimen=gap -->
# Parity: ListContainer

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/list-container.md`
- Svelte (authoritative): `packages/svelte/components/src/ListContainer.svelte`
- GPUI: `packages/gpui/components/src/composites/list_container.rs`
- Jetstream: `packages/jetstream/components/src/list_container.rs`
- Spec: `packages/contracts/components/src/list_container.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/ListContainerSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/list_container_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/list_container.rs`

## Contract ↔ Svelte

One real divergence, plus a class-name namespacing note.

- **Pagination alignment**: contract §8 "Pagination Composition" says `.pagination { justify-self: start }`; Svelte sets `justify-self: end` (`ListContainer.svelte:212`). Svelte is the reference. **Fix: contract → `end`.**
- Class prefix: contract anatomy uses bare `.list-container*`; Svelte emits `.poodle-list-container*`. Cosmetic namespacing, contract uses the unprefixed convention throughout — accepted, note once.
- Everything else (24 props, all snippets, all 4 states, derived `shouldShowPagination*`, `aria-label ?? title`, `data-state`, error Callout `announceMode="assertive"`) matches contract exactly.

## GPUI gap (vs Svelte + contract)

GPUI hand-rolls header + state text instead of delegating to PageHeader/Callout/EmptyState/Pagination composites, so most contract anatomy is faked rather than composed.

- [ ] Hardcoded px literals throughout: `min_w(px(70.0))` not present here but `pl(px(70.0 + 8.0))` — N/A; actual literals: none in floats except spacing resolves from tokens. Recheck: header/state text sizes all resolve from tokens. No raw px literals. (clean)
- [ ] No Filters / Batch regions — `ListContainerSpec` has no filter/batch slots; contract §2 requires `.list-container__filters` and `.list-container__batch` between header and content. Add slot fields + render (`list_container.rs:134-138` only renders `self.content`).
- [ ] No real Pagination control — only summary text "Page X of Y" + "Showing N–M of T" (`list_container.rs:147-182`); contract requires composed `Pagination` primitive with `onPageChange`. Wire the GPUI pagination primitive; current render is summary-only.
- [ ] No `onPageChange` callback — builder exposes no page-change handler; pagination is non-interactive.
- [ ] Header is hand-built (eyebrow/title/subtitle divs, `list_container.rs:50-79`) instead of delegating to PageHeader composite; breadcrumbs + actions slots absent entirely (contract §2 + props).
- [ ] Loading/error/empty render plain text blocks (`list_container.rs:83-133`) instead of Callout(pending)/Callout(danger, assertive)/EmptyState; `emptyVariant`, `loadingMessage` default, error `announceMode` semantics all dropped.
- accepted: no ARIA (gpui has no accessibility API) — `aria-label`/`data-state`/`role` not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded px literals: `heading_size = rem_to_px(1.125)`, `body_size = rem_to_px(0.8125)`, `eyebrow_size = rem_to_px(0.6875)` (`list_container.rs:21-23`) — resolve from typography tokens (`typography.heading.size`, `typography.body.size`, `typography.label.size`), not raw rem constants.
- [ ] Root gap uses `space.stack.md` (`list_container.rs:18`); contract §8 root gap is `space.stack.lg`. GPUI got this right (`stack_gap = space.stack.lg`). **Fix token.**
- [ ] No Filters / Batch regions (same as GPUI — spec lacks slots).
- [ ] No real Pagination — only centered "Page X of Y" text (`list_container.rs:97-105`); no `Pagination` primitive, no `onPageChange`, no PaginationSummary.
- [ ] Header hand-built (`list_container.rs:27-46`) not delegated to PageHeader; breadcrumbs + actions slots absent.
- [ ] Loading/error/empty hand-built (icon+label / label / centered label, `list_container.rs:52-88`) instead of Callout/EmptyState delegation; `emptyVariant` dropped, empty uses `pt/pb 2.0rem` raw literals.
- accepted: interaction (page change) would live in preview event loop; no pager exists to wire.

## Specimen parity

- Svelte covers: Ready (full shell: header, batch slot, list cards, summary + pagination), plus interactive state toggle (loading/error/empty) on a second instance.
- GPUI covers: Ready-with-pagination-summary, Loading, Error, Empty (`list_container_specimen.rs`). — missing: filters/batch slots, breadcrumbs, real pagination controls (none exist in impl).
- Jetstream covers: only Empty + Loading groups (`list_container.rs:31-44`). — missing: **Ready-with-content**, **Error**, pagination, breadcrumbs/actions. Thinnest of the three.

## Notes

- `consv=gap` driven solely by the pagination `justify-self` value (contract says `start`, Svelte says `end`).
- Root structural gap across both Rust targets: `ListContainerSpec` (`list_container.rs:21-`) has no filter/batch/breadcrumbs/actions slot surface and no pagination wiring, so neither Rust target can reach contract parity without spec expansion. This is the dominant work item — flag spec-level before component-level.
