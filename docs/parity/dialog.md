<!-- parity consv=ok gpui=0 jetstream=0 specimen=ok | specimen backfill: GPUI added alert role (danger Delete) + Full width preset; Jetstream replaced fake label() footer with real js_button, added alert role, Xl/Full widths, scrollable body — both previews build clean with real Dialog + Button -->
<!-- pass: backdrop→overlay token (gpui); jet per-section gaps (header mb=stack.md, actions mt=stack.lg, gap=inline.sm), chrome-sized close button, removed non-contract 1px divider, max-height 42rem cap + body min-width-0; +7 jet probe tests -->
# Parity: Dialog

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/dialog.md`
- Svelte (authoritative): `packages/svelte/components/src/Dialog.svelte`
- GPUI: `packages/gpui/components/src/primitives/dialog.rs`
- Jetstream: `packages/jetstream/components/src/dialog.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/DialogSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/dialog.rs` · jetstream `packages/jetstream/preview/src/specimens/dialog.rs`

## Contract ↔ Svelte

All contract props present in Svelte with matching types/defaults: `open`, `defaultOpen`, `title`, `description`, `role`, `kind` (deprecated, `effectiveRole = kind ?? role` at `Dialog.svelte:87`), `width`, `bare`, `dismissOnEscape`, `dismissOnBackdrop`, `ariaLabel`, content/overlay class+style, `showCloseButton`, `closeLabel`, `size`, `sizeRole`, `density`. ARIA correct (`aria-modal="true"`, `tabindex="-1"`, backdrop button with aria-label, header/body/actions conditional rendering).

- Svelte adds `closeButtonSize?: ControlSize | null` (independent close-button sizing) not in contract §3. Minor extension. **Fix: add to contract props for completeness** (does not flip `consv` — non-breaking, low-risk; treat as documentation follow-up).

## GPUI gap (vs Svelte + contract)

Robust impl; all token-resolved.

- [x] Shadow — already token-resolved via `theme_ext::elevation_dialog_shadow()` (typed `ELEVATION_DIALOG`); the prior raw-HSLA stack referenced by the stale line numbers no longer exists in the code.
- [x] Backdrop fill now resolves `spec.backdrop_fill_token()` (`color.background.overlay`) instead of the `hsla(0,0,0,0.5)` literal.
- [x] Escape dismissal is backend-owned through overlay dismiss layers.
- accepted: outside-interact, A1 (focus trapping, accessibility tree, modal background suppression, initial focus, focus restoration), and nested modal stacks remain unproved.

## Jetstream gap (vs Svelte + contract)

- [x] Flat panel `gap(rem_to_px(1.0))` removed — per-section token spacing now: header internal gap `0.375rem` + `margin-bottom = space.stack.md`; actions internal gap `space.inline.sm` + `margin-top = space.stack.lg`.
- [x] Close button is now chrome-sized: square at `control_height_rem(chrome_size)` (one stop down from the dialog size via `resolve_semantic_size`), glyph at `size_font_rem(chrome_size)`, `radius.control` corners, hit-test id `poodle-dialog-close`. No `*1.5` multiplier.
- [x] Non-contract 1px pre-actions divider removed (Svelte has none).
- additive: panel `max-height` capped at `42rem` (contract §7 rem term; the 80vh term is viewport-owned by the centering parent) + `overflow-y-hidden`; body children wrapped in a `min-width-0` body container (contract §8 `.dialog__body`).
- accepted: no ARIA / aria-modal at runtime.
- accepted: escape + backdrop dismissal handled in preview `main.rs` event loop, not the component (preview-loop).

## Specimen parity

- Svelte covers: informational, form (width=lg), custom header, custom footer, bare, scrollable, width presets (sm/md/lg/xl), non-dismissible.
- GPUI covers: all eight of the above — full parity.
- Jetstream covers: title+description, title-only, empty, closeable, with-actions, width Sm, width Lg, bare. — missing: **width xl**, **width full**, **custom header snippet**, **custom footer snippet**, **dismissal-control variants** (dismissOnBackdrop/Escape false), **scrollable**.

## Notes

- `consv=ok`: contract and Svelte aligned (sole delta is the non-breaking `closeButtonSize` extension).
- GPUI's two todos are pure token-hygiene (shadow + backdrop literals); behavior is otherwise complete.
