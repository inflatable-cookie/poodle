<!-- parity consv=ok gpui=1 jetstream=6 specimen=gap -->
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

- [ ] **No implementation.** No `inline_remediation.rs` under `packages/gpui/components/src/`. Contract §7 references `InlineRemediationSpec` (which exists) and §10/baseline names a GPUI form-validation baseline, so GPUI is expected to have this. Either implement `InlineRemediation` (aside root, tone left-border, title/message content, optional Button action) resolving from the spec's `border_token()`/`gap_token()`, or fold into a GPUI `Callout` per §8 Known Delta and document that choice.
- accepted: no ARIA (gpui has no accessibility API) — `aria-labelledby`/`aria-describedby` field wiring not emitted.

## Jetstream gap (vs Svelte + contract)

Jetstream is the only implementation, but it diverges from the contract anatomy and hardcodes nearly every dimension.

- [ ] Anatomy mismatch — full border vs left-border: `js_inline_remediation` uses `.border(1.0)` on all four sides (`inline_remediation.rs:41`). Contract §2 specifies a tone-colored **left border** (`.inline-remediation__border`), not a full box border. Apply a left accent border only.
- [ ] Anatomy mismatch — undocumented tone icon: Jetstream renders a leading status icon (`inline_remediation.rs:46-51`, `tone_icon()`). Contract §2 anatomy has **no icon part** (Root → Border → Content[Title, Message] → Action). Either remove the icon or amend the contract to add an icon part (contract is authority; the icon is currently an invention).
- [ ] Action is a plain label, not a Button: `inline_remediation.rs:78-84` renders `action.label` as a clickable `label`. Contract §2 Action "delegates to Button primitive" — use `js_button` with `action.variant`/`action.is_disabled`.
- [ ] Hardcoded dimensions ignore tokens: `rem_to_px(0.8125)` font, `rem_to_px(0.75)` small, `rem_to_px(1.0)` icon, `rem_to_px(0.375)` radius, `pad_x=0.75`, `pad_y=0.5`, row `gap=0.5`, `content_gap=0.25` (`inline_remediation.rs:31-37`). Contract §6: gap must come from `gap_token()` (= `space.stack.sm`) — spec exposes it but it is unused. Resolve font/padding/radius from tokens too.
- [ ] Magic fill tint `tone_color.mix(panel, 0.08)` (`inline_remediation.rs:30`) — contract §6 lists no background-fill token (only Border + Root gap). Either add a fill token to the contract or drop the tint; the `0.08` is unsourced.
- [ ] Message text color wrong: uses `text_primary` (`inline_remediation.rs:65`); contract §2 Message → `text-secondary`. (Title correctly uses primary; message should be secondary.)
- accepted: `referencedFieldIds` aria-describedby wiring lives in the host form/event loop, not the component.
- accepted: action click handler lives in preview `main.rs` event loop.

## Specimen parity

- Svelte covers: **none** (no Svelte component).
- GPUI covers: **none** (no GPUI component).
- Jetstream covers: Info suggestion, Warning with action (+ referenced fields), Danger (`inline_remediation.rs` specimen, 3 groups). Reasonable tone coverage, but demonstrates the divergent anatomy (full border + icon).

specimen=gap because two of three targets have no specimen, and the one specimen renders the non-contract anatomy.

## Notes

- This is the inverse of the usual parity case: the **authoritative reference (Svelte) is intentionally missing**, so the contract alone is the source of truth and Jetstream is being graded against it directly. Jetstream took liberties (full border, leading icon, label-as-action) that the contract does not sanction.
- The cleanest resolution is a decision: either (a) treat InlineRemediation as a real cross-target component — build Svelte (likely via `Callout`) and GPUI, and bring Jetstream's anatomy back in line with the contract — or (b) formally retire it in favor of `Callout`/`RemediationBanner` and delete the Jetstream one-off. The contract's §8 Known Delta leans toward (b).
- consv=ok is a "contract is self-consistent" signal, not "Svelte parity achieved" — there is no Svelte to compare. Treat the whole component as an open design question.
