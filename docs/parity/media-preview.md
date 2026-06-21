<!-- parity consv=fixed gpui=1 jetstream=1 specimen=gap -->
# Parity: MediaPreview

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/media-preview.md`
- Svelte (authoritative): `packages/svelte/components/src/MediaPreview.svelte`
- GPUI: `packages/gpui/components/src/composites/media_preview.rs`
- Jetstream: `packages/jetstream/components/src/media_preview.rs`
- Spec: `packages/contracts/components/src/media_preview.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/MediaPreviewSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/media_preview_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/media_preview.rs`

## Contract ↔ Svelte

Contract and Svelte agree on all 17 props + 2 snippets, composition (Card + MediaThumbnail), size/density tables, and ARIA. `consv=fixed`: contract↔Svelte is clean — no contract edit. The `footer_actions` invention is Rust-spec-side only and must not be added to the contract (Svelte has no footer); the fix is to remove it from the Rust spec/GPUI.

- [x] VERIFIED Contract↔Svelte: all props match `MediaPreview.svelte:10-52` (`title`, `description`, `eyebrow`, `caption`, `meta`, `badge`, `thumbnailMeta`, `kind`, `state`, `aspectRatio`, `variant`, `ariaLabel`, `stateTitle`, `stateMessage`, `size`, `sizeRole`, `density`) plus `mediaContent`/`children` snippets. Contract carries no footer region — confirmed clean. Meta-list `aria-label`, `data-size`/`data-density`, UiPresentationProvider wrap all present.
- **Spec invention (Rust-side, not contract↔Svelte)**: `MediaPreviewSpec` adds `footer_actions: Vec<RemediationAction>` (`media_preview.rs:15`) with no counterpart in contract or Svelte. GPUI renders a footer-actions bar (`media_preview.rs:229-255`). **Fix is in the Rust spec/GPUI: remove the fabricated footer region.** The contract is intentionally left without a footer to match Svelte.

## GPUI gap (vs Svelte + contract)

GPUI builds its own card border + info block instead of composing the `Card` primitive; several anatomy parts missing or fabricated.

- [ ] Hardcoded px literals: `gap(px(12.0))` (`:133`), `gap(px(6.0))` (`:171`), `py(px(8.0))` (`:236`) — resolve from `space.stack.*` / density tokens (contract §9 header-gap/section-gap tables).
- [ ] Does not compose `Card` — hand-rolls border+radius container (`:129-137`); contract §10 requires `Card` with `variant` + `media=true`. `variant` prop entirely ignored (spec has no `variant`).
- [ ] No `eyebrow` — spec lacks the field; contract Eyebrow part (uppercase label above title) unimplemented.
- [ ] No `caption` body paragraph — spec lacks `caption`; contract Body caption unimplemented.
- [ ] Metadata rendered as middot-separated inline text (`:206-221`) instead of pill-styled `<li>` chips with surface bg + control radius (contract §9 Meta List Item).
- [ ] `footer_actions` region (`:229-255`) is a fabricated surface not in contract/Svelte — remove or contractualize.
- [ ] No `size`/`sizeRole`/`density` resolution — spec lacks these; size-variant + density tables (contract §9) unimplemented; `data-size`/`data-density` not published.
- [ ] No `thumbnailMeta` prepend distinct from metadata styling — it is chained into the same middot row (`:198-204`), losing the contract's "prepended meta item" semantics (minor, but pill styling missing).
- accepted: no ARIA (Card `aria-label`, meta `aria-label` not emitted).

## Jetstream gap (vs Svelte + contract)

Skeletal — surface box + title label only (`media_preview.rs:15-22`). Almost nothing implemented.

- [ ] Hardcoded px literals: padding `rem_to_px(0.75)` ×4 (`:20`), title `text_size(rem_to_px(0.875))` (`:22`), gap `rem_to_px(0.5)` (`:19`) — magic numbers; resolve from tokens.
- [ ] No MediaThumbnail composition — the framed media region (the defining part) is absent; `kind`/`state`/`aspectRatio`/`badge` all ignored.
- [ ] No eyebrow, description, caption, metadata chips — only `title` renders.
- [ ] No `variant`/`size`/`density` handling (spec lacks the fields).
- [ ] No state posture (loading/error/empty) — `state` ignored.
- accepted: no ARIA / interaction.

## Specimen parity

- Svelte covers: Image preview (eyebrow, 3 meta chips, mediaContent placeholder), Video preview (video ratio, 2 meta), Error state (document, `state="error"`, stateTitle/stateMessage), plus size/density demonstration instances (`MediaPreviewSpecimen.svelte`).
- GPUI covers: Image preview, Video preview, Error state (`media_preview_specimen.rs`). — missing: eyebrow rendering (impl drops it), pill-styled meta, size/density variants.
- Jetstream covers: Image preview, Audio-with-description (`media_preview.rs:13-19`). — missing: media frame, eyebrow, meta, error state. Demonstrates only the skeletal box+title.

## Notes

- RESOLVED (2026-06-21): `MediaPreviewSpec` gained additive `eyebrow`,
  `caption`, `variant` (`CardVariant`), `size` (`ControlSize`), `size_role`,
  `density` (`ControlDensity`) plus size/density rem tables (`eyebrow_size_rem`,
  `title_size_rem`, `body_size_rem`, `meta_padding_rem`, `header_gap_rem`,
  `section_gap_rem`) and meta/title token accessors. Both Rust targets now
  compose Card (GPUI `Card` primitive / Jetstream `js_card`) with a nested
  MediaThumbnail in the media slot, a header (eyebrow uppercased / title /
  description + pill-styled meta chips with surface-mix bg + control radius),
  and a body caption. `thumbnail_meta` prepends to the meta list. Jetstream
  covered by 4 render_probe tests.
- The fabricated footer region is no longer rendered by either target (matches
  the contract, which carries no footer). The `footer_actions` spec field +
  `with_footer_actions`/`has_footer_actions` are RETAINED (additive) because the
  GPUI preview `demo_view.rs` and the contracts smoke test still reference them
  and could not be build-verified in this pass (gpui preview target lock /
  skipped build). Follow-up: drop the field once the preview can be rebuilt.
- Residual (preview-loop only): inherited from MediaThumbnail (frame gradient,
  spinner animation); light-theme meta box-shadow override is not painted. ARIA
  remains accepted-out.
