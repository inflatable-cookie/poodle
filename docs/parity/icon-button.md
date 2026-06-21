<!-- parity consv=fixed gpui=0 jetstream=0 specimen=gap | pass: success tone closed on both targets — ButtonTone::Success added cross-cutting; GPUI + Jetstream resolve ghost-text/primary-fill via shared token methods and apply the secondary 16%/46% color-mixes. Remaining: specimen coverage only (Jetstream loading/pressed specimens). -->
# Parity: IconButton

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/icon-button.md`
- Svelte (authoritative): `packages/svelte/components/src/IconButton.svelte`
- GPUI: `packages/gpui/components/src/primitives/icon_button.rs`
- Jetstream: `packages/jetstream/components/src/icon_button.rs`
- Spec: `packages/contracts/components/src/icon_button.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/IconButtonSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/icon_button.rs` · jetstream `packages/jetstream/preview/src/specimens/icon_button.rs`

## Contract ↔ Svelte

Svelte has surface the contract does not document, and its pressed treatment diverged. Svelte is authoritative — contract reconciled.

- [x] FIXED `success` tone: added `"success"` to `ButtonTone` union (§3), §1 in-scope, §4 tone note, plus a Tone: success token table + Ghost success hover table in §8 mirroring Svelte (`IconButton.svelte:326-363`).
- [x] FIXED `defaultPressed` type: contract §3 now `boolean | null` default `null`, with toggle-mode note (`pressed !== null || defaultPressed !== null`), matching Svelte (`IconButton.svelte:38,60`).
- [x] FIXED Pressed treatment: §8 "Root — Pressed" rewritten to Svelte's solid-accent custom-property treatment for non-primary variants — fill `accent-base`, fill-hover `color-mix(white 12%, accent-base)`, border `color-mix(accent-base 85%, black)`, text `text-inverse`, shadow `none` (`IconButton.svelte:365-371`). §4 states row + §11 checklist updated.
- [x] FIXED Hover box-shadow: added `box-shadow: var(--poodle-treatment-interactive-shadow-active, var(--poodle-icon-button-shadow))` row to §8 Hover (`IconButton.svelte:376`).
- [x] FIXED Tooltip tokens: added full Tooltip token table to §8 (position/z-index/max-width/padding/border/radius `calc(radius-control - 0.125rem)`/background/box-shadow/color/font-size `0.6875rem`/line-height/white-space/pointer-events) from `IconButton.svelte:449-463`.

## GPUI gap (vs Svelte + contract)

Fix pass (2026-06-21): pressed treatment rewritten to the contract §8 table,
glyph size now derives from the supporting-size mapping, and the ad-hoc inset
shadow literals are gone.

- [x] FIXED `success` tone — `ButtonTone::Success` added to the shared enum. Ghost-text and primary-fill resolve via `ButtonVariant::{fill,border,text}_token(tone)`; the secondary base now applies `color-mix(success 16%, surface)` fill + `color-mix(success 46%, border-default)` border per icon-button.md §8 (same path also fixes the pre-existing secondary-danger mix).
- accepted: no ARIA (gpui has no accessibility API) — `aria-label`/`aria-pressed`/`aria-busy` not emitted.
- accepted: no tooltip — `spec.tooltip` carried for consumer wiring; contract §2 tooltip surface + §6 `role="tooltip"`/`aria-describedby` + 300ms delay are host/overlay-driven (contract §11 Tier 3).
- accepted: glyph uses the `IconSize` token scale (Sm/Md/Lg) derived from the supporting-size mapping rather than a literal `width:45%` of the square — the contract glyph %-sizing is a CSS detail; GPUI sizes icons by token. Visual result is comparable (contract §12 known-delta: icon rendering is platform-owned).

### Resolved in fix pass

- [x] FIXED Pressed treatment (non-primary) now matches contract §8 "Root — Pressed": solid `accent-base` fill, border `color-mix(accent-base 85%, black)` via `color_mix_black(accent, 0.85)`, `text-inverse` glyph, and shadow `none`. Primary keeps its own variant styling when pressed.
- [x] FIXED Removed the ad-hoc pressed inset-shadow stack (`px(1.0)`/`px(2.0)` + `accent.opacity(0.08/0.12)`) — contract pressed shadow is `none`.
- [x] FIXED Pressed border no longer reuses the hover mix (`0.74`) — now the contract `accent 85% black`.
- [x] FIXED Pressed hover fill is `color-mix(white 12%, accent-base)` per the contract pressed `fill-hover`.
- [x] FIXED Glyph size derives from `IconSize::from(resolve_supporting_visual_size(effective_size))` (contract §13) — tracks the effective control size, no longer the constant `IconSize::Sm`.

## Jetstream gap (vs Svelte + contract)

Rebuild pass (2026-06-21): `js_icon_button` rebuilt from a near-stub to a
contract-faithful component — per-variant × tone fill/border/text, pressed,
loading/spinner, border, hover/active, and the contract per-size square all
resolve from tokens. 9 `render_probe` tests cover the closed gaps.

- [x] FIXED `success` tone — `js_icon_button` now resolves danger/success via a shared `status` color: ghost success = transparent fill + success glyph, primary success = solid success fill, secondary success = `success 16% surface` fill + `success 46% border`. Probe-verified (`success_tone_recolors_ghost_glyph`, `primary_success_fills_with_status_success`).
- accepted: no focus-visible outline — the `JsEl` builder exposes `.hover`/`.active` but no `.focus` state modifier, so the contract §8 focus ring is not expressible in Jetstream. `.focusable()` is set for hit-testing.
- accepted: no ARIA channel; no tooltip surface (host/overlay-driven).
- accepted: click/keyboard interaction lives in preview `main.rs` event loop.

### Resolved in rebuild pass

- [x] FIXED Per-variant differentiation — ghost (transparent fill/border), primary (accent fill, `accent 84% black` border, inverse text), secondary (surface fill, border-default). `spec.variant` now drives color resolution.
- [x] FIXED Tone handling — danger × {ghost,primary,secondary} per contract §8 (ghost danger = transparent fill + danger text; primary danger = solid danger; secondary danger = `danger 16% surface` fill + `danger 46% border`).
- [x] FIXED Hover fill now `color-mix(fill 76%, elevated)` resolved per-variant; hover border `74%` toward text-primary; active `color-mix(fill 64%, elevated)` — no flat `0.84`-on-surface literal.
- [x] FIXED Pressed (non-primary) — solid accent fill, `accent 85% black` border, inverse text per contract §8.
- [x] FIXED Loading swaps the glyph for the `loader` spinner icon and sets disabled (suppresses activation).
- [x] FIXED Border now `1px solid` resolved per-variant border token (was none).
- [x] FIXED Square size uses the contract per-size deltas (xs −0.25, sm −0.375, md 0, lg +0.375, xl +0.5) off the md control-height — via a local `icon_button_size_delta_rem` (distinct from the Button height-offset scale, which uses xs −0.5).
- [x] FIXED Glyph tracks the supporting-visual size (contract §13), not a constant.

## Specimen parity

- Svelte covers: Variants (primary/secondary/ghost), Danger tone (×3 variants), Toggle (text-editor toolbar, bind:pressed), Toggle (secondary), Disabled and loading, Sizes (xs–xl).
- GPUI covers: Variants, Danger tone (×3), Toggle/pressed (bold/italic/underline + pinned), States (pressed/disabled/loading), Sizes (365 lines). — ok, matches Svelte closely.
- Jetstream covers: Ghost/Primary/Secondary, Danger tone, Small/Large size, Disabled. — missing: **loading/spinner state**, **pressed/toggle state**. And the component ignores variant/tone/pressed so the rendered specimen does not actually demonstrate those differences (mockup risk).

## Notes

- `success` tone is now closed on both targets — `ButtonTone::Success` was added cross-cutting (Button/SplitButton/IconButton + the three exhaustive `ButtonVariant::*_token` matches; alert-dialog/confirm-action/bulk-action-bar map *to* ButtonTone and were unaffected). Both impls resolve it from `color.status.success`.
- Jetstream is no longer a stub — `js_icon_button` now resolves variant/tone/pressed/loading/border/hover from tokens, so the specimen demonstrates real differences (mockup risk closed). Probe-verified (9 tests).
- GPUI pressed treatment now matches the contract §8 table exactly (solid accent, `accent 85% black` border, inverse text, no shadow); glyph size derives from the supporting-size mapping. Build-verified.
