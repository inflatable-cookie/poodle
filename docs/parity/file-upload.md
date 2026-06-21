<!-- parity consv=ok gpui=0 jetstream=0 specimen=ok -->
<!-- pass: file-list anatomy built on both targets; FileUploadItem/FileUploadStatus added to spec; tokenized item rows (preview/icon, meta, progress, remove, error), accent browse, density padding, size table, color-mix helpers; specimens populated. -->
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

- [x] **File-list anatomy built.** `file_item()` now renders File List → File Item rows: Preview surface OR File Icon (2rem square), Meta (Name ellipsis + Size/status or error text), conditional Progress track+bar (uploading), and the Remove pill — all token-resolved, driven by `spec.files`.
- [x] **Spec carries file-list data.** Added `FileUploadItem` (`id`, `name`, `size`, `progress`, `status`, `has_preview`, `error`) + `FileUploadStatus` enum + `files: Vec<FileUploadItem>` field and item-level token methods to `FileUploadSpec`. Additive — defaults preserve prior behaviour.
- [x] Validation-error stack + error text now use `space.stack.sm` gap and the size-table hint font (no raw `0.375`/`0.75`).
- [x] Dropzone hover routes the 50% panel mix through `color_mix(panel, transparent, 0.50)`; active state through `color_mix(accent, transparent, 0.08)`.
- [x] **Copy aligned.** Label is `"Drop files here or"` + an inline underlined accent `browse` span (drag → `"Drop to upload"`); hint is `accept · Max <formatFileSize>`.
- [x] Browse affordance is inline underlined accent text inside the label (no bordered button).
- [x] **Size variants applied.** Dropzone min-height (5/6/8/10/12rem), icon (1.5/2/2.5rem via `Icon::with_px_size`), and label/hint font now resolve from the contract §8 size table.
- accepted: no ARIA (gpui has no accessibility API) — dropzone `role="button"`/`tabindex` and remove-button labels not emitted.
- accepted: native file dialog + drag-and-drop platform integration owned by runtime (contract §10, Known Delta).

## Jetstream gap (vs Svelte + contract)

- [x] **File-list anatomy built.** `js_file_upload` now renders the conditional file list via `file_item()`: Preview surface OR File Icon, Meta (Name ellipsis + Size/status), conditional Progress (flex-proportional accent fill — JsEl has no % width, noted), and the Remove pill button. Driven by `spec.files`.
- [x] Content gap, item padding, and density padding now resolve from per-density helpers (compact 1rem / default 1.5rem / comfortable 1.75rem); border-width and root-gap are contract-exact rem (`rem_to_px` of 0.125/0.5 — no token exists for those, noted).
- [x] Drag-tint routes through `tint(accent, 0.08)`; hover-bg through `color_mix(panel, transparent, 0.50)` — no raw `Color::new` alpha math.
- [x] **Padding driven by density** (compact/comfortable overrides applied).
- [x] **Copy aligned.** Label `"Drop files here or"` + separate accent `browse` run; hint is `accept · Max <format_file_size>`.
- [x] `show_preview` now gates the preview-vs-file-icon leading element per item.
- [x] Validation-error line rendered below the dropzone (status.danger).
- [x] Size variants verified against the contract §8 table.
- accepted: no ARIA channel (documented pattern).
- accepted: drag/drop + file-dialog interaction lives in the preview event loop, not the component. Image preview bitmaps host-owned — `has_preview` only drives anatomy.
- note: JsEl has no `%` width, so the progress fill is modelled as a flex pair (filled run grows `progress` parts, remainder grows the rest) — proportional + token-driven.

## Specimen parity

- Svelte covers: Image upload with preview, Document upload (single, no preview), Compressed image upload + custom validation, Disabled, Last-error display; plus size and density snippets (`FileUploadSpecimen.svelte`).
- GPUI covers: Image upload, **Populated file list** (preview + progress + complete + error rows + remove), Document upload (no preview, single row), Dragging, Disabled, Max-files cap, Image compression, Validation error.
- Jetstream covers: Default, Accept-images-only, **Image upload with list** (preview + progress), **Document upload + error row**, Validation error, Dragging, **Small/compact + Large/comfortable size/density**, Disabled.

## Notes

- `consv=ok`: contract and Svelte are aligned on props, methods, callbacks, and `FileUploadItem`. Only token-name drift (`accent-default` vs `accent-base`) and an internal Svelte inconsistency on the accent token — neither is a behavioral divergence.
- The dominant gap across both Rust targets is the **missing file-list anatomy** (required parts: File Item, Meta, Name, Size, Remove). This is a spec gap first: `poodle-specs` has no `FileUploadItem` type or `files` field, so neither Rust target can render rows. Fix the spec before the component work.
- Both Rust impls otherwise resolve dropzone colors/radius from tokens correctly; the literal violations are confined to a handful of rem/alpha constants noted above.
