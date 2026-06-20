<!-- parity consv=ok gpui=9 jetstream=10 specimen=gap -->
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

- [ ] Root padding hardcoded `px(rem_to_px(1.0))` / `px(rem_to_px(0.875))` at `nav_card.rs:193-194` — does not use `NavCardSpec` methods and uses the wrong padding (Svelte default is `0.625rem` y + `space-panel-x` token, not `0.875 1.0`). Resolve padding-x from `space.panel-x` token and padding-y from a density-aware spec method.
- [ ] Content gap `px(rem_to_px(0.25))` at `nav_card.rs:163` — Svelte/contract §8 Content gap is `0.125rem`. Wrong value, hardcoded literal. Use `0.125rem` via spec.
- [ ] No `density` support — spec has no density field/methods; root gap `rem_to_px(0.75)`, icon `rem_to_px(2.0)`, content/title gaps are all fixed `default`-density literals (`nav_card.rs:126-128,137,192`). Add density prop + per-density resolution for root gap, padding-y, icon size, content gap, title gap.
- [ ] Icon size hardcoded `w/h(px(rem_to_px(2.0)))` and radius `rounded(px(rem_to_px(0.5)))` at `nav_card.rs:126-128` — bypasses spec; not density-aware. Resolve icon box from a spec icon-size method and radius from `radius.control` token (Svelte uses `--poodle-radius-control`, not a raw `0.5rem`).
- [ ] Badge pill radius hardcoded `rounded(px(9999.0))` at `nav_card.rs:150` — use a pill/full-radius token, not raw `9999.0`.
- [ ] Badge/title gap `gap(px(rem_to_px(0.5)))` at `nav_card.rs:137` — Svelte title gap is `0.375rem` (`--poodle-nav-card-title-gap`), GPUI uses `0.5rem`. Wrong value; resolve from title-gap spec method.
- [ ] Badge font-weight `FontWeight::BOLD` at `nav_card.rs:153` — Svelte badge weight is `600` (MEDIUM/semibold), title is `600`; GPUI title uses BOLD too (line 141). Align both to weight 600.
- [ ] Arrow has no hover reveal — opacity pinned `0.0` (`nav_card.rs:181`); comment at 221-225 admits root-hover child-opacity isn't wired. Arrow never appears. Implement hover→arrow opacity 1 (contract §8 Arrow root-hover).
- [ ] No `href`/link-vs-button distinction in rendered element — `href` stored on spec but root is always a `div` with `on_click`; disabled correctly suppresses click but link semantics absent. Contract §6 / Tier-1 "renders as link when href provided".
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored on spec, not emitted.
- accepted: arrow opacity animation approach is platform-owned (contract §12 Known Delta) — but the reveal itself must still work (see arrow todo above).

## Jetstream gap (vs Svelte + contract)

- [ ] Content gap uses `spec.content_gap_rem()` which returns `0.25` (`contracts/.../nav_card.rs:131-133`) — Svelte/contract §8 Content gap is `0.125rem`. Spec value is wrong; fix `content_gap_rem()` to `0.125` (also fixes GPUI if it adopts the method).
- [ ] Root padding via `padding_x_rem()=1.0` / `padding_y_rem()=0.875` (`contracts/.../nav_card.rs:121-128`) — Svelte default padding is `0.625rem` y + `space-panel-x` token x, not `0.875 1.0`. Correct the spec values (padding-x should resolve `space.panel-x`, not a fixed rem).
- [ ] No icon slot rendered — `js_nav_card` never emits a `__icon` region (`nav_card.rs:34-86`); contract §2 marks icon optional but the specimen/anatomy expect the snippet region. Add accent-tinted `2rem` icon box.
- [ ] No arrow rendered — contract §2 marks Arrow **required**; `js_nav_card` emits no arrow svg/glyph. Add arrow with hover opacity reveal.
- [ ] Root is `flex_col` (`nav_card.rs:39`) — Svelte/contract root is `flex` row (icon | content | arrow). Current layout stacks title/desc only, no row composition. Restructure to row with content column nested.
- [ ] No `density` support — spec has no density field; title gap, badge dims fixed at `default` literals. Add density-aware sizing.
- [ ] No focus ring — `.focusable()` set (`nav_card.rs:42`) but no focus border/ring color applied. Contract §8 Root focus = `border-width-focus` + `accent-focusRing`. Wire focus visual.
- [ ] No hover border-color change — hover only swaps `bg` (`nav_card.rs:41`); Svelte hover also sets `border-color: color-mix(accent 28%, border-subtle)`. Add hover border.
- [ ] Hover fill uses `fill.mix(elevated, 0.92)` at `nav_card.rs:31-32` — Svelte is `color-mix(elevated 52%, surface)` i.e. mix ratio 0.52, not 0.92. Wrong blend factor.
- [ ] No `href`/link semantics and no `onClick` wiring in component — `onClick` is not on the spec at all; interaction (if any) lives in preview event loop. Contract §5 callback unrepresented.
- accepted: no ARIA channel (`aria_label` stored on spec, not surfaced).
- accepted: click handling may live in preview `main.rs` event loop rather than the component.

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
