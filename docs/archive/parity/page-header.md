<!-- parity consv=fixed gpui=1 jetstream=1 specimen=ok -->
# Parity: PageHeader

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/page-header.md`
- Svelte (authoritative): `packages/svelte/components/src/PageHeader.svelte`
- GPUI: `packages/gpui/components/src/composites/page_header.rs`
- Jetstream: `packages/jetstream/components/src/page_header.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/PageHeaderSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/page_header_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/page_header.rs`

## Contract ↔ Svelte

Svelte carries props + anatomy the contract does not document. Svelte is authoritative — update the contract.

- [x] FIXED — `posture?: "default" | "entity-detail"` (default `"default"`) added to contract §3 props; entity-detail behavior (section↔title swap, breadcrumbs into subtitle slot) documented in §4 states + §4 derived component states (`isEntityDetailPosture`, `resolvedSubtitle`) + §9 notes.
- [x] FIXED — `showSubtitleWithBreadcrumbs?: boolean` (default `false`) added to contract §3 props with its entity-detail gating note.
- [x] FIXED — `sizeRole` type/default: contract §3 changed to `SemanticControlSizeRole | null`, default `null` (falls back to inherited `sizeScale`); §9 note added.
- [x] FIXED — Count anatomy: §2 anatomy + parts + §8 `.page-header__count` rewritten to "Pill primitive (`tone="neutral"`, `appearance="subtle"`)"; the standalone pill token table (bg/radius/min-size) dropped — count wrapper is now just `inline-flex; align-items: center`.
- [x] FIXED — Back link is dual: §2 anatomy + parts now document both `--text` and `--icon` variants and the `max-width: 45rem` swap; §8 adds the `.page-header__back--icon` table.
- [x] FIXED — Back label transform: §6 adds a "Back-Link Label Resolution" subsection documenting `resolveBackDisplayLabel` (strips leading `Back`/`Back to`) and `resolveBackAriaLabel` (`"Back to {label}"`).
- [x] FIXED — Context-dot color: §8 `.page-header__context-dot` background → `var(--poodle-color-status-success, #22c55e)`; the `--overlay` icon-variant dot table added.
- [x] FIXED — Root `align-items`: §8 root → `start`.
- [x] FIXED — Banner placement: §7 + §8 + §2 reconciled to the stacked/flex model — banner is a root grid child with `margin-top: var(--poodle-page-header-banner-margin-top)`, not a `grid-column: 1 / -1` span; the `data-align="between"` two-column split is documented on `__top-row`, not the root.
- [x] FIXED — `PageHeaderSpec::new` default `banner_tone` corrected `StatusTone::Info → Warning` (matches contract + Svelte).
- [x] FIXED — `PageHeaderSpec::new` default `level` corrected `1 → 2` (matches contract + Svelte).
- `meta`/`breadcrumbs`/`banner`/`children` snippets already match (contract §3 lists them). No change.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] FIXED — title-row gap now `spec.title_gap_token()` (`space.inline.sm`); title-block gap via `spec.title_block_gap_token()`.
- [x] FIXED — actions gap now `spec.actions_gap_token()` (`space.inline.sm`, closest token to Svelte `0.375rem`).
- [x] FIXED — banner padding now contract-exact rems (`pl/pr 0.75`, `pt/pb 0.5`) via `rem_to_px`; radius via `spec.banner_radius_token()`.
- [x] FIXED — eyebrow `0.6875rem` and section `0.75rem` are contract-exact rems (sanctioned `rem_to_px`); back-link gap `0.35rem` is the contract literal. Title size is now token-driven (below).
- [x] FIXED — banner tint now via `theme_ext::color_mix(banner_tone, surface, 0.12/0.38)`, not raw `Hsla{a: …}` floats; left-accent border (`border_l_2`).
- [x] FIXED — title size now `heading_size_token()` (`typography.heading.size`) × per-level scale (level 2 = base, level 1 larger, 3–6 compact); default level corrected `1 → 2`.
- [x] FIXED — count renders via the `Pill` primitive (`tone=Neutral`, `appearance=Subtle`, size from `resolve_supporting_visual_size`), not bare text.
- [x] FIXED — banner tone icon rendered (`Icon arrow-left`/tone glyph); eyebrow AND section both render as distinct stacked rows; contextual back dot now drawn (`status.success` color) when `back_is_contextual`; back link uses `arrow-left` icon + stripped display label.
- accepted: no ARIA (gpui has no accessibility API) — heading level not emitted as a role; `aria_label` stored, unused.
- accepted: responsive back-link `--text`/`--icon` swap omitted (no viewport breakpoints in GPUI).

## Jetstream gap (vs Svelte + contract)

