<!-- parity consv=ok gpui=9 jetstream=8 specimen=gap -->
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
- [ ] No audit-mode states — no loading/error/empty status surface, no filters toolbar, no refresh/export, no pagination (contract §3 audit props entirely unimplemented).
- [ ] No `entryDetails` / `actionIcon` slot equivalents.
- [ ] No actor/resource link activation — href stored but inert (documented `:346-350`); accepted-ish but note.
- [ ] `LogLevel` adds a `Debug` variant + `DBG`/`INF`/`WRN`/`ERR` labels (`log_list.rs:87-103`) not in contract `LogLevel = info|warn|error`; either drop Debug or add to contract.
- accepted: no ARIA (`role="log"` not emitted).

## Jetstream gap (vs Svelte + contract)

- [ ] **MOCKUP VIOLATION**: entries are fabricated — `format!("Log message {}", i + 1)` with synthetic timestamps `00:{i/60}:{i%60}` and round-robin levels (`log_list.rs:78-104`). No real entry data flows through `LogListSpec` (spec only carries `entry_count`). This is a placeholder per CLAUDE.md "No Mockups" — worse than unimplemented.
- [ ] Hardcoded px literals: `code_font_size = rem_to_px(0.8125)` (`:24`), `label_font = ...size - 0.0625` (`:14`), and pervasive `rem_to_px(0.5)` gaps (`:41,43,91`) — resolve from typography/space tokens.
- [ ] No text-search input (contract stream mode).
- [ ] Level chips present but non-counting — chips render info/warn/error (`:45-60`) but show no per-level counts; Svelte shows counts.
- [ ] No audit-mode states (loading/error/empty), filters toolbar, refresh/export, pagination, link builders, slots — entire audit variant absent.
- [ ] `auto_scroll` renders a static "Auto-scrolling enabled" label (`:110-115`) instead of actual scroll-to-latest behavior + "New entries" affordance.
- [ ] No `variant=auto` shape detection — spec has no variant field.
- accepted: interaction (filter/search clicks) would live in preview event loop; absent.

## Specimen parity

- Svelte covers: Stream "Log output with filtering" (live add-entry button, level filtering, text search) + "Audit activity list" (filters, filterValues, actor/resource links) (`LogListSpecimen.svelte:101-121`).
- GPUI covers: Log-output-with-filtering, Filtered (errors only) with `with_filter_level("error")`, plus audit-style entries via `with_actor`/`with_resource`/`with_action` (`log_list_specimen.rs`). — missing: real text search, audit states, pagination.
- Jetstream covers: two groups, both `with_entry_count(N)` rendering fabricated entries (`log_list.rs:16-25`). — missing: real entry data, audit mode entirely; specimen demonstrates the mockup, not the contract.

## Notes

- The Jetstream specimen + impl are the headline problem: fake entry rendering masquerading as a working component. Either feed real entries through an expanded `LogListSpec` or leave the specimen unimplemented (per CLAUDE.md, a fake specimen is worse than none).
- Both Rust targets implement only a partial stream mode; the entire audit variant (the larger half of the contract) is unimplemented in both.
- `LogListSpec` carries no entry payload, no audit fields, and no variant flag — the spec is the upstream blocker for both targets.
