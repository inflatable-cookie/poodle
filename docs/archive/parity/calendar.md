<!-- parity consv=fixed gpui=2 jetstream=2 specimen=ok -->
<!-- pass 82: Jetstream calendar specimen backfilled to full Svelte coverage — added
     With pre-selected date, Range selection (pre-seeded start/end/in-range), Range with
     pre-selected range, Range disabled, Sizes (xs..xl), Densities. All groups render the
     real js_calendar + CalendarSpec fields only (no hand-rolled grid). Both previews build
     clean (gpui + jetstream/preview cargo build, 0 errors). specimen gap→ok. No
     disabled-dates group: CalendarSpec has no disabled_dates field and Svelte (authority)
     omits it — not a renderable state, not faked. -->
<!-- pass 81: typography.caption.size adapter gap fixed on BOTH adapters (gpui + jetstream
     theme.rs were missing the arm → resolved to 0). Closes the calendar weekday-caption
     token-gap on both targets (also benefits tabs badge, menu meta, select, color-picker).
     Remaining 2/2: weekday-row-height 1.5rem (no token) + roving-tabindex (preview-loop). -->
<!-- pass 43: adapter caption-size gap CLOSED for Jetstream. The cross-cutting
     `typography.caption.size` → 0 bug (missing match arm in the Jetstream adapter)
     is fixed — adapter now resolves caption/heading/code sizes + label/heading
     weights from the typed semantic constants (caption.size = 11px). The weekday
     caption no longer relies on a 0px token miss; it stays the contract-exact
     0.6875rem (== 11px) for self-documentation. Jetstream count 3 → 2 (the
     caption-size half of the old token-gap item is closed; weekday-row-height
     1.5rem remains a genuine no-token gap). GPUI unchanged (its adapter still
     lacks the caption arm too — noted, out of this pass's scope).
     pass 42: finalize audit — re-confirmed against code that EVERY representable visual/token gap
     is closed on both targets (outside-month muted opacity 0.72; per-size cell/nav/day-font/month-
     label scales; Month+Year edit-affordance triggers; exact week count; selected / range-start/end /
     in-range / today / hover / selected-hover / disabled treatments; weekday header). The remaining
     gpui=3 / jet=2 are NOT representable gaps — they are reclassified runtime limits, held as-is:
       • GPUI: (1) roving-tabindex focus model + Enter/Space focus-cursor selection — preview-loop /
         focus-model (current keyboard is selection-driven, mutates committed value); (2) Home/End —
         blocked behind the focus cursor, preview-loop; (3) weekday-row-height 1.5rem — TOKEN GAP
         (no token; contract-exact rem, accepted).
       • Jetstream: (1) keyboard + roving-tabindex + month-change wiring — preview-loop; (2) weekday-
         row-height 1.5rem — TOKEN GAP (contract-exact rem); no ARIA channel — accepted (runtime has
         no a11y API). -->
<!-- pass 37: Jetstream calendar rebuilt to match GPUI — outside-month opacity 0.4 →
     state.opacity.muted (0.72, the core bug); static month label → Month + Year edit-affordance
     triggers (dashed underline, current values); per-size cell/nav/day-font scales (calendar
     table); selected/range/today/in-range/disabled day treatments; weekday header; exact week
     count. 7 probe tests (outside ≈0.72 NOT 0.4, selected accent fill); suite 135. Remaining
     jetstream: roving-tabindex keyboard + month-change = preview-loop; weekday height no-token. -->
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

- [x] FIXED **Root width.** Contract §7/§8 mandated a fixed `width: 18rem` at md plus `min-width: 16rem`. Svelte uses `width: fit-content` + per-size `--calendar-cell-size`. Contract §7 Sizing, §8 Root/Weekday/Week grid-columns, the size table (cell-size rows added), the Root token-target, and the Tier 2 checklist line are now reconciled to the `fit-content` + cell-size model (xs 1.75 / sm 2 / md 2.25 / lg 2.5 / xl 2.75 rem).
- **Header layout.** Contract §8 Header gap is `0.5rem` with `grid-template-columns: auto minmax(0,1fr) auto`. Svelte `.poodle-calendar__header` matches (lines 555-558). OK.
- [x] FIXED **aria-selected placement.** Contract §6 now states `aria-selected` is on the gridcell wrapper of the selected day, with `data-selected` on the button — matches Svelte (line 512) and the ARIA grid pattern.
- Month/year inline editing (triggers, select, year input) — contract anatomy lists all parts; Svelte implements all (lines 416-485). OK.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] FIXED Hardcoded width literal — root width now derived fit-content style from `cell_size_rem × 7 + 6 gaps(0.125) + 2 padding(0.75)`, tracking the per-size cell scale. No fixed `18.0`.
- [x] FIXED Hardcoded cell-size literal — now `calendar_cell_size_rem(effective_size)` (xs 1.75 / sm 2 / md 2.25 / lg 2.5 / xl 2.75rem) per contract §8 size table. New helper in `presentation.rs`; this is the calendar-specific scale, NOT `control_height_rem` (which gives 1.5/1.75/2.25/2.75/3.25 — wrong for cells).
- [x] FIXED Hardcoded nav-button-size literal — now `calendar_nav_size_rem(effective_size)` (xs 1.5 … xl 2.5rem) per contract size table. New helper.
- [x] FIXED Hardcoded outside-month opacity literal — now `resolve_opacity(theme, "state.opacity.muted")` (= 0.72). The token exists; no longer a raw literal.
- token gap (accepted): weekday-header row height `1.5rem` — no token exists; kept as a named contract-exact `rem_to_px(1.5)` (`weekday_row_height`, calendar.rs:585). Acceptable per rules (contract-exact rem; not a hardcode violation). Closing it requires adding a `size.calendar.weekdayRowHeight` token — deferred, not invented.
- [x] FIXED Month/year header is now a composed editable control — Month Trigger + Year Trigger buttons with the dashed-underline edit affordance and hover treatment (Svelte `.month-button`/`.year-button`), rendered at the current month/year. Double-click-to-edit + inline Month Select / Year Input editors remain preview-loop interaction.
- preview-loop (reclassified, confirmed pass 42): roving tabindex / Enter-Space focus-cursor selection — the `on_key_down` handler (calendar.rs:519) is selection-driven: arrows mutate the committed value via `on_select`/`on_navigate`, not a roving focus cursor. A faithful focus cursor needs per-frame focus state the render-immediate model doesn't hold here, so the focus-roving + Enter/Space-on-cursor semantics stay host-owned. Not a representable visual/token gap.
- preview-loop (reclassified, confirmed pass 42): Home/End keyboard handling — absent from the handler and blocked behind the focus cursor above (without a cursor, Home/End would wrongly mutate the committed value to a week boundary). Held until the focus model lands.
- [x] FIXED Per-size scaling — cell size, nav button, day font (`calendar_day_font_rem`), and month-label font (`size_font_rem`) now all vary by `effective_size` per the contract size table.
- [x] FIXED Added selected / range-endpoint hover treatment — `color-mix(accent 88%, white 8%)` via `color_mix(accent, white, 0.88)` (matches button.rs danger-hover pattern); was missing entirely.
- accepted: no ARIA (gpui has no accessibility API) — grid/row/gridcell roles + aria-selected/aria-live not emitted.

