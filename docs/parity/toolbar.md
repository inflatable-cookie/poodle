<!-- parity consv=fixed gpui=1 jetstream=1 specimen=ok | pass: both targets size/density padding+gap from contract rem scales; Jetstream gained panel-mix fill + full border + radius.surface + block padding + vertical orientation; roving focus remains the one accepted runtime-owned gap each. Specimens: GPUI specimen rebuilt (Formatting/Actions/Vertical/Sizes/Densities) + builds clean; Jetstream specimen rewritten to real composed primitives (Formatting/Actions/Sizes/Densities/Vertical) but build BLOCKED by external jetstream-renderer wgpu break. GPUI specimen done; Jetstream pending engine recovery — flag stays gap until Jetstream preview builds clean. -->
# Parity: Toolbar

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/toolbar.md`
- Svelte (authoritative): `packages/svelte/components/src/Toolbar.svelte`
- GPUI: `packages/gpui/components/src/primitives/toolbar.rs`
- Jetstream: `packages/jetstream/components/src/toolbar.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/ToolbarSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/toolbar.rs` · jetstream `packages/jetstream/preview/src/specimens/toolbar.rs`

## Contract ↔ Svelte

Contract §8 token values and §6/§9 ARIA notes drift from the authoritative Svelte source. Svelte is authoritative — update the contract.

- [x] FIXED `gap`: contract §8 said `0.25rem`; Svelte base is `0.375rem` (`Toolbar.svelte:86`). Updated §8 + Tier-2 checklist to `0.375rem`.
- [x] FIXED `padding`: contract §8 said `0.25rem` (uniform); Svelte base is `0.25rem 0.375rem` (`Toolbar.svelte:87`). Updated §8 to `0.25rem 0.375rem` (block 0.25, inline 0.375).
- [x] FIXED Size variants: Svelte defines per-size padding/gap for xs/sm/lg/xl (`Toolbar.svelte:94-97`). Added the full size table to contract §8.
- [x] FIXED Density variants: Svelte overrides only `padding-inline` + `gap` for compact/comfortable (`Toolbar.svelte:100-101`). Added the density table to contract §8 (block padding untouched, per density contract).
- [x] FIXED `aria-orientation`: §6 vs §9 contradiction reconciled — Svelte sets only `data-orientation`/`data-size`/`data-density` + `aria-label` (`Toolbar.svelte:54-60`), never `aria-orientation`. §9 now states `aria-orientation` is not currently set by Svelte; the contract still requires it for a11y (§6, not weakened — Svelte-side gap).
- [x] FIXED `alignment` prop: Svelte has no `alignment` prop and the contract §3 correctly omits it. No contract change — `ToolbarSpec.alignment` is a Rust-only invention to be dropped/landed in code (out of contract scope).

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] accepted (runtime limit): no roving focus / arrow-key navigation — contract §6 Tier-1 requires ArrowLeft/Right, ArrowUp/Down, wrapping, Tab-as-single-stop. GPUI `Toolbar` is `IntoElement` (render-only), consistent with the other GPUI primitives; focus is owned by the host view. Tracked as a behavioral parity gap, not a token/visual one.
- [x] FIXED Token violation: size/density padding+gap now resolve from the contract rem scales (`presentation::toolbar_pad_block_rem` / `toolbar_pad_inline_rem` / `toolbar_gap_rem` + density overrides), not the inline `(0.25, 0.375, 0.375)` literal table. Chrome tokens via `bg_token()`/`border_token()`/`radius_token()`.
- [x] noted (Rust-only): `has_separator` adds `border_b_2()` — a convenience with no contract basis. Kept (used by callers/specimens), explicitly flagged here rather than removed, to avoid churn outside the contract.
- [x] noted (Rust-only): `alignment` branch renders `justify_*` for a prop the contract does not define. Kept as a Rust-only convenience (drives the Jetstream specimen), flagged here.
- accepted: no ARIA (gpui has no accessibility API) — `role="toolbar"` / `aria-orientation` / `aria-label` not emitted.
- note: color-mix replicated correctly via alpha-scaling (`panel*0.94`, `border*0.78`); vertical orientation → `flex_col`.

## Jetstream gap (vs Svelte + contract)

- [x] FIXED Background fill — `tint(panel, 0.94)` (`color-mix(panel 94%, transparent)`) via `bg_token()`. Probe test asserts the resolved fill.
- [x] FIXED Container border — full `border_1()` (border-subtle 78% via `tint(border, 0.78)`) on all sides, no longer gated on `has_separator`.
- [x] FIXED Border-radius — `rounded(resolve_radius(radius_token()))` = `radius.surface` (new additive `ToolbarSpec::radius_token()`).
- [x] FIXED Vertical padding/height — dropped `min_h(control_height)`; now uses block padding `pt(pad_v)`/`pb(pad_v)` from `toolbar_pad_block_rem` so the toolbar auto-sizes to content (Svelte `inline-flex`). Inline padding + gap from the size/density scales.
- [x] FIXED `orientation` handling — vertical → `flex_col().items_stretch()`, horizontal → `flex_row().items_center()`. Probe test asserts vertical stacking. Roving focus/arrow-keys remain a runtime-owned behavioral gap (preview event loop), as in GPUI.
- accepted: no ARIA channel (`role`/`aria-label` not emitted).
- note: padding/gap now resolved from the contract size/density rem scales (`toolbar_*_rem`), mirroring GPUI exactly.

## Specimen parity

- Svelte covers: Formatting toolbar (ghost icon buttons + separator), Actions toolbar (secondary + separator + primary), Sizes snippet, Densities snippet (`ToolbarSpecimen.svelte`).
- GPUI covers: Horizontal (ghost B/I/U + separator + align arrows), With primary action (Discard/Save draft + separator + Publish), **Vertical orientation** (added), Sizes, Densities, click-action feedback. Builds clean. **GPUI specimen complete** — start/middle/end item groups separated by real `Separator`s, all items real `Button`s inside a real `Toolbar`. Uses glyph labels (`B`/`I`/`U`/arrows) instead of real icon buttons because the GPUI icon registry lacks `bold`/`italic`/`align-*` SVGs (icon-registry gap, not a contract gap); a labeled real `Button` is the correct non-fake fallback there.
- Jetstream covers (rewritten): **Formatting toolbar** (real `js_icon_button` Bold/Italic/Underline + `js_separator` + Align left/center/right), **Actions toolbar** (secondary text buttons + separator + primary Publish), **Sizes** (xs–xl), **Densities** (compact/default/comfortable), **Vertical orientation**. All items compose real primitives (`js_icon_button`/`js_button`/`js_separator`) inside real `js_toolbar`; no hand-rolled boxes, no Rust-only `alignment` groups. **Build NOT verified** — external `jetstream-renderer`/`jetstream-platform` (sibling repo) fails to compile against a newer `wgpu` (`wgpu::Maintain` removed, `BindingType::AccelerationStructure` signature change, `request_adapter` now returns `Result`); every error is under `/Dev/projects/jetstream/crates/`, none Poodle-side. Re-verify once the sibling finishes its wgpu migration.

## Notes

- Biggest `consv=gap` driver: contract §8 `gap`/`padding` literals are stale vs Svelte (`0.375rem` gap, `0.25rem 0.375rem` padding), and the size/density variant tables are undocumented.
- `alignment` is a three-target Rust invention with no Svelte/contract counterpart — decide whether to promote it to the contract or remove it; it currently shapes the Jetstream specimen.
- Roving focus is the dominant cross-target behavioral gap: neither Rust target implements the arrow-key/Tab roving pattern that is Tier-1 strict parity in the contract.
