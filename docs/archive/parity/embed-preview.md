<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok --><!-- pass: both targets gain trustedHtml state, real Skeleton(block) loading, ratio-derived placeholder height; unsanctioned pill+heading removed; token-resolved --><!-- pass: GPUI specimen done; Jetstream pending engine recovery --><!-- pass: Jetstream specimen now mirrors GPUI 7 cases (YouTube/Vimeo/generic iframe/trusted raw/loading/error/empty) via real js_embed_preview; both previews build clean -->
# Parity: EmbedPreview

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/embed-preview.md`
- Svelte (authoritative): `packages/svelte/components/src/EmbedPreview.svelte`
- GPUI: `packages/gpui/components/src/composites/embed_preview.rs`
- Jetstream: `packages/jetstream/components/src/embed_preview.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/EmbedPreviewSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/embed_preview_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/embed_preview.rs`

## Contract ↔ Svelte

`consv=fixed`. Svelte matches contract props/anatomy/states; class-prefix drift + fallback-link composition reconciled.

- [x] FIXED Class prefix reconciled to `.poodle-embed-preview*` throughout anatomy + §8 token tables + §9 note.
- [x] FIXED (extra) Fallback link is the `TextLink` primitive (`.poodle-text-link`, `EmbedPreview.svelte:96-100,187`), not a bare `<a>` — anatomy, parts table, §7/§8/§10 + §9 notes updated; `embedType?` added to the shared `ParsedEmbed` type for cross-contract consistency.
- All 6 props (parsed/trustedHtml/aspectRatio/loading/error/emptyMessage) with matching defaults (`EmbedPreview.svelte:7-21`), all 7 states in priority order (loading > error > empty > iframe > raw embed > trusted HTML > fallback), derived `embedUrl`/`isAudio`/`effectiveAspectRatio`, and ARIA (iframe `title`/`sandbox`/`loading=lazy`/`allowfullscreen`/`frameborder=0`, fallback `target=_blank rel=noopener noreferrer`) all match.

## GPUI gap (vs Svelte + contract)

- [x] FIXED `trustedHtml` render state added — `EmbedPreviewSpec.trusted_html` (additive field) renders caller-sanitized HTML in the aspect-ratio container in the contract priority slot (after raw embed, before fallback). Empty-state trigger now `is_empty_state()` (`!parsed && !trustedHtml`).
- [x] FIXED `aspectRatio` drives placeholder height — the media container derives min-height from `effective_aspect_ratio()` against a nominal reference width; `"auto"`/None falls back to the contract static 10rem media height. (Live iframe remains the sanctioned placeholder per GPUI Notes.)
- [x] FIXED Loading uses the real `Skeleton` primitive (`shape="block"`) + LoadingText — hand-built bars gone.
- [x] FIXED Removed unsanctioned provider pill + "Raw embed code" heading. Root is now radius-surface + overflow-hidden (no extra border/padding); each state child carries the panel bg, matching the Svelte anatomy. Placeholder media frame retained (sanctioned).
- [x] FIXED Token-resolved: state gap → `space.inline.sm`, text → `typography.label.size`, fallback padding → `space.panel.y`/`space.panel.x`. min-h 8rem / padding 1.5rem / icon 2rem have no named token → exact rem (noted). Icon size set via `with_px_size(2rem)`.
- accepted: placeholder panel instead of live iframe (GPUI Notes §sanctioned); no iframe ARIA (no live web view).

## Jetstream gap (vs Svelte + contract)

- [x] FIXED `trustedHtml` render state added (shared `EmbedPreviewSpec.trusted_html`) — renders in the aspect-ratio container in the contract priority slot; empty-state uses `is_empty_state()`.
- [x] FIXED `aspectRatio` drives placeholder min-height (ratio against nominal reference width); `"auto"`/None → static 10rem media height.
- [x] FIXED Loading uses the real `js_skeleton` primitive (`shape="block"`) — hand-built bars gone.
- [x] FIXED Removed unsanctioned provider pill + "Raw embed code" heading; root is radius-surface + overflow-hidden with per-state panel bg. Placeholder media frame retained (sanctioned).
- [x] FIXED Token-resolved where the contract rem maps to a named token (gap → `space.inline.sm`, text → `typography.label.size`, fallback padding → `space.panel.y`/`.x`); contract-exact rems with no token (8rem/1.5rem/2rem/10rem) kept as explicit rem (noted). Fallback now composes the real `js_text_link`.
- accepted: placeholder panel instead of live iframe; no ARIA (Jetstream has no accessibility API).

## Probe tests (Jetstream)

`embed_preview::tests` (render_probe, theme DARK): loading shows the Skeleton block (6rem) + "Loading preview..."; error shows the alert icon + message (no parsed leak); empty shows the custom message + icon; YouTube parsed shows the provider title + derived `youtube-nocookie.com/embed/...` URL; trustedHtml renders in-container (not empty state); fallback renders the TextLink to the original id/url when no embed URL.

## Specimen parity

- Svelte covers: YouTube, Vimeo, Trusted raw embed (`trustedHtml` + `aspectRatio="auto"`), Loading, Error, Empty (custom message). 6 cases incl. trusted.
- GPUI covers: YouTube, Vimeo, Generic iframe (raw `original_embed`), **Trusted raw embed (`with_trusted_html` + `with_auto_aspect_ratio`)**, Loading, Error, Empty (custom). 7 cases. **GPUI specimen complete** — every contract state plus the trustedHtml priority slot now exercised; matches the Svelte specimen's case set.
- Jetstream covers: YouTube, Loading, Empty (default message). 3 cases. — missing: **Vimeo, Error, raw embed, custom empty message, trustedHtml** (weakest coverage).

## Notes

- `trustedHtml` is missing in both Rust targets — the clearest cross-target prop gap. `aspectRatio` is degraded to a binary min-height switch in both rather than driving real ratio layout.
- Both Rust targets converge on the same divergence pattern: provider pill + "Raw embed code" heading + hand-built skeleton + placeholder media frame. Of these, only the placeholder frame is contract-sanctioned (GPUI Notes); the pill/heading/hand-skeleton are unsanctioned additions, and the Skeleton primitive should replace the hand-built bars.
- GPUI carries 9 raw `px()` literals; Jetstream uses `rem_to_px` magic numbers (softer but still bypasses named tokens).
