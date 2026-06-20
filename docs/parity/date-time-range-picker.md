<!-- parity consv=gap gpui=7 jetstream=4 specimen=gap -->
# Parity: DateTimeRangePicker

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/date-time-range-picker.md`
- Svelte (authoritative): `packages/svelte/components/src/DateTimeRangePicker.svelte`
- GPUI: `packages/gpui/components/src/primitives/date_time_range_picker.rs`
- Jetstream: `packages/jetstream/components/src/date_time_range_picker.rs`
- Spec: `packages/contracts/components/src/date_time_range_picker.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/DateTimeRangePickerSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/date_time_range_picker.rs` · jetstream `packages/jetstream/preview/src/specimens/date_time_range_picker.rs`

## Contract ↔ Svelte

Props, anatomy, ARIA, and callbacks match (`value`, `defaultValue`, `open`, `defaultOpen`, `placeholder`, `weekStartsOn`, `locale`, `size`, `sizeRole`, `density`, `disabled`, `ariaLabel`; `onValueChange`, `onOpenChange`; trigger `aria-haspopup="dialog"` / `aria-expanded` / `aria-controls`; surface `role="dialog"`). The size table diverges:

- **Size table mismatch.** Contract §8 specifies trigger `min-height` as `calc(var(--poodle-size-control-height) ± Nrem)` and per-size `padding` offsets (`xs`/`sm` shrink, `lg`/`xl` grow). Svelte instead sets **absolute** trigger heights (`xs:1.5rem`, `sm:1.75rem`, `lg:2.75rem`, `xl:3.25rem`; `DateTimeRangePicker.svelte:353-377`) and applies **no per-size padding** — only per-density padding (`:379-380`). Svelte is authoritative. **Fix: rewrite contract §8 size table to absolute heights + drop the per-size padding column (or move padding deltas to the density rows Svelte actually uses).**
- **Indicator font-size per size not in contract.** Svelte scales the indicator (`xs:0.625rem … xl:0.875rem`, `:356,361,372,377`); contract §8 only fixes indicator at `0.75rem`. **Fix: add the per-size indicator font-size to the contract.**
- Indicator glyph: Svelte renders `▾` (`:194`); contract anatomy calls it a "disclosure chevron." Cosmetic, Svelte authoritative.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] **Overlay is a hand-coded mockup, not composed primitives.** Contract §2 requires the surface to compose `Calendar mode="range"` + two `TimeInput`s. GPUI hand-builds a fake 6×7 dash grid (`date_time_range_picker.rs:266-285`), fake bordered time fields (`:310-336`), and an invented "Today"/"Done" action bar (`:347-374`) that is not in the contract or Svelte. This violates "No Mockups" — replace with real `Calendar` (range) + `TimeInput` GPUI primitives.
- [ ] **Invented action bar.** The Today link + Done button (`:347-374`) have no contract or Svelte counterpart. Remove.
- [ ] Hardcoded grid/cell literals: `gap(px(rem_to_px(0.125)))` (`:253,267,269`), cell/time/done `h(px(rem_to_px(1.75)))` (`:274,325,364`) — resolve from tokens once real primitives replace the mockup.
- [ ] Separator `h(px(1.0))` raw pixel (`:304`) — use a border-width token.
- [ ] Shadow uses raw `hsla(0.0, 0.0, 0.0, 0.10/0.06)` + `px(4.0)/px(16.0)/px(1.0)` literals (`:385-394`). Contract maps box-shadow to `elevation.overlay`; resolve the elevation token instead of an inline two-layer shadow.
- [ ] Surface border `Hsla { a: border.a * 0.72, ..border }` inline alpha (`:382`) — route the 72% border-mix through `color_mix`/token helper.
- [ ] Time label typography not matched: contract requires label-family, **0.6875rem**, weight 600, 0.04em tracking, **uppercase** (`time_field` label uses plain `label_size`, no weight/tracking/transform; `:316-321`). Apply the time-label token treatment.
- accepted: no ARIA (gpui has no accessibility API) — trigger haspopup/expanded/controls + dialog role not emitted.
- accepted: overlay absolute-positioning posture is platform-owned (contract Known Delta).

## Jetstream gap (vs Svelte + contract)

- [ ] **Overlay not rendered** — `js_date_time_range_picker` emits trigger only (`date_time_range_picker.rs:99-108`). Calendar(range)+paired-time surface is deferred to the runtime event loop. Acceptable per the established trigger-only pattern, but the range calendar + two time fields must exist once overlay composition lands. Tracked as a gap.
- [ ] Hardcoded `gap(rem_to_px(0.75))` (`:77`) and `min_w(rem_to_px(18.0))` (`:100`) — GPUI resolves min-width from `size.dateTimeRangePicker.minWidth` token; Jetstream should resolve the same token and a gap token, not raw rem floats.
- [ ] Hover blend uses raw `fill_c.mix(elevated_c, 0.14)` (`:42`) — the contract hover is `color-mix(surface 86%, elevated)`; `0.14` is the inverted ratio passed to a color helper. Confirm the helper semantics match 86%/14% and route via a named helper, not a bare literal.
- [ ] Indicator uses `chevron-down` icon (`:92`) while Svelte/contract use a `▾` text glyph indicator with per-size font scaling — acceptable icon substitution, but indicator is not size-scaled. Note for visual parity.
- accepted: no ARIA channel for haspopup/expanded (documented pattern).
- accepted: overlay interaction (open/close, calendar, time fields) lives in the preview event loop, not the component.

## Specimen parity

- Svelte covers: Default, With default range, Disabled; plus size and density snippets (`DateTimeRangePickerSpecimen.svelte`).
- GPUI covers: Default (toggle-open wired), With default range (toggle-open), Disabled; plus size/density via `specimen_layout` (`date_time_range_picker.rs`). Open state demonstrates the mocked overlay. — missing: real composed calendar/time content (shows mockup); otherwise state coverage matches.
- Jetstream covers: With range, Placeholder, Disabled (`date_time_range_picker.rs`). — missing: **size and density groups** (Svelte/GPUI both demo these via snippets/specimen_layout); **open-overlay state** (trigger-only, so no surface shown).

## Notes

- `consv=gap` driver: the contract §8 size table (calc-based heights + per-size padding) does not match Svelte's absolute heights + density-only padding, and the contract omits the per-size indicator font-size Svelte ships. Both belong in the contract per "Svelte is parity authority."
- `DateTimeRangePickerSpec` is well-modeled — it carries the full prop surface (`value`, `default_value`, `open`, `default_open`, `placeholder`, `week_starts_on`, `locale`, `is_disabled`, `aria_label`, size/role/density) matching the contract. The Rust gaps are rendering-side, not spec-side.
- Biggest single issue: the GPUI mocked overlay (fake grid + invented Today/Done bar). It must be rebuilt on real `Calendar`(range)/`TimeInput` primitives — the hand-coded version hides that the composition is incomplete.
