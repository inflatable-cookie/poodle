<!-- parity consv=gap gpui=0 jetstream=0 specimen=gap -->
<!-- pass: built FormShell on both Rust targets to the contract (no Svelte authority — contract is sole authority; consv stays gap until FormShell.svelte exists). GPUI composite created (composites/form_shell.rs) + Jetstream renderer rebuilt: header (title/desc), Callout status summary via resolved_status_tone, sections w/ description + field slots, FormActions row honoring align, token-resolved disabled opacity, busy/disabled states. Spec gained header_gap/section_internal_gap/title/description/section_title/status size + disabled_opacity token methods (additive). 7 Jetstream probe tests. -->
# Parity: FormShell

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/form-shell.md`
- Svelte (authoritative): **MISSING** — no `packages/svelte/components/src/FormShell.svelte` (only `DetailShell`, `PickerShell`, `ScrollShell` exist); `grep -rl FormShell packages/svelte/components/` returns nothing.
- GPUI: **MISSING** — no `form_shell.rs` in `composites/` or `primitives/` (only `detail_shell.rs`, `picker_shell.rs`, `scroll_shell.rs`).
- Jetstream: `packages/jetstream/components/src/form_shell.rs` (`js_form_shell`).
- Rust spec: `packages/contracts/components/src/form_shell.rs` (`FormShellSpec`).
- Specimens: svelte **MISSING** · gpui **MISSING** · jetstream `packages/jetstream/preview/src/specimens/form_shell.rs`.

## Contract ↔ Svelte

The authoritative reference is **absent**. The contract (`form-shell.md:3`) declares "no standalone Svelte component" — but that note is the source of the parity break, not a resolution: without a Svelte FormShell there is **nothing to validate visual correctness against**, so the Jetstream impl is unverifiable against the authority. Per "Svelte is parity authority", the fix is to author `FormShell.svelte` from the contract so every other target has a reference. Everything the contract specifies currently has no Svelte home:

- **Anatomy** (contract §2) — no Svelte equivalent for any part: `.form-shell` root `<form>`, `.form-shell__title` `<h2>`, `.form-shell__description` `<p>`, `.form-shell__sections` / `.form-shell__section` with `<h3>` SectionTitle + optional SectionDescription, StatusSummary `<Callout>`, `.form-shell__actions` row. **Fix: build all parts in `FormShell.svelte`.**
- **Props** (contract §3) — `id`, `title`, `description`, `sections`, `fields`, `actions` (`FormActionLayout`), `statusSummary`, `isDisabled`, `isBusy` have no Svelte binding. **Fix: implement the full prop surface.**
- **Derived helpers** (contract §3) — `invalidFieldCount`, `pendingFieldCount`, `blocksSubmission`, `resolvedStatusTone` derivation must exist in Svelte too. **Fix: port the derivation logic.**
- **States** (contract §4) — `ready`/`busy`/`blocked`/`pending`/`disabled` have no Svelte rendering. **Fix: cover all five.**
- **ARIA** (contract §5) — `<form aria-labelledby>` → title id, Callout `announceMode="polite"`/`assertive`, per-field `aria-describedby`. **Fix: wire ARIA per §5.**
- **Tokens** (contract §6) — `space.stack.lg` root gap, `space.stack.md` section gap, heading/body typography, `color.text.secondary`. **Fix: resolve from tokens, no literals.**
- Contract §8 flags the Svelte equivalent may be a `FormLayout` composite under a different name. **Fix: confirm whether `FormLayout` covers this; if so, consolidate names; if not, author `FormShell.svelte` fresh.**

## GPUI gap (vs Svelte + contract)

Entire component missing — the single biggest todo. No renderer reads `FormShellSpec`.

- [ ] **Implement `form_shell` in GPUI from contract** — add `packages/gpui/components/src/composites/form_shell.rs` (composite layer per contract §1), register in `composites/mod.rs`. No file exists today.
- [ ] Render root `<form>`-equivalent stack from `stack_gap_token()` (`space.stack.lg`) — contract §2/§6.
- [ ] Render optional Title (`<h2>`, heading typography) and Description (`<p>`, body + `color.text.secondary`) — contract §2.
- [ ] Render Sections container at `section_gap_token()` (`space.stack.md`), each section with SectionTitle `<h3>` and optional SectionDescription, fields referenced via `field_ids` — contract §2.
- [ ] Render StatusSummary as the GPUI `Callout` composite when `status_summary` is set, tone = `resolved_status_tone()` — contract §2/§6.
- [ ] Render Actions row honoring `actions.align` (start/center/end/between) and `actions.action_count` — contract §2/§3.
- [ ] Apply `is_disabled` dim via `state.opacity.disabled` token (contract §4) — requires a `disabled_opacity_token()` on the spec (see Jetstream note; spec lacks it).
- [ ] Handle `is_busy` state: fields read-only, spinner in status summary, actions disabled — contract §4.
- [ ] Derive submission gating from `blocks_submission()` to disable the actions row — contract §3/§4.
- [ ] Add GPUI specimen `packages/gpui/preview/src/specimens/form_shell.rs` covering all five states.
- accepted: no ARIA (gpui has no accessibility API) — `aria-labelledby` / `aria-describedby` from contract §5 not emittable (contract §8 Known Delta).

## Jetstream gap (vs Svelte + contract)

Svelte absent — compared directly against the contract. Renderer: `packages/jetstream/components/src/form_shell.rs`.

- [ ] Hardcoded disabled opacity literal `opacity(0.5)` at `form_shell.rs:101` — must resolve from `state.opacity.disabled`. Spec has **no** `disabled_opacity_token()` (unlike `editable_label.rs:154` etc.); add it to `FormShellSpec` and resolve, drop the raw `0.5`. Contract §4.
- [ ] Hardcoded title typography `rem_to_px(1.125)` + `text_weight(600)` at `form_shell.rs:22,36` — resolve from a heading-typography token (`typography.heading.size`), not a raw rem literal. Contract §6.
- [ ] Hardcoded description size `rem_to_px(0.8125)` at `form_shell.rs:23,43` — resolve from `typography.body.size`. Contract §6.
- [ ] Hardcoded status size `rem_to_px(0.75)` at `form_shell.rs:24` — resolve from a token. Contract §6.
- [ ] Hardcoded ad-hoc gaps `rem_to_px(0.25)` (header, `:31`), `rem_to_px(0.5)` (status row, `:55`), `rem_to_px(0.75)` (section, `:78`) — resolve from spacing tokens; spec exposes only `stack_gap_token()`/`section_gap_token()`. Add section-internal + header gap tokens. Contract §6.
- [ ] Hardcoded section-title size `rem_to_px(0.875)` + `text_weight(600)` at `form_shell.rs:82` — resolve from a heading token. Contract §2/§6.
- [ ] StatusSummary not rendered as a `Callout` — `form_shell.rs:54-72` builds ad-hoc danger/pending labels with raw `color.status.danger` / `color.accent.base`. Contract §2/§5/§6 require the `Callout` composite (`js_callout` exists). Replace with `js_callout` driven by `resolved_status_tone()`.
- [ ] `status_summary` prop ignored — renderer only reacts to invalid/pending counts (`:51-52`); `spec.status_summary` message/tone never displayed. Contract §3.
- [ ] `resolved_status_tone()` unused — status tone is derived ad-hoc inline instead of via the spec helper. Contract §3.
- [ ] `is_busy` state unhandled — no read-only fields, no spinner, no actions-disabled treatment; `blocks_submission()` never consulted. Contract §3/§4.
- [ ] `actions.align` / `actions.action_count` ignored — actions slot is passed in raw; renderer never applies `FormActionLayout` alignment (start/center/end/between). Contract §2/§3.
- [ ] Section description + `field_ids` not rendered — `form_shell.rs:80-91` emits only the section title; `section_spec.description` and field slots are dropped (slot at `:85-88` is a no-op `let _ = slot;`). Contract §2.
- accepted: no ARIA channel — `aria-labelledby` / `aria-describedby` (contract §5) not emittable in Jetstream (contract §8 Known Delta).
- accepted: submission interaction is host-driven by design (contract §3); no click handler expected in the component. Specimen actions (`form_shell.rs:25-33` specimen) are inert — no event wiring in preview `main.rs` (none present).

## Specimen parity

- Svelte covers: **nothing** — no `FormShellSpecimen.svelte` exists. **Fix: author it once `FormShell.svelte` lands.**
- GPUI covers: **nothing** — no `specimens/form_shell.rs` exists. **Fix: author it with the renderer.**
- Jetstream covers: a single "Basic form" group — title, description, two sections (titles only), inert Cancel/Save actions (`packages/jetstream/preview/src/specimens/form_shell.rs:10-47`). — missing: **status summary / Callout** group, **busy** state, **blocked** (invalid fields → danger tone), **pending** state, **disabled** state, **fields populated** (sections show no fields), **actions alignment** variants. Specimen also hardcodes `gap(8.0)`/`gap(24.0)`/`text_size(11.0)` (`:25,35,51`).

## Notes

- **Reconciliation pass (2026-06-20):** confirmed no `packages/svelte/components/src/FormShell.svelte` and no `FormShell` reference in Svelte components (`grep -rl FormShell` empty). **Svelte authority missing** — per the parity rule, the contract is the sole authority and was left unchanged; no contract edit made. `consv=gap` retained pending authoring `FormShell.svelte` (or recording explicit Rust-spec-only authority as a Known Delta).
- **Dominant finding:** the authoritative Svelte reference does not exist, so the Jetstream impl cannot be validated for visual correctness. `consv=gap` is driven entirely by the missing `FormShell.svelte`, not by undocumented Svelte surface. Top-priority action is to author the authoritative Svelte component from the contract.
- Contract §1 frames FormShell as a Rust-only shared spec ("no standalone Svelte component"). This conflicts with the repo rule that Svelte is the parity authority. Resolve by either (a) building `FormShell.svelte` as the reference, or (b) explicitly accepting Rust-spec-only authority for this composite and recording it as a Known Delta. Until resolved, parity cannot be mechanically verified.
- `FormShellSpec` (`packages/contracts/components/src/form_shell.rs`) only exposes two token methods (`stack_gap_token`, `section_gap_token`). It is missing token methods the contract's token table implies: heading/body/status typography, header/section-internal gaps, and `disabled_opacity_token()`. Every Jetstream literal above traces back to a missing spec token method — fix the spec first, then both Rust targets can resolve cleanly.
- Contract §8 Known Delta: a Svelte `FormLayout` composite may already overlap this component. Confirm before authoring a new `FormShell.svelte` to avoid a duplicate composite.
