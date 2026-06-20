<!-- parity consv=ok gpui=2 jetstream=3 specimen=gap -->
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

Robust impl; shadow/gap/color values align with `elevation-dialog` token semantics but are written as raw literals.

- [ ] Shadow color literals `hsla(0.0, 0.0, 0.0, 0.12)` (`dialog.rs:249`) and `hsla(0.0, 0.0, 0.0, 0.08)` (`dialog.rs:255`), plus offset/blur floats `px(8.0)`/`px(24.0)`/`px(2.0)`/`px(8.0)` (`dialog.rs:250-258`) — resolve the dialog elevation from a token, not raw HSLA + float stack.
- [ ] Backdrop fill literal `hsla(0.0, 0.0, 0.0, 0.5)` at `dialog.rs:395` — resolve from `--poodle-color-background-overlay` token.
- accepted: no ARIA (gpui has no accessibility API) — role/aria-modal not emitted; focus trap is platform-owned (Tier 3).

## Jetstream gap (vs Svelte + contract)

- [ ] Uniform panel gap `gap(rem_to_px(1.0))` at `dialog.rs:49` — contract uses distinct gaps (header 0.375rem, actions ~0.5rem); resolve per-section gaps from tokens rather than a flat 1rem.
- [ ] Close button sized by ad-hoc `icon_size * 1.5` (`dialog.rs:118-119`) — contract close button is an `IconButton` at chrome size (one step down); compute from size token, not a 1.5 multiplier.
- [ ] Divider `h(1.0)` before actions (`dialog.rs:148`) — not in contract (Svelte divider is CSS treatment); reconcile to a token-resolved border or remove.
- accepted: no ARIA / aria-modal at runtime.
- accepted: escape + backdrop dismissal handled in preview `main.rs` event loop, not the component.

## Specimen parity

- Svelte covers: informational, form (width=lg), custom header, custom footer, bare, scrollable, width presets (sm/md/lg/xl), non-dismissible.
- GPUI covers: all eight of the above — full parity.
- Jetstream covers: title+description, title-only, empty, closeable, with-actions, width Sm, width Lg, bare. — missing: **width xl**, **width full**, **custom header snippet**, **custom footer snippet**, **dismissal-control variants** (dismissOnBackdrop/Escape false), **scrollable**.

## Notes

- `consv=ok`: contract and Svelte aligned (sole delta is the non-breaking `closeButtonSize` extension).
- GPUI's two todos are pure token-hygiene (shadow + backdrop literals); behavior is otherwise complete.
