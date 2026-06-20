<!-- parity consv=ok gpui=4 jetstream=6 specimen=gap -->
# Parity: RadioGroup

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/radio-group.md`
- Svelte (authoritative): `packages/svelte/components/src/RadioGroup.svelte`
- GPUI: `packages/gpui/components/src/primitives/radio_group.rs`
- Jetstream: `packages/jetstream/components/src/radio_group.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/RadioGroupSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/radio_group.rs` · jetstream `packages/jetstream/preview/src/specimens/radio_group.rs`

## Contract ↔ Svelte

Svelte matches the contract on every prop (name/type/default), the full anatomy
(root → option label → hidden control → indicator → dot → label), all states, and
the documented ARIA surface. No divergence.

- Props: `value`/`defaultValue`/`options`/`orientation`/`disabled`/`ariaLabel`/`describedBy`/`name`/`selectedColor`/`size`/`sizeRole`/`density` all present with contract defaults (`RadioGroup.svelte:9-39`). `onValueChange` callback present (`:59-67`) and matches §5.
- Anatomy: `role="radiogroup"` root (`:70-80`), `<label>` option, visually-hidden `<input type="radio">`, indicator span, dot span, label span (`:81-98`). Matches §2 exactly.
- ARIA: `aria-label`, `aria-describedby`, `data-orientation`/`data-disabled`/`data-size`/`data-density` all emitted; `aria-orientation` intentionally NOT set — contract §6 explicitly documents this. Consistent.
- Indicator/dot md sizing: Svelte uses `calc(--poodle-size-icon-md + 0.125rem)` / `calc(--poodle-size-icon-md * 0.5)` (`:146-147,156-157`); contract §8 lists md as the literal `1.125rem`/`0.5rem` these calc to. Same value, no divergence.
- Note (not a divergence): contract §8 xs/sm size table says `calc(icon-default − 0.125rem)` (indicator) and `icon-default × 0.4/0.45` (dot), but Svelte uses `calc(icon-xs/sm + 0.25rem)` and literal `0.4rem/0.45rem` (`:195-213`). Svelte resolves per-size icon tokens rather than always offsetting `icon-md`; results land near the contract numbers. Both Rust impls follow Svelte's xs/sm `+0.25rem` form, so all three agree. Flagged as a contract §8 wording cleanup, not a parity gap.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] No `name` builder/prop wiring — `RadioGroupSpec.name` is never read; no shared form-name grouping (Svelte `name ?? generatedName`, `RadioGroup.svelte:86`). `radio_group.rs:44-94`.
- [ ] No `density` visual effect — `density()` builder stores the value (`radio_group.rs:81-84`) but the group gap only branches on orientation (`:160-163`); Svelte adjusts vertical gap for `compact`/`comfortable` (`RadioGroup.svelte:186-192`).
- [ ] No `description_id` → no `aria-describedby` channel (stored at `radio_group.rs:69-72`, never emitted). Distinct from the blanket ARIA delta.
- [ ] Indicator border has no transition/motion-token treatment — contract §8 specifies `border-color`/`box-shadow` transition via motion tokens; GPUI renders static border (`radio_group.rs:174-184`). Accept only if confirmed a runtime limit.
- accepted: no ARIA (gpui has no accessibility API) — `role="radiogroup"`, `aria-checked`, `aria-orientation` not emitted.
- accepted: `px(999.0)` pill radius (`radio_group.rs:177,187`) and `rem_to_px(0.25/0.125/0.4/0.45/0.55/0.6)` size offsets (`:117-130`) are contract-§8 literals via the sanctioned rem helper, not arbitrary hardcodes.
- accepted: roving-focus, arrow-key nav, space-to-select, and click selection are implemented (`radio_group.rs:202-244`) — keyboard parity present.

## Jetstream gap (vs Svelte + contract)

- [ ] `ICON_DEFAULT: f32 = 1.0` is hardcoded in `indicator_size_rem`/`dot_size_rem` (`radio_group.rs:25,44`) instead of resolving `size.icon.md` from the theme (GPUI does: `resolve_px(theme, "size.icon.md")`). Token-resolution gap — md/xs/sm/lg/xl all derive from this literal.
- [ ] No `selected_color` support — `spec.selected_color` is never read; selected indicator/dot always use `color.accent.base` (`radio_group.rs:67,81,97`). Svelte maps `selectedColor` to the local selected color (`RadioGroup.svelte:57,164-170`); GPUI honors it (`radio_group.rs:138-143`).
- [ ] No focus ring — contract §8 focus state (`accent.focusRing`, `outline-offset 0.125rem`) absent; no focus styling on the option row.
- [ ] No `name`/`density`/`description_id` wiring — none of these spec fields are read.
- [ ] No indicator border motion/transition treatment (contract §8 motion tokens) — static render (`radio_group.rs:85-90`).
- [ ] Group-level disabled applies opacity but does NOT block selection/interaction in the component; per-option `is_disabled` only dims, no `not-allowed` cursor or nav-skip (`radio_group.rs:113-124`). Svelte sets `disabled` on the input and `cursor: not-allowed` (`RadioGroup.svelte:89,125-128`).
- accepted: no ARIA channel (`role`/`aria-checked`/`aria-orientation`) — Jetstream has no accessibility surface.
- accepted: selection/click + keyboard interaction lives in the preview event loop, not `js_radio_group` (component is pure render).
- accepted: `rem_to_px(0.0625)` border width (`radio_group.rs:60`) is the contract §8 literal via the rem helper.

## Specimen parity

- Svelte covers: Vertical (default), Horizontal, Disabled, **Custom selected color**, **size matrix** (xs–xl snippet), **density matrix** (snippet) — plus live "Selected:" readout (`RadioGroupSpecimen.svelte`).
- GPUI covers: Vertical, Horizontal, Disabled, **Disabled option**, **Custom selected color**, size matrix + density matrix (via `specimen_layout`), live "Selected:" readout. — fullest of the three; no gaps vs Svelte (adds Disabled-option case).
- Jetstream covers: Vertical, Horizontal, Disabled only (`radio_group.rs:18-33`). — missing: **Custom selected color**, **Disabled option**, **size matrix**, **density matrix**, live "Selected:" readout. Uses generic `Option A/B/C` instead of the contract §13 `Free/Pro/Enterprise` + `Small/Medium/Large/Extra large` labels.

## Notes

- consv=ok: Svelte fully implements the contract; the only quibble is §8 xs/sm wording (offset-from-`icon-md` vs per-size-icon-token), which is a contract clarification, not an implementation bug — all three targets already agree on the resolved values.
- Jetstream's `selected_value` correctly mirrors the contract's controlled→uncontrolled fallback (`value.or(default_value)`, `radio_group.rs:70`), matching `RadioGroupSpec::current_value()`.
- Biggest Jetstream gaps are the hardcoded `ICON_DEFAULT` (token-resolution violation) and the thin specimen (3 of 7 Svelte groups, wrong labels).
- GPUI is the strongest Rust target here: interactive selection, keyboard nav, custom color, and roving focus all present.
