<!-- parity consv=gap gpui=1 jetstream=8 specimen=gap -->
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

- [ ] **No GPUI implementation at all** — contract §Next Task: "Implement `RemediationBannerSpec` in the GPUI composite renderer and wire primary/secondary actions through to caller-supplied click handlers." Add `packages/gpui/components/src/composites/remediation_banner.rs` + a specimen.
- accepted: GPUI will not emit `aria-live` (contract §8 Known Delta).

## Jetstream gap (vs Svelte + contract)

- [ ] **Actions render as bare `label` text, not Button components** — `actions_row` uses `ui_element::label(&primary.label)` (`remediation_banner.rs:77,84`); contract §2 anatomy specifies `RemediationAction button` (primary + secondary). Use real `js_button` with the action's `variant`.
- [ ] **`RemediationAction.variant` ignored** — primary forced accent text, secondary forced secondary text (`remediation_banner.rs:78,85`); contract `RemediationAction` carries `variant: ButtonVariant`. Respect it.
- [ ] **`RemediationAction.is_disabled` ignored** — no disabled-opacity / non-interactive treatment for disabled actions (`composite_types.rs:127`).
- [ ] **Hardcoded radius literal** `rounded(rem_to_px(0.5))` (`remediation_banner.rs:44`) — contract §6 root has a `radius` token target; resolve it, don't bake `0.5rem`.
- [ ] **Hardcoded type/spacing rem literals** — `font_size 0.8125`, `small 0.75`, `icon 1.25`, `pad_x 1.0`, `pad_y 0.75`, `gap 0.75`, `content_gap 0.25`, `action_gap 0.5` (`remediation_banner.rs:32-39`) and `text_weight(600)/(500)` — all ad-hoc; resolve from typography/space tokens. None trace to a spec token method.
- [ ] **Dismiss has no `aria-label`/role channel** — contract §5 requires `aria-label="Dismiss"`; only a clickable `x` icon is rendered (`remediation_banner.rs:97-103`). Accept ARIA absence but note the dismiss affordance lacks the labelled-button semantics.
- [ ] **`announce_mode` / `accessibility_role()` unused** — comment at `remediation_banner.rs:48` defers role to "host runtime" but nothing wires it; the spec exposes `accessibility_role()`. Confirm the preview event loop applies it, else it's dropped.
- [ ] **Border = tone color, fill = tone.mix(panel, 0.08)** — `mix(...,0.08)` (`remediation_banner.rs:30`) is an ad-hoc blend, not a token; contract §6 root bg is `color.background.panel`. Validate the tint approach against Svelte once it exists.
- accepted: interaction (action clicks, dismiss) lives in preview event loop, not the component.

## Specimen parity

- Svelte covers: **n/a** (no component, no specimen).
- GPUI covers: **n/a** (no component, no specimen).
- Jetstream covers: Info (1 action + dismiss), Warning (2 actions + dismiss), Danger (no actions). — missing: **success** and **pending** tones (contract §4 lists 5 tones), **announceMode** variants (polite/assertive/none), a disabled-action example. And actions are rendered as text, so the specimen does not prove button parity.

## Notes

- This component is the least-complete of the assigned set: 1 of 3 targets implemented, and that target lacks real action buttons. The contract is solid; the work is to build Svelte (reference) + GPUI, then upgrade Jetstream actions to real buttons.
- Specimen `group()` helper hardcodes `text_size(11.0)` (jetstream `specimens/remediation_banner.rs:53`) — specimen chrome.
