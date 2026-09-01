<!-- parity consv=gap gpui=1 jetstream=0 specimen=ok -->
<!-- g16.034 promotes one 1.6s opacity pulse in full policy and static reduced/frozen frames. Web still implements the retired gradient-position shimmer; GPUI has a pulse but lacks the new host policy, first-frame, and teardown proofs. The card owns both migrations. Existing preset/shape fixes remain valid. -->
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
- [ ] G16.034 PROMOTED CHANGE: contract §8 now normalizes full mode to a
  1.6s opacity pulse and reduced/frozen to static output. Svelte still carries
  the retired gradient-position shimmer until the card lands.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] G16.034: GPUI's existing opacity pulse is the accepted property shape,
  but it must consume the effective policy, start after the first committed
  frame, use the contracted endpoints/easing, stop in reduced/frozen, and prove
  teardown leaves no loop.
- [x] FIXED (GPUI): static fill is now the token-resolved shimmer mid-tone (`color_mix(highlight, base, 0.5)` from `shimmer_*_token`), not a bare opacity. Static fill uses hardcoded `el.opacity(0.5)` (`skeleton.rs:99,222`) — no token; the gradient fill is also absent (single flat `bg(fill)`), so the contract's 3-stop gradient never renders.
- [x] FIXED (GPUI): preset bones now percentage-driven (`w(relative(frac))` for 40/60/60/20%, 80/100/60%) or exact rem (avatar 2.25rem, header 6rem, line 10rem). No raw px. Hardcoded pixel literals throughout preset bones: `px(120.0)`, `px(80.0)`, `px(200.0)`, `px(60.0)` (table-row `skeleton.rs:115-118`); `px(9999.0)`, `px(180.0)`, `px(240.0)` (card `skeleton.rs:127-129`); `px(160.0)`, `px(100.0)` (list-item `skeleton.rs:146-148`); `px(140.0)`, `px(9999.0)` (detail `skeleton.rs:156,159`); `px(120.0)`, `px(80.0)` (avatar-line `skeleton.rs:175-176`). Contract widths are percentages/rem (40/60/60/20%, 80/100/60%, 10rem) — resolve from rem or percentage relatives, not raw px.
- [x] FIXED (GPUI): line-sm height now `rem_to_px(0.6875)` constant; ad-hoc `*1.2`/`*0.85` multipliers gone (heading 1rem, lines 0.875rem per contract). Hardcoded radius magic number `px(rem_to_px(0.6875))` for line-sm height (`skeleton.rs:148`) and ad-hoc multipliers `default_height * 1.2` (`skeleton.rs:156`), `default_height * 0.85` (`skeleton.rs:176`) — invent values not traceable to any contract token.
- [x] FIXED (GPUI): card now renders block-header (6rem) + 3 body lines (80/100/60%) + 2 footer pills (3.5×1.25rem, pill radius). GPUI card preset omits the contract's pill footer (`.skeleton--pill` 3.5rem × 1.25rem, 999rem radius) — card renders header + 2 body lines only (`skeleton.rs:121-130`), missing the body's 3rd line (80/100/60%) and both footer pills.
- [x] FIXED (GPUI): full-width bones now use `w_full()`; the `px(9999.0)` sentinel is gone. GPUI `Card` preset uses `bone(px(9999.0), …)` as a faux full-width hack (`skeleton.rs:127,159`) instead of `w_full()` — magic sentinel width.
- accepted: no ARIA (gpui has no accessibility API) — skeletons stay decorative; `aria-hidden` cannot be emitted.

## Jetstream gap (vs Svelte + contract)

