<!-- parity consv=ok gpui=8 jetstream=8 specimen=gap -->
# Parity: FileUpload

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/file-upload.md`
- Svelte (authoritative): `packages/svelte/components/src/FileUpload.svelte`
- GPUI: `packages/gpui/components/src/primitives/file_upload.rs`
- Jetstream: `packages/jetstream/components/src/file_upload.rs`
- Spec: `packages/contracts/components/src/file_upload.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/FileUploadSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/file_upload.rs` · jetstream `packages/jetstream/preview/src/specimens/file_upload.rs`

## Contract ↔ Svelte

Contract and Svelte agree on the prop surface (`accept`, `maxSize`, `multiple`, `maxFiles`, `showPreview`, `disabled`, `files`, `validate`, `compress`, `compressionOptions`, `size`, `sizeRole`, `density`), the `FileUploadItem` shape, the public methods (`updateProgress`, `setError`, `clear`), and the callbacks (`onChange`, `onUpload`, `onError`, `onRemove`). No divergence. Minor cosmetic notes only:

- Contract §8 names the active-dropzone and browse colors `--poodle-color-accent-default`; Svelte uses `--poodle-color-accent-base` (`FileUpload.svelte:370,371,409`). Same intent, token-name drift. Low priority — reconcile contract token name to `accent-base` if that is the canonical semantic name, else fix Svelte. Not a behavioral gap → `consv=ok`.
- Progress-bar fill uses `--poodle-color-accent-default` (`FileUpload.svelte:490`) while dropzone-active uses `accent-base`. Internal inconsistency in Svelte; pick one.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] **Entire file-list half of anatomy missing.** GPUI renders only the dropzone. No File List (`ul`), File Item, Preview/File Icon, Meta (Name/Size), Error Text, Progress/Progress Bar, or Remove button — contract §2 marks File Item, Meta, Name, Size, Remove as **required**. `file_upload.rs:195-249` stops at the dropzone + optional validation-error line. Add the file-list rendering driven by a `files` collection on the spec.
- [ ] **Spec carries no file-list data.** `FileUploadSpec` (`file_upload.rs:5-30`) has no `files: Vec<FileUploadItem>` field and no `FileUploadItem` type exists in `poodle-specs`. Add the item struct (file name, size, status, progress, preview, error) so the file list can render.
- [ ] Hardcoded gap literal `px(rem_to_px(0.375))` at `file_upload.rs:238` for the validation-error stack — resolve from a space token, not raw `0.375`.
- [ ] Hardcoded font literal `px(rem_to_px(0.75))` at `file_upload.rs:242` for error text — resolve from `typography.caption.size` (already loaded as `caption_size` on line 117) or a danger-text token.
- [ ] Dropzone hover uses raw `Hsla { a: panel_bg.a * 0.50, ..panel_bg }` at `file_upload.rs:213` — the 50% panel-mix should go through a `color_mix`/token helper, not an inline Hsla literal.
- [ ] **Helper/label copy diverges from contract.** Label text is `"Drag files here or click to browse"` (`file_upload.rs:133-137`) and helper lines read `"Accepted: {accept}"` / `"Up to N files"` / `"Images will be compressed…"`. Svelte renders `"Drop files here or browse"` with an underlined accent `browse` span and a single `accept · Max <size>` hint line. Align copy + accent `browse` styling (contract anatomy requires the Browse part).
- [ ] Browse affordance is a bordered button (`file_upload.rs:146-155`); contract/Svelte Browse is inline underlined accent text inside the label. Fix to match anatomy.
- [ ] **Size variants not applied.** Contract §8 size table (dropzone min-height 5/6/8/10/12rem, icon 1.5/2/2.5rem, label/hint font per size) is ignored — `dropzone_min_h` reads a single `size.fileUpload.dropZoneMinHeight` token (`file_upload.rs:119`) regardless of `effective_size`. Drive min-height/icon/label/hint from the size table.
- accepted: no ARIA (gpui has no accessibility API) — dropzone `role="button"`/`tabindex` and remove-button labels not emitted.
- accepted: native file dialog + drag-and-drop platform integration owned by runtime (contract §10, Known Delta).

## Jetstream gap (vs Svelte + contract)

- [ ] **Entire file-list half of anatomy missing.** `js_file_upload` renders root + dropzone only (`file_upload.rs:148-161`); the comment at `:72,74` even admits file-list items are "used at runtime" but nothing renders them. No File Item, Preview, Meta, Progress, or Remove parts. Render the file list (needs spec `files` field per the GPUI todo above).
- [ ] Hardcoded sizing literals: `rem_to_px(1.5)` padding (`file_upload.rs:85`), `rem_to_px(0.375)` content-gap (`:86`), `rem_to_px(0.5)` root-gap (`:87`), `rem_to_px(0.125)` border-width (`:88`). Contract maps these to `space.panel.{x,y}`, a content-gap token, `space.stack.sm`, and a border-width token — resolve from tokens, not raw rem floats.
- [ ] Drag-tint + hover-bg built with raw `Color::new(...)` alpha math (`file_upload.rs:132,141`) — route the 8% accent and 50% panel mixes through a token/color-mix helper.
- [ ] **Padding ignores density and size.** `padding = rem_to_px(1.5)` is flat (`file_upload.rs:85`); Svelte drives dropzone padding from `space.panel.{x,y}` with compact/comfortable density overrides (1rem / 1.75rem). Resolve panel padding from density like the GPUI impl does.
- [ ] **Label/hint copy diverges.** Label `"Drop files here or browse"` lacks the accent `browse` span styling; `build_hint_text` emits `"Accepted: … · Max N MB · Multiple files allowed"` (`file_upload.rs:165-182`) vs Svelte's `accept · Max <formatFileSize>`. Align copy + accent browse styling.
- [ ] No `show_preview` influence (spec field exists, unused) — only matters once the file list renders, but note it.
- [ ] No error/validation-error rendering — `FileUploadSpec.validation_error` exists and GPUI renders it, but Jetstream ignores it. Render the error line below the dropzone.
- [ ] Size variants: dropzone min-height/icon/label/hint use per-size helper fns (`file_upload.rs:17-52`) ✓ — but verify they match the contract size table once the rest lands; flagged for review, not broken.
- accepted: no ARIA channel (documented pattern).
- accepted: drag/drop + file-dialog interaction lives in the preview event loop, not the component.

## Specimen parity

- Svelte covers: Image upload with preview, Document upload (single, no preview), Compressed image upload + custom validation, Disabled, Last-error display; plus size and density snippets (`FileUploadSpecimen.svelte`).
- GPUI covers: Image upload, Document upload, Dragging state, Disabled, Max-files cap, Image compression, Validation error (`file_upload.rs`). Broad on dropzone states. — missing: **no populated file list** in any specimen (no item rows, previews, progress, error rows, remove buttons) because the component can't render them; **no Compressed + custom-validation** parity with Svelte's combined group.
- Jetstream covers: Default dropzone, Accept-images-only, Dragging, Disabled (`file_upload.rs`). — missing: **populated file list**, **document/no-preview**, **compression**, **validation-error**, **size/density** groups present in Svelte/GPUI. Thinnest specimen of the three.

## Notes

- `consv=ok`: contract and Svelte are aligned on props, methods, callbacks, and `FileUploadItem`. Only token-name drift (`accent-default` vs `accent-base`) and an internal Svelte inconsistency on the accent token — neither is a behavioral divergence.
- The dominant gap across both Rust targets is the **missing file-list anatomy** (required parts: File Item, Meta, Name, Size, Remove). This is a spec gap first: `poodle-specs` has no `FileUploadItem` type or `files` field, so neither Rust target can render rows. Fix the spec before the component work.
- Both Rust impls otherwise resolve dropzone colors/radius from tokens correctly; the literal violations are confined to a handful of rem/alpha constants noted above.
