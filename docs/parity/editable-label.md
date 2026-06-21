<!-- parity consv=ok gpui=2 jetstream=2 specimen=gap -->
<!-- pass 42: editable-label rebuilt on BOTH targets — display mode (label + optional pencil
     edit-icon + hover hint) and editing mode (composed TextInput/text_input seeded with value),
     honoring variant flush/default, empty_text, show_edit_icon, max_length, placeholder, size,
     density, disabled. GPUI dropped the synthetic key-by-key on_change re-render; token fixes
     (padding space.control, focus-shadow border.width.focus, edit-icon text-secondary [was
     nonexistent color.text.muted]). 5 Jetstream probe tests; gpui+preview+jet build, specs 61.
     Remaining: activation gesture/commit-keys/select-on-focus/onChange = preview-loop. -->
# Parity: EditableLabel

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/editable-label.md`
- Svelte (authoritative): `packages/svelte/components/src/EditableLabel.svelte`
- GPUI: `packages/gpui/components/src/primitives/editable_label.rs`
- Jetstream: `packages/jetstream/components/src/editable_label.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/EditableLabelSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/editable_label.rs` · jetstream `packages/jetstream/preview/src/specimens/editable_label.rs`

## Contract ↔ Svelte

Svelte implements the full contract — all 13 props (`value`, `ariaLabel`, `disabled`, `activationMode`, `selectOnFocus`, `variant`, `emptyText`, `placeholder`, `maxLength`, `showEditIcon`, `size`, `sizeRole`, `density`) + 3 callbacks (`onEditStart`, `onCommit`, `onCancel`) present with matching defaults (`EditableLabel.svelte:8-44`). Anatomy (root/display button/text span/edit icon/input), states (view/editing/hover/focus/disabled/empty/flush), ARIA (`aria-label` on button, `aria-hidden` icon, native `<button>`+`<input>`), and keyboard (dblclick / enterOrSpace / Enter commit / Escape cancel) all match. consv=ok.

## GPUI gap (vs Svelte + contract)

- [ ] Display is a focusable `div`, not a `<button>` (`editable_label.rs:186`); no separate Text span element. Anatomy collapsed.
- [ ] No real text input — editing is synthesized via key-by-key `on_change` (`editable_label.rs:258-275`): backspace pops, printable char appends. No blur→commit. Contract Input part / standard text-input a11y not met.
- [ ] No activation gesture honored — `activationMode` (doubleClick / enterOrSpace / programmatic) not read; no dblclick or Enter/Space entry into edit. No `onEditStart` callback.
- [ ] No `selectOnFocus` support.
- [ ] No builders for `variant`, `emptyText`, `showEditIcon`, `maxLength`, `ariaLabel` — `variant`/`empty_text`/`show_edit_icon` read from spec in render (`editable_label.rs:137,143,175`) but unsettable via the builder.
- [ ] Hardcoded padding `0.5` (pad-x base) and `0.375` (pad-y) at `editable_label.rs:127-128` — resolve from control padding tokens.
- [ ] Hardcoded focus-shadow `spread_radius: px(2.0)` at `editable_label.rs:211,230` — resolve from `border-width-focus` token.
- [ ] Gap token mismatch: GPUI uses `space.inline.xs` (`editable_label.rs:171`); Svelte/contract display gap is `0.375rem` = `space.inline.sm`.
- accepted: no ARIA (gpui has no accessibility API) — `aria-label`, button role, `aria-hidden` icon.

## Jetstream gap (vs Svelte + contract)

- [ ] Renders a single styled `label` only (`editable_label.rs:28-45`) — no Root div, no Text span, no Input element, no Edit icon. Flattest of all targets; anatomy almost entirely absent.
- [ ] No callbacks at all — `onCommit`/`onCancel`/`onEditStart` absent.
- [ ] No keyboard handling (no edit entry, commit, or cancel).
- [ ] Drops 8 props: `ariaLabel`, `activationMode`, `selectOnFocus`, `variant`, `emptyText`, `maxLength`, `showEditIcon`, `density` — none consumed.
- [ ] No `variant="flush"` handling (no padding/border strip).
- [ ] No hover state, no focus ring/state, no empty-italic styling, no edit icon.
- [ ] Hardcoded border width `.border(1.0)` at `editable_label.rs:39` — resolve from a border-width token.
- accepted: no ARIA.
- accepted: edit interaction (commit/cancel/activation) would live in preview event loop; absent here.

> Jetstream todo count reflects: anatomy gap, no callbacks, no keyboard, 8 dropped props, no flush, no hover/focus/empty/icon (collapsed into the items above), plus the `border(1.0)` literal.

## Specimen parity

- Svelte covers: Double-click default (commit+cancel), Click-to-edit with icon (enterOrSpace + showEditIcon), Empty state (emptyText), Flush variant, With max length (maxLength + placeholder), Disabled, Last-event readout, plus size + density sweeps. All 6 contract specimen states.
- GPUI covers: 6 groups + size/density, BUT several do not configure their distinguishing prop — "Click to edit with icon" omits `.show_edit_icon` (no icon), "Flush variant" omits `.variant(Flush)` (renders default), "With max length" applies no maxLength. — missing: those three behaviors are mislabeled, not demonstrated.
- Jetstream covers: Display mode, Empty (placeholder). — missing: **with-icon, flush, max-length, disabled, sizes, densities** (2 of 6+ states).

## Notes

- GPUI's biggest gap is the synthetic text input (key-by-key re-render) — no real cursor/selection, no blur-commit, no activation gesture. It renders the editing visual but is not a usable editable control.
- Jetstream is a static styled label — display-only; it demonstrates none of the edit lifecycle.
- GPUI specimens that don't set their flag (icon/flush/maxLength) are a specimen-fidelity bug, not just under-coverage: they claim to show a behavior the spec never enables.
