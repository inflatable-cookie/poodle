<!-- parity consv=ok gpui=1 jetstream=0 specimen=gap pass=jetstream-rebuilt-to-contract -->
# Parity: InlineRemediation

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/inline-remediation.md`
- Svelte (authoritative): **intentionally absent** — contract header says "Svelte component not yet built"; Known Delta §8 says Svelte uses the `Callout` primitive for inline recovery
- GPUI: **missing** — no `inline_remediation.rs` in composites/ or primitives/
- Jetstream: `packages/jetstream/components/src/inline_remediation.rs`
- Spec: `packages/contracts/components/src/inline_remediation.rs`
- Specimens: svelte — none · gpui — none · jetstream `packages/jetstream/preview/src/specimens/inline_remediation.rs`

## Contract ↔ Svelte

This component has **no Svelte implementation by design**. The contract header explicitly states "Svelte component not yet built" and §8 Known Delta records that Svelte composes inline recovery from the `Callout` primitive rather than a dedicated component. So the contract↔Svelte axis is not a divergence — the contract is the standalone authority here. `consv=ok` reflects "contract is internally consistent and documents the Svelte absence", not "Svelte matches".

Caveat: because there is no Svelte reference, the contract is the *only* source of truth, so any Jetstream deviation below is measured strictly against the contract anatomy/token tables.

## GPUI gap (vs Svelte + contract)

- [ ] **No implementation** (open design decision, not addressed in this pass). No `inline_remediation.rs` under `packages/gpui/components/src/`; only `composites/remediation_banner.rs` exists. Contract §8 Known Delta leans toward folding inline recovery into a GPUI `Callout` rather than a dedicated primitive. Resolving this means a decision (build `InlineRemediation` resolving `border_token()`/`gap_token()`, vs fold into Callout) — out of scope for this Jetstream-focused pass, which was bounded to menu + inline-remediation Jetstream files. Left as gpui=1.
- accepted: no ARIA (gpui has no accessibility API) — `aria-labelledby`/`aria-describedby` field wiring not emitted.

## Jetstream gap (vs Svelte + contract)

Rebuilt to the contract anatomy; all six gaps closed.

- [x] FIXED **left border** (was full box): root now uses `.border_l(0.125rem)` + `.border_color_left(tone)` only (contract §2 `.inline-remediation__border`). No four-side border.
- [x] FIXED **icon removed**: the undocumented leading tone icon and `tone_icon()` helper are deleted. Anatomy is now Root → left Border → Content[Title?, Message, field-hint] → Action, matching contract §2 (no icon part). Probe test asserts zero `Icon` nodes.
- [x] FIXED **action is a real Button**: the action delegates to `js_button` with `action.variant`/`action.is_disabled` (contract §2 "delegates to Button primitive"), carrying an `inline-remediation-action:<id>` interaction id. Matches the sibling `remediation_banner.rs` pattern.
- [x] FIXED **gap from token**: title-to-message and row gap now resolve from `gap_token()` (= `space.stack.sm`). Font/padding use contract-reasonable rems (`0.8125rem` body/title, `0.75rem` hint, `0.75/0.5rem` padding) — see token note.
- [x] FIXED **magic fill tint dropped**: the unsourced `tone_color.mix(panel, 0.08)` background is removed — contract §6 lists no background-fill token (only Border + Root gap).
- [x] FIXED **message color**: message now uses `color.text.secondary` (contract §2 Message → text-secondary); title keeps `color.text.primary`.
- accepted: `referencedFieldIds` aria-describedby wiring lives in the host form/event loop, not the component (the field-count hint is rendered as ambient context).
- accepted: action click handler lives in preview `main.rs` event loop.

### Token note

Contract §6 only tokenizes Border + Root gap. Title/message font-sizes and root padding have no contract token, so they use contract-reasonable rems (`0.8125rem`, `0.75rem`, `0.75/0.5rem`). The left-border weight (`0.125rem`) is likewise a literal — contract §2 names the border part but §6 gives no width token. These are noted, not token-resolvable today.

## Specimen parity

- Svelte covers: **none** (no Svelte component).
- GPUI covers: **done** — `inline_remediation_specimen.rs` (registered as `inline-remediation`, Form tag). 6 groups: tones (info/warning/danger), actionless, with-action, referenced-fields (with count hint), disabled-action. There is no GPUI `InlineRemediation` *component* yet, so the specimen materializes the contract anatomy directly from real, token-resolved primitives (left-border via `border_token()`, gap via `gap_token()`, padding/typography via `space.panel.*`/`typography.*`, plus a real `Button` for the action — the same construction the Jetstream `js_inline_remediation` uses). Zero hand-coded visual values.
- Jetstream covers: Info suggestion, Warning with action (+ referenced fields), Danger (`inline_remediation.rs` specimen, 3 groups). The specimen renders the corrected contract anatomy (left border, no icon, Button action) via the unchanged `js_inline_remediation` signature.
- Jetstream probe tests (`inline_remediation.rs` `#[cfg(test)]`): title + message + zero-icon anatomy; message uses secondary text; action renders as a real Button with its interaction id; tone drives the border token. All pass.

**GPUI specimen done; Jetstream pending engine recovery.**

specimen=gap remains: Svelte is intentionally absent and the Jetstream engine is externally build-blocked (cannot be runtime-verified here). The GPUI specimen now exists and builds (`packages/gpui/preview` → 0 errors).

## Notes

- This is the inverse of the usual parity case: the **authoritative reference (Svelte) is intentionally missing**, so the contract alone is the source of truth and Jetstream is graded against it directly. The prior Jetstream liberties (full border, leading icon, label-as-action, magic tint) are now removed — Jetstream matches the contract anatomy.
- Remaining open decision (gpui=1): either (a) build a GPUI `InlineRemediation` *component* resolving `border_token()`/`gap_token()`, or (b) formally retire it in favor of `Callout`/`RemediationBanner`. The contract's §8 Known Delta leans toward (b). The GPUI **specimen** now exists (built from real token-resolved primitives) even though the component does not — so the preview demonstrates the contract anatomy regardless of which way the decision lands. The gpui=1 todo is about the component, not the specimen.
- consv=ok is a "contract is self-consistent" signal, not "Svelte parity achieved" — there is no Svelte to compare.
