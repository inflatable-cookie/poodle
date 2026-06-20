<!-- parity consv=fixed gpui=2 jetstream=2 specimen=gap -->
<!-- pass 39: Jetstream mirrors GPUI — invalid-state border via spec.border_token
     (status.danger); separator 2-row spacer+glyph grid; segment-focus accent-12% highlight;
     size-driven field width + padding (density on inline only); line-heights 1. 5 probe tests
     (incl invalid-border ≈ status.danger ≠ default); suite 140. Remaining jetstream:
     keyboard/onChange/focus-tracking = preview-loop. -->
<!-- pass 38: GPUI — field width + vertical padding now size-driven from the contract table
     (were fixed); invalid state border via spec.border_token (ValidationState::Invalid →
     status.danger); segment-focus accent-12% highlight; line-height/label-font/separator-grid/
     fit-content root fixes; presentation helpers added (unit-tested). Specimen gained Invalid
     group. Remaining GPUI: keyboard increment/onChange = preview-loop; segments as text-elements
     (contract Field is border:0/transparent — composing NumberInput would violate it, accepted). -->
# Parity: DurationInput

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/duration-input.md`
- Svelte (authoritative): `packages/svelte/components/src/DurationInput.svelte`
- GPUI: `packages/gpui/components/src/primitives/duration_input.rs`
- Jetstream: `packages/jetstream/components/src/duration_input.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/DurationInputSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/duration_input_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/duration_input.rs`

## Contract ↔ Svelte

`consv=fixed`. Stale contract size/anatomy values reconciled to Svelte.

- [x] FIXED Field width per size: contract §8 size table now md `1.875rem`, sm `1.625rem` (`DurationInput.svelte:335,331`); base `1.75rem` documented explicitly.
- [x] FIXED Dropped `pattern="[0-9]*"` from contract §9 — Svelte sets only `inputmode="numeric"` (`DurationInput.svelte:155,176,197`). Added the on-focus `select()` note.
- [x] FIXED Separator spacer/glyph split documented in §2 anatomy + §8 token tables (`DurationInput.svelte:166-169`): separator is a 2-row grid (spacer sized to label, glyph carries font-size/weight).
- [x] FIXED (extra, contract was stale) Root `align-items` corrected `flex-end` → `stretch` (`DurationInput.svelte:223`) in §7/§8; density padding/gap adjusts noted.
- All props (hours/minutes/seconds/showSeconds/maxHours/min+maxTotalSeconds/disabled/ariaLabel/size/sizeRole/density), states (focus-within, segment-focus, disabled, invalid), ARIA (`role="group"`, per-field `aria-label`, `<label for>`, separators `aria-hidden`), and keyboard (Arrow±1 with carry) match.

## GPUI gap (vs Svelte + contract)

- [ ] Value model mismatch: GPUI uses a single `value: Option<String>` ("HH:MM:SS") vs contract/Svelte separate `hours`/`minutes`/`seconds` numbers (`duration_input.rs:50-88`). No `max_hours`/`min_total_seconds`/`max_total_seconds`/`ariaLabel` builders surfaced.
- [ ] `validation_state` builder exists (`duration_input.rs:58-61`) but `into_element` never reads it — **invalid danger-border state not rendered** (contract §4 invalid).
- [ ] Segment-focus highlight (accent-base 12% bg on focused segment, contract §8) not implemented — no per-segment focus styling.
- [ ] Keyboard adjusts total seconds by ±60 (`duration_input.rs:214-246`), not ±1 per focused segment with carry as contract §6 specifies; no per-segment focus tracking.
- [ ] `onChange` passes a formatted `&str` ("HH:MM:SS") (`duration_input.rs:240-245`), not the contract `{hours,minutes,seconds,totalSeconds}` payload.
- [ ] Field width hardcoded `px(rem_to_px(1.75))` at `duration_input.rs:162` — does NOT vary by size; contract requires per-size widths (1.5/base/1.875/2/2.25rem).
- [ ] Root vertical padding hardcoded `px(rem_to_px(0.25))` at `duration_input.rs:184` — does NOT vary by size; contract per-size pad-y is 0.125–0.375rem.
- [ ] Field/separator line-height `px(rem_to_px(0.875))` at `duration_input.rs:135,165` — contract line-height is `1`, not 0.875rem. Label font uses caption token (~11px) vs contract `0.5625rem`; none size-vary.
- [ ] Root uses `.items_center()` + `.w_full()` (`duration_input.rs:184,180`) — contract is `align-items: flex-end` + `width: fit-content` (Jetstream gets this right with `items_end`).
- accepted: no ARIA (gpui has no accessibility API) — `role="group"`, per-field `aria-label`, `<label for>` not emittable.
- accepted: GPUI text-element vs HTML `<input>` (contract Known Delta).

## Jetstream gap (vs Svelte + contract)

- [ ] No `onChange` callback at all — contract §5 `onChange` entirely absent (parse-only display).
- [ ] focus-within accent border + focus-ring not implemented — root is `.focusable()` (`duration_input.rs:156`) but no focus border/shadow styling (contract §8 focus-within).
- [ ] Segment-focus highlight (accent 12% bg) not implemented.
- [ ] Keyboard Arrow±1 increment/carry not implemented — no key handler.
- [ ] Labels hardcoded uppercase "H"/"M"/"S" (`duration_input.rs:159,165,170`); contract/Svelte source is lowercase h/m/s (CSS uppercases). Cosmetic but diverges from anatomy.
- [ ] `field_font_rem` Sm/Md hardcode `0.8125` (`duration_input.rs:52`) with a "typography-body-size" comment — resolve the body-size token instead of the literal.
- accepted: no ARIA (`role="group"`, field labels).
- accepted: interaction (key handling, onChange) would live in preview event loop; absent here.

## Specimen parity

- Svelte covers: Hours/minutes/seconds (bound + total readout + onChange), Hours-and-minutes-only (`showSeconds=false`), Disabled, Last-change readout, plus size + density sweeps.
- GPUI covers: Full (H:M:S) + seconds caption, Hours-and-minutes-only, Disabled, plus size + density sweeps. — missing: **invalid-state** group, interactive onChange readout.
- Jetstream covers: With value (HM), With seconds, Sizes (Sm/Md/Lg), Disabled. — missing: **invalid-state** group, hours-and-minutes-only labeled group, total-seconds/onChange readout.

## Notes

- No target's specimen demonstrates the **invalid** (out-of-bounds danger border) state — Svelte included.
- GPUI is the weakest on sizing: field width and root pad-y are fixed regardless of `size`, so the GPUI size-sweep specimen visually under-differentiates. Jetstream is fully size-responsive but lacks all interaction.
- consv=fixed: stale contract size-table values (md/sm field width), the `pattern` note Svelte does not honor, and the separator spacer/glyph split are now reconciled to Svelte.