## Jetstream gap (vs Svelte + contract)

All visual/token gaps below were closed in the pass-37 rebuild; verified against
`calendar.rs` in this audit pass. 7 probe tests cover header controls, the
underline affordance, outside-month muted opacity, the selected accent fill,
range endpoints + in-range tint, per-size day font, and exact week count.

- [x] FIXED **Outside-month opacity** — now `resolve_opacity(theme, "state.opacity.muted")` (= 0.72), not the old `0.4` value bug. Probe-asserted (`outside_month_day_uses_muted_opacity_not_point_four`).
- [x] FIXED Nav-button size — `calendar_nav_size_rem(effective_size)` per-size (xs 1.5 … xl 2.5rem), no longer a fixed `rem_to_px(2.0)`.
- [x] FIXED Root padding `rem_to_px(0.75)` and grid gap `rem_to_px(0.125)` are now the contract-exact Root/grid rems (§8 Root gap 0.75rem, Grid/Week gap 0.125rem). `rem_to_px` of a contract-exact rem is not a hardcode violation.
- [x] FIXED Caption font — the magic `size_font_rem − 0.125` offset is gone; weekday caption is the contract-exact `rem_to_px(0.6875)` (contract §8 weekday label). The `typography.caption.size` token now resolves correctly too (11px) after the adapter fix.
- [x] FIXED Month/year inline editing — composed Month Trigger + Year Trigger controls with the dashed-underline edit affordance (`border_b_1`, color-mix underline + accent hover), rendered at the current month/year. Inline Select / Input editors remain preview-loop.
- [x] FIXED Exact week count — `total_cells.div_ceil(7)` rows; no fixed 6-row grid, no trailing all-outside row for short months. Probe-asserted (`exact_week_count_no_trailing_blank_row`).
- [x] FIXED Per-size scaling — cell size, nav button, day font, and month-label font all vary by `effective_size` (contract §8 size table).
- [x] FIXED `typography.caption.size` adapter gap — the missing match arm was added (pass 43); it now resolves to `11px` from `TYPOGRAPHY_CAPTION_SIZE`, no longer `0`. The weekday caption still uses the contract-exact `rem_to_px(0.6875)` (== 11px) for self-documentation, but the token miss that previously forced that workaround is gone.
- token gap (remaining): weekday-row-height `1.5rem` has no token; kept as a contract-exact rem.
- preview-loop: navigation wiring (prev/next click), keyboard (arrows/Home/End/PageUp/PageDown/Enter/Space), and roving-tabindex live in the preview event loop, not the component. The component renders at the current spec state and exposes interaction ids.
- accepted: no ARIA channel (grid/row/gridcell roles, aria-selected, aria-live).

