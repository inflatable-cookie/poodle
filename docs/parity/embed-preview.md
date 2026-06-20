<!-- parity consv=gap gpui=7 jetstream=5 specimen=gap -->
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

Svelte matches the contract props/anatomy/states; one class-prefix drift.

- Class prefix drift: implementation uses `.poodle-embed-preview*`; contract anatomy/CSS uses `.embed-preview*`. **Fix: reconcile contract to `.poodle-` prefix.**
- All 6 props (parsed/trustedHtml/aspectRatio/loading/error/emptyMessage) with matching defaults (`EmbedPreview.svelte:7-21`), all 7 states in priority order (loading > error > empty > iframe > raw embed > trusted HTML > fallback), derived `embedUrl`/`isAudio`/`effectiveAspectRatio`, and ARIA (iframe `title`/`sandbox`/`loading=lazy`/`allowfullscreen`/`frameborder=0`, fallback `target=_blank rel=noopener noreferrer`) all match. consv otherwise ok — gap is the prefix only.

## GPUI gap (vs Svelte + contract)

- [ ] `trustedHtml` prop entirely absent — no trusted-HTML render state (contract §4 trusted HTML). Tier-1 prop-parity break.
- [ ] `aspectRatio` only consumed as `effective_aspect_ratio().is_some()` for a binary min-height pick (`embed_preview.rs:165-167`) — no actual aspect-ratio layout; contract fixed-ratio container not honored.
- [ ] Skeleton is 3 hand-built bars (`embed_preview.rs:69-96`), not the Skeleton primitive (`shape="block"`) the contract anatomy mandates.
- [ ] Adds contract-undefined anatomy: provider pill (`:155-184`) + "Raw embed code" heading (`:224`) + media placeholder panel. (Placeholder-vs-iframe is sanctioned by GPUI Notes; the pill/heading are not.)
- [ ] Hardcoded px: skeleton `.h(px(12.0))` `:72,77,82` / `.w(px(160.0))` `:76` / `.w(px(240.0))` `:81` / `.py(px(4.0))` `:91`; media frame `px(200.0)` `:165` / `px(160.0)` `:167`; pill `.py(px(2.0))` `:216` — resolve via `resolve_px`.
- [ ] No iframe ARIA (`title`/`sandbox`/etc.) — N/A given placeholder render, but the trusted/raw paths also lack semantics.
- accepted: placeholder panel instead of live iframe (GPUI Notes §sanctioned).

## Jetstream gap (vs Svelte + contract)

- [ ] `trustedHtml` prop entirely absent — no trusted-HTML render state.
- [ ] `aspectRatio` only consumed as `.is_some()` for min-height (`embed_preview.rs:98`) — no aspect-ratio layout.
- [ ] Skeleton is 2 hand-built bars (`embed_preview.rs:34-47`), not the Skeleton primitive.
- [ ] Adds contract-undefined anatomy: provider pill (always when parsed, `:77-88`) + "Raw embed code" heading (`:117`) + media placeholder panel.
- [ ] Literal rem dimensions via `rem_to_px(...)` magic numbers throughout (`embed_preview.rs:19-25,99,101,110-113` etc.) — bypass named-token resolution; resolve from space/typography tokens.
- accepted: placeholder panel instead of live iframe; no ARIA (Jetstream has no accessibility API).

## Specimen parity

- Svelte covers: YouTube, Vimeo, Trusted raw embed (`trustedHtml` + `aspectRatio="auto"`), Loading, Error, Empty (custom message). 6 cases incl. trusted.
- GPUI covers: YouTube, Vimeo, Generic iframe (raw `original_embed`), Loading, Error, Empty (custom). 6 cases. — missing: **trustedHtml** path (uses raw-embed instead).
- Jetstream covers: YouTube, Loading, Empty (default message). 3 cases. — missing: **Vimeo, Error, raw embed, custom empty message, trustedHtml** (weakest coverage).

## Notes

- `trustedHtml` is missing in both Rust targets — the clearest cross-target prop gap. `aspectRatio` is degraded to a binary min-height switch in both rather than driving real ratio layout.
- Both Rust targets converge on the same divergence pattern: provider pill + "Raw embed code" heading + hand-built skeleton + placeholder media frame. Of these, only the placeholder frame is contract-sanctioned (GPUI Notes); the pill/heading/hand-skeleton are unsanctioned additions, and the Skeleton primitive should replace the hand-built bars.
- GPUI carries 9 raw `px()` literals; Jetstream uses `rem_to_px` magic numbers (softer but still bypasses named tokens).
