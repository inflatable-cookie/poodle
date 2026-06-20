<!-- parity consv=fixed gpui=8 jetstream=8 specimen=gap -->
# Parity: FormLayout

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/form-layout.md`
- Svelte (authoritative): `packages/svelte/components/src/FormLayout.svelte`
- GPUI: `packages/gpui/components/src/composites/form_layout.rs`
- Jetstream: `packages/jetstream/components/src/form_layout.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/FormLayoutSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/form_layout.rs` · jetstream `packages/jetstream/preview/src/specimens/form_layout.rs`

## Contract ↔ Svelte

Props/anatomy/class-name/grid divergences between the contract and the authoritative Svelte source. Svelte is right unless it's missing contract-specified functionality. Contract reconciled; remaining Rust-spec change tracked below.

- [x] FIXED **Class-name namespace differs.** Contract §2/§8 used `.form-layout*`; Svelte emits `.poodle-form-layout*` (`FormLayout.svelte:30,32,44,54`). Contract §2 anatomy + §8 selector headings (Root/Description/FieldErrors/Grid/container-query) repointed to the `poodle-` prefix.
- [x] FIXED **Grid row-gap differs from contract.** Contract §8 grid said `gap: stack-lg inline-md`; Svelte uses `row-gap: calc(space-stack-lg + 0.625rem)` and `column-gap: space-inline-md` via the two local `--poodle-form-layout-row-gap`/`-column-gap` custom props (`FormLayout.svelte:108-114`). Documented in §8 (custom-prop table + asymmetric row/column gap), §7, §2, §9.
- [x] FIXED **`:only-child` full-span rule.** Added `.poodle-form-layout__grid .poodle-field:only-child { grid-column: 1 / -1 }` (`FormLayout.svelte:117-119`) to contract §8 + §7 + §9.
- **`role="alert"` placement matches; Rust spec omits the data (code track).** Contract §6 + Svelte both render the field-error summary with `role="alert" aria-live="polite"` and the `<strong>{field}</strong>: {message}` list (`FormLayout.svelte:44-52`). Contract is already correct. Downstream `FormLayoutSpec` lacks a `field_errors` field — a spec-side code change, not a contract divergence.
- **`columns` default** — Svelte/contract agree on `6` (`FormLayout.svelte:18`, contract §3). No contract change; GPUI default-of-`1` is a code-track fix.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] No `fieldErrors` support — `FormLayout` struct has no field-errors input (`form_layout.rs:10-20`); the entire accessible error-summary anatomy part (contract §2 FieldErrors, §6 `role="alert"`) is absent. Add field-errors plumbing + summary block.
- [ ] Adds an undocumented `title` prop (`form_layout.rs:16,35-38,109-117`) — not in contract/Svelte (Svelte has only `description`). Remove `title` or get it added to the contract.
- [ ] `columns` defaults to `1` (`form_layout.rs:32`); contract/Svelte default is `6` (`FormLayoutSpec` default also `6`). Align default to `6`.
- [ ] Callout reimplemented inline via `callout_banner` (`form_layout.rs:66-90`) instead of delegating to the existing `primitives::callout` primitive — contract §8 "Composed Primitives" requires delegation to the Callout contract. Compose the real Callout.
- [ ] Actions not wrapped in `FormActions` — `with_actions` renders the raw element (`form_layout.rs:171-173`); contract §2 Actions part delegates to the `primitives::form_actions` primitive. Wrap in FormActions.
- [ ] Hardcoded min-width literal `min_w(px(180.0))` at `form_layout.rs:164` — column min-width is a raw float; resolve from a token, not `180.0`.
- [ ] Hardcoded callout mix ratios `0.12` / `0.30` at `form_layout.rs:75,77` and inline-pad `rem_to_px(1.0)` at `form_layout.rs:70` — match the inline literals in Svelte's color-mix (8%/40%) and panel padding; once Callout is composed these disappear, but flag the divergence: GPUI uses 12%/30%, Svelte uses 8%/40% (`FormLayout.svelte:84-85`).
- [ ] No responsive collapse — flex-wrap basis `relative(basis_pct/100.0 - 0.01)` (`form_layout.rs:158-163`) approximates columns but has no container-query equivalent for the 600px/480px breakpoints (contract §7). Accept as Tier-3 freedom OR add platform thresholds.
- accepted: no ARIA (gpui has no accessibility API) — `role="alert"`/`aria-live` cannot be emitted even once `fieldErrors` is added.

## Jetstream gap (vs Svelte + contract)

- [ ] No `fieldErrors` support — `js_form_layout` never reads field errors and `FormLayoutSpec` lacks the field (`form_layout.rs:53-58`, `packages/contracts/components/src/form_layout.rs:6-16`). FieldErrors anatomy part + `role="alert"` summary absent. Add spec field + summary block.
- [ ] Callout reimplemented inline via `form_callout_banner` (`form_layout.rs:18-40`) instead of composing the existing `callout` primitive — contract §8 requires delegation. Compose the real `js_callout`.
- [ ] Actions not wrapped in `FormActions` — `actions` arg appended raw (`form_layout.rs:107-109`); contract §2 Actions delegates to the `form_actions` primitive. Wrap in `js_form_actions`.
- [ ] Hardcoded min-width `rem_to_px(11.25)` at `form_layout.rs:101` — column min-width is a raw rem literal; resolve from a token.
- [ ] Hardcoded body font size `rem_to_px(0.8125)` at `form_layout.rs:23,64` — comment says "~13px body text"; resolve from `typography.body.size` token (GPUI uses `resolve_px(theme,"typography.body.size")`).
- [ ] Hardcoded inline-pad `rem_to_px(1.0)` at `form_layout.rs:21` and border width `border(1.0)` at `form_layout.rs:34` — raw floats; resolve padding from a panel-x token and border from a border-width token.
- [ ] Hardcoded callout mix ratios `0.12` / `0.30` at `form_layout.rs:26-27` — Svelte uses 8% / 40% (`FormLayout.svelte:84-85`); ratios diverge and are hardcoded. Resolve once Callout is composed.
- [ ] No responsive collapse — wrapping flex row (`form_layout.rs:93-105`) has no container-query equivalent for 600px/480px breakpoints (contract §7). Accept as Tier-3 OR add thresholds.
- accepted: no ARIA channel for `role="alert"`/`aria-live` (documented platform limit).
- accepted: interaction (submit/click) lives in preview event loop / host, not the component — contract §5 declares no component-owned events, so this is conformant.

## Specimen parity

- Svelte covers: Two-column (span 3), Mixed 2-col+3-col rows, Single column (`columns=1`), With error + **fieldErrors** (`role="alert"` list) + invalid Fields, With success. Uses real `Field`/`TextInput`/`Select`/`Checkbox`. (`FormLayoutSpecimen.svelte`)
- GPUI covers: Two-column, Mixed rows, Single column, With error, With success — using real `Field`/`TextInput`/`Checkbox`/`Button`. — missing: **fieldErrors summary** group (no field-errors plumbing); error specimen shows the callout only, not the `role="alert"` list (`form_layout.rs:244-286`).
- Jetstream covers: Single-column, Two-column, With description, With error, With success. — missing: **Mixed 2-col+3-col** group, **fieldErrors** group, and uses **fake hand-rolled `field()` placeholders** (`form_layout.rs:15-26`) with hardcoded `text_size(11.0)`, `h(rem_to_px(2.0))`, raw border — not real `Field` components, which violates the "no fakes" rule. Replace with real Field/TextInput once available; add Mixed + fieldErrors groups.

## Notes

- Biggest structural gap: `FormLayoutSpec` has no `fieldErrors` field, so the **FieldErrors anatomy part (contract §2) and `role="alert"` summary (contract §6, Parity Checklist Tier-1) are entirely absent from both Rust targets.** This is the dominant Tier-1 parity miss.
- Both Rust targets duplicate the Callout/FormActions primitives inline instead of composing them, despite both primitives existing (`primitives/callout.rs`, `primitives/form_actions.rs`, and Jetstream `callout.rs`, `form_actions.rs`). Composing them removes most hardcoded callout literals (mix ratios, padding, body size, border) in one move.
- Callout color-mix ratios diverge: Svelte uses 8% background / 40% border (`FormLayout.svelte:84-85`); both Rust targets hardcode 12% / 30%. Reconcile via the composed Callout primitive.
- GPUI adds a non-contract `title` prop; if intentional, promote to contract + Svelte, else drop. Svelte has no `title`.
- Responsive container-query breakpoints (600px/480px, contract §7) are Svelte-only; Rust targets use flex-wrap approximations. Contract §7/Tier-3 already grants grid-vs-flex freedom, but neither Rust target collapses to single-column at narrow widths — note as a visual-parity follow-up, not a hard bug.
