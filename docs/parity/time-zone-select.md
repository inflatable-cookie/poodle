<!-- parity consv=fixed gpui=6 jetstream=6 specimen=gap -->
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

GPUI hand-rolls a trigger + dropdown instead of delegating to a `Select` primitive, so most of `Select`'s contract surface is reimplemented partially.

- [ ] Hardcoded shadow literals: `hsla(0.0, 0.0, 0.0, 0.10)` / `hsla(0.0, 0.0, 0.0, 0.06)` plus `px(4.0)`/`px(16.0)`/`px(1.0)` offsets and blur at `time_zone_select.rs:214-224` — resolve elevation from a shadow/elevation token, not raw HSLA + px floats.
- [ ] Timezone list hardcoded inline in the component (`time_zone_select.rs:193-205`, 11 entries) — should pull from a shared `default_time_zone_options()` Rust equivalent (contract §10 requires one); currently no such source exists and the list duplicates the Svelte fallback rather than the full IANA set.
- [ ] Options are not selectable: dropdown rows only `hover` (`time_zone_select.rs:230-238`); clicking a row does nothing, so there is no `onValueChange` / value-pick path. Svelte/`Select` selects on click. **Toggle-only, not a working select.**
- [ ] No searchable mode: contract §7 says searchable is always enabled; there is no query input, no filtering, no `emptyMessage`. GPUI renders a static list only.
- [ ] Placeholder fallback mismatch: `trigger_text().unwrap_or("Select timezone...")` (l.110-112) hardcodes `"Select timezone..."`; Svelte/contract placeholder is `"Search time zones..."`. Use the spec placeholder default, not an ad-hoc string.
- [ ] No `onQueryChange` / `onOpenChange` channels — only `on_toggle(bool)` exists (l.77, 167-184). Query callback absent entirely.
- accepted: no ARIA combobox/listbox/option (gpui has no accessibility API).
- accepted: overlay timing/positioning runtime-owned (contract §11 Tier 3).

## Jetstream gap (vs Svelte + contract)

Jetstream renders only the closed trigger chrome. The dropdown, search, listbox, and open state are entirely absent — `is_open` is never read.

- [ ] No open/dropdown rendering: `js_time_zone_select` ignores `spec.is_open` (`time_zone_select.rs:20-80`); there is no listbox, option rows, or overlay. Contract §4 requires an `open` state. Not wired in preview `main.rs` either (no tz/search/dropdown handling found).
- [ ] No searchable mode: no query field, no filtering, no `emptyMessage`. Contract §7 says searchable always enabled.
- [ ] Dead focus-shadow computation: `focus_ring_c` + `_focus_shadow = focus_ring_c.with_alpha(0.28)` computed then discarded (`time_zone_select.rs:35-36`) — focus ring is never actually applied to the shell. Wire it or drop it.
- [ ] Hardcoded gap literal `rem_to_px(0.375)` at `time_zone_select.rs:53` — resolve from an inline-space token, not a rem literal.
- [ ] Hardcoded chevron size `rem_to_px(0.75)` ×2 at `time_zone_select.rs:68-69` — resolve from an icon-size token.
- [ ] No value-selection / `onValueChange` / `onQueryChange` / `onOpenChange` path — component is display-only chrome.
- accepted: no ARIA (jetstream has no accessibility API).
- accepted: interaction lives in preview event loop, not the component — but note here it is absent in both.

## Specimen parity

- Svelte covers: Default (interactive, value echoed below), Pre-selected (`America/New_York`), Disabled, plus Sizes and Densities snippets (`TimeZoneSelectSpecimen.svelte`). Real `Select`, so open/search/keyboard all live.
- GPUI covers: Default (toggleable open via `tz-select-open` state), Pre-selected, Disabled, Sizes, Densities (`specimens/time_zone_select.rs`). — missing: searchable/query demo, value-pick interaction (open list isn't selectable).
- Jetstream covers: With value, Placeholder, Disabled (`specimens/time_zone_select.rs`). — missing: **open/dropdown state**, **Sizes** group, **Densities** group, searchable demo. Closed-chrome only.

## Notes

- `consv=gap` driver is minor: contract's `options` default and the undocumented `emptyMessage` string. Behavior is faithful; the contract text just lags Svelte. Both fixes are doc-side.
- The shared `TimeZoneSelectSpec` (`contracts/components/src/time_zone_select.rs`) is itself thin: no `options`, no `searchable`, no query/empty-message fields, no `aria_label`/`described_by`. The Rust targets therefore cannot reach `Select` parity without spec growth — this is the structural root of both Rust gaps, not just rendering laziness.
- Contract §10 asks for a Rust `defaultTimeZoneOptions()` equivalent; none exists. GPUI inlines a partial copy; Jetstream has no list at all. Add a shared Rust default-options source.
- Offset display: not implemented or required anywhere (out of scope per contract §1). No offset-computation divergence across targets.
