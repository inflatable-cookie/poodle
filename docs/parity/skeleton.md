<!-- parity consv=fixed gpui=6 jetstream=8 specimen=gap -->
# Parity: Skeleton

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/skeleton.md`
- Svelte (authoritative): `packages/svelte/components/src/Skeleton.svelte`
- GPUI: `packages/gpui/components/src/primitives/skeleton.rs`
- Jetstream: `packages/jetstream/components/src/skeleton.rs`
- Spec: `packages/contracts/components/src/skeleton.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/SkeletonSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/skeleton.rs` · jetstream `packages/jetstream/preview/src/specimens/skeleton.rs`

## Contract ↔ Svelte

Contract and Svelte agree on the public prop surface (`shape`/`preset`/`width`/`height`/`lines`/`animated`), all six render branches, and the `aria-hidden="true"` + `data-animated` attributes. Svelte is faithful. The divergences are between the **contract/Svelte vocabulary** and the **Rust `SkeletonSpec`** — those are spec bugs, listed in the target sections. Two contract-side items:

- [x] FIXED (contract already correct): Contract §3 names the shape union `"line" | "block" | "circle"` and Svelte matches it (`SkeletonShape`, `data-shape={shape}`). The divergence is the Rust `SkeletonSpec.shape` (`"rectangle"`/`"text"`/`"circle"`, `skeleton.rs:35,86-89`) — a **Rust spec rename (code, out of scope here)**. The contract needs no change; do NOT change the contract.
- [x] FIXED: Contract §6 now states the single-shape element also sets `aria-hidden="true"` (matching Svelte's single-shape `<span>`, `Skeleton.svelte:78`), not just preset containers.
- Animation is fully specified (contract §8 keyframes `200% 0` → `-20% 0`, `1.6s linear infinite`, `background-size: 220% 100%`, 3-stop `color-mix` gradient) and Svelte implements it verbatim (`Skeleton.svelte:87-118`). This is a real shimmer (background-position sweep), not a pulse — relevant to the Rust gaps below.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Animation is a **pulsing opacity fade**, not the contract shimmer — `el.opacity(0.3 + delta * 0.4)` over a 1500ms ease-in-out repeat (`skeleton.rs:95,209-218`). Contract §8 requires a 1.6s *linear* `background-position` sweep across a 3-stop gradient. BEHAVIOR gap: implement a moving-gradient shimmer (or document a Known Delta); also wrong duration (1500ms vs 1600ms) and wrong easing (ease-in-out vs linear).
- [ ] Static fill uses hardcoded `el.opacity(0.5)` (`skeleton.rs:99,222`) — no token; the gradient fill is also absent (single flat `bg(fill)`), so the contract's 3-stop gradient never renders.
- [ ] Hardcoded pixel literals throughout preset bones: `px(120.0)`, `px(80.0)`, `px(200.0)`, `px(60.0)` (table-row `skeleton.rs:115-118`); `px(9999.0)`, `px(180.0)`, `px(240.0)` (card `skeleton.rs:127-129`); `px(160.0)`, `px(100.0)` (list-item `skeleton.rs:146-148`); `px(140.0)`, `px(9999.0)` (detail `skeleton.rs:156,159`); `px(120.0)`, `px(80.0)` (avatar-line `skeleton.rs:175-176`). Contract widths are percentages/rem (40/60/60/20%, 80/100/60%, 10rem) — resolve from rem or percentage relatives, not raw px.
- [ ] Hardcoded radius magic number `px(rem_to_px(0.6875))` for line-sm height (`skeleton.rs:148`) and ad-hoc multipliers `default_height * 1.2` (`skeleton.rs:156`), `default_height * 0.85` (`skeleton.rs:176`) — invent values not traceable to any contract token.
- [ ] GPUI card preset omits the contract's pill footer (`.skeleton--pill` 3.5rem × 1.25rem, 999rem radius) — card renders header + 2 body lines only (`skeleton.rs:121-130`), missing the body's 3rd line (80/100/60%) and both footer pills.
- [ ] GPUI `Card` preset uses `bone(px(9999.0), …)` as a faux full-width hack (`skeleton.rs:127,159`) instead of `w_full()` — magic sentinel width.
- accepted: no ARIA (gpui has no accessibility API) — skeletons stay decorative; `aria-hidden` cannot be emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] No animation at all — file header states "Jetstream cannot animate, so skeletons render as static gray boxes" (`skeleton.rs:3`); `is_animated` / the `with_animated(false)` specimen path are inert. BEHAVIOR gap vs contract §4 shimmer. Note: contract Known-Delta covers technique variance but not *absence*; if shimmer is driven by the preview event loop, it is not present here — confirm and document, else flag as missing.
- [ ] `shape` vocabulary mismatch: Jetstream matches on `"circle"` / `"text"` / default-rectangle (`skeleton.rs:35-65`), not the contract's `line`/`block`/`circle`. `"line"`/`"block"` from contract fall through to the rectangle default. Fix once the spec is renamed.
- [ ] Hardcoded rem literals in single-shape: `rem_to_px(0.75)` text height (`skeleton.rs:44`). Contract line height is `0.875rem`, not `0.75rem`.
- [ ] Hardcoded radius literal `rounded(9999.0)` in the `circle` helper (`skeleton.rs:89`) — bypasses `radius_token()` (which already resolves `RADIUS_PILL`); use the resolved pill radius.
- [ ] Preset dimensions hardcoded as rem literals not matching contract: avatar-line avatar `rem_to_px(2.0)` (`skeleton.rs:96`) vs contract `2.25rem`; list-item icon `rem_to_px(1.25)` (`skeleton.rs:113`) vs contract avatar `2.25rem`; card image `rem_to_px(8.0)` (`skeleton.rs:139`) vs contract block-header `6rem`. All magic numbers.
- [ ] Preset structural gaps vs contract: avatar-line renders avatar + **two** stacked lines (`skeleton.rs:97-108`) but contract avatar-line is avatar + **single** 10rem line; card omits the 3rd body line and both footer pills (`skeleton.rs:134-142`); list-item is a single growing line (`skeleton.rs:119`) but contract is primary 60% line + secondary `line-sm` 40% line; table-row uses 4 equal grow cells (`skeleton.rs:122-132`) but contract widths are 40/60/60/20%.
- [ ] Detail-section omits the per-row label/value structure — renders header + flat full-width lines (`skeleton.rs:144-155`), but contract detail rows are `label` (6rem, flex-shrink 0) + `value` (flex, max-width 14rem) pairs.
- [ ] Gap tokens are guessed: `gap_md = space.inline.sm` (`skeleton.rs:75`) used where contract preset gaps are `0.75rem`; no token maps the contract's exact `0.75`/`0.625`/`0.5`/`0.375`/`0.25`rem preset gaps — verify each against a real space token rather than reusing one inline value.
- accepted: no ARIA channel (Jetstream has no accessibility API).

## Specimen parity

- Svelte covers: Basic shapes (line / circle / block), avatar-line, list-item ×3, table-row ×3, card (2-col grid), detail-section `lines={4}`, Static (no-animation) — all 7 contract §13 groups (`SkeletonSpecimen.svelte:6-45`).
- GPUI covers: Basic shapes, avatar-line, list-item ×3, table-row ×3, card, detail-section, Static. — but **the presets are hand-coded in the specimen** (`list_item_preset`/`table_row_preset`/`card_preset`/`detail_section_preset` at `skeleton.rs:168-323`) using raw `Skeleton` single-shapes wrapped in bespoke `div()` layout, **not** the real `Skeleton::preset(...)` builder. This violates the no-mockup rule: the specimen reconstructs preset layout with hardcoded px (`px(36.0)`, `relative(0.6)`, `px(224.0)`, `px(96.0)`, etc.) instead of exercising the component's preset path. The component's own `SkeletonPreset::*` rendering goes untested. Treat as specimen gap.
- Jetstream covers: Rectangle (default + 75%/50%), Circle (3 sizes), Text lines, all five presets (TableRow/Card/ListItem/DetailSection/AvatarLine), DetailSection with `lines=5`, Non-animated. — exercises the real `js_skeleton` preset path (good), broader than contract §13. Missing: a `block`-shape specimen (contract Basic-shapes group has line/circle/**block**; Jetstream has rectangle/circle/text). Static group is present but inert (no animation to toggle).

## Notes

- `consv=fixed`: the contract is faithful to Svelte (shape vocab `line`/`block`/`circle` already correct; §6 `aria-hidden` on single shapes now documented). The remaining shape-vocab divergence lives in the Rust `SkeletonSpec` (`rectangle`/`text`/`circle`) — a code rename, out of scope for this contract-reconciliation pass. Per "Svelte is parity authority," rename the Rust spec to match; that single fix unblocks both Rust targets' shape handling.
- GPUI is the worst offender on the no-hardcoded-literals rule: nearly every preset dimension is a raw `px(...)` float and the animation is a pulse, not the contract shimmer. The GPUI *specimen* additionally fakes presets rather than calling the component's preset builder, hiding that gap.
- Jetstream is honest about lacking animation (header comment) and does exercise the real preset builder, but its preset internals (avatar sizes, line counts, label/value rows, footer pills, cell widths) drift materially from the contract.
- Contract §11 Tier-2 visual checks (shimmer 3-stop color-mix, 220% bg-size, 1.6s linear, keyframes 200%→-20%) are unmet in both Rust targets — neither renders the gradient at all (flat `bg(fill)`).
