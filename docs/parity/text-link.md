<!-- parity consv=ok gpui=4 jetstream=8 specimen=gap -->
# Parity: TextLink

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/text-link.md`
- Svelte (authoritative): `packages/svelte/components/src/TextLink.svelte`
- GPUI: `packages/gpui/components/src/primitives/text_link.rs`
- Jetstream: _missing_ — no `packages/jetstream/components/src/text_link.rs`, no `js_text_link` export in `lib.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/TextLinkSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/text_link.rs` · jetstream _missing_

## Contract ↔ Svelte

Svelte matches the contract on every prop, anatomy part, state, and ARIA rule. No divergence.

- All 9 props present with matching types/defaults: `href`/`target`/`rel`/`ariaLabel` (`null`), `disabled` (`false`), `tone` (`"accent"`), `className` (`""`), `onClick` (`null`), `children` (`TextLink.svelte:4-26` vs contract §2). ✔
- Render branching matches §3: `<a>` when `href && !disabled`, else `<button type="button">`; disabled always renders the button path (`TextLink.svelte:40-63`). ✔
- Disabled activation: `preventDefault()` + skips `onClick` (`TextLink.svelte:30-37`) per §3. ✔
- Tone CSS matches §4: accent → `--poodle-color-accent-base`, inherit → `inherit`/currentColor, secondary → `--poodle-color-text-secondary` (`TextLink.svelte:71,82-88`). ✔
- Underline + hover/focus strengthening + focus-visible ring (`--poodle-border-width-focus` solid `--poodle-color-accent-focusRing`) + disabled opacity (`--poodle-state-opacity-disabled`) all match §4 (`TextLink.svelte:76-104`). ✔
- ARIA: anchor keeps native link semantics, button keeps native button semantics, `aria-label` forwarded both paths (`TextLink.svelte:47,58`) per §5. ✔
- Minor (not a contract bug): underline geometry literals `text-decoration-thickness: 0.0625rem` / `text-underline-offset: 0.125rem` (`TextLink.svelte:78-79`) and `outline-offset: 0.125rem` / `border-radius: 0.125rem` (`TextLink.svelte:97-98`) are raw rem, not tokens — contract §4 specifies no token for these, so accepted on the authoritative side. Note if a token table is later added.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] `tone="inherit"` resolves to `color.text.primary` instead of `currentColor`/`inherit` — `TextLinkSpec::color_token()` maps `Inherit => "color.text.primary"` (`text_link.rs` spec at `packages/contracts/components/src/text_link.rs:55`), but Svelte/contract §4 use `inherit`/currentColor. Picks up primary text color rather than inheriting the surrounding run.
- [ ] No underline strengthening on hover/focus — Svelte mixes underline to 55% currentColor at rest and to full currentColor on hover/focus-visible (`TextLink.svelte:77,90-93`); GPUI calls bare `.underline()` (`text_link.rs:51`) with no rest-vs-hover decoration-color distinction.
- [ ] No anchor semantics / `href`/`target`/`rel`/`ariaLabel` rendering — builder always emits a `div` with a click handler (`text_link.rs:44-64`); spec carries `href`/`target`/`rel`/`aria_label` but none are read. Render-as-anchor (§3) is not represented.
- [ ] Disabled does not gate to a button-only path or block `onClick` wiring beyond the `!disabled` guard — guard exists (`text_link.rs:58-62`) but there is no anchor-vs-button distinction to gate in the first place (consequence of the div-only render above).
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored on spec but not emitted.
- accepted: focus ring uses shared `focus_ring_shadow` helper whose `spread_radius: px(2.0)` and `* 0.28` alpha are literals (`theme_ext.rs:70,75`); shared across all primitives, not a text-link-local violation. `text_link.rs` itself has zero hardcoded px/color/opacity literals (color, opacity, focus-ring color all resolved via `resolve_color`/`resolve_opacity`, `text_link.rs:40-42`).

## Jetstream gap (vs Svelte + contract)

Jetstream has no TextLink at all — the dominant gap. Confirmed: no `text_link.rs` in `packages/jetstream/components/src/`, no `js_text_link`/`TextLink` symbol in `lib.rs`, no specimen, no specimen `mod` entry.

- [ ] Create `packages/jetstream/components/src/text_link.rs` with `js_text_link(spec, theme)`.
- [ ] Export `js_text_link` from `packages/jetstream/components/src/lib.rs`.
- [ ] Render anchor-vs-button branch per §3 (`href && !disabled` → anchor semantics, else button).
- [ ] Resolve text color from `TextLinkSpec::color_token()` for all three tones; map `inherit` to currentColor, not primary.
- [ ] Underline with rest 55%-currentColor mix + hover/focus strengthening to full currentColor (§4).
- [ ] Focus-visible ring from `border.width.focus` + `color.accent.focusRing` (§4).
- [ ] Disabled state: `state.opacity.disabled` + default cursor, `preventDefault`/skip `onClick` (§3-§4).
- [ ] Add `packages/jetstream/preview/src/specimens/text_link.rs` + register in `specimens/mod.rs` and the component registry.

## Specimen parity

- Svelte covers: Inline prose, Tones (accent/secondary/inherit), Button action, Disabled (anchor + action) — `TextLinkSpecimen.svelte:7-32`.
- GPUI covers: Tones (accent/secondary/inherit) via `with_href`, Button action, single Disabled action — `text_link.rs:6-40`. — missing: **Inline-prose** usage (link embedded in a `Text` run) and the **disabled anchor** case (only disabled button shown).
- Jetstream covers: nothing — no specimen exists. — missing: entire specimen.

## Notes

- `consv=ok`: contract and Svelte are fully aligned; nothing to reconcile on the authoritative surface.
- Biggest gap is Jetstream — whole component, export, and specimen absent.
- GPUI's `Inherit` tone mapping to `color.text.primary` is a real visual divergence (not currentColor); worth fixing in the shared `TextLinkSpec::color_token()` since the spec is consumed by both Rust targets, but only GPUI exercises it today.