- [x] NOTED (Jetstream): JsEl cannot animate and has no gradient — flat shimmer mid-tone, no motion; shimmer is a preview-loop / Known-Delta concern (documented in file header). No animation at all — file header states "Jetstream cannot animate, so skeletons render as static gray boxes" (`skeleton.rs:3`); `is_animated` / the `with_animated(false)` specimen path are inert. BEHAVIOR gap vs contract §4 shimmer. Note: contract Known-Delta covers technique variance but not *absence*; if shimmer is driven by the preview event loop, it is not present here — confirm and document, else flag as missing.
- [x] FIXED (Jetstream): `single_shape` now matches `circle`/`block`/`line` (contract) and still accepts legacy `text`/`rectangle`; `radius_token()` maps all of them. `shape` vocabulary mismatch: Jetstream matches on `"circle"` / `"text"` / default-rectangle (`skeleton.rs:35-65`), not the contract's `line`/`block`/`circle`. `"line"`/`"block"` from contract fall through to the rectangle default. Fix once the spec is renamed.
- [x] FIXED (Jetstream): line/text default height now `rem_to_px(0.875)` per contract. Hardcoded rem literals in single-shape: `rem_to_px(0.75)` text height (`skeleton.rs:44`). Contract line height is `0.875rem`, not `0.75rem`.
- [x] FIXED (Jetstream): circle helper now uses the resolved `pill_radius` (`resolve_radius(radius.pill)`). Hardcoded radius literal `rounded(9999.0)` in the `circle` helper (`skeleton.rs:89`) — bypasses `radius_token()` (which already resolves `RADIUS_PILL`); use the resolved pill radius.
- [x] FIXED (Jetstream): avatar 2.25rem (list-item + avatar-line), card image 6rem (block-header), all per contract. Preset dimensions hardcoded as rem literals not matching contract: avatar-line avatar `rem_to_px(2.0)` (`skeleton.rs:96`) vs contract `2.25rem`; list-item icon `rem_to_px(1.25)` (`skeleton.rs:113`) vs contract avatar `2.25rem`; card image `rem_to_px(8.0)` (`skeleton.rs:139`) vs contract block-header `6rem`. All magic numbers.
- [x] FIXED (Jetstream): avatar-line = avatar + single 10rem line; card = header + 3 body lines + 2 pills; list-item = 60% line + line-sm 40%; table-row = 4 cells (40/60/60/20% via flex-basis seeds). Preset structural gaps vs contract: avatar-line renders avatar + **two** stacked lines (`skeleton.rs:97-108`) but contract avatar-line is avatar + **single** 10rem line; card omits the 3rd body line and both footer pills (`skeleton.rs:134-142`); list-item is a single growing line (`skeleton.rs:119`) but contract is primary 60% line + secondary `line-sm` 40% line; table-row uses 4 equal grow cells (`skeleton.rs:122-132`) but contract widths are 40/60/60/20%.
- [x] FIXED (Jetstream): detail rows now label (6rem, flex-shrink-0) + value (flex, max-width 14rem) pairs; heading 8rem. Detail-section omits the per-row label/value structure — renders header + flat full-width lines (`skeleton.rs:144-155`), but contract detail rows are `label` (6rem, flex-shrink 0) + `value` (flex, max-width 14rem) pairs.
- [x] FIXED (Jetstream): preset gaps now exact contract rem (0.75/0.625/0.5/0.375/0.25) — no token maps them precisely, so contract-exact literals used (noted). Gap tokens are guessed: `gap_md = space.inline.sm` (`skeleton.rs:75`) used where contract preset gaps are `0.75rem`; no token maps the contract's exact `0.75`/`0.625`/`0.5`/`0.375`/`0.25`rem preset gaps — verify each against a real space token rather than reusing one inline value.
- accepted: no ARIA channel (Jetstream has no accessibility API).

## Specimen parity

- Svelte covers: Basic shapes (line / circle / block), avatar-line, list-item ×3, table-row ×3, card (2-col grid), detail-section `lines={4}`, Static (no-animation) — all 7 contract §13 groups (`SkeletonSpecimen.svelte:6-45`).
- [x] FIXED (GPUI specimen): the hand-coded preset helpers are gone. All five presets now render via the real builder — `Skeleton::from_spec(SkeletonSpec::new().with_preset(SkeletonPreset::*))` — so the component's own `SkeletonPreset::*` rendering path is what's under test. Single shapes use rem-string widths (`12rem`/`2.5rem`/`8rem`); a Partial-width-lines group uses rem widths because GPUI `parse_dim` has no `%` support. GPUI covers: Basic shapes (line/circle/block), Partial-width lines, avatar-line, list-item ×3, table-row ×3, card (2-up), detail-section `lines=4`, Static.
- [x] FIXED (Jetstream specimen): now exercises the real `js_skeleton` builder for all shapes + all five presets. Added the missing `block`-shape (Basic-shapes group is line/circle/**block**). The old `75%/50%/80%/60%` single-shape lines were silently rendering full width (JsEl `parse_dim` ignores `%` → `w_full()`); replaced with a Partial-width-lines group using real rem widths (`18/13.5/9rem`) so the width is honest. Jetstream covers: Basic shapes, Partial-width lines, AvatarLine, ListItem ×3, TableRow ×3, Card, DetailSection `lines=4`, Static. Static group is present but inert (JsEl has no animation regardless — noted).

## Notes

- `consv=fixed`: the contract is faithful to Svelte (shape vocab `line`/`block`/`circle` already correct; §6 `aria-hidden` on single shapes now documented). The remaining shape-vocab divergence lives in the Rust `SkeletonSpec` (`rectangle`/`text`/`circle`) — a code rename, out of scope for this contract-reconciliation pass. Per "Svelte is parity authority," rename the Rust spec to match; that single fix unblocks both Rust targets' shape handling.
- GPUI is the worst offender on the no-hardcoded-literals rule: nearly every preset dimension is a raw `px(...)` float and the animation is a pulse, not the contract shimmer. The GPUI *specimen* additionally fakes presets rather than calling the component's preset builder, hiding that gap.
- Jetstream is honest about lacking animation (header comment) and does exercise the real preset builder, but its preset internals (avatar sizes, line counts, label/value rows, footer pills, cell widths) drift materially from the contract.
- Contract §11 now checks the shared opacity pulse, static reduced/frozen
  frames, first-frame posture, and teardown. G16.034 owns those migrations.
