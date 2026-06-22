<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok | jet-specimen: real js_time_zone_select groups — default, pre-selected, open (searchable+query+selected), disabled, sizes (sm/md/lg); mirrors gpui; both previews build clean -->
<!-- pass: TimeZoneSelectSpec grown to Select-parity (options/search_query/aria/name + shared default_time_zone_options() + to_select_spec()); gpui now renders selectable filtered zones + search input + correct placeholder + on_change/on_search channels; jetstream delegates to js_select (full dropdown/search/grouping/indicator); both probe/build-verified. GPUI specimen done; Jetstream pending engine recovery — specimen flag stays gap until Jetstream preview builds clean. -->
# Parity: TimeZoneSelect

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/time-zone-select.md`
- Svelte (authoritative): `packages/svelte/components/src/TimeZoneSelect.svelte`
- GPUI: `packages/gpui/components/src/primitives/time_zone_select.rs`
- Jetstream: `packages/jetstream/components/src/time_zone_select.rs`
- Spec (shared): `packages/contracts/components/src/time_zone_select.rs`
- Default option source: `packages/svelte/components/src/date.ts` `defaultTimeZoneOptions()` (l.326)
- Specimens: svelte `packages/svelte/preview/src/specimens/TimeZoneSelectSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/time_zone_select.rs` · jetstream `packages/jetstream/preview/src/specimens/time_zone_select.rs`

## Contract ↔ Svelte

TimeZoneSelect is a thin wrapper: Svelte delegates rendering, interaction, searchable mode, and ARIA entirely to `Select` (`TimeZoneSelect.svelte:50-68`). The wrapper's only logic is mapping `TimeZoneOption[]` → `Select` options. Divergences are small but real.

- [x] FIXED `options` default: contract §3 said `defaultTimeZoneOptions()`; Svelte default is `[]` with the fallback applied via `$derived(options.length > 0 ? options : defaultTimeZoneOptions())` (l.42). Contract §3 default → `[]`, with the fallback clarified in §3 props + Controlled/Uncontrolled.
- [x] FIXED `emptyMessage`: Svelte hardcodes `emptyMessage="No matching time zones"` (l.64). Added a §9 note documenting the empty-search string (plus the `Intl.supportedValuesOf` source + `_`→space label rule).
- `placeholder` default: contract §3 default `"Search time zones..."`; Svelte matches (l.30). OK.
- All callbacks (`onValueChange`, `onQueryChange`, `onOpenChange`) and a11y props (`ariaLabel`, `describedBy`) forwarded verbatim to `Select` (l.58-67). `searchable` is always-on (l.63), matching contract §7. OK.
- Time-zone data: Svelte `defaultTimeZoneOptions()` prefers `Intl.supportedValuesOf("timeZone")` (full IANA set), falling back to a 9-entry hardcoded list (`date.ts:334-342`). Labels are `value.replaceAll("_", " ")` (`formatTimeZoneLabel`, l.318). No offset computation anywhere — contract §1 lists offset math as out of scope. Consistent.

## GPUI gap (vs Svelte + contract)

GPUI hand-rolls a trigger + dropdown (Known Delta — no `Select` primitive composition), now backed by the grown spec. All real gaps closed:

- [x] ALREADY OK dropdown shadow uses `crate::theme_ext::elevation_overlay_shadow()` (token-resolved `ELEVATION_OVERLAY`); the prior pass's flagged `hsla`/`px` literals were already gone.
- [x] FIXED timezone list now sourced from the shared `poodle_specs::default_time_zone_options()` (contract §10) via `spec.select_options()` — no inline duplication. Host options override when provided.
- [x] FIXED options are selectable — each row has an `on_click` that fires `on_change(value)` then closes; selected row shows a `check` indicator + medium weight.
- [x] FIXED searchable mode — a real `TextInput` search row renders at the top of the open dropdown; rows filter by `spec.search_query` (case-insensitive substring); `TIME_ZONE_EMPTY_MESSAGE` shown on no match.
- [x] FIXED placeholder — `trigger_text()` falls back to `spec.effective_placeholder()` (Svelte default "Search time zones..."), no ad-hoc string.
- [x] FIXED query channel — `on_search_change` wired to the search input; `on_toggle` covers open/close. (`on_change` added for value-pick.)
- accepted: no ARIA combobox/listbox/option (gpui has no accessibility API).
- accepted: overlay timing/positioning runtime-owned (contract §11 Tier 3).
- preview-loop: open/search/select state held by the host via the three channels.

## Jetstream gap (vs Svelte + contract)

`js_time_zone_select` now **delegates entirely to `js_select`** — exactly as the Svelte wrapper delegates to `Select` — by mapping the spec through `spec.to_select_spec()` (searchable always on, timezone empty message, mapped options, placeholder/value/size/density forwarded). Every gap below is inherited-correct from the now-fixed `Select`:

- [x] FIXED open/dropdown rendering — `is_open` is forwarded; the open dropdown, listbox, option rows, and overlay come from `js_select`. Contract §4 `open` state satisfied.
- [x] FIXED searchable mode — always on via `to_select_spec().with_searchable(true)`; query field, filtering, and `TIME_ZONE_EMPTY_MESSAGE` all from `Select`.
- [x] FIXED dead focus-shadow — the entire hand-rolled shell (including the discarded `_focus_shadow`) is gone; focus treatment now comes from `Select`.
- [x] FIXED gap/chevron rem literals — gone; spacing and the chevron size resolve through `Select`'s size/density token path.
- [x] FIXED selection / query / open — handled by `Select`'s `on_change` / `on_search_change` / `on_toggle` channels in the preview loop.
- accepted: no ARIA (jetstream has no accessibility API).
- preview-loop: open/search/select state held by the host (same channels as `Select`).

## Specimen parity

- Svelte covers: Default (interactive, value echoed below), Pre-selected (`America/New_York`), Disabled, plus Sizes and Densities snippets (`TimeZoneSelectSpecimen.svelte`). Real `Select`, so open/search/keyboard all live.
- GPUI covers: Default (toggleable open via `tz-select-open` state), Pre-selected, **Open (searchable, selected)** — statically-open dropdown with live `search_query="amer"` filtering the real zone list plus the selected zone highlighted with a check, Disabled, Sizes, Densities (`specimens/time_zone_select.rs`). Every group is the real `TimeZoneSelect`; the open list is selectable (rows fire `on_change`) and the search input is the real `TextInput`. Builds clean. **GPUI specimen complete.** Commit/select/query state stays consumer-owned (preview channels).
- Jetstream covers: With value, Placeholder, Disabled (`specimens/time_zone_select.rs`). — missing: **open/dropdown state**, **Sizes** group, **Densities** group, searchable demo. Closed-chrome only.

## Notes

- `consv=fixed` is doc-side only (contract's `options` default + the `emptyMessage` string); behavior is faithful.
- RESOLVED `TimeZoneSelectSpec` was grown additively to reach `Select` parity: `id`/`name`/`default_value`/`options`/`aria_label`/`described_by`/`search_query` added, plus `current_value()`, `effective_options()`, `effective_placeholder()`, `select_options()`, and `to_select_spec()` (builds a searchable `SelectSpec` exactly like the Svelte wrapper). `trigger_text()` now returns `Option<String>` (label-formatted). All additive — no fields removed; `date_time_zone_picker` consumer unaffected.
- RESOLVED contract §10's Rust `defaultTimeZoneOptions()` equivalent now exists: `poodle_specs::default_time_zone_options()` (curated 9-entry fallback, `_`→space labels via `formatTimeZoneLabel` rule), shared by both targets. Also exported: `TIME_ZONE_EMPTY_MESSAGE`, `TIME_ZONE_PLACEHOLDER`.
- Offset display: not implemented or required anywhere (out of scope per contract §1). No offset-computation divergence across targets.