- [x] FIXED — title size now `heading_size_token()` × per-level scale (level 2 = base), not `size_font_rem + 0.4375`.
- [x] FIXED — eyebrow `0.6875rem` and section `0.75rem` are contract-exact rems; back `0.8125rem` and count via the `Pill` size ladder — magic `* 0.75` / `* 0.875` factors gone.
- [x] FIXED — title-block gap now `spec.title_block_gap_token()`; title→count gap via `spec.title_gap_token()`; actions gap via `spec.actions_gap_token()`.
- [x] FIXED — count renders via `js_pill` (`tone=Neutral`, `appearance=Subtle`, size from `resolve_supporting_visual_size`); the bespoke badge geometry is gone.
- [x] FIXED — banner radius via `spec.banner_radius_token()`; fill via `color_mix(banner_tone, panel, 0.12)`; left-accent border; icon at contract `1rem`.
- [x] FIXED — eyebrow AND section both render as distinct stacked rows (eyebrow first, then section); both uppercased (JsEl has no text-transform so strings are pre-uppercased — documented runtime delta for the `0.08em` tracking).
- [x] FIXED — back link uses `arrow-left` icon; contextual dot (`status.success`) rendered when `back_is_contextual`; back label resolved via `spec.back_display_label()`. (Responsive icon-only variant omitted — no viewport breakpoints in Jetstream.)
- [x] FIXED — added `js_page_header_with_slots(spec, theme, breadcrumbs, actions, meta)` (the `js_app_header_with_slots` convention); `js_page_header` is now a thin wrapper. Actions-row (back + actions cluster), breadcrumbs, and meta anatomy all present.
- accepted: no ARIA channel (documented runtime limit).
- accepted: back-link click handler lives in preview event loop, not the component.

Probe tests (`page_header.rs` `#[cfg(test)] mod tests`): title + subtitle render; eyebrow AND section render as distinct uppercased rows; count Pill carries "128" + back link uses `arrow-left` + stripped "Library" label; actions slot renders; banner renders with `alert-triangle` tone icon.

## Specimen parity

- Svelte covers (`PageHeaderSpecimen.svelte`): Basic, With back link + actions, With eyebrow + actions, With count, Section + banner (contextual back), With breadcrumbs, With MetaBar, Title only, plus `Sizes` (xs–xl) and `Densities` (compact/default/comfortable) tabs.
- GPUI covers (`page_header_specimen.rs`): Basic, back link + actions, eyebrow + actions, count, section + banner, MetaBar, Title only, breadcrumbs. — missing: **Sizes tab**, **Densities tab** (contract §12 requires both variant tabs).
- Jetstream covers (`page_header.rs`): Basic (title + subtitle), Eyebrow + count + actions, Section + back link (contextual) + banner, Title only, Sizes (xs–xl) — now broad. (Densities tab still optional; size coverage in place.)

## Notes

- **Fix pass (both Rust targets):** count now renders via the `Pill` primitive (GPUI `Pill`, Jetstream `js_pill`; `tone=Neutral`/`appearance=Subtle`, size from `resolve_supporting_visual_size`); title size is token-driven (`typography.heading.size` × per-level scale, level 2 = base); eyebrow AND section render as distinct stacked rows; back link uses `arrow-left` + stripped display label + contextual `status.success` dot; banner tint via `color_mix` (no magic alpha floats) with token radius. Jetstream gained `js_page_header_with_slots` (breadcrumbs/actions/meta `JsEl` slots) — the actions-row/breadcrumbs/meta anatomy is no longer absent. Additive `PageHeaderSpec` surface: field `entity_detail_posture` + `with_entity_detail_posture`, derived helpers (`has_section_title_split`, `is_entity_detail_posture`, `primary_title`, `resolved_subtitle`, `back_display_label`, `back_aria_label`), and token methods (`context_dot_color_token`, `section_color_token`, `heading_size_token`, `title_gap_token`, `title_block_gap_token`, `actions_gap_token`, `banner_radius_token`); `gap_token()` corrected `space.stack.sm → space.stack.md`. The two spec-default bugs (`banner_tone` Info→Warning, `level` 1→2) are fixed. Jetstream `#[cfg(test)] mod tests` added (render_probe). Preview-loop (not closed): back-link click + action behavior; responsive back-link icon-only swap is a no-breakpoint runtime delta. JsEl gap noted: no text-transform/letter-spacing, so eyebrow/section are pre-uppercased and the `0.08em` section tracking is approximated.
- `consv=fixed`: the undocumented Svelte surface (`posture`, `showSubtitleWithBreadcrumbs`, Pill-based count, dual back link, transformed back label) and the three token/layout mismatches (context-dot color, root `align-items`, banner placement) are all now in the contract. The two remaining items — `PageHeaderSpec` `banner_tone` (`Info`→`Warning`) and `level` (`1`→`2`) defaults — are Rust spec bugs where the contract already agrees with Svelte; code-side fixes, out of scope for contract reconciliation.
- Jetstream is the furthest from parity overall: missing actions/breadcrumbs/meta slots entirely and a 2-group specimen.
