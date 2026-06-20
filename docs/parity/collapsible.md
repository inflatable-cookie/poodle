<!-- parity consv=fixed gpui=6 jetstream=8 specimen=gap -->
# Parity: Collapsible

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/collapsible.md`
- Svelte (authoritative): `packages/svelte/components/src/Collapsible.svelte`
- GPUI: `packages/gpui/components/src/primitives/collapsible.rs`
- Jetstream: `packages/jetstream/components/src/collapsible.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/CollapsibleSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/collapsible.rs` · jetstream `packages/jetstream/preview/src/specimens/collapsible.rs`

## Contract ↔ Svelte

All exact-value rows in contract §8 now match the authoritative Svelte CSS. FIXED.

- [x] **Root background** → `color-mix(background-elevated 40%, background-panel)` (Svelte line 134). FIXED.
- [x] **Root border** → `border-subtle 36%` (Svelte line 132). FIXED.
- [x] **Root padding** → `0.625rem var(--poodle-space-panel-x)` + density `padding-inline` rows (0.5rem/1rem). FIXED.
- [x] **Root gap (open)** → `var(--poodle-space-stack-md)`. FIXED.
- [x] **Trigger gap** → `var(--poodle-space-inline-md)` token. FIXED.
- [x] **Heading gap** → `var(--poodle-space-inline-sm)` token. FIXED.
- [x] **Highlighted state** → added Root-highlighted token table (accent-base 55% border + 12% halo) to §8. FIXED.
- **Indicator**: Svelte renders `<Icon name="chevron-down" />` rotated 180deg on open; the §8 indicator span still carries code-family/0.75rem CSS (real Svelte values, kept). Rust targets swap icon name instead of rotating — Tier-3 freedom, no contract change.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] `highlighted` unsupported — no builder method, no accent border/halo branch; spec/prop ignored. Add highlighted rendering (`collapsible.rs` has no `highlighted` reference).
- [ ] Background mix wrong direction: `color_mix(panel_bg, elevated_bg, 0.82)` at `collapsible.rs:147` uses 82% panel; Svelte is `elevated 40%, panel` (i.e. 60% panel). Recompute to match the elevated/panel 40% mix.
- [ ] Border alpha `* 0.42` at `:142` — Svelte is 36%. Use 0.36 (and source from a token, not a raw multiply).
- [ ] Description uses `typography.label.size` (`:132, 196-203`); contract size table wants per-size description font (`xs` 0.6875 … `xl` 0.9375rem). Title scales per-size (`:123-129`) but description does not.
- [ ] No chevron rotation/transition — icon name swapped `chevron-down`/`chevron-right` (`:207-211`); contract §8 indicator rotates 180deg over `motion-duration-interaction`. Accepted as Tier-3 if visual result matches, else flag.
- [ ] Content has no `padding-top: 0.125rem` (contract §8 Content); content child appended raw at `:244-248`.
- accepted: no ARIA (gpui has no accessibility API) — trigger `aria-expanded`/`aria-controls` + content `role="region"`/`aria-labelledby` not emitted.
- accepted: no slide/height animation (`~180ms`) — static show/hide.

## Jetstream gap (vs Svelte + contract)

- [ ] Background uses flat `color.background.surface` at `collapsible.rs:21` — contract/Svelte want the elevated/panel color-mix; no mix applied.
- [ ] Border alpha not reduced — `border(1.0)` with full-alpha `color.border.subtle` (`:32`); Svelte is 36% alpha.
- [ ] Padding-Y hardcoded `rem_to_px(0.625)` at `:27`; padding-X via `panel_space_x_rem` (`:26`) gives compact=0.75/comfortable=1.25, but Svelte collapsible density is 0.5/1.0 — wrong density token (GPUI notes the same divergence and special-cases it).
- [ ] Layout wrong: chevron + title in a single `flex_row` header (`:38-58`), description appended **inline in the same row** (`:61-70`). Contract anatomy is a `1fr auto` grid — heading block (title over description) on the left, indicator on the right. Description must stack under title, not sit beside the chevron.
- [ ] No gap collapse — root has no open/closed gap distinction; contract gap is `space.stack.md` open, 0 closed.
- [ ] `highlighted` unsupported — prop ignored, no accent border/halo.
- [ ] No focus ring on the header (no `focusable`/ring); contract §8 trigger focus-visible unimplemented.
- [ ] Title weight `600` (`:56`); contract/Svelte title is `700`/`FontWeight::BOLD`.
- accepted: interaction (toggle) lives in preview event loop, not the component.

## Specimen parity

- Svelte covers: Default (closed, interactive), Default open, Disabled, **Highlighted**, **Custom trigger** (snippet), Sizes (xs–xl), Densities (compact/default/comfortable).
- GPUI covers: Default (closed, interactive), Default open (interactive), Disabled, Sizes + Densities via `specimen_layout`. — missing: **Highlighted** group, **Custom trigger** snippet group.
- Jetstream covers: Open, Closed, Disabled (all static). — missing: **Highlighted**, **Custom trigger**, **Sizes**, **Densities** groups; no interactive toggle.

## Notes

- The big `consv=gap` driver is contract §8 carrying stale literal values (88% surface bg, 42% border, 0.875rem padding, 0.5rem gap, 0.3125rem heading gap) that the authoritative Svelte CSS contradicts. All should be re-sourced from the tokens Svelte actually references.
- Both Rust targets swap icon name instead of rotating a single chevron — acceptable under Tier-3 freedom provided the open/closed visual reads identically; the rotation transition itself is unreproduced.
