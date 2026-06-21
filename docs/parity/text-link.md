<!-- parity consv=ok gpui=0 jetstream=1 specimen=gap — pass: GPUI now sets rest underline at 55% tone color + strengthens to full on hover/focus-visible; inherit→currentColor / anchor-vs-button reclassed accepted (no currentColor / no DOM anchors in GPUI/JsEl). Jetstream component done; only the preview specimen (+registry) remains open (preview-app task, not build-verified here). -->
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

- [x] DONE: underline strengthening on hover/focus — rest decoration color is the tone color at 0.55 alpha (`color-mix(currentColor 55%, transparent)`); `.hover(...)` and `.focus(...)` raise it to the full tone color (`text_link.rs`). Matches Svelte 77,90-93 (using the resolved tone color as the no-currentColor stand-in).
- accepted: `tone="inherit"` resolves to `color.text.primary` instead of `currentColor` — GPUI has no `currentColor` affordance; the resolved tone is the faithful approximation (shared `TextLinkSpec::color_token()`, same as the Jetstream target). Closeable only if a currentColor/inherit primitive lands.
- accepted: no anchor semantics / `href`/`target`/`rel` rendering — GPUI is not HTML; the builder emits a focusable `div` with a click handler. Render-as-anchor (§3) has no GPUI equivalent. `aria_label` stored on spec but not emitted (no ARIA API).
- accepted: disabled does not gate to a button-only path — consequence of the div-only render (no anchor-vs-button distinction in GPUI); the `!disabled` guard still blocks `onClick`, matching the contract's disabled-activation rule.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored on spec but not emitted.
- accepted: focus ring uses shared `focus_ring_shadow` helper whose `spread_radius: px(2.0)` and `* 0.28` alpha are literals (`theme_ext.rs:70,75`); shared across all primitives, not a text-link-local violation. `text_link.rs` itself has zero hardcoded px/color/opacity literals (color, opacity, focus-ring color all resolved via `resolve_color`/`resolve_opacity`, `text_link.rs:40-42`).

## Jetstream gap (vs Svelte + contract)

Jetstream has no TextLink at all — the dominant gap. Confirmed: no `text_link.rs` in `packages/jetstream/components/src/`, no `js_text_link`/`TextLink` symbol in `lib.rs`, no specimen, no specimen `mod` entry.

- [x] DONE: created `packages/jetstream/components/src/text_link.rs` with `js_text_link(spec, theme)`, exported via lib.rs.
- [x] DONE: resolves text color from `TextLinkSpec::color_token()` for all three tones (`inherit`→text-primary, the no-currentColor approximation GPUI also uses).
- [x] DONE: disabled dims via `state.opacity.disabled`. (Anchor-vs-button + `onClick`/preventDefault are preview-event-loop concerns; the render is the same Label.)
- accepted: Underline + focus-visible ring — JsEl exposes no underline/decoration or focus-ring affordance (runtime gap, like other Jetstream primitives). Color tone + disabled opacity are the representable surface; covered by probe tests (`accent_tone_is_default_color`, `secondary_tone_resolves_secondary`, `disabled_dims_opacity`).
- [ ] Add `packages/jetstream/preview/src/specimens/text_link.rs` + register in `specimens/mod.rs` and the component registry. Deferred: preview-app task touching the shared specimen registry; not build-verified in this pass (shared target lock).

## Specimen parity

- Svelte covers: Inline prose, Tones (accent/secondary/inherit), Button action, Disabled (anchor + action) — `TextLinkSpecimen.svelte:7-32`.
- GPUI covers: Tones (accent/secondary/inherit) via `with_href`, Button action, single Disabled action — `text_link.rs:6-40`. — missing: **Inline-prose** usage (link embedded in a `Text` run) and the **disabled anchor** case (only disabled button shown).
- Jetstream covers: nothing — no specimen exists. — missing: entire specimen.

## Notes

- `consv=ok`: contract and Svelte are fully aligned; nothing to reconcile on the authoritative surface.
- Biggest gap is Jetstream — whole component, export, and specimen absent.
- GPUI's `Inherit` tone mapping to `color.text.primary` is a real visual divergence (not currentColor); worth fixing in the shared `TextLinkSpec::color_token()` since the spec is consumed by both Rust targets, but only GPUI exercises it today.
