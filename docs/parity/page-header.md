<!-- parity consv=fixed gpui=8 jetstream=8 specimen=gap -->
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
- Spec-struct defaults below are Rust bugs (code edits out of scope here), NOT contract↔Svelte divergences — the contract already matches Svelte:
  - `PageHeaderSpec::new` defaults `banner_tone: StatusTone::Info` (`page_header.rs:57`); contract + Svelte default `"warning"`. Spec bug → `Warning`.
  - `PageHeaderSpec::new` defaults `level: 1` (`page_header.rs:60`); contract + Svelte default `2`. Spec bug → `2`.
- `meta`/`breadcrumbs`/`banner`/`children` snippets already match (contract §3 lists them). No change.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded title-row gap `px(8.0)` at `page_header.rs:270` — resolve from `space.inline.sm` token (Svelte title gap `0.5rem`), not a raw `8.0`.
- [ ] Hardcoded actions gap `px(6.0)` at `page_header.rs:307` — use `spec` actions-gap token (Svelte `0.375rem`).
- [ ] Hardcoded banner padding `px(12.0)`/`px(8.0)` at `page_header.rs:333-334` — resolve from panel space tokens.
- [ ] Hardcoded literal rem args: back gap `rem_to_px(0.35)` (`:199`), section size `rem_to_px(0.75)` (`:234`), eyebrow size `rem_to_px(0.6875)` (`:262`) — these bypass tokens; resolve from typography/space tokens.
- [ ] Banner color built from raw `Hsla { a: … * 0.12 }` / `* 0.38` literals at `page_header.rs:321-328` — derive tint/border alpha from tokens, not magic floats.
- [ ] Heading size uses a hardcoded per-level multiplier table (`0.85/0.72/0.62/0.55/0.50`) at `page_header.rs:167-175` instead of size-token-driven hierarchy; Svelte drives title size from `data-size` + `data-level` CSS-var ladders. Approximation, not token resolution.
- [ ] No count Pill — count renders as bare text `div().text_size(body_size)` (`page_header.rs:279-286`); Svelte uses a neutral subtle Pill. No pill chrome (bg/radius/min-size).
- [ ] No banner tone icon, no `section` vs `eyebrow` precedence parity, no breadcrumbs/meta token spacing match, no contextual back dot (`back_is_contextual` only swaps text size at `:193-197`, never renders the dot).
- accepted: no ARIA (gpui has no accessibility API) — heading level not emitted as a role; `aria_label` stored, unused.
- accepted: responsive back-link `--text`/`--icon` swap omitted (no viewport breakpoints in GPUI).

## Jetstream gap (vs Svelte + contract)

- [ ] Title size is ad-hoc `size_font_rem + 0.4375` at `page_header.rs:28` — not a token; Svelte drives title size from `--poodle-page-header-title-size` per `data-size`. Resolve from a heading-size token.
- [ ] Eyebrow/back/count sizes are heuristic multipliers `* 0.75` / `* 0.875` at `page_header.rs:30-32` — resolve from typography tokens, drop magic factors.
- [ ] Hardcoded left-col + back gaps `rem_to_px(0.25)` at `page_header.rs:57,82` — Svelte title-block gap is `space.inline.sm`; resolve from token.
- [ ] Count badge geometry hardcoded: `rounded(rem_to_px(0.75))`, `pl/pr(rem_to_px(0.375))` at `page_header.rs:115-117` — Svelte uses Pill primitive (radius `999px`, padding `0 0.5rem`, min-size `1.75rem`). No Pill, wrong radius/padding, no min-size.
- [ ] Banner geometry hardcoded: `pl/pr(0.75)`, `pt/pb(0.5)`, `gap(0.5)`, icon `w/h(1.0)` at `page_header.rs:150-161` — resolve from panel/icon tokens.
- [ ] `section` clobbers `eyebrow` — only one renders (`spec.section.or(spec.eyebrow)`, `page_header.rs:85`). Svelte renders eyebrow AND section as distinct stacked rows. Also section is not uppercased and lacks the `0.08em` tracking the contract §8 section row requires.
- [ ] Back link uses `chevron-left` icon (`page_header.rs:60`); Svelte/contract specify `arrow-left`. Also no contextual dot for `back_is_contextual`, and no responsive icon-only variant.
- [ ] No actions/meta/breadcrumbs slots — `js_page_header` has no parameter or spec channel for actions, breadcrumbs, or meta content; the entire actions-row + breadcrumbs + meta anatomy is absent (Svelte snippets, GPUI `with_actions`/`with_breadcrumbs`/`with_meta`).
- accepted: no ARIA channel (documented runtime limit).
- accepted: back-link click handler lives in preview event loop, not the component.

## Specimen parity

- Svelte covers (`PageHeaderSpecimen.svelte`): Basic, With back link + actions, With eyebrow + actions, With count, Section + banner (contextual back), With breadcrumbs, With MetaBar, Title only, plus `Sizes` (xs–xl) and `Densities` (compact/default/comfortable) tabs.
- GPUI covers (`page_header_specimen.rs`): Basic, back link + actions, eyebrow + actions, count, section + banner, MetaBar, Title only, breadcrumbs. — missing: **Sizes tab**, **Densities tab** (contract §12 requires both variant tabs).
- Jetstream covers (`page_header.rs`): **Title only**, **With subtitle** — only 2 groups. Missing: back link, actions, eyebrow, count, section, banner, breadcrumbs, MetaBar, Sizes, Densities. Heaviest specimen gap of the three.

## Notes

- Two spec-struct defaults disagree with Svelte/contract and should be fixed in `packages/contracts/components/src/page_header.rs`: `banner_tone` (`Info` → `Warning`) and `level` (`1` → `2`).
- `consv=fixed`: the undocumented Svelte surface (`posture`, `showSubtitleWithBreadcrumbs`, Pill-based count, dual back link, transformed back label) and the three token/layout mismatches (context-dot color, root `align-items`, banner placement) are all now in the contract. The two remaining items — `PageHeaderSpec` `banner_tone` (`Info`→`Warning`) and `level` (`1`→`2`) defaults — are Rust spec bugs where the contract already agrees with Svelte; code-side fixes, out of scope for contract reconciliation.
- Jetstream is the furthest from parity overall: missing actions/breadcrumbs/meta slots entirely and a 2-group specimen.
