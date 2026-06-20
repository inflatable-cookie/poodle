<!-- parity consv=gap gpui=8 jetstream=9 specimen=gap -->
# Parity: ValidationSummary

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/validation-summary.md`
- Svelte (authoritative): **MISSING** — no `packages/svelte/components/src/ValidationSummary.svelte` (contract header states "Svelte component not yet built").
- GPUI: **MISSING** — no `validation_summary.rs` in `packages/gpui/components/src/{composites,primitives}/`; not exported from `packages/gpui/components/src/lib.rs`.
- Jetstream: `packages/jetstream/components/src/validation_summary.rs`
- Spec: `packages/contracts/components/src/validation_summary.rs` · entry/announce helpers `packages/contracts/components/src/composite_types.rs`
- Specimens: svelte **MISSING** · gpui **MISSING** · jetstream `packages/jetstream/preview/src/specimens/validation_summary.rs`

## Contract ↔ Svelte

The authoritative reference does not exist. The contract self-declares this (`validation-summary.md:3`). With no Svelte to arbitrate, the contract stands as the sole spec, and the Jetstream impl diverges from it in several places (see below) with no reference to confirm intent. `consv=gap` because the authoritative side is absent.

- **Svelte component missing.** No `ValidationSummary.svelte`; not exported from `packages/svelte/components/src/index.ts` (only `ValidationState` family is). **Fix: build the Svelte component to lock the visual reference; until then every other target is unverifiable.**
- **Svelte specimen missing.** No `ValidationSummarySpecimen.svelte`. **Fix: add once the component exists.**
- Contract gaps that a Svelte build must resolve (currently under-specified, so Jetstream invented answers):
  - Anatomy (§2) lists Root/Title/List/Entry with Entry = `<a href>` Label + `<span>` Message. It does **not** define a per-entry bullet/indicator dot, nor a "blocking issue count" footer line — Jetstream added both. **Fix: contract must either adopt or forbid the indicator dot and the count footer.**
  - Token Usage (§6) defines **only** `border_token()`. No tokens for padding, gap, radius, font-size, item-spacing, or indicator size. The contract cannot be implemented without hardcoding. **Fix: extend §6 with space/radius/typography token targets (the missing spec methods drive every Jetstream violation below).**

## GPUI gap (vs Svelte + contract)

GPUI has **no implementation at all** — this is the dominant gap.

- [ ] Create `packages/gpui/components/src/composites/validation_summary.rs` (`composites` layer per contract §1); component is entirely absent.
- [ ] Export it from `packages/gpui/components/src/composites/mod.rs` and re-export via `packages/gpui/components/src/lib.rs` (currently only `ValidationState` is imported, `lib.rs:20`).
- [ ] Implement anatomy: Root surface + optional Title + List + Entry rows (contract §2).
- [ ] Resolve border from `spec.border_token()` (danger vs accent by blocking count) — contract §6.
- [ ] Render `active_entries()` and handle empty → render nothing (contract §4 empty state).
- [ ] Emulate href focus-jump with imperative GPUI focus calls (contract §8 known delta — web-only href).
- [ ] Add a GPUI specimen `packages/gpui/preview/src/specimens/validation_summary.rs` + register it.
- [ ] Resolve all spacing/size/typography from tokens (blocked on contract §6 gaining those token targets — see Notes).
- accepted: no ARIA / aria-live (gpui has no accessibility API) — contract §8 known delta.

## Jetstream gap (vs Svelte + contract)

Spec exposes only `border_token()` — there is **no token method** for any spacing, sizing, radius, or typography value, so every dimension below is a hardcoded `rem_to_px(LITERAL)` violation with no token to resolve from. Per CLAUDE.md these are "always wrong"; the fix requires adding tokens + spec methods, not just swapping call sites.

- [ ] Hardcoded font size `rem_to_px(0.8125)` at `validation_summary.rs:16` — no `title_font_size_token()`.
- [ ] Hardcoded small/body size `rem_to_px(0.75)` at `validation_summary.rs:17` — no entry-text size token.
- [ ] Hardcoded padding `rem_to_px(0.75)` / `rem_to_px(0.5)` at `validation_summary.rs:18-19` (applied `:26`) — no `padding_x/padding_y_token()`.
- [ ] Hardcoded gaps `rem_to_px(0.5)` and `rem_to_px(0.375)` at `validation_summary.rs:20-21` (applied `:27`, `:45`) — no list-gap / item-gap token.
- [ ] Hardcoded radius `rem_to_px(0.375)` at `validation_summary.rs:25` — no `radius_token()`.
- [ ] Hardcoded indicator-dot size/radius `rem_to_px(0.375)` ×2 and `rem_to_px(0.1875)` at `validation_summary.rs:50-51` — and the dot itself is **not in contract anatomy** (§2).
- [ ] Hardcoded inner text-column gap `rem_to_px(0.125)` at `validation_summary.rs:56` — no token.
- [ ] Anatomy mismatch: uses `div()`/`label()` for Root/Entry; contract §2/§5 require `<aside>` root, `<ul>`/`<li>` list semantics, and an `<a href="#field-id">` label per entry. No anchor → no focus-jump (contract §5, §8). `js_validation_summary` never emits a link.
- [ ] Extra "blocking issue count" footer (`validation_summary.rs:73-80`) — not in contract anatomy; either delete or get it adopted into the contract.
- accepted: no ARIA channel — `accessibility_role()` exists on spec but comment at `validation_summary.rs:29` defers role emission to host runtime (contract §8 known delta).
- accepted: focus-jump is web-href in contract; Jetstream must emulate imperatively (contract §8) — currently does neither (counted as the anatomy todo above).

## Specimen parity

- Svelte covers: **nothing** — no specimen exists.
- GPUI covers: **nothing** — no specimen exists.
- Jetstream covers: "With errors" (2 Invalid + 1 Pending, `includePending=true`, with title) and "Clean (no entries)" (`validation_summary.rs` specimen :26-40). — missing: **pending-only / accent border** state, **mixed** state, **assertive vs polite** announce-mode variants, and a **blocking-only without pending** case (contract §4 lists 6 states; specimen shows 2).

## Notes

- Root cause of the Jetstream token violations is upstream: `ValidationSummarySpec` (`packages/contracts/components/src/validation_summary.rs`) defines only `border_token()`. Fixing the call sites is impossible until the spec gains space/size/radius/typography token methods and the contract §6 documents them. Sequence: contract §6 → spec methods → Jetstream resolves → GPUI built against the same methods.
- The whole component is blocked on the authoritative Svelte build. Until `ValidationSummary.svelte` exists, the Jetstream indicator-dot and count-footer additions are unreviewable — they may be correct enrichment or contract drift.
- `is_blocking()` is currently synonymous with `Invalid` (contract §3 derived helpers; `composite_types.rs:117`); pending entries are non-blocking, so a pending-only summary should render the accent border — untested by any specimen.
- Contract↔Svelte reconciliation pass: **Svelte authority missing** — no `ValidationSummary.svelte` exists, and the contract header self-declares "Svelte component not yet built." Per the reconciliation rules, the contract is the sole authority here and was left unchanged this pass; `consv` stays `gap` until the authoritative Svelte component is built to arbitrate the open Jetstream/contract divergences (indicator dot, count footer, missing §6 token targets).
