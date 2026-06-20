<!-- parity consv=gap gpui=9 jetstream=10 specimen=gap -->
# Parity: Calendar

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/calendar.md`
- Svelte (authoritative): `packages/svelte/components/src/Calendar.svelte`
- GPUI: `packages/gpui/components/src/primitives/calendar.rs`
- Jetstream: `packages/jetstream/components/src/calendar.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/CalendarSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/calendar.rs` · jetstream `packages/jetstream/preview/src/specimens/calendar.rs`

## Contract ↔ Svelte

Mostly aligned (props, anatomy parts, states, ARIA all present in Svelte). Divergences:

- **Root width.** Contract §7/§8 mandate a fixed `width: 18rem` at md, scaling per size (xs=14.5rem … xl=23rem) plus `min-width: 16rem`. Svelte sets `width: fit-content` (line 544) and sizes columns via `--calendar-cell-size` per `data-size` (lines 750-753) — no fixed root width, no min-width. Svelte's actual widths differ from the contract table. **Fix: update contract §7/§8 to describe the `fit-content` + per-size `--calendar-cell-size` model (xs 1.75 / sm 2 / md 2.25 / lg 2.5 / xl 2.75 rem), drop the fixed-rem width column.**
- **Header layout.** Contract §8 Header gap is `0.5rem` with `grid-template-columns: auto minmax(0,1fr) auto`. Svelte `.poodle-calendar__header` matches (lines 555-558). OK.
- **aria-selected placement.** Contract §6 says `aria-selected` on the selected day *button*; Svelte sets it on the *cell* (`role="gridcell"`, line 512) and `data-selected` on the button. Minor. **Fix: contract should say aria-selected on the gridcell wrapper (matches ARIA grid pattern).**
- Month/year inline editing (triggers, select, year input) — contract anatomy lists all parts; Svelte implements all (lines 416-485). OK.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded width literal `px(rem_to_px(18.0))` at `calendar.rs:326` — Svelte uses `fit-content`; derive width from cell size × 7 + gaps, or drop fixed width. No token.
- [ ] Hardcoded cell-size literal `px(rem_to_px(2.25))` at `calendar.rs:327` — should resolve per-size from `control_height_rem(effective_size)` (the `_cell_size` computed at line 224 is discarded with `_` prefix and the hardcoded 2.25 used instead).
- [ ] Hardcoded nav-button-size literal `px(rem_to_px(2.0))` at `calendar.rs:328` — no size token; per contract nav scales per size (xs 1.5 … xl 2.5rem), GPUI is fixed.
- [ ] Hardcoded outside-month opacity literal `.opacity(0.72)` at `calendar.rs:578,595` — resolve from a token, not raw `0.72`.
- [ ] Hardcoded weekday-header height literal `.h(px(rem_to_px(1.5)))` at `calendar.rs:538` — no token.
- [ ] No month/year inline editing — contract §2 anatomy requires Month Trigger / Month Select / Year Trigger / Year Input; GPUI renders a static month label only (`calendar.rs:441-447`). No double-click-to-edit.
- [ ] No roving tabindex / Enter-Space day selection — keyboard handler lives on the container and derives the "current day" from the *selected* value not a focus cursor (`calendar.rs:464-524`); arrow keys move selection, not focus. Contract §6 requires roving focus with Enter/Space committing the focused day.
- [ ] Missing Home/End keyboard handling — contract §6 lists Home/End (week boundaries); GPUI handles only left/right/up/down/pageup/pagedown (`calendar.rs:466-522`).
- [ ] Per-size scaling absent — day min-height, font-size, nav size, month-label font do not vary by size (contract §8 size table); GPUI uses fixed 2.25/2.0 regardless of `effective_size`.
- accepted: no ARIA (gpui has no accessibility API) — grid/row/gridcell roles + aria-selected/aria-live not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] **Wrong outside-month opacity** — `.opacity(0.4)` at `calendar.rs:321,335`. Contract §8 and Svelte specify `0.72`. This is a value bug, not just a literal. **Fix: resolve from token / use 0.72.**
- [ ] Hardcoded nav-button-size literal `rem_to_px(2.0)` at `calendar.rs:107` — no size token; fixed regardless of size.
- [ ] Hardcoded root padding literal `rem_to_px(0.75)` at `calendar.rs:106` — should resolve from a space token.
- [ ] Hardcoded grid gap literal `rem_to_px(0.125)` at `calendar.rs:107` (`gap_sm_px`) — resolve from `space.*` token.
- [ ] Ad-hoc caption font `size_font_rem(effective_size) - 0.125` at `calendar.rs:104` — magic `0.125` offset; use a caption-size token like GPUI's `typography.caption.size`.
- [ ] No month/year inline editing — `js_calendar` renders a static label (`calendar.rs:266-272`); no Month Trigger / Select / Year Trigger / Input from contract §2.
- [ ] No navigation wiring — `_prev_month_str` / `_next_month_str` computed then discarded (`calendar.rs:193-200`); nav buttons have no click handlers. Interaction must live in preview `main.rs` event loop; **verify it exists** (none referenced in specimen).
- [ ] No keyboard navigation — none of arrows/Home/End/PageUp/PageDown/Enter/Space (contract §6) handled in component or specimen.
- [ ] Fixed 6-row grid always — `for row in 0..6u32` (`calendar.rs:301`); months needing fewer rows render a trailing all-outside row. Svelte builds exact week count via `buildCalendarWeeks`. Cosmetic but a layout delta.
- [ ] Per-size scaling absent for nav button / month-label font (same as GPUI).
- accepted: interaction (click/keyboard handlers) lives in preview event loop, not the component.
- accepted: no ARIA channel (grid/row/gridcell roles, aria-selected, aria-live).

## Specimen parity

- Svelte covers: Default (interactive + double-click-edit hint), With pre-selected date, Disabled, Range selection, Range with pre-selected range, Range disabled, sizes, densities (`CalendarSpecimen.svelte`).
- GPUI covers: Default (interactive select + navigate), With pre-selected date, Range selection (interactive), Range with pre-selected range, Disabled, Range disabled, sizes, densities — **broad, matches Svelte**. — missing: month/year-editing demo (not implementable until inline editing exists).
- Jetstream covers: Default, With selected date, Range mode (no pre-seeded range), Disabled. — missing: **With pre-selected date** group, **Range with pre-selected range**, **Range disabled**, **sizes**, **densities**. Under-covers vs Svelte → `specimen=gap`.

## Notes

- GPUI keyboard model is selection-driven, not focus-driven — it mutates the selected value on arrow keys rather than moving a roving focus cursor. This diverges from the contract's roving-tabindex requirement and means arrowing changes the committed value, not just focus. Flagged as a behavior todo above.
- Jetstream's `0.4` outside-month opacity is the single clearest value bug in this component (everywhere else it mirrors the Svelte color-mix formulas faithfully via `color_mix`).
- `consv=gap` driver is the root-width model (contract describes a fixed-rem width Svelte does not implement) plus the aria-selected element placement.
