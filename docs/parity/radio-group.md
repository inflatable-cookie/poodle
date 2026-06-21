<!-- parity consv=ok gpui=1 jetstream=1 specimen=gap -->
<!-- pass 30: selected_color now honored — custom hex parsed via theme_ext::hex_to_rgb255
     → indicator ring + dot use it (else accent.base); ICON_DEFAULT hardcode replaced with
     resolve_px(size.icon.md) base. Probe-tested (custom #ff0000 ≠ accent). -->
<!-- pass 41: density now drives the group gap on BOTH targets — Svelte cascade where a
     compact/comfortable override wins over the orientation gap (compact→space-stack-sm,
     comfortable→space-stack-lg, default→orientation gap). Jetstream: disabled options now
     use default cursor (not pointer) + enabled rows .focusable() for the preview focus
     loop. Probe-tested (labels, selected dot, vertical/horizontal/density gap, per-size
     indicator, per-option + group disabled opacity). Remaining each target: focus ring
     (preview/runtime); name/described-by are a11y-tree only (no Rust a11y surface). -->
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

- [x] `density` now drives the group gap — `group_gap` folds in a compact/comfortable override (compact→space-stack-sm, comfortable→space-stack-lg) that wins over the orientation gap, matching the Svelte cascade (`RadioGroup.svelte:107/186-192`). Both orientations now use `group_gap`.
- accepted: no `name` wiring — `name` is a native-`<input>` form-grouping attribute (Svelte `name ?? generatedName`); GPUI has no native inputs and no a11y tree, so it has no visual or accessible effect here (rolls into the ARIA limit below).
- accepted: no `description_id` → no `aria-describedby` channel — non-visual; GPUI emits no ARIA (rolls into the ARIA limit below).
- accepted: indicator border has no transition/motion-token treatment — contract §12 known delta marks "transition timing is platform-owned"; GPUI has no CSS transition primitive. Static border, runtime limit.
- accepted: no ARIA (gpui has no accessibility API) — `role="radiogroup"`, `aria-checked`, `aria-orientation` not emitted.
- accepted: `px(999.0)` pill radius (`radio_group.rs:177,187`) and `rem_to_px(0.25/0.125/0.4/0.45/0.55/0.6)` size offsets (`:117-130`) are contract-§8 literals via the sanctioned rem helper, not arbitrary hardcodes.
- accepted: roving-focus, arrow-key nav, space-to-select, and click selection are implemented (`radio_group.rs:202-244`) — keyboard parity present.

## Jetstream gap (vs Svelte + contract)

- [x] `ICON_DEFAULT` literal removed (pass 30) — `indicator_size_px`/`dot_size_px` now derive from `resolve_px(theme, "size.icon.md")`, matching GPUI.
- [x] `selected_color` honored (pass 30) — custom hex parsed via `hex_to_rgb255` drives the selected indicator ring + dot, else `color.accent.base`.
- [x] `density` now drives the group gap — same Svelte cascade as GPUI (compact→space-stack-sm, comfortable→space-stack-lg, default→orientation gap).
- [x] Per-option / group disabled now also revert the cursor to default (no `not-allowed` cursor in JsEl — Svelte uses it; noted runtime limit) and enabled rows are `.focusable()` so the preview loop can drive the focus ring.
- accepted: no focus ring outline — JsEl has no CSS `outline` primitive; focus state (`accent.focusRing`, offset 0.125rem) is driven by the preview/runtime, not the pure-render component.
- accepted: no `name`/`description_id` wiring — non-visual form/a11y attributes; Jetstream has no native inputs or accessibility surface.
- accepted: no indicator border motion/transition treatment (contract §8 motion tokens) — JsEl has no CSS transition; contract §12 marks timing platform-owned.
- accepted: group-level disabled blocking selection + nav-skip lives in the preview event loop (component is pure render); the component dims correctly (group + per-option opacity).
- accepted: no ARIA channel (`role`/`aria-checked`/`aria-orientation`) — Jetstream has no accessibility surface.
- accepted: selection/click + keyboard interaction lives in the preview event loop, not `js_radio_group` (component is pure render).
- accepted: `rem_to_px(0.0625)` border width (`radio_group.rs:66`) is the contract §8 literal via the rem helper.

## Specimen parity

- Svelte covers: Vertical (default), Horizontal, Disabled, **Custom selected color**, **size matrix** (xs–xl snippet), **density matrix** (snippet) — plus live "Selected:" readout (`RadioGroupSpecimen.svelte`).
- GPUI covers: Vertical, Horizontal, Disabled, **Disabled option**, **Custom selected color**, size matrix + density matrix (via `specimen_layout`), live "Selected:" readout. — fullest of the three; no gaps vs Svelte (adds Disabled-option case).
- Jetstream covers: Vertical, Horizontal, Disabled only (`radio_group.rs:18-33`). — missing: **Custom selected color**, **Disabled option**, **size matrix**, **density matrix**, live "Selected:" readout. Uses generic `Option A/B/C` instead of the contract §13 `Free/Pro/Enterprise` + `Small/Medium/Large/Extra large` labels.

## Notes

- consv=ok: Svelte fully implements the contract; the only quibble is §8 xs/sm wording (offset-from-`icon-md` vs per-size-icon-token), which is a contract clarification, not an implementation bug — all three targets already agree on the resolved values.
- Jetstream's `selected_value` correctly mirrors the contract's controlled→uncontrolled fallback (`value.or(default_value)`, `radio_group.rs:70`), matching `RadioGroupSpec::current_value()`.
- Biggest Jetstream gaps are the hardcoded `ICON_DEFAULT` (token-resolution violation) and the thin specimen (3 of 7 Svelte groups, wrong labels).
- GPUI is the strongest Rust target here: interactive selection, keyboard nav, custom color, and roving focus all present.
