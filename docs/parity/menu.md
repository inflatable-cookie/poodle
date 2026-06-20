<!-- parity consv=gap gpui=2 jetstream=5 specimen=ok -->
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

- **`sizeRole` default**: contract §3 says default `"control"`; Svelte default is `"chrome"` (`Menu.svelte:38`), and the Rust spec also defaults `Chrome` (`menu.rs:28`). Svelte is authoritative. **Fix: contract `sizeRole` default → `"chrome"`.**
- Everything else (`items`, `open`, `defaultOpen`, `placement`, `size`, `density`, `ariaLabel`, `triggerAriaLabel`, `onOpenChange`, `onAction`, `trigger` snippet, item kinds, roles, placement tokens) matches `Menu.svelte:16-44`. The actual item rendering + roles live in `MenuSurface.svelte` (split out), which is a structural choice, not a divergence.

## GPUI gap (vs Svelte + contract)

GPUI menu is well-built: token-resolved sizes, color-mix via opacity, real keyboard nav (Enter/Space/Esc/arrows), hover/active/disabled/checked, destructive tone. Few gaps.

- [ ] Hardcoded HSLA shadow literals: `hsla(0.0, 0.0, 0.0, 0.10)` and `hsla(0.0, 0.0, 0.0, 0.06)` (`menu.rs:201,207`) — contract overlay shadow is `elevation-overlay`; resolve from an elevation token, not raw HSLA.
- [ ] Item check indicator renders a leading `check` icon (`menu.rs:347-351`); contract/Svelte put the meta (shortcut) in column 2 and use no leading-check for `action` kind — checkbox/radio semantics are flattened to "is_checked → leading check" regardless of kind. Verify checkbox vs radio role distinction (spec carries kind; render does not branch role visually beyond the check). Minor.
- accepted: no ARIA (`role=menu`/`menuitem*`, `aria-expanded`, `aria-checked` not emitted) — GPUI has no accessibility API.

## Jetstream gap (vs Svelte + contract)

Renders items but with hardcoded geometry and a wrong min-width.

- [ ] Wrong overlay min-width: `min_w(rem_to_px(10.0))` (`menu.rs:47`) — contract §7 + token is `14rem` (`size.menu.minWidth`). GPUI uses the token; Jetstream hardcodes 10rem. **Fix: resolve from `size.menu.minWidth`.**
- [ ] Hardcoded font multipliers: `meta_font_size = size_font_rem * 0.85` (`menu.rs:18`), `section_label_font = size_font_rem * 0.75` (`:24`) — contract meta is a fixed `0.6875rem` code-font; these ad-hoc multipliers drift from the token. Resolve meta from `typography.caption.size` / code-family.
- [ ] Hardcoded px: `item_py = panel_space_y_rem − 0.375` (`:20`), `menu_py = rem_to_px(0.25)` (`:21`), `item_gap = rem_to_px(0.5)` (`:22`), `separator_my = rem_to_px(0.25)` (`:23`) — magic offsets; resolve from spacing tokens (contract item padding `0.375rem 0.5rem`, separator margin `0.25rem`).
- [ ] No code-family on meta — shortcut label uses default font, not `typography.code-family` (contract §8 Meta).
- [ ] "Section header" rendering for non-empty separators (`menu.rs:62-78`) is an invention — contract separators are always plain dividers; a labeled separator is not in the model. Either drop or contractualize.
- accepted: interaction (click/keyboard) lives in the preview event loop; component is render-only.

## Specimen parity

- Svelte covers: With shortcuts (5 items + separator + disabled), With checkboxes (checked/unchecked + separator + action) (`MenuSpecimen.svelte`).
- GPUI covers: With shortcuts, With checkboxes (toggleable `dark_mode`/`notifications` state) (`menu.rs`). — matches Svelte; arguably richer (interactive toggles).
- Jetstream covers: Basic, Extended items (`menu.rs:26-31`). — covers shortcuts + checkboxes-equivalent via extended set. `specimen=ok` (both states represented across the three).

## Notes

- `consv=gap` driver: single `sizeRole` default mismatch (contract `control` vs Svelte/spec `chrome`).
- GPUI is the strong target here — close to parity, only the shadow literal + ARIA delta remain. Jetstream's 10rem min-width and ad-hoc font multipliers are the substantive fixes.
