<!-- parity consv=gap gpui=5 jetstream=6 specimen=gap -->
# Parity: Spinner

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/spinner.md`
- Svelte (authoritative): `packages/svelte/components/src/Spinner.svelte`
- GPUI: `packages/gpui/components/src/primitives/spinner.rs`
- Jetstream: `packages/jetstream/components/src/spinner.rs`
- Spec: `packages/contracts/components/src/spinner.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/SpinnerSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/spinner.rs` · jetstream `packages/jetstream/preview/src/specimens/spinner.rs`

## Contract ↔ Svelte

Prop set matches (`variant`/`size`/`sizeRole`/`density`/`tone`/`ariaLabel`, all defaults agree). Divergences are in animation timing and anatomy detail. Svelte is authoritative — update the contract.

- Grid animation duration: contract §8 grid table says `spinner-grid 1.12s linear infinite`. Svelte uses **`1.24s`** for every cell keyframe (`Spinner.svelte:147`). **Fix: change contract §8 to `1.24s`.**
- Grid idle baseline: Svelte cells start at `opacity: 0.2` with a `poodle-spinner-grid-idle` keyframe holding `0.2` (`Spinner.svelte:146-148, 180-185`). Contract §8 lists only "phase-specific opacity keyframes" and never states the rest/baseline opacity. **Fix: document the `0.2` idle floor in contract §8.**
- Grid keyframe peak opacity: Svelte phases peak at `0.76` (e.g. `tl` `Spinner.svelte:187-209`). Contract gives no opacity range. **Fix: record the `0.2 → 0.76` range in contract §8 so runtimes can match.**
- Anatomy nesting: contract §2 lists "6 `span` children" directly under Visual, but Svelte wraps the six `__cell` spans in a `poodle-spinner__grid` container span (`Spinner.svelte:59-66`) and the ring in a single `poodle-spinner__ring` span (`Spinner.svelte:57`). Contract's `[Visual]` row covers this loosely. **Fix: clarify §2 that ring/grid each have a wrapper span, cells are its children.**
- `aria-live`: contract §6 says "polite announcement behavior" in prose; Svelte emits `aria-live="polite"` only when `ariaLabel` is set (`Spinner.svelte:53`). Consistent — no change, noted for runtime reference.
- Passthrough props: Svelte accepts `class`, `style`, `...restProps` (`Spinner.svelte:13-25`). Not in contract §3. Common passthrough surface — optional contract note, low priority.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded cell radius literal `rem_to_px(0.125)` at `spinner.rs:126` — Svelte cell radius is `0.125rem` but should resolve from a spec token method (e.g. `SpinnerSpec::cell_radius_token()`), not a raw float; the spec exposes no such method (`packages/contracts/components/src/spinner.rs`). Add token + spec method.
- [ ] Hardcoded grid cell/gap size table at `spinner.rs:99-105` — five `(cell_size, gap)` rows of raw `rem_to_px(...)` floats. These are derived per-size constants with no spec/token backing; move into `SpinnerSpec` (`grid_cell_px()`/`grid_gap_px()`) so the magic rem origins live in one place.
- [ ] Grid color is flat `bg(color)` with animated opacity (`spinner.rs:122-152`) — acceptable approximation, but the opacity curve `0.2 + smooth * 0.56` (`spinner.rs:148`) hardcodes the `0.2` floor and `0.56` span; these come from Svelte's `0.2 → 0.76` keyframe range and belong on the spec, not as inline literals.
- [ ] Ring track + top-color highlight absent — GPUI rotates a single-color `spinner.svg` (`spinner.rs:81-95`); Svelte's ring is a `0.125rem` ring at `currentColor 24%` with a `currentColor` top arc (`Spinner.svelte:125-126`). Verify the SVG asset encodes the 24% track + bright arc, else ring reads as a solid spinning disc, not contract's two-tone ring.
- [ ] Ring size from `spec.size_px()` (`spinner.rs:80`) returns raw px constants `10/12/16/24/32` (`packages/contracts/components/src/spinner.rs:69-77`) — these are the rem sizes pre-multiplied; acceptable since centralized in spec, but flag that they bypass `rem_to_px` and assume a fixed root font size.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored on spec but `role="status"`/`aria-live` not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] No rotation animation — `build_ring` (`spinner.rs:83-101`) renders a static bordered circle; comment at `spinner.rs:96-97` admits rotation is "runtime-level". Spin must be driven by the preview event loop or a runtime animation hook; currently the ring never moves. Wire animation in preview main loop or note as accepted runtime delta.
- [ ] No grid opacity animation — `build_grid` uses six **static** descending opacities `[1.0, 0.85, 0.7, 0.55, 0.4, 0.25]` (`spinner.rs:126`), not Svelte's animated snake. Cells are frozen. Animate via runtime hook or accept as static delta.
- [ ] Hardcoded ring border width literal `rem_to_px(0.125)` at `spinner.rs:85` — resolve from a spec token method, not raw `0.125`.
- [ ] Hardcoded track alpha `tone_color.a * 0.24` at `spinner.rs:88` — the `0.24` is Svelte's `currentColor 24%` mix; move to a spec/token constant, drop the inline magic number.
- [ ] Ad-hoc grid gap heuristic `(width * 0.1).max(rem_to_px(0.0625))` at `spinner.rs:119` — Svelte has explicit per-size gap values (`0.0625`–`0.15625rem`, `Spinner.svelte:91-119`); replace the `* 0.1` guess + cell math (`spinner.rs:121-122`) with the real per-size table via spec.
- [ ] Hardcoded cell radius literal `rem_to_px(0.125)` at `spinner.rs:115` and static opacity ramp `spinner.rs:126` — radius needs a token method; the `[1.0..0.25]` ramp hardcodes a non-contract curve (Svelte floor is `0.2`, peak `0.76`).
- accepted: tone `Current` resolves to `color.text.primary` (`spinner.rs:65`) rather than literal inherited `currentColor` — Jetstream has no CSS inheritance; same approximation GPUI uses.
- accepted: no ARIA channel for `aria_label`.

## Specimen parity

- Svelte covers: Ring (5 sizes), CLI grid (5 sizes with mixed muted/accent/current tones), Context tones (current on inverse chip, accent chip, muted grid chip) (`SpinnerSpecimen.svelte`).
- GPUI covers: Ring (5 sizes), CLI grid (5 sizes, tones muted/current/accent/current/current), Context tones (inverse/accent/muted chips). — matches Svelte closely; the only gap is the unused `sizes`/`densities` snippets Svelte defines but does not render, so effectively at parity.
- Jetstream covers: Variants (ring + grid, md only), Sizes (5, ring only), Tones (3, ring only). — missing: **grid variant at multiple sizes**, **mixed-tone grid row**, and the **Context tones inverse-chip / accent-chip / muted-grid-chip** grouping. Grid is shown once at md; Svelte exercises grid across all 5 sizes and 3 tones.

## Notes

- `consv=gap` driver: contract §8 grid duration (`1.12s`) is stale vs Svelte (`1.24s`), and the grid opacity range (`0.2`–`0.76` floor/peak) is undocumented. Both block runtimes from matching motion.
- Spec struct (`packages/contracts/components/src/spinner.rs`) exposes only `size_px()` and `tone_color_token()`. It has **no** token methods for ring border width, cell radius, grid cell/gap sizes, or the opacity range — so every Rust runtime is forced into hardcoded literals. Adding those spec methods is the root fix for most GPUI/Jetstream todos above.
- Jetstream ring + grid are structurally correct but motionless; GPUI animates both. The biggest single behavior gap is Jetstream having no spin/pulse at all.
- GPUI grid is a hand-rolled 3-row × 2-col flex stack (`spinner.rs:107-156`) rather than a true grid, but visually matches the 2×3 contract layout.
