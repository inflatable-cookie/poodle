<!-- parity consv=ok gpui=1 jetstream=2 specimen=ok -->
<!-- pass 25: LogListSpec gained audit/state fields (additive): loading, error, empty_message,
     filter_text, filters (+LogFilter/LogFilterOption/LogFilterKind), filter_values (BTreeMap),
     page, page_size, total, plus accessors (total_pages/show_pagination/is_loading/
     has_active_filters/has_audit_toolbar/filter_value). 8 spec unit tests pass. GPUI audit
     branch now renders loading (Spinner), error (Callout danger), filter toolbar (from
     filters/filter_values + Clear), and pagination (composed Pagination + range copy);
     dims/colors token-resolved. Jetstream wired the same surfaces (composed js_spinner/
     js_callout/js_pagination) — built clean here (renderer was up). Specimens (gpui+jetstream)
     now cover toolbar+pagination, loading, error. Remaining gpui/jetstream todos are
     interaction-loop bound (live select/date/search editing) not spec-blocked. -->
# Parity: LogList

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/log-list.md`
- Svelte (authoritative): `packages/svelte/components/src/LogList.svelte`
- GPUI: `packages/gpui/components/src/composites/log_list.rs`
- Jetstream: `packages/jetstream/components/src/log_list.rs`
- Spec: `packages/contracts/components/src/log_list.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/LogListSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/log_list_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/log_list.rs`

## Contract ↔ Svelte

Contract and Svelte agree across all props, both variants (stream/audit), types, slots, and ARIA. `consv=ok`.

- All shared props (`entries`, `variant`, `ariaLabel`, `size`, `sizeRole`, `density`), stream props (`maxEntries`, `autoScroll`, `filterLevel`, `filterText`), and the full audit prop set (filters, pagination, callbacks, link builders, `actionIcon`/`entryDetails` snippets, formatters) match `LogList.svelte:25-91`.
- `role="log"` (stream) / labelled `<section>` (audit), `variant="auto"` shape-detection all present (`LogList.svelte:100-101,239,419`).
- No divergence found.

## GPUI gap (vs Svelte + contract)

GPUI collapses stream + audit into one flat entry model (`LogEntry` with optional actor/resource/action) and renders a static toolbar. Major behavioral + token gaps.

- [ ] Hardcoded HSLA color literals: `hsla(0.0, 0.0, 0.5, 0.08)` at `log_list.rs:244` and `:272` (filter/search hover bg) — resolve from a hover token, not a raw HSLA.
- [ ] Hardcoded px literals: `py(px(2.0))` (`:243`,`:271`), `py(px(16.0))` (`:305`), `min_w(px(70.0))` (`:328`), `pl(px(70.0 + 8.0))` (`:356`), `py(px(1.0))` (`:321`), `py(px(4.0))` (`:441`), `gap(px(rem_to_px(0.375)))` (`:341`), `py(px(rem_to_px(0.875)))` (`:407`), `gap(px(rem_to_px(0.3)))` (`:408`) — all magic numbers; resolve from spacing tokens.
- [ ] No stream/audit variant split — single render path; contract §2 requires distinct stream (level chips + text search + scroll-to-latest) vs audit (filter toolbar + states + pagination + links) modes.
- [ ] No level-filter chips with counts — toolbar shows one static "All levels" button (`:228-258`); Svelte renders All/Info/Warn/Error chips with live counts (`LogList.svelte:424-456`).
- [ ] No text-search input — static "Search logs…" button (`:261-280`), not a bound filter input.
- [x] Audit-mode states implemented — loading surface (composed `Spinner`), error surface (composed `Callout` danger), empty surface (uses `spec.empty_message`), filters toolbar (from `spec.filters`/`filter_values` + Clear), and pagination (composed `Pagination` + "Showing X-Y of Z" copy). Driven by the new `LogListSpec` audit fields.
- [ ] No `entryDetails` / `actionIcon` slot equivalents.
- [ ] No actor/resource link activation — href stored but inert (documented `:346-350`); accepted-ish but note.
- [ ] Filter controls are static select/date affordances (display-only); live value editing lives in the preview event loop via `on_filter_change`.
- [ ] `LogLevel` adds a `Debug` variant + `DBG`/`INF`/`WRN`/`ERR` labels (`log_list.rs:87-103`) not in contract `LogLevel = info|warn|error`; either drop Debug or add to contract.
- accepted: no ARIA (`role="log"` not emitted).

## Jetstream gap (vs Svelte + contract)

- [x] **MOCKUP VIOLATION resolved**: fabricated entry loop removed. Stream mode now surfaces a real entry count (or "No log entries") instead of fake `format!("Log message {}")` lines — the spec still carries no entry payload, so a real count beats fake messages per CLAUDE.md.
- [x] Audit-mode states implemented — loading (composed `js_spinner`), error (composed `js_callout` danger), empty (`spec.empty_message`), filter toolbar (from `spec.filters`/`filter_values` + Clear), pagination (composed `js_pagination` + range copy). Driven by the new `LogListSpec` audit fields. Built clean here.
- [x] `auto_scroll` "New entries" affordance now matches the stream contract label (was "Auto-scrolling enabled").
- [ ] Level chips present but non-counting — chips render info/warn/error but show no per-level counts (spec carries no entry payload to count); Svelte shows counts.
- [ ] No text-search live input (display-only filter affordance; editing would live in preview event loop).
- [ ] No `variant=auto` shape detection — spec has no variant field; audit mode entered when audit-only state present.
- accepted: interaction (filter/search clicks) would live in preview event loop; absent.

## Specimen parity

- Svelte covers: Stream "Log output with filtering" (live add-entry button, level filtering, text search) + "Audit activity list" (filters, filterValues, actor/resource links) (`LogListSpecimen.svelte:101-121`).
- GPUI covers: Log-output-with-filtering, Filtered (errors only) with `with_filter_level("error")`, plus audit-style entries via `with_actor`/`with_resource`/`with_action` (`log_list_specimen.rs`). — missing: real text search, audit states, pagination.
- Jetstream covers: two groups, both `with_entry_count(N)` rendering fabricated entries (`log_list.rs:16-25`). — missing: real entry data, audit mode entirely; specimen demonstrates the mockup, not the contract.

## Notes

- `LogListSpec` now carries the audit/state fields (additive, both targets unblocked): `loading`, `error`, `empty_message`, `filter_text`, `filters` (with `LogFilter`/`LogFilterOption`/`LogFilterKind`), `filter_values` (`BTreeMap`), `page`, `page_size`, `total`, plus accessors. Pure logic is unit-tested in poodle-specs (8 tests).
- Jetstream fabricated-entry mockup removed; both Rust targets now render the audit variant (toolbar/loading/error/empty/pagination) from real spec state, composing the shared `Spinner`/`Callout`/`Pagination` primitives.
- Remaining gap on both targets: no entry payload on the spec (only `entry_count`), so stream entries and per-level chip counts are still surfaced as counts rather than rendered rows. That's the next spec expansion (entry vec), separate from this audit-field pass.
- Live filter/search/select editing remains preview-event-loop bound on both Rust targets (display-only affordances driven by `on_filter_change`).
