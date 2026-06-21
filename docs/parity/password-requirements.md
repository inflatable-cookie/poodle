<!-- parity consv=ok gpui=0 jetstream=0 specimen=gap -->
<!-- pass 41: both Rust targets verified — size ladder (xs→xl) on spec drives padding/title/body/indent/rhythm; GPUI uses ✓/✗ + Jetstream check/x non-color indicators; title only in requirements branch; hint gated on requirements; label wording aligned to Svelte; no strength meter (Svelte ignores minStrengthScore — parity authority). Jetstream render_probe tests cover met/unmet icons, tone via indicator, size ladder, all-met. GPUI builds; jet 7/7 pass. Remaining JsEl/GPUI rem literals (list gap 0.125, row gap 0.375, indicator box 0.875/0.75) are layout micro-approximations with no Svelte token. -->
# Parity: PasswordRequirements

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/password-requirements.md`
- Svelte (authoritative): `packages/svelte/components/src/PasswordRequirements.svelte`
- GPUI: `packages/gpui/components/src/primitives/password_requirements.rs`
- Jetstream: `packages/jetstream/components/src/password_requirements.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/PasswordRequirementsSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/password_requirements.rs` · jetstream `packages/jetstream/preview/src/specimens/password_requirements.rs`

## Contract ↔ Svelte

Contract and Svelte agree on every public prop (name/type/default), the data contract, and the four checklist rules. No divergence in surface.

- Props match: `password`/`requirements`/`loading`/`error`/`title`/`hint`/`loadingLabel`/`size`/`sizeRole` all present with matching defaults. Default `hint` text matches contract's "default hint text" (`PasswordRequirements.svelte:23`).
- Anatomy match: title shown only when requirements present (Svelte hides title in loading/error states, `:42-44`); contract §4 says title is part of the checklist branch. Consistent.
- Accessibility note (not a divergence, but Svelte under-delivers vs its own contract §6): contract requires per-item `aria-label` with "met / not met" text and a non-color indicator (checkmark/cross). Svelte conveys pass/fail by **color only** (`__item--met` swaps text color, `:136-138`) with no icon and no `aria-label`. Svelte is the authority for visual reference but is missing contract-specified a11y. **Fix: add per-item `aria-label` + visible indicator to Svelte, or relax contract §6.** Flagging as a Svelte-side a11y gap, not a contract↔Svelte prop divergence — `consv` stays `ok`.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] `size`/`sizeRole` support — `PasswordRequirementsSpec` now carries `size`/`size_role` + the contract §7 ladder methods (`padding_rem`/`title_size_rem`/`body_size_rem`/`list_indent_rem`/`description_gap_rem`/`hint_gap_rem`). GPUI resolves the effective size via `resolve_semantic_size` and reads all size-driven properties from the spec.
- [x] Hardcoded rem constants — title/body/padding/description-gap/hint-gap all resolve from the spec ladder methods now; radius via `resolve_radius`, border-width via `resolve_px`. Remaining raw `rem_to_px` literals are the list inter-row gap (0.125), indicator↔label gap (0.375), and indicator box (0.875/0.75) — JsEl/GPUI layout micro-approximations with no Svelte token counterpart (noted).
- [x] Indicator aligned to contract §6 — GPUI renders `✓`/`✗` (checkmark/cross, `build_rule_row`), a non-color indicator supplementing the tone change. (Cross-target note: Svelte remains color-only; contract §6 mandates a non-color indicator, so the Rust targets are the conformant ones. Svelte-side a11y gap tracked in Contract↔Svelte.)
- [x] Title color: GPUI applies `title_color` (text-primary) to the title, row tone (met/unmet) to rows — matches Svelte. Verified.
- [x] `description` vs `hint` ordering: GPUI renders description then hint matching Svelte. Verified.
- accepted: no ARIA (gpui has no accessibility API) — per-item met/not-met `aria-label` and `aria-live="polite"` cannot be emitted.

## Jetstream gap (vs Svelte + contract)

- [x] `size`/`sizeRole` support — Jetstream resolves the effective size and reads the spec ladder methods; default is now the contract `md` rung (padding `1rem`, body `0.875rem`), no longer the xs/sm rung.
- [x] Hardcoded rem constants — title/body/padding now from spec ladder; `radius` via `resolve_radius(theme, spec.radius_token())`, border-width via `resolve_px`. `item_gap` (0.375) is a JsEl indicator↔label approximation with no Svelte token (noted inline). `text_weight(600)` is the title weight matching Svelte `font-weight: 600`.
- [x] Label wording aligned — mixed-case row reads "Mix of uppercase and lowercase letters" (matches Svelte/contract/GPUI); covered by `requirements_branch_shows_title_rules_and_hint`.
- [x] State branch ordering matches Svelte — title is emitted only inside the requirements branch; loading and error branches render their single paragraph with no title. Covered by `loading_state_shows_loading_label_no_title` + `error_state_shows_error_no_title`.
- [x] Indicator: Jetstream uses icon registry `check`/`x` (non-color, contract §6). Reconciled story: both Rust targets carry a non-color indicator (GPUI `✓`/`✗`, Jetstream `check`/`x`); Svelte stays color-only (tracked as Svelte-side a11y gap).
- [x] Hint gated on requirements — hint renders only inside the requirements branch (matches Svelte); empty spec renders nothing. Covered by `empty_spec_renders_nothing_inside_panel`.
- accepted: no ARIA channel — per-item status labels / live region not emitted; interaction n/a (display-only component).

## Specimen parity

- Svelte covers: Default (live `TextInput` + checklist), Loading, Error, plus a **size ladder** (xs→xl via `SpecimenLayout showSizes`) (`PasswordRequirementsSpecimen.svelte:18-48`).
- GPUI covers: Default (static password `ClayRules123!`), Loading, Error — **missing: size ladder** (no size variants; spec can't express size). Uses min-length 12 policy like Svelte.
- Jetstream covers: Partially met, All met, Loading — **missing: Error state, size ladder, description rendering.** Uses min-length **8** policy (Svelte/GPUI use 12) and no `description`, so the description path is never exercised.

## Notes

- Root cause of most gaps: `PasswordRequirementsSpec` (`packages/contracts/components/src/password_requirements.rs`) has **no `size`/`size_role` field and no size/typography token methods** — only color tokens (`fill`/`border`/`title`/`text`/`met`/`error`). Adding size to the spec unblocks the ladder in both Rust targets and removes the rem-literal hardcodes.
- Three different indicator treatments across targets (Svelte color-only, GPUI `✓`/`•`, Jetstream `check`/`x` icons). Contract §6 mandates a non-color indicator, so Svelte is the one that should change — but per repo policy Svelte is the visual authority. Needs an explicit decision; until then all three "differ" but none is unambiguously wrong.
- Jetstream's default size resolves to the contract's xs rung, not md — even before a ladder exists, the single hardcoded size is the wrong one.