## Specimen parity

- Svelte covers: Default (interactive + double-click-edit hint), With pre-selected date, Disabled, Range selection, Range with pre-selected range, Range disabled, sizes, densities (`CalendarSpecimen.svelte`).
- GPUI covers: Default (interactive select + navigate), With pre-selected date, Range selection (interactive), Range with pre-selected range, Disabled, Range disabled, sizes, densities — **broad, matches Svelte**. — missing: month/year-editing demo (not implementable until inline editing exists).
- Jetstream covers: Default, With pre-selected date, Range selection (pre-seeded start/end/in-range), Range with pre-selected range, Disabled, Range disabled, Sizes (xs..xl), Densities — **matches Svelte**, all via real `js_calendar` + `CalendarSpec` (no hand-rolled grid). `specimen=ok`. — not covered: month/year-editing demo (preview-loop, same as GPUI); a "with disabled dates" group is absent on every target (no `disabled_dates` field; Svelte omits it) — not a renderable state, not faked.

## Notes

- GPUI keyboard model is selection-driven, not focus-driven — it mutates the selected value on arrow keys rather than moving a roving focus cursor. This diverges from the contract's roving-tabindex requirement and means arrowing changes the committed value, not just focus. Flagged as a behavior todo above.
- Jetstream's `0.4` outside-month opacity is the single clearest value bug in this component (everywhere else it mirrors the Svelte color-mix formulas faithfully via `color_mix`).
- `consv=fixed`: the root-width model and the aria-selected element placement are now reconciled to Svelte. Remaining gpui/jetstream todos are code-side, not contract↔Svelte.
