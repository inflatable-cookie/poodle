<!-- parity consv=gap gpui=6 jetstream=9 specimen=gap -->
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

Svelte has surface the contract does not document, and its pressed treatment diverges. Svelte is authoritative — update the contract.

- Svelte adds a `success` tone: full CSS for `[data-tone="success"]`, primary×success, ghost×success, ghost×success:hover (`IconButton.svelte:326-363`). Contract §3 `ButtonTone = "default" | "danger"` only. **Fix: add `"success"` to contract tone union + token tables.**
- `defaultPressed` type mismatch: contract §3 says `boolean` default `false`; Svelte is `boolean | null` default `null` (`IconButton.svelte:38,60`) and toggle mode derives from `pressed !== null || defaultPressed !== null`. **Fix: reconcile contract type to `boolean | null` default `null`.**
- Pressed treatment diverges: contract §8 "Root — Pressed" specifies `background: color-mix(accent 20%, fill)`, `border-color: color-mix(accent 56%, border)`, double inset shadow. Svelte (`IconButton.svelte:365-371`) instead sets full `accent-base` fill, `accent-base 85% black` border, `text-inverse` color, `shadow: none` for non-primary. **Fix: rewrite contract §8 pressed table to match Svelte's solid-accent pressed style.**
- Hover adds box-shadow: Svelte hover sets `box-shadow` (`IconButton.svelte:376`); contract §8 hover table lists only background + border-color. **Fix: add box-shadow row to contract hover.**
- Tooltip radius unspecified: Svelte tooltip uses `border-radius: calc(radius-control - 0.125rem)` and `font-size: 0.6875rem` (`IconButton.svelte:449-463`); contract §2/§8 describe the tooltip part but give no token values. **Fix: add tooltip token table to contract §8.**

## GPUI gap (vs Svelte + contract)

- [ ] Hardcoded shadow px literals in pressed inset shadow at `icon_button.rs:219-232` — `px(1.0)`, `px(2.0)`, `px(0.0)` and `accent.opacity(0.08)`/`(0.12)`. Contract §8 pressed shadow is `inset white 12%` + `inset accent 18%`; resolve mix percentages from contract, avoid ad-hoc 0.08/0.12.
- [ ] No `success` tone — variant/tone color resolution (`icon_button.rs:124-148`) only handles default/danger via spec token methods; success falls through.
- [ ] No tooltip — `spec.tooltip` carried but never rendered; contract §2 tooltip part + §6 `role="tooltip"`/`aria-describedby` + 300ms delay absent.
- [ ] Icon size fixed to `IconSize::Sm` (`icon_button.rs:266`) — contract §13 (line 471) and Svelte derive glyph size from the resolved control size via the supporting-size mapping; should track `effective_size`, not a constant.
- [ ] Pressed border uses `color_mix(border, text_primary, 0.74)` (`icon_button.rs:171`) — that is the *hover* mix, not the pressed border (contract §8 pressed border = `accent 56%`). Wrong mix reused.
- [ ] Glyph 45% width/height sizing (contract §8 glyph table) not replicated — icon scales via `IconSize::Sm` instead of `width:45%` of the square.
- accepted: no ARIA (gpui has no accessibility API) — `aria-label`/`aria-pressed`/`aria-busy` not emitted.
- accepted: tooltip positioning/delay is host-driven (contract §11 Tier 3).

## Jetstream gap (vs Svelte + contract)

`js_icon_button` is a near-stub: one fill, one hover, no variant/tone/state differentiation. Most of the contract is unimplemented.

- [ ] Hardcoded hover mix literal `surface.mix(elevated, 0.84)` at `icon_button.rs:29` — contract §8 hover fill is `color-mix(fill 76%, elevated)`; resolve per-variant fill then mix at the contract ratio, not a flat 0.84 on surface.
- [ ] No variant differentiation — primary/secondary/ghost all render identical surface fill (`icon_button.rs:27,34-47`). Contract §8 requires distinct fill/border/text per variant (primary=accent, secondary=surface+border-default, ghost=transparent). `spec.variant` ignored.
- [ ] No `tone` handling — danger/success ignored; specimen passes `ButtonTone::Danger` but component renders default. Add tone color resolution.
- [ ] No pressed state — `spec.is_pressed` unread; contract §8 pressed accent treatment absent.
- [ ] No loading/spinner — `spec.is_loading` unread; contract §2 spinner part + §6 `aria-busy` absent.
- [ ] No border — `button("")` has no `.border(...)`; contract §8 base requires `0.0625rem solid var(--poodle-icon-button-border)`.
- [ ] No focus ring — `.focusable()` set but no focus-visible outline (contract §8 focus table).
- [ ] No tooltip — contract §2 tooltip part absent.
- [ ] Size offsets: uses `control_height_rem(effective_size)` directly; verify it matches the contract's per-size square deltas (xs −0.25, sm −0.375, lg +0.375, xl +0.5 from md). Currently relies on the size scale, not the contract's explicit offsets.
- accepted: no ARIA channel.
- accepted: click/keyboard interaction lives in preview `main.rs` event loop.

## Specimen parity

- Svelte covers: Variants (primary/secondary/ghost), Danger tone (×3 variants), Toggle (text-editor toolbar, bind:pressed), Toggle (secondary), Disabled and loading, Sizes (xs–xl).
- GPUI covers: Variants, Danger tone (×3), Toggle/pressed (bold/italic/underline + pinned), States (pressed/disabled/loading), Sizes (365 lines). — ok, matches Svelte closely.
- Jetstream covers: Ghost/Primary/Secondary, Danger tone, Small/Large size, Disabled. — missing: **loading/spinner state**, **pressed/toggle state**. And the component ignores variant/tone/pressed so the rendered specimen does not actually demonstrate those differences (mockup risk).

## Notes

- consv=gap driver: undocumented `success` tone + pressed treatment that disagrees with the contract's documented color-mix. Both belong in the contract per "Svelte is parity authority".
- Jetstream is the biggest gap overall — the component renders a single generic icon button regardless of variant/tone/state, so its specimen visually misrepresents the props it is passed. This is the "mockup risk" CLAUDE.md warns about: the specimen passes real props but the component drops them.
- GPUI is broadly faithful (variant/tone/pressed/loading via spec token methods) — gaps are tooltip, success tone, icon-size derivation, and a few hardcoded shadow literals.
