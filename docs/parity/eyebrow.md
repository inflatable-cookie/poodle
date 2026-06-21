<!-- parity consv=ok gpui=1 jetstream=2 specimen=gap -->
<!-- pass 48: EyebrowSpec extended (size xs/sm/md, spacing none/bottom, element
     span/p/h2/h3/h4, ariaLabel + font_family/font_weight/line_height/
     letter_spacing_em/margin_bottom_rem methods). GPUI: size-based font-size,
     label font-family resolved from token, weight 600 (contract literal),
     line-height 1.5, spacing="bottom" margin. Jetstream: size-based font-size,
     spacing margin; weight 600. Both uppercase (pass 47). 3 jet probe tests
     (uppercase/md-size/spacing). REMAINING: gpui=letter-spacing (no GPUI text
     channel — accepted delta); jet=letter-spacing + font-family (no JsEl channel
     — runtime gap). ARIA/heading semantics on both = accepted (no a11y API). -->
# Parity: Eyebrow

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/eyebrow.md`
- Svelte (authoritative): `packages/svelte/components/src/Eyebrow.svelte`
- GPUI: `packages/gpui/components/src/primitives/eyebrow.rs`
- Jetstream: `packages/jetstream/components/src/eyebrow.rs`
- Spec (shared): `packages/contracts/components/src/eyebrow.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/EyebrowSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/eyebrow.rs` · jetstream `packages/jetstream/preview/src/specimens/eyebrow.rs`

## Contract ↔ Svelte

Svelte matches the contract exactly. No divergence.

- Props: `as` / `ariaLabel` / `size` / `spacing` all present with contract defaults (`Eyebrow.svelte:5-18`) — `as="span"`, `ariaLabel=null`, `size="sm"`, `spacing="none"`. Matches contract §3.
- Anatomy: root is the bare element (`span`/`p`/`h2`/`h3`/`h4`), no wrapper, `children()` inside (`Eyebrow.svelte:21-41`). Matches contract §2 + §9.
- ARIA: `aria-label={ariaLabel ?? undefined}` on every branch (`Eyebrow.svelte:22-38`); heading semantics via real `h2/h3/h4`. Matches contract §6.
- Tokens: `color`/`font-family` from vars; `font-size 0.6875rem`, `font-weight 600`, `letter-spacing 0.12em`, `line-height 1.5`, `text-transform uppercase` (`Eyebrow.svelte:44-70`). Matches contract §8 (the contract itself specifies literal rem values for the size variants, so the literal `0.85rem`/`0.04em` for `md` is contract-faithful, not a hardcode bug).

## GPUI gap (vs Svelte + contract)

`EyebrowSpec` now carries `element` (`as`), `aria_label`, `size`, `spacing` plus
`font_family()`/`font_weight()`/`line_height()`/`letter_spacing_em()`/
`margin_bottom_rem()` methods (`contracts/components/src/eyebrow.rs`). GPUI builder
resolves them all except the noted delta.

- [x] FIXED `size` variant — `font_size_rem()` branches `xs`/`sm` → `0.6875rem`, `md` → `0.85rem` (contract-exact literals). Builder uses it.
- [x] FIXED `spacing="bottom"` — builder applies `.mb(rem_to_px(margin_bottom_rem()))` (`0.5rem`, `0.35rem` for xs) when spacing is bottom.
- [x] FIXED `font-family` — builder calls `.font_family(spec.font_family())` resolving `TYPOGRAPHY_LABEL_FAMILY`.
- [x] FIXED `font-weight`/`line-height` — `FontWeight(spec.font_weight())` (600, contract literal — heavier than the 500 label-weight token), `relative(spec.line_height())` (1.5).
- [x] ACCEPTED `as`/heading semantics — `element` stored on the spec; GPUI has no a11y/heading API so it is not emitted, but the visual treatment is identical on every element (contract §4 "same visual on a heading element"). Same accepted bucket as ARIA.
- [ ] `letter-spacing` (0.12em / 0.04em md) — `letter_spacing_em()` exists on the spec but GPUI has no text letter-spacing channel. Accepted typographic delta (contract §12-eligible).
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` cannot be emitted.

## Jetstream gap (vs Svelte + contract)

Same extended `EyebrowSpec`. `js_eyebrow` resolves size/spacing/weight; uppercase
done in-component (pass 47).

- [x] FIXED `text-transform: uppercase` — `js_eyebrow` uppercases `spec.content` (`eyebrow.rs`), matching GPUI + Svelte. Probe-tested with mixed-case input.
- [x] FIXED `text_weight` — now `spec.font_weight()` (600, contract-exact literal; the label-weight token is 500, so this is NOT a missing token resolution — the contract specifies the literal).
- [x] FIXED `size` variant — `font_size_rem()` branches per size; md probe-tested (font larger than sm).
- [x] FIXED `spacing="bottom"` — `.mb(rem_to_px(margin_bottom_rem()))` applied; probe-tested.
- [x] ACCEPTED `as`/heading semantics — `element` stored; Jetstream has no heading/a11y channel. Visual treatment identical across elements.
- [ ] `letter-spacing` + `font-family` — JsEl has no `letter_spacing`/`font_family`/`text_transform` builder methods. Runtime gap (note): `letter_spacing_em()`/`font_family()` live on the spec, applied once the runtime exposes the channels.
- accepted: no ARIA channel for `aria_label`.
- accepted: this component is non-interactive — no preview event-loop wiring needed.

## Specimen parity

- Svelte covers: Section label, Primitive category, Status category, and the `as="h3" size="md" spacing="bottom"` semantic-heading variant (`EyebrowSpecimen.svelte:7-28`) — the only specimen exercising `as`/`size`/`spacing`.
- GPUI covers: Section label, Primitive, Composite (`gpui/.../eyebrow.rs:14-49`). — missing: **semantic-heading variant** (`as`/`size="md"`/`spacing="bottom"`) — not demonstrated (and unsupportable until spec gains those props).
- Jetstream covers: Default ("SECTION TITLE"), Multiple ("OVERVIEW"/"DETAILS"/"SETTINGS") (`jetstream/.../eyebrow.rs:13-21`). — missing: **semantic-heading variant**, and labels are pre-uppercased to mask the missing `text-transform`; a mixed-case input would expose the bug.

## Notes

- `EyebrowSpec` now models all four contract props: `element` (`as`), `aria_label`, `size`, `spacing`, plus typographic metric methods. Both Rust targets resolve them; the only open items are runtime-channel gaps (letter-spacing on both; font-family on Jetstream) and the accepted no-a11y/heading-semantics bucket.
- `consv=ok`: contract and Svelte agree on every prop, default, anatomy part, and ARIA rule.
- The pass-47 prompt note mentioned a possible eyebrow `tone` prop and a leading icon/dot — neither is in the contract (§14 flags `tone` only as a *future* follow-up) or in Svelte, so both are out of scope for parity. Do not add them speculatively.
- `font-weight: 600` is a contract literal (§8), intentionally heavier than the `TYPOGRAPHY_LABEL_WEIGHT` token (500). Resolving weight "from a token" would regress it — keep the literal.
