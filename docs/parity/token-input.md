<!-- parity consv=fixed gpui=9 jetstream=11 specimen=gap -->
# Parity: TokenInput

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/token-input.md`
- Svelte (authoritative): `packages/svelte/components/src/TokenInput.svelte`
- GPUI: `packages/gpui/components/src/primitives/token_input.rs`
- Jetstream: **ABSENT** — `packages/jetstream/components/src/token_input.rs` does not exist; no `js_token_input`; no jetstream specimen.
- Spec: `packages/contracts/components/src/token_input.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/TokenInputSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/token_input.rs` · jetstream **NONE**

## Contract ↔ Svelte

Svelte exposes props the contract does not document. Svelte is authoritative — update the contract.

- [x] FIXED Svelte adds `resolveToken?: (value, values) => string | null | undefined` (`TokenInput.svelte:29`) — a per-token validation/transform hook; returning non-string rejects the draft. Added to contract §3 props + §4 Commit Semantics.
- [x] FIXED Svelte adds `onTokenReject?: (value: string) => void` (`:31`) — fires when `resolveToken` rejects a draft. Added to contract §3 props + §6 callbacks.
- Everything else aligns: props (`id`/`values`/`name`/`placeholder`/`disabled`/`readOnly`/`required`/`spellcheck`/`autocapitalize`/`autocomplete`/`ariaLabel`/`describedBy`/`size`/`sizeRole`/`density`/`separators`/`dedupe`/`commitOnBlur`/`maxLength`/`onValuesChange`) match contract §3 names, types, and defaults. Anatomy parts (root, hidden inputs, token row, token, label, remove button, input control), commit/removal/dedupe/form semantics, and ARIA all match.
- Rust `TokenInputSpec` (`token_input.rs`) is **missing** `separators` usage, `dedupe`, `commit_on_blur`, `max_length`, `spellcheck`/`autocapitalize`/`autocomplete` as live fields — it has `separators`/`dedupe`/`commit_on_blur`/`max_length` in the struct but no `resolveToken`/`onTokenReject` analog and no interaction surface. Spec is structurally close; the contract-vs-Svelte divergence is the two undocumented Svelte props above.

## GPUI gap (vs Svelte + contract)

GPUI is a static stub — it renders pills + a placeholder string, not a working token entry. `[ ]` open.

- [ ] **No input control.** The draft `<input type="text">` (contract anatomy "Input Control", required) is faked as a plain `div` showing the placeholder text (`token_input.rs:56-60`). No editable field, no caret, no typing.
- [ ] **No Token Row part.** Tokens + control are dumped directly into one flex row; the contract's `.token-input__tokens` wrapping row with its own padding is absent.
- [ ] **No remove button.** Pills render without the per-token `.token-input__remove` clear affordance (contract anatomy + §4 removal semantics). No way to remove a token.
- [ ] **No hidden form inputs.** `name`-driven hidden `<input type="hidden">` payload slots (contract §2 + §4 Form Submission) not emitted.
- [ ] **No focus state.** Contract §5 `focus` (border/fill/shadow switch to focus treatment) + Svelte `:focus-within` not modeled; no focus ring, no focus fill.
- [ ] **No read-only state distinction.** `with_read_only` is accepted by the spec but `into_element` only branches on `disabled` for opacity (`:36-40`); read-only renders identically to default and the missing remove buttons mean read-only/disabled hiding is moot.
- [ ] **Unused spec props.** `separators`, `dedupe`, `commit_on_blur`, `max_length`, `required`, `name`, `aria_label`, `described_by`, `size_role` are never read by the builder. No commit/dedupe/separator logic exists.
- [ ] Hardcoded `py(rem_to_px(0.3125))` vertical padding (`:65`) — Svelte derives padding-block from `space.control.y` with per-size offsets; resolve from a token, not the `0.3125` literal.
- [ ] Hardcoded `gap(px(rem_to_px(0.375)))` (`:46`) — the contract gap is density-driven (`compact 0.25 / default 0.375 / comfortable 0.5 rem`); only the default is hit. Resolve from density.
- accepted: no ARIA (gpui has no accessibility API).

## Jetstream gap (vs Svelte + contract)

**Entire target missing.** No component file, no builder, no specimen.

- [ ] **Implement `js_token_input` from scratch — component absent** (`packages/jetstream/components/src/token_input.rs` does not exist; not registered in `lib.rs`). This is the single biggest gap across both audited components.
- [ ] Root field chrome: border, radius, fill, focus ring resolved from `interactive-subtle` treatment tokens.
- [ ] Token Row part: wrapping flex row with density-driven gap and size/density padding.
- [ ] Render committed tokens via `js_pill` (neutral/subtle) with wrapping long-value labels.
- [ ] Per-token remove button (`aria-label="Remove <token>"`), hidden in disabled/read-only.
- [ ] Live text input control (or its Jetstream equivalent) with placeholder, `maxLength`, draft state.
- [ ] Hidden form-input payload slots when `name` is set.
- [ ] Commit semantics: separator split, Enter/Tab commit, Backspace-removes-last, blur commit, trim, dedupe.
- [ ] States: default / focus / disabled (opacity + cursor) / read-only / empty / populated / wrapped / long-value.
- [ ] Size + density inheritance and the per-size padding/font-size offsets (xs/sm/lg/xl).
- [ ] Build the missing Jetstream specimen (`packages/jetstream/preview/src/specimens/token_input.rs`) covering all contract states; register it.
- accepted: no ARIA channel.

## Specimen parity

- Svelte covers: Default (+ Field + live JSON), Multiple separators, Narrow and long values, Read only, Disabled, Sizes snippet, Densities snippet (`TokenInputSpecimen.svelte`).
- GPUI covers: Default, long-value (sm), read-only (compact), disabled — but every one renders the static stub (no editing, no remove, no focus). — missing: multiple-separators, sizes group, densities group, **and all are non-functional** (no real input).
- Jetstream covers: **NONE** — no specimen file exists. Fully missing.

## Notes

- `consv=gap` driver: Svelte's `resolveToken` + `onTokenReject` props/callbacks are undocumented in the contract. Both belong in the contract per "Svelte is parity authority".
- GPUI count (9) reflects that the component is a placeholder mockup, not a real implementation — it shows pills and placeholder text but has no editable input, remove buttons, hidden inputs, focus state, or commit logic. Per project policy a non-resolving stub specimen is "worse than no specimen"; flag the GPUI specimen as misleading.
- Jetstream count (11) is the from-scratch implementation plus its specimen. This is the top-priority missing implementation in this audit pair.
