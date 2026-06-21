<!-- parity consv=ok gpui=0 jetstream=0 specimen=ok -->
<!-- pass 41: both Rust targets closed. Added additive EmptyStateSpec.density
     (ControlDensity) + density-aware layout_gap_token(). GPUI: density-aware
     vertical padding (compact→stack.lg / default→panel_y*1.5 / comfortable→
     panel_y*2) and compact message font 0.75rem (was fixed body_size). Jetstream:
     dashed-border-approx (solid; JsEl has no dashed) + variant root tint
     (neutral surface@76% / search accent@7% / firstRun success@7%) + radius
     (surface-0.125rem) + density-aware vertical padding (was fixed 2rem). Probe
     tests added (icon+title+message, variant icon/tint, actions, compact font).
     Remaining: ARIA (accepted, no a11y API); JsEl dashed border (approx solid). -->
# Parity: EmptyState

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/empty-state.md`
- Svelte (authoritative): `packages/svelte/components/src/EmptyState.svelte`
- GPUI: `packages/gpui/components/src/composites/empty_state.rs`
- Jetstream: `packages/jetstream/components/src/empty_state.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/EmptyStateSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/empty_state_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/empty_state.rs`

## Contract ↔ Svelte

Svelte implements the full contract — props (`title`, `message`, `variant`, `size`, `density`, `ariaLabel`) + snippets (`visual`, `actions`) with matching defaults (`EmptyState.svelte:10-30`). Anatomy (`<section aria-label>` root, `aria-hidden` visual + variant icon, copy with `<h3>` + optional `<p>`, optional actions), variant background tints (76%/7%/7%), variant icons (inbox/search/plus), compact size, density compact/comfortable, and ARIA (`aria-label` fallback to title, `aria-hidden` visual, `<h3>` heading) all match. consv=ok.

## GPUI gap (vs Svelte + contract)

All static-render gaps closed. GPUI honors variant (dashed border, root radius,
per-variant tint, default visual circle + icon), composes real Buttons, wires
`on_action`, resolves message max-width from rem, and (pass 41) scales the message
font with compact (0.75rem) and applies density-aware vertical padding.

- accepted: no ARIA (gpui has no accessibility API) — `<section>` aria-label, `aria-hidden`, heading role. The visual circle is decorative; the textual message/action labels carry the semantic core.
- note: JsEl/GPUI render uses a real dashed border (`border_dashed`).

## Jetstream gap (vs Svelte + contract)

All static-render gaps closed (pass 41): root now carries border + variant tint
(neutral surface@76% / search accent@7% / firstRun success@7%) + radius
(surface-0.125rem), and vertical padding is density-aware (was a fixed 2rem). Real
`js_button` composition, variant icon selection, and compact font/icon scaling were
already present.

- accepted: no ARIA (no `<section>`/aria-label, no `aria-hidden`, no heading).
- note: JsEl has no dashed-border style — the root border is rendered solid (visual approximation of the contract's dashed border). All other border properties (width, color) match.
- note: no custom `visual` slot equivalent (Jetstream has no snippet channel); default variant icon always used.

## Specimen parity

- Svelte covers: Neutral (primary action), Search (secondary action), First run (no actions), Compact + custom `visual` snippet. 4 cases incl. visual override.
- GPUI covers: Neutral, Search (aria-label), First run, Compact (4 groups). Note: Search/FirstRun render identically to neutral (variant ignored), so variant differentiation is not actually visible — but state set is complete.
- Jetstream covers: Neutral, Search, FirstRun, Single action, Multiple actions, Compact+search, Compact+message (7 groups). Richest specimen.

specimen=ok: every target's specimen exercises the contract's neutral/search/firstRun/compact/actionable states; Jetstream exceeds Svelte. (Quality caveat: GPUI variants render uniformly because the component ignores `variant`, but the specimen wiring itself covers the states.)

## Notes

- GPUI is the weakest target: ignores `variant` entirely (no tint, no default icon), no border/background/circular-visual/aria, dead `on_action`, hand-rolled buttons with raw px. It barely resembles the contract surface.
- Jetstream is close — main functional gaps are the dashed border + variant background tint + density + aria; its 2rem fixed padding also breaks density.
- The variant background tint (76% neutral / 7% search / 7% firstRun) and dashed border are absent in BOTH Rust targets — the defining visual of this component.
