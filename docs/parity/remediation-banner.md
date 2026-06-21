<!-- parity consv=gap gpui=0 jetstream=0 specimen=ok -->
<!-- pass: GPUI RemediationBanner built (composites/remediation_banner.rs, mirrors Callout+EmptyState); Jetstream rebuilt to contract — real action Buttons (variant+is_disabled), token-resolved radius/border/typography, tone icon map via spec. Svelte still absent → consv=gap (sole authority is the contract). -->
# Parity: RemediationBanner

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/remediation-banner.md`
- Svelte (authoritative): **MISSING** — no `packages/svelte/components/src/RemediationBanner.svelte` exists. Contract §header notes "Svelte component not yet built."
- GPUI: **MISSING** — no `packages/gpui/components/src/{composites,primitives}/remediation_banner.rs`. Contract §Next Task asks for it.
- Jetstream: `packages/jetstream/components/src/remediation_banner.rs`
- Spec: `packages/contracts/components/src/remediation_banner.rs` · types `packages/contracts/components/src/composite_types.rs:123` (`RemediationAction`), `:7` (`AnnouncementMode`)
- Specimens: jetstream `packages/jetstream/preview/src/specimens/remediation_banner.rs` (only target with a specimen)

## Contract ↔ Svelte

**No Svelte implementation exists** — the authoritative reference is absent. `consv=gap` reflects this: contract is `detailed`, Svelte is the proof reference for visual correctness (CLAUDE.md), and it is unbuilt. The contract cannot be validated against the reference until Svelte ships.

- [ ] **Top priority: build `RemediationBanner.svelte`** matching contract anatomy (`<section>`, icon, content title+message, actions row, dismiss). Until then there is no parity authority — Jetstream's rendering is unvalidated.
- Spec (`RemediationBannerSpec`) and types (`RemediationAction { variant, is_disabled }`, `AnnouncementMode`) already match the contract exactly, including `accessibility_role()` (polite→status, assertive→alert) and `action_count()`. So the Rust side is ready; only the web reference is missing.

## GPUI gap (vs Svelte + contract)

- [x] **GPUI implementation built** — `packages/gpui/components/src/composites/remediation_banner.rs` (`RemediationBanner`, registered in `composites/mod.rs`). Mirrors `Callout` (tone-mix surface/border, dismiss affordance) and `EmptyState` (real `Button` per `RemediationAction`, wired to `on_action`/`on_dismiss` handlers). Resolves fill via `color_mix(tone, panel, spec.fill_tone_ratio())`, radius/border/typography from spec token methods. Build-verified (`cargo build` in gpui/components).
- accepted: GPUI will not emit `aria-live` (contract §8 Known Delta).
- note: no GPUI specimen added — gpui/preview shares a target lock and is build-skipped this pass (cannot verify). Pattern matches other composites (real component lives in the components crate; adapter `RenderComponent` impl remains a stub handle, unchanged).

## Jetstream gap (vs Svelte + contract)

- [x] **Actions now render as real `js_button` components** — `action_button()` builds a `js_button` per `RemediationAction` (primary + secondary), carrying an `id` (`remediation-action-<id>`). Probe test asserts ≥2 `Button` widgets with the action labels.
- [x] **`RemediationAction.variant` respected** — passed straight through to `ButtonSpec::with_variant(action.variant)`; specimen exercises Primary + Secondary.
- [x] **`RemediationAction.is_disabled` respected** — passed to `ButtonSpec::with_disabled`; `js_button` applies the disabled-opacity token. Probe test asserts the disabled action dims.
- [x] **Radius resolved from token** — `resolve_radius(theme, spec.radius_token())` (`RADIUS_SURFACE`), no `0.5rem` literal.
- [x] **Type/spacing resolve from tokens** — typography from `typography.body.size` / `typography.label.size`; padding/gaps from `space.panel.x/y`, `space.inline.md/sm/xs`. note: icon size (1.25rem) + dismiss size (1.0rem) are contract-exact rem via `rem_to_px` (no semantic token); root row `gap` uses `space.inline.md` as an approximation (no exact token).
- [x] **Dismiss carries an interaction id** — `id("remediation-banner-dismiss")` so the affordance is addressable; ARIA `aria-label="Dismiss"` still absent (JsEl has no accessibility channel — accepted, noted).
- [~] **`announce_mode` / `accessibility_role()`** — still host-applied (JsEl has no accessibility metadata channel). The spec exposes `accessibility_role()`; the component documents the host responsibility. Specimen now exercises polite/assertive/none modes.
- [x] **Tone tint via spec** — fill = `tone.mix(panel, spec.fill_tone_ratio())` (ratio is a spec method, TOKEN GAP noted: no semantic surface-tint ratio token; 0.08 matches Callout pending). Border = `spec.border_token()` (tone color).
- accepted: interaction (action clicks, dismiss) lives in preview event loop, not the component.

## Specimen parity

- Svelte covers: **n/a** (no component, no specimen).
- GPUI covers: **n/a** — component built, but no specimen (gpui/preview build-skipped this pass; see GPUI note).
- Jetstream covers: Info (polite, primary + dismiss), Warning (assertive, 2 actions + dismiss), Danger (no actions), **Success** (recovery confirmed), **Pending** (silent, disabled Secondary action). Covers all 5 tones, all 3 announceModes, and a disabled action. Actions are real buttons, so the specimen now proves button parity.

## Notes

- **Svelte authority missing** (contract-reconciliation pass, 2026-06-20): no `packages/svelte/components/src/RemediationBanner.svelte` exists, so there is no parity authority to reconcile the contract against. Per the reconciliation rule, the contract was left unchanged and is the sole authority for this component until Svelte ships. `consv=gap` retained for this reason — not a contract defect.
- This component is the least-complete of the assigned set: 1 of 3 targets implemented, and that target lacks real action buttons. The contract is solid; the work is to build Svelte (reference) + GPUI, then upgrade Jetstream actions to real buttons.
- Specimen `group()` helper hardcodes `text_size(11.0)` (jetstream `specimens/remediation_banner.rs:53`) — specimen chrome.
