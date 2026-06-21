<!-- parity consv=ok gpui=5 jetstream=5 specimen=gap -->
<!-- pass 47: js_eyebrow now uppercases content (.to_uppercase()) matching Svelte
     `text-transform: uppercase` + GPUI — JsEl has no CSS transform. Cross-cutting:
     fixes every eyebrow section title across consumers. Probe-tested. -->
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

Root cause: shared `EyebrowSpec` (`contracts/components/src/eyebrow.rs:4-6`) only carries `content`. No `as`/`size`/`spacing`/`ariaLabel` fields and no size/spacing/letter-spacing/family/weight/line-height tokens — so most gaps below require extending the spec first, not just the GPUI builder.

- [ ] No `size` variant support — spec lacks a `size` field + per-size font tokens; `font_size_rem()` is a flat `0.6875` (`eyebrow.rs:58`). Add `size` to `EyebrowSpec` (`xs`/`sm`/`md`) resolving `0.6875`/`0.6875`/`0.85rem`, then branch in the builder.
- [ ] No `spacing` support — contract `spacing="bottom"` (margin-bottom `0.5rem`, `0.35rem` for xs) has no spec field; GPUI never applies bottom margin. Add `spacing` to spec + apply `.mb(...)` from a token.
- [ ] No `as` / heading semantics — `into_element` always emits a plain `div` (`eyebrow.rs:60-66`); `as` not in spec. Carry `as` and select element/semantics.
- [ ] `letter-spacing 0.12em` not applied — flagged as known delta in code comment (`eyebrow.rs:57`) but contract §8 + Tier-2 checklist require it. Track as a delta or implement once GPUI exposes letter-spacing.
- [ ] `font-family` not resolved — builder sets size/weight/color but never reads `font.family` / `typography.label.family`; relies on GPUI default font. Resolve the label family token.
- accepted: no ARIA (gpui has no accessibility API) — `ariaLabel` cannot be emitted.

## Jetstream gap (vs Svelte + contract)

Same shared-spec root cause as GPUI: `EyebrowSpec` only carries `content`.

- [ ] `text-transform: uppercase` NOT applied — `js_eyebrow` passes `spec.content` through verbatim (`eyebrow.rs:15-17`); GPUI uppercases the string (`gpui/.../eyebrow.rs:54`) but Jetstream does not, so casing depends entirely on caller-supplied caps. Uppercase in the component.
- [ ] Hardcoded `text_weight(600)` literal (`eyebrow.rs:20`) — resolve weight from a token (e.g. `font.weight.semibold`), not a raw `600`.
- [ ] No `size` variant support — spec lacks `size`; flat `font_size_rem()` only (`eyebrow.rs:13`). Add spec field + per-size tokens, then branch.
- [ ] No `spacing="bottom"` support — no margin applied; spec field absent. Add field + bottom-margin token.
- [ ] No `as` / heading semantics — always emits `label()` (`eyebrow.rs:17`); `as` not in spec.
- [ ] `letter-spacing 0.12em` not applied — contract §8 requires it; `js_eyebrow` sets only color/size/weight. Apply tracking from a token (or record as accepted delta if runtime lacks tracking).
- accepted: no ARIA channel for `ariaLabel`.
- accepted: this component is non-interactive — no preview event-loop wiring needed.

## Specimen parity

- Svelte covers: Section label, Primitive category, Status category, and the `as="h3" size="md" spacing="bottom"` semantic-heading variant (`EyebrowSpecimen.svelte:7-28`) — the only specimen exercising `as`/`size`/`spacing`.
- GPUI covers: Section label, Primitive, Composite (`gpui/.../eyebrow.rs:14-49`). — missing: **semantic-heading variant** (`as`/`size="md"`/`spacing="bottom"`) — not demonstrated (and unsupportable until spec gains those props).
- Jetstream covers: Default ("SECTION TITLE"), Multiple ("OVERVIEW"/"DETAILS"/"SETTINGS") (`jetstream/.../eyebrow.rs:13-21`). — missing: **semantic-heading variant**, and labels are pre-uppercased to mask the missing `text-transform`; a mixed-case input would expose the bug.

## Notes

- Biggest driver: `EyebrowSpec` is impoverished (`content` only). The contract/Svelte define four props (`as`, `ariaLabel`, `size`, `spacing`); the shared spec models none of them, so every GPUI/Jetstream gap above is downstream of the spec, not the renderers. Fix the spec first, then both Rust targets.
- Jetstream's missing `text-transform: uppercase` is the one functional (non-typographic-polish) bug — GPUI does it, Jetstream does not, and the specimen hides it by feeding pre-capitalized strings.
- `consv=ok`: contract and Svelte agree on every prop, default, anatomy part, and ARIA rule.
- Contract §14 flags a possible future `tone` prop — not yet in Svelte or contract props, so out of scope for parity.
