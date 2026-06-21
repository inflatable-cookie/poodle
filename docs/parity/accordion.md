<!-- parity consv=fixed gpui=2 jetstream=2 specimen=gap -->
<!-- pass 48: both targets. Additive AccordionSpec token methods (gaps/bg/border/inset). GPUI:
     item bg color_mix(elevated, panel, 0.40) (was wrong surface/text mix), all raw float literals
     → spec token methods. Jetstream: item bg from token (was hand-blended Vec4 average), chevron
     from icon registry (was unicode ▾/▸), description renders in collapsed summary (was hidden),
     title weight 700, density padding, expanded panel region. 3 probe tests; specs 61, gpui clean.
     Toggle = preview-loop; JsEl inset-highlight no box-shadow. -->
# Parity: Accordion

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/accordion.md`
- Svelte (authoritative): `packages/svelte/components/src/Accordion.svelte`
- GPUI: `packages/gpui/components/src/primitives/accordion.rs`
- Jetstream: `packages/jetstream/components/src/accordion.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/AccordionSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/accordion.rs` · jetstream `packages/jetstream/preview/src/specimens/accordion.rs`

## Contract ↔ Svelte

Svelte and contract diverge on item geometry and tokens. Svelte is authoritative for resolved values; contract values are stale.

- [x] FIXED Item padding: contract §8 said `0.875rem 1rem`; updated to Svelte's `0.625rem var(--poodle-space-panel-x)` (`Accordion.svelte:144`). §7 sizing + Tier-2 checklist line also updated.
- [x] FIXED Item background: contract §8 said `color-mix(surface 93%, text-primary)`; updated to Svelte's `color-mix(background-elevated 40%, background-panel)` (`Accordion.svelte:147`). Tier-2 checklist "84% elevated" line corrected to "40% background-elevated, background-panel".
- [x] FIXED Item gap: contract §8 item `gap` was `0.75rem`; updated to `var(--poodle-accordion-item-gap)` (`0.625rem` default, `Accordion.svelte:140`). Root gap corrected to `space-stack-md` (`:135`); §7 sizing notes both.
- [x] FIXED Summary gap: contract §8 said `0.3125rem`; updated to `var(--poodle-space-inline-sm)` token (`Accordion.svelte:183`).
- [x] FIXED Trigger gap: contract §8 said `0.75rem`; updated to `var(--poodle-space-inline-md)` token (`Accordion.svelte:159`).
- [x] FIXED Indicator: Svelte renders `<Icon name="chevron-down" />` with no `size` prop (`Accordion.svelte:112`); dropped `size="sm"` from anatomy §2 and §9. Indicator wrapper-span glyph rules (code-family, 0.75rem, rotate 180deg) retained — they style the span, not the Icon.
- Panel content placement: anatomy §2 + §4 say the description lives inside the panel/summary and panel renders `children(item, isOpen)`. Svelte description renders **inside the trigger summary** (`:108-110`) and the panel renders only the `children` snippet (`:124`). Matches anatomy. GPUI/Jetstream diverge (see below).
- Density: contract §8 size table covers size only; Svelte density overrides `padding-inline` (`:284`, `:288`) and `--accordion-item-gap`, never vertical padding — compliant with the size/density orthogonality rule. **No fix; note Svelte is the density reference.**

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded vertical padding literal `px(rem_to_px(0.625))` at `accordion.rs:137` — resolve item block padding from a token, not a raw `0.625`.
- [ ] Hardcoded inset-highlight offset literal `px(rem_to_px(0.0625))` at `accordion.rs:286` — Svelte's `0.0625rem` inset shadow offset should come from a border-width token, not a raw float.
- [ ] Item background uses `color_mix(surface_bg, text_primary, 0.93)` at `accordion.rs:157` — Svelte is `color-mix(background-elevated 40%, background-panel)` (`Accordion.svelte:147`). Wrong source colors and ratio; recolor to elevated/panel blend.
- [ ] Indicator rotation faked by swapping `chevron-up`/`chevron-down` at `accordion.rs:242-246` — contract requires a single chevron-down rotated 180deg with `motion-duration-interaction` transition; no rotation animation is applied.
- [ ] Density padding hardcoded in match arms `0.5 / 1.0 / 1.0` at `accordion.rs:131-135` — resolve from a density-aware space token instead of inline floats.
- [ ] Panel renders only injected `with_content`; the per-item `description` is shown in the trigger (`accordion.rs:227-235`) which matches Svelte, but there is no `panel` region wrapper carrying the trigger/panel id association — `aria-controls`/`role="region"` linkage absent (also see accepted ARIA note).
- accepted: no ARIA (gpui has no accessibility API) — `aria_expanded`, `aria-controls`, `role="region"`, group role in multiple mode all unexpressed.
- accepted: panel slide/height animation (~180ms) not implemented — GPUI has no equivalent transition primitive here.

## Jetstream gap (vs Svelte + contract)

- [ ] Item background hand-blended via `glam::Vec4::new(...)` at `accordion.rs:78-83` — manual `surface*0.5 + elevated*0.5` averaging is wrong source AND wrong math vs Svelte's `color-mix(elevated 40%, panel)`; use a `color_mix`/`tint` helper on elevated+panel.
- [ ] Chevron glyph hardcoded as unicode `"▾"`/`"▸"` at `accordion.rs:86` — contract indicator is `<Icon name="chevron-down">` rotated; use the icon registry, not literal glyphs.
- [ ] Chevron font-size heuristic `title_font_size * 0.85` at `accordion.rs:92` — contract indicator font-size is `0.75rem`; resolve from a token, drop the `* 0.85` fudge.
- [ ] Panel vertical padding heuristic `pad_y * 0.85` at `accordion.rs:129` — invent a real panel-padding token; no `* 0.85` magic.
- [ ] Description rendered ONLY inside the expanded panel (`accordion.rs:124-136`) — Svelte renders description inside the trigger summary always-visible (`Accordion.svelte:108-110`); collapsed items lose their description text entirely. **Move description into the trigger row.**
- [ ] Summary has no title/description stacking with `space-inline-sm` gap — trigger is a flat row with title + chevron only (`accordion.rs:95-112`); no summary container matching Svelte anatomy.
- [ ] Item internal gap not applied — trigger and panel are direct flex_col children with no `item_gap_token` gap between them inside the card (`accordion.rs:115-121`).
- [ ] Inset top-highlight box-shadow (`text-inverse 8%`) from contract §8 / Svelte `:148` is absent — item card has border+bg but no inset shadow.
- accepted: no ARIA channel (`aria-expanded`/`aria-controls`/`role=region`/group role) — documented platform limit.
- accepted: interaction (toggle/click handler) lives in the preview event loop, not `js_accordion`; the component is render-only.

## Specimen parity

- Svelte covers: Single selection (3 items, first open), Multiple selection (2 of 3 open), Sizes (xs–xl via `sizes` snippet), Densities (via `densities` snippet), panel `children` slot content (`AccordionSpecimen.svelte`).
- GPUI covers: Single selection (interactive toggle), Multiple selection (interactive toggle), panel content via `with_content`. — missing: **Sizes group**, **Densities group**, **disabled item** state.
- Jetstream covers: Single selection, Multiple selection, "All collapsed" group. — missing: **Sizes group**, **Densities group**, **disabled item** state; also items carry descriptions Svelte items do not, so visual content diverges.

## Notes

- Spec (`packages/contracts/components/src/accordion.rs`) has no `panel_padding`/`item_padding` token methods — both Rust targets hardcode the `0.625rem` block padding. Add token methods so neither target needs literals.
- `item_gap_token()` and `trigger_gap_token()` both return `SPACE_STACK_SM` (`accordion.rs:88-94`); Svelte uses `space-stack-md` for the root gap and a separate `0.625rem` for item internal gap — the spec token does not distinguish root gap from item gap.
- No pending/disabled-per-item specimen in any Rust target despite `is_disabled` existing on `AccordionItemSpec`; both Rust impls handle disabled (`gpui:202`, `jetstream:140`) but no specimen exercises it.
- The big `consv=gap` driver is stale contract §8 values (padding/background/gap) that no longer match the authoritative Svelte CSS.
