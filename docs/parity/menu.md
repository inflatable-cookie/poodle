<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok pass=menu-both-targets-token-resolved -->
# Parity: Menu

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/menu.md`
- Svelte (authoritative): `packages/svelte/components/src/Menu.svelte` (+ `MenuSurface.svelte`)
- GPUI: `packages/gpui/components/src/primitives/menu.rs`
- Jetstream: `packages/jetstream/components/src/menu.rs`
- Spec: `packages/contracts/components/src/menu.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/MenuSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/menu.rs` · jetstream `packages/jetstream/preview/src/specimens/menu.rs`

## Contract ↔ Svelte

One default-value divergence.

- [x] FIXED **`sizeRole` default**: contract §3 now defaults `"chrome"` (was `"control"`), matching Svelte (`Menu.svelte:38`) and the Rust spec (`menu.rs:28`).
- Everything else (`items`, `open`, `defaultOpen`, `placement`, `size`, `density`, `ariaLabel`, `triggerAriaLabel`, `onOpenChange`, `onAction`, `trigger` snippet, item kinds, roles, placement tokens) matches `Menu.svelte:16-44`. The actual item rendering + roles live in `MenuSurface.svelte` (split out), which is a structural choice, not a divergence.

## GPUI gap (vs Svelte + contract)

GPUI menu is well-built: token-resolved sizes, color-mix via opacity, real keyboard nav (Enter/Space/Esc/arrows), hover/active/disabled/checked, destructive tone. Gaps closed.

- [x] FIXED **shadow literal**: the overlay shadow already resolves via `crate::theme_ext::elevation_overlay_shadow()` (`menu.rs:199`); there are no remaining `hsla(...)`/raw color literals in the file (the parity flag's line refs `201,207` were stale). Build-verified.
- accepted: item check indicator renders a leading `check` icon rather than placing the ✓ in the meta column as Svelte does — a minor visual-placement delta, not a token/anatomy violation. Checkbox/radio kind is carried by the spec; GPUI branches the leading check but does not emit distinct ARIA roles (no GPUI accessibility API).
- accepted: no ARIA (`role=menu`/`menuitem*`, `aria-expanded`, `aria-checked` not emitted) — GPUI has no accessibility API.

## Jetstream gap (vs Svelte + contract)

Rebuilt: token-resolved geometry, correct min-width, contract-only separators.

- [x] FIXED **overlay min-width**: now `min_w(rem_to_px(14.0))` (contract §7 / `size.menu.minWidth` = 224px). The Jetstream adapter has no `size.menu.minWidth` mapping, so the contract-exact rem is used directly (see token-gap note).
- [x] FIXED **meta font**: shortcut/meta now uses `rem_to_px(0.6875)` (contract §8 Meta fixed `0.6875rem`, == `typography.caption.size`) — the ad-hoc `* 0.85` multiplier is gone. Resolved as contract-exact rem because the adapter has no `typography.caption.size` mapping (token-gap note).
- [x] FIXED **item geometry**: item vertical padding = `0.375rem`, horizontal = density `control.x`, separator margin = `0.25rem`, separator height `0.0625rem`, item radius = `radius.control − 0.125rem`, inner gap = `space.inline.sm` (token). No magic offsets remain; the prior `panel_space_y − 0.375` derivation is removed.
- [x] FIXED **hover/disabled/destructive**: hover = `tint(accent, 0.16)`; destructive hover = `tint(danger, 0.14)`; disabled = `state.opacity.disabled`; destructive foreground = `color.status.danger` (all contract §8).
- [x] FIXED **section-header invention removed**: non-empty separators no longer render an uppercase label band — separators are always plain `0.0625rem` dividers per contract.
- accepted: **code-family on meta** not applied — the Jetstream `JsEl` builder has no font-family setter, so the shortcut renders in the default font at the contract meta size/color (JsEl gap).
- accepted: **shadow** uses `shadow_md()` (approximates `elevation-overlay`; runtime has no token-driven box-shadow setter).
- accepted: interaction (click/keyboard) lives in the preview event loop; component is render-only. Items now carry `menu-item:<value>` interaction ids for host-loop hit-testing.

### Token gaps (Jetstream adapter)

- `size.menu.minWidth` and `typography.caption.size` have no mapping in `packages/jetstream/adapter/src/theme.rs::match_semantic_space` (both resolve to `0.0`). Menu uses the contract-exact rems (`14rem`, `0.6875rem`) instead. Adding these adapter mappings is a separate, shared-infra change (out of scope for this menu-only pass).

## Specimen parity

- Svelte covers: With shortcuts (5 items + separator + disabled), With checkboxes (checked/unchecked + separator + action) (`MenuSpecimen.svelte`).
- GPUI covers: With shortcuts, With checkboxes (toggleable `dark_mode`/`notifications` state) (`menu.rs`). — matches Svelte; arguably richer (interactive toggles).
- Jetstream covers: Basic, Extended items (`menu.rs:26-31`). — covers shortcuts + checkboxes-equivalent via extended set. `specimen=ok` (both states represented across the three).
- Jetstream probe tests (`menu.rs` `#[cfg(test)]`): items + shortcut meta + separator + interaction ids; checkbox check-indicator + danger tone; overlay min-width 14rem. All pass.

## Notes

- `consv=fixed`: the single `sizeRole` default mismatch (contract `control` → Svelte/spec `chrome`) is resolved.
- Both targets are now token-resolved and at parity. GPUI's shadow already used `elevation_overlay_shadow()` (the flagged literal was stale). Jetstream was rebuilt: 14rem min-width, contract meta font, contract-only separators, token-resolved geometry, real hover/disabled/destructive. Remaining deltas are accepted JsEl/adapter limitations (font-family, box-shadow, two unmapped tokens) noted above.
