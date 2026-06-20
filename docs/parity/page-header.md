<!-- parity consv=gap gpui=8 jetstream=8 specimen=gap -->
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

- Svelte adds `posture?: "default" | "entity-detail"` (default `"default"`, line 18) → when `entity-detail` + section/title split, swaps title/section roles (`primaryTitle = section`, subtitle = title) and moves breadcrumbs into the subtitle slot (lines 64–76). Not in contract §3. **Fix: add `posture` to contract props + a `with-posture` state.**
- Svelte adds `showSubtitleWithBreadcrumbs?: boolean` (default `false`, line 15) gating subtitle visibility in entity-detail posture (line 75). Not in contract. **Fix: add to contract props.**
- `sizeRole` type/default mismatch. Contract §3 says `SemanticControlSizeRole` non-null, default `"prominent"`. Svelte declares `sizeRole?: SemanticControlSizeRole | null` default `null` (lines 28, 54), falling back to `$uiPresentation.sizeScale`. **Fix: contract type → `… | null`, default `null`.**
- Count anatomy mismatch. Contract §2 says Count is a bare `<span>` badge pill with its own background/padding/min-size token table (§8 `.page-header__count`). Svelte renders count via the `Pill` primitive (`tone="neutral" appearance="subtle"`, lines 137–141); the `.page-header__count` rule is just an `inline-flex` wrapper. **Fix: rewrite §2 + §8 count rows to "Pill primitive", drop the standalone pill token table.**
- Back link is dual, not single. Svelte renders a `--text` variant and a `--icon` variant (lines 150–168); at `max-width:45rem` the text variant hides and the icon variant shows (lines 572–578). Contract §2 documents only one `<a>`. **Fix: document both back variants + the responsive swap.**
- Back label is transformed, not raw. Svelte strips a leading `"Back"`/`"Back to "` prefix and rebuilds aria as `"Back to {label}"` (lines 90–101). Contract §2/§6 imply the literal `backLabel` is shown. **Fix: document the `resolveBackDisplayLabel`/`resolveBackAriaLabel` behavior.**
- Context-dot color mismatch. Contract §8 `.page-header__context-dot` `background: var(--poodle-color-fill-info-strong, var(--poodle-color-border-info))`. Svelte uses `var(--poodle-color-status-success, #22c55e)` (line 335). **Fix: contract token → status-success (Svelte authoritative).**
- Root `align-items` mismatch. Contract §8 root says `align-items: end`. Svelte root uses `align-items: start` (line 246). **Fix: contract → `start`.**
- Banner placement mismatch. Contract §8 + anatomy say banner spans grid columns (`grid-column: 1 / -1`). Svelte is not a grid; banner is a flex child with `margin-top: var(--poodle-page-header-banner-margin-top)` (lines 461–463). **Fix: reconcile §7/§8 layout description to the flex/stacked model Svelte uses.**
- `banner_tone` default mismatch (spec vs Svelte). Svelte default `bannerTone="warning"` (line 49) and contract §3 agrees (`"warning"`), but `PageHeaderSpec::new` defaults `banner_tone: StatusTone::Info` (`page_header.rs:57`). **Fix: spec default → `Warning` to match Svelte/contract.** (spec bug, not Svelte)
- `level` default mismatch (spec vs contract/Svelte). Contract §3 + Svelte default `level = 2`. `PageHeaderSpec::new` defaults `level: 1` (`page_header.rs:60`, comment says "Defaults to 1"). **Fix: spec default → `2`.** (spec bug)
- Svelte exposes `meta`/`breadcrumbs`/`banner`/`children` snippets (lines 31–34) — contract §3 lists them, OK. No divergence there.

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
- The `consv=gap` driver is undocumented Svelte surface (`posture`, `showSubtitleWithBreadcrumbs`, Pill-based count, dual back link, transformed back label) plus three contract token mismatches (context-dot color, root `align-items`, banner placement). All belong in the contract per "Svelte is parity authority".
- Jetstream is the furthest from parity overall: missing actions/breadcrumbs/meta slots entirely and a 2-group specimen.
