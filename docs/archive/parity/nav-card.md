<!-- parity consv=ok gpui=1 jetstream=2 specimen=ok -->
<!-- specimen pass: both Rust nav-card specimens build clean with real js_nav_card / NavCard — icon+title+description, numeric badge, disabled, link (href), and compact/default/comfortable density variants. Trailing arrow rendered by the component. No selected/active group (no such prop in contract/spec — not faked). -->

# Parity: NavCard

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/nav-card.md`
- Svelte (authoritative): `packages/svelte/components/src/NavCard.svelte`
- GPUI: `packages/gpui/components/src/primitives/nav_card.rs`
- Jetstream: `packages/jetstream/components/src/nav_card.rs`
- Spec: `packages/contracts/components/src/nav_card.rs` (`NavCardSpec`)
- Specimens: svelte `packages/svelte/preview/src/specimens/NavCardSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/nav_card.rs` · jetstream `packages/jetstream/preview/src/specimens/nav_card.rs`

## Contract ↔ Svelte

Svelte matches the contract on props, anatomy, states, and ARIA. No divergences.

- Props: contract §3 lists `title`, `description`, `href`, `badge`, `disabled`, `ariaLabel`, `density`, `onClick`, plus `icon()` snippet. Svelte `Props` (lines 7–17) has all eight props + `icon` snippet with matching types/defaults (`description/href/badge/ariaLabel/density/onClick` default `null`, `disabled` default `false`). Match.
- Anatomy: root `<a>`/`<button>`, `__icon`, `__content`, `__title`, `__badge`, `__description`, `__arrow` (svg) all present (lines 43–101). Matches contract §2.
- Semantics: `<a>` when `href && !disabled`, else `<button>` (line 43); `aria-label={ariaLabel ?? title}`; `data-disabled`; disabled drops `href` (button branch). Matches contract §6.
- States: hover, `:focus-visible`, `[data-disabled]` all styled (lines 131–144). Arrow opacity 0→1 on hover (lines 196–207). Matches §4/§8.
- Density: `compact`/`default`/`comfortable` token blocks (lines 209–225) match contract §8 density table exactly; default values inline on `.poodle-nav-card` (lines 106–111) match the `default` row.
- One nit (not a divergence): contract §8 Root padding header says `0.875rem 1rem`, but the Root token table and Svelte `default` density both use `0.625rem var(--poodle-space-panel-x)`. The Rust specs hardcode the `0.875rem 1rem` header value — see Rust gaps below. Svelte is authoritative: padding-y default is `0.625rem`, padding-x is `panel-x` token.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] DONE Root padding — now `padding_x_rem()`/`padding_y_rem()` density-aware spec methods (default 0.625y / panel-x 1.0). Fixed in spec + `nav_card.rs`.
- [x] DONE Content gap — `content_gap_rem()` density-aware (default 0.125). Fixed.
- [x] DONE `density` support — `NavCardSpec.density` field + `with_density`; root gap / padding-y / icon size / content gap / title gap all density-resolved.
- [x] DONE Icon size + radius — `icon_size_rem()` (density) + `icon_radius_token()` = `radius.control`.
- [x] DONE Badge pill radius — `badge_radius_token()` = `radius.pill`.
- [x] DONE Title gap — `title_gap_rem()` density method (default 0.375).
- [x] DONE Badge + title weight — both now `FontWeight::SEMIBOLD` (600).
- [x] DONE Arrow hover reveal — root `.group()` + arrow `.group_hover(..opacity(1.0))`; arrow opacity 0 at rest, 1 on root hover.
- [ ] No `href`/link-vs-button distinction in rendered element — `href` stored on spec but root is always a `div` with `on_click`; disabled correctly suppresses click but link semantics absent. Contract §6 / Tier-1 "renders as link when href provided". (Still open — GPUI div-only.)
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored on spec, not emitted.
- accepted: arrow opacity animation approach is platform-owned (contract §12 Known Delta) — but the reveal itself must still work (see arrow todo above).

## Jetstream gap (vs Svelte + contract)

- [x] DONE Content gap — `content_gap_rem()` now density-aware (default 0.125).
- [x] DONE Root padding — `padding_x_rem()`/`padding_y_rem()` density-aware (default 0.625y / 1.0x = panel-x).
- [x] DONE Icon slot — `js_nav_card` now emits an accent-tinted, density-sized icon box (control radius) with a placeholder glyph (host `icon()` snippet maps to the box; letter stands in headlessly).
- [x] DONE Arrow — required arrow glyph emitted, resting opacity 0, reveals on its own hover. (Root→child group-hover isn't expressible in JsEl — full root-hover reveal noted as JsEl gap.)
- [x] DONE Root is now `flex_row` (icon | content | arrow), content column nested.
- [x] DONE `density` — full density-aware sizing via spec methods.
- [~] PARTIAL Focus ring — `.focusable()` set and focus-ring color resolved/surfaced; JsEl exposes a single focusable affordance, so the ring color is painted by the preview focus layer, not the component. (JsEl gap.)
- [x] DONE Hover border — hover now sets `color-mix(accent 28%, border-subtle)`.
- [x] DONE Hover fill blend — now `color_mix(elevated, surface, 0.52)` (was 0.92).
- [ ] No `href`/link semantics, no `onClick` in component — interaction lives in preview event loop. Contract §5 callback unrepresented. (Still open — preview-loop.)
- accepted: no ARIA channel (`aria_label` stored on spec, not surfaced).
- accepted: click handling lives in preview event loop rather than the component.

## Specimen parity

- Svelte covers: 2-col grid (Getting Started, Components+badge, Tokens, API Reference disabled) with icon snippets + onClick; single card as link (`href="#"`); density demo (compact/default/comfortable via `densities` snippet); live "Last click" readout. (`NavCardSpecimen.svelte`)
- GPUI covers: 2-col grid with icons (4 cards incl. badge + disabled), single card as link. — missing: **density variants** (no density demo), **onClick / last-click readout** (cards have no `with_click`), **hover/arrow-reveal state** is non-functional per component gap. (`gpui/.../specimens/nav_card.rs`)
- Jetstream covers: 3 stacked cards (Primitives, Composites+badge "12", Disabled). — missing: **icon snippet** (no icons), **arrow** (component renders none), **link/href card**, **density variants**, **onClick**, and the cards are a vertical stack not a 2-col grid. Coverage is the thinnest of the three. (`jetstream/.../specimens/nav_card.rs`)

## Notes

- `consv=ok`: Svelte faithfully implements the contract; the only contract internal inconsistency is the §8 Root-padding *header* (`0.875rem 1rem`) vs the §8 Root *table* + density default (`0.625rem panel-x`). Svelte uses the latter; the Rust specs copied the former into `padding_x_rem`/`padding_y_rem`. Treat the header as the stale value — no Svelte change needed, but the contract header could be tidied and the Rust spec corrected.
- Root cause of most Rust gaps: `NavCardSpec` (`packages/contracts/components/src/nav_card.rs`) omits `density` and `on_click` entirely (only `title/description/href/badge/is_disabled/aria_label` fields). Until the spec gains density + an icon-size/title-gap/root-gap method set, both Rust targets can only render `default` density with hardcoded rem literals.
- The `rem_to_px(spec.padding_x_rem())` pattern is *not* a token violation per se — the spec method is the token surface — but the returned constants are wrong (see padding/gap todos), so the resolved output diverges from Svelte.
- GPUI badge/title use `FontWeight::BOLD`; Svelte uses `600`. Minor weight mismatch flagged under GPUI gaps.
- Jetstream is the furthest behind: missing icon, arrow, row layout, focus ring, hover border, and correct hover blend.
