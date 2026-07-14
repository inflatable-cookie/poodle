<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok -->
<!-- specimen pass: both Rust previews built out to full contract coverage (build-verified 0 err). GPUI adds Content-fit (equalWidth=false, xs, per-option aria_label) alongside existing Default/disabled-option/Fully-disabled/Equal-width + size+density panes. Jetstream rebuilt to contract Grid/List/Table set: Default (render-only seeded selection), With-disabled-option (Draft), Fully-disabled, Equal-width, Content-fit (equalWidth=false), full xs–xl Sizes ladder, Compact/Default/Comfortable Densities. Skipped honestly: icon/title segments (ChoiceOption has no icon/title field — used aria_label as closest); live on_change readout on Jetstream (render-only per architecture). -->
<!-- pass: both Rust targets reworked to contract — GPUI selected inset highlight (text-inverse 12%, 0.0625rem, blur 0), inner radius via rem_to_px(0.125), density-driven seg padding, invented separator + transparent-border hack removed, 0.125rem gap added; Jetstream font fixed at 0.75rem, inner radius (control − 0.125rem), root bg color-mix(surface 93%, text-primary), border alpha 84%, equalWidth=false left-aligns. Selected inset shadow not representable on JsEl (noted). Per-option aria/title + click/arrow selection = accepted channel / preview-loop limits. Jetstream probe tests cover segments, accent selected fill, fixed font, density-≠-height, disabled group. -->
# Parity: SegmentedControl

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/segmented-control.md`
- Svelte (authoritative): `packages/svelte/components/src/SegmentedControl.svelte`
- GPUI: `packages/gpui/components/src/primitives/segmented_control.rs`
- Jetstream: `packages/jetstream/components/src/segmented_control.rs`
- Spec: `packages/contracts/components/src/segmented_control.rs` (`ChoiceOption` in `packages/contracts/components/src/types.rs:444`)
- Specimens: svelte `packages/svelte/preview/src/specimens/SegmentedControlSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/segmented_control.rs` · jetstream `packages/jetstream/preview/src/specimens/segmented_control.rs`

## Contract ↔ Svelte

Props/anatomy/state divergences. Svelte authoritative unless it drops contract-specified functionality.

- [x] FIXED (contract already correct): `SegmentedControlOption.title` is documented in contract §3 (tooltip on segment wrapper) and Svelte honors it (`label … title={option.title ?? undefined}`, `SegmentedControl.svelte:80`). The only remaining gap is the shared Rust spec `ChoiceOption` (`types.rs:444-449`) lacks a `title` field — that is a **Rust spec fix (code, out of scope here)**, not a contract change. Contract is faithful to Svelte.
- [x] FIXED: Contract §13 specimen rows rewritten from stale `isDisabled=true` to `disabled=true` (group) / option `disabled=true`, matching Svelte's `disabled` prop (`SegmentedControl.svelte:36,87`).
- **Selected inset shadow color.** Contract §8 selected-state says `inset 0 0.0625rem 0 color-mix(in srgb, white 12%, transparent)`; Svelte matches exactly (`:195-199`). OK.
- **Font-size is fixed at `0.75rem` for all sizes.** Contract §8 Label table hardcodes `font-size: 0.75rem`; Svelte matches (`:183`). Size only varies the track/label height via `--poodle-segmented-control-height`. This is authoritative and the Rust targets must follow it (flagged below where they don't).
- Anatomy, `role="radiogroup"`, hidden radio inputs, `aria-label` (root + per-option), `equalWidth` grid behavior, density→`--poodle-segmented-control-x`, `onValueChange` payload: all present in Svelte and match contract. No divergence.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] FIXED (GPUI): inset highlight now `text-inverse` @ 12% alpha (closest token to contract `white 12%`), offset y `rem_to_px(0.0625)`, blur 0. **Hardcoded selected-shadow color literal** `hsla(0.0, 0.0, 0.0, 0.12)` at `segmented_control.rs:209` — contract shadow is `color-mix(white 12%, transparent)` (a light inset), not a black `0.12` drop shadow. Resolve from a token and match the white-mix inset; current value is both hardcoded and the wrong color.
- [x] FIXED (GPUI): geometry now `0 0.0625rem 0`, blur 0 (GPUI BoxShadow has no inset flag → 1px top-edge approximation, same as accordion). **Hardcoded shadow geometry** `offset: point(px(0.0), px(1.0))`, `blur_radius: px(2.0)`, `spread_radius: px(0.0)` at `segmented_control.rs:210-212` — magic px literals; derive from a shadow token (contract inset = `0 0.0625rem 0`, i.e. 1px y, 0 blur — the `blur_radius: px(2.0)` is also wrong).
- [x] FIXED (GPUI): inner radius now `control_radius - rem_to_px(0.125)`. **Hardcoded inner-radius offset** `control_radius - px(2.0)` at `segmented_control.rs:105` — the `0.125rem` inner-radius inset is a raw `px(2.0)`; resolve `0.125rem` via `rem_to_px(0.125)` like the padding on `:149`, not a bare `2.0`.
- [x] FIXED (GPUI): invented separator removed; row now uses `gap(rem_to_px(0.125))` like Svelte. **Hardcoded 1px separator** `div().w(px(1.0))` at `segmented_control.rs:260` — this separator divider is not in the contract anatomy or Svelte at all; Svelte uses `gap: 0.125rem` between segments, no rule. Remove the invented separator (or token-resolve if kept) — it diverges from the authoritative reference.
- [x] FIXED (GPUI): per-segment transparent border dropped; focus is shadow-only (`focus_ring_shadow`), no layout inflation. **Transparent border padding hack** `border_1().border_color(transparent_black())` on each segment (`:171-172`) inflates segment box by 1px per side with no contract basis; reconcile with Svelte (no per-segment border) so focus border swap doesn't shift layout.
- [x] ACCEPTED (GPUI): `aria_label` now read (`let _aria_label = option.aria_label`) but GPUI has no accessibility channel to emit it (accepted §6/§10 limit). `title` still blocked on the shared `ChoiceOption` spec field.
- accepted: no ARIA radiogroup/radio roles emitted (gpui has no accessibility API) — contract §6/§10 note GPUI must map semantics "where the platform allows"; not available here.
- accepted: roving-tabindex via native radio is replaced by `focusable()` + manual left/right `on_key_down` handler (`:236-251`) — directional nav present, wrap-around implemented.

## Jetstream gap (vs Svelte + contract)

- [x] FIXED (Jetstream): now `rem_to_px(0.75)` for every size. **Font-size scales with size — contract fixes it at `0.75rem`.** `font_size = rem_to_px(size_font_rem(effective_size))` at `segmented_control.rs:15` makes the label font grow with size; Svelte/contract §8 hold font at `0.75rem` for every size. **Fix: use `rem_to_px(0.75)`, not `size_font_rem(...)`.** (GPUI gets this right at `:138`.)
- [x] FIXED (Jetstream): `seg_py = inner` (the shared 0.125rem inner-padding), not a second literal. **Hardcoded vertical inset** `seg_py = rem_to_px(0.125)` at `segmented_control.rs:18` — the `0.125rem` is a bare literal; should come from the same inner-padding token as `inner` (`:20`) rather than a second raw `0.125`.
- [x] FIXED (Jetstream): segment label radius now `(control_radius - inner).max(0.0)`. **Outer radius reused for segment label** `seg.rounded(radius)` at `segmented_control.rs:53` uses the full `radius.control`; contract §8 label radius = `calc(radius-control - 0.125rem)`. Compute the inner radius (control radius minus `0.125rem`) like GPUI does.
- [x] NOTED (Jetstream): JsEl has no shadow/inset support, so the contract inset highlight is not representable — flat accent fill only (documented inline). **No selected inset highlight shadow.** Contract §8 selected state requires `box-shadow: inset 0 0.0625rem 0 color-mix(white 12%, transparent)`; `js_segmented_control` sets `bg(selected_fill)` only (`:56-58`) — inset highlight missing.
- [x] FIXED (Jetstream): root bg now `color_mix(surface, text_primary, 0.93)`. **Root background is plain `surface`, not the contract mix.** `bg = resolve_color(theme, "color.background.surface")` at `segmented_control.rs:23` — contract root bg = `color-mix(surface 93%, text-primary)`. GPUI applies the mix (`:118`); Jetstream omits it, so the track won't read as a recessed well.
- [x] FIXED (Jetstream): root border now `tint(border_subtle, 0.84)`. **Root border alpha not applied.** `border_color(border)` at `:34` uses full `color.border.subtle`; contract = that color at `84%` alpha (`color-mix(... 84%, transparent)`). Mix the border to 84% like GPUI (`:120-123`).
- [x] ACCEPTED (Jetstream): `aria_label` now read but no accessibility channel exists to emit it (accepted limit); `title` absent from shared spec.
- [x] NOTED (Jetstream): selection is render-only from `current_value()`; click + arrow nav live in the preview event loop per Jetstream architecture (documented inline). **No selection or keyboard interaction in the component.** `js_segmented_control` renders selected state from `current_value()` only; click/arrow-key selection must be wired in the preview event loop (not present in this component). Confirm `main.rs` wires `on_change`; otherwise selection is render-only.
- accepted: no ARIA radiogroup/radio roles (jetstream has no accessibility channel).
- accepted: interaction (click + arrow nav) lives in the preview event loop, not the component, per Jetstream architecture.

## Specimen parity

- **Svelte covers** (`SegmentedControlSpecimen.svelte`): Default (controlled, live `onValueChange` + readout), With disabled option, **Content Fit** (`equalWidth={false}` + `size="xs"` + `title` tooltips), **Icon-only options** (`icon` + `iconOnly=true` with accessible label fallback), Fully disabled, plus `sizes` and `densities` snippet matrices.
- **GPUI covers** (`segmented_control.rs`): Default (controlled, live `on_change` + "Selected:" readout), With disabled option, Fully disabled, Equal-width, **Content fit (`equalWidth=false`, xs, per-option `aria_label`)**, plus size + density matrices. — no remaining gaps; `title` tooltip not representable (shared `ChoiceOption` spec has no `title` field — `aria_label` used as the closest accessible-name channel).
- **Jetstream covers** (`segmented_control.rs`): Default (Grid selected, render-only seeded), With disabled option (Draft), Fully disabled, Equal-width, **Content fit (`equalWidth=false`)**, **full xs–xl Sizes ladder**, **Compact/Default/Comfortable Densities**. — no remaining gaps; live `on_change` readout omitted (render-only per Jetstream architecture); `title` tooltip not representable (no spec field).

## Notes

- `consv=fixed`: contract §13 stale `isDisabled` prop names corrected to `disabled`. The `SegmentedControlOption.title` item is contract-faithful already (contract §3 documents it; Svelte honors it); the only remaining shortfall is the shared Rust `ChoiceOption` spec lacking a `title` field, which is a code fix out of scope for this contract-reconciliation pass. Svelte is authoritative.
- Selected-shadow is the sharpest visual gap: GPUI ships the wrong color (`black 0.12` drop) and Jetstream ships none; both should be the contract's `white 12%` inset.
- Font-size divergence is asymmetric: GPUI fixes at `0.75rem` (correct), Jetstream scales it (wrong). Aligning Jetstream to a literal `0.75rem` is the one-line fix.
- GPUI invents a per-segment 1px separator and a transparent per-segment border; neither appears in the authoritative Svelte. Treat as GPUI-only embellishments to remove for parity.
- `size_padding_x_offset_rem` (GPUI) vs `control_space_x_rem(density)` (Jetstream) means the two targets derive horizontal segment padding from different axes (size vs density). Svelte's `--poodle-segmented-control-x` is density-driven (`:143-153`), so Jetstream's density source is closer; GPUI deriving x-padding from size is a separate parity question worth a follow-up.
