<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok -->
<!-- pass 41: both Rust targets now compose real PageHeader/Callout/EmptyState/Pagination/PaginationSummary; filters+batch slots added; root gap → stack.lg; Jetstream typography resolves from tokens; specimens cover ready+filters/batch+loading+error+empty; Jetstream probe tests added. -->
<!-- header-slots pass: Jetstream gained `js_list_container_with_slots(..., breadcrumbs, actions)` forwarding both into the composed PageHeader; `js_list_container(spec, theme, content, filters, batch)` delegates with empty header slots (5-arg callers unchanged). GPUI already had `with_breadcrumbs`/`with_actions`. Both header host slots now match the contract §2 anatomy across targets. Jetstream specimen gains a "Breadcrumbs and actions" group (real breadcrumb trail + primary js_button); 1 new render_probe test. -->

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

- [x] FIXED Pagination alignment: contract §8 "Pagination Composition" now `.pagination { justify-self: end }`, matching Svelte (`ListContainer.svelte:212`).
- Class prefix: contract anatomy uses bare `.list-container*`; Svelte emits `.poodle-list-container*`. Cosmetic namespacing, contract uses the unprefixed convention throughout — accepted (noted), not a contract fix.
- Everything else (24 props, all snippets, all 4 states, derived `shouldShowPagination*`, `aria-label ?? title`, `data-state`, error Callout `announceMode="assertive"`) matches contract exactly.

## GPUI gap (vs Svelte + contract)

GPUI now composes the real PageHeader / Callout / EmptyState / Pagination / PaginationSummary components. Slot builders cover filters/batch/breadcrumbs/actions (the latter two forwarded into the composed PageHeader); page-change is wired.

- [x] FIXED No Filters / Batch regions — added `with_filters` / `with_batch` builders rendering `stack.md` regions between header and content (Ready state only). No spec field needed — slots are component-level `AnyElement`.
- [x] FIXED No real Pagination control — now composes `Pagination::from_spec` (current/total/sibling/aria) + `PaginationSummary::from_spec`; pagination region justifies controls to the end (contract §8 `justify-self: end`).
- [x] FIXED No `onPageChange` callback — added `on_page_change` builder forwarding to the composed `Pagination`.
- [x] FIXED Header hand-built — now delegates to `PageHeader::from_spec` with `with_breadcrumbs` / `with_actions` slot forwarding.
- [x] FIXED Loading/error/empty plain-text blocks — now `Callout(pending)` / `Callout(danger, announceMode=Assertive)` / `EmptyState`, resolving message/title/announce semantics from the composed specs.
- accepted: no ARIA (gpui has no accessibility API) — `aria-label`/`data-state`/`role` not emitted; the composed Callout carries `announce_mode` for cross-target parity.
- note: `emptyVariant` is not on `ListContainerSpec` (host-driven via the EmptyState slot in Svelte); GPUI uses EmptyState's default neutral variant. Not a spec gap — matches contract prop ownership.

## Jetstream gap (vs Svelte + contract)

Jetstream now mirrors GPUI: composes the real PageHeader / Callout / EmptyState / Pagination / PaginationSummary functions; typography + spacing resolve from tokens.

- [x] FIXED Hardcoded px typography (`rem_to_px(1.125/0.8125/0.6875)`) — header now delegated to `js_page_header_with_slots`, which resolves heading/subtitle/eyebrow sizes from tokens. (The old constants were also wrong: heading token = 1rem, body = 0.875rem.)
- [x] FIXED Root gap `space.stack.md` → `space.stack.lg` (contract §8); regions use `space.stack.md`.
- [x] FIXED No Filters / Batch regions — `js_list_container` gained `filters` / `batch` params rendering `stack.md` regions (Ready state).
- [x] FIXED No real Pagination — now `js_pagination` (current/total/sibling/aria) + `js_pagination_summary` ("Showing N – M of T") when totals known; gated by `show_pagination && total_pages > 1`.
- [x] FIXED Header hand-built — delegated to `js_page_header_with_slots`.
- [x] FIXED No breadcrumbs / actions header slots — `js_list_container_with_slots(..., breadcrumbs, actions)` now forwards both into the composed PageHeader (contract §2). `js_list_container` delegates with empty header slots (5-arg back-compat). Mirrors GPUI's `with_breadcrumbs`/`with_actions`.
- [x] FIXED Loading/error/empty hand-built — now `js_callout(pending)` / `js_callout(danger, Assertive)` / `js_empty_state`; the raw `pt/pb 2.0rem` empty literals are gone.
- preview-loop: page-change interaction lives in the preview event loop; `js_pagination` renders controls but emits no callback here.

## Specimen parity

- Svelte covers: Ready (full shell: header, batch slot, list cards, summary + pagination), plus interactive state toggle (loading/error/empty) on a second instance.
- GPUI covers: Ready-with-content + built-in pagination (real Pagination + summary), Loading, Error, Empty (`list_container_specimen.rs`). Slot builders (filters/batch/breadcrumbs/actions) available; specimen can add them without impl changes.
- Jetstream covers: Ready-with-pagination, Filters+batch, Breadcrumbs+actions (real breadcrumb trail + primary `js_button` forwarded into the PageHeader), Empty, Loading, Error (`list_container.rs` specimen). Pagination summary + controls render via composed primitives.

## Notes

- `consv=fixed`: the only Contract↔Svelte divergence was the pagination `justify-self` value (now `end` to match Svelte). The Rust-side composition work (filters/batch/breadcrumbs/actions slots + pagination wiring) is complete on both targets.
- Slot surfaces are intentionally NOT `ListContainerSpec` fields — filters/batch/breadcrumbs/actions/content are host-composed element content (`JsEl` / `AnyElement`) passed through `js_list_container_with_slots` (Jetstream) / `with_filters`/`with_batch`/`with_breadcrumbs`/`with_actions` (GPUI). Both targets now expose every contract §2 slot region; no spec expansion was needed. Remaining residual is `onPageChange` interaction in Jetstream (preview event loop) — a runtime-loop limit, not a slot gap.
