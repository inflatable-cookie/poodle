<!-- parity consv=fixed gpui=2 jetstream=3 specimen=gap | pass: real TextInput draft control, token row, per-token remove button, read-only hides remove, token-resolved size/density padding+gap+font. Specimens: GPUI specimen rebuilt to full coverage (Default/Multiple-separators/Empty/Max-length/Long/Read-only/Disabled + Sizes/Densities tabs) + builds clean; Jetstream specimen CREATED + registered (pub mod + dispatch + PRIMITIVES entry) with real js_token_input across all states, but build BLOCKED by external jetstream-renderer wgpu break — flag stays gap until Jetstream preview builds clean. -->
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

Now a real token entry surface (pills + remove buttons + live `TextInput` draft), not a static stub.

- [x] FIXED — **input control**: the draft is a real `TextInput::from_spec` (id, placeholder-when-empty, maxLength, aria-label, disabled/read-only, size/density all forwarded), grown into the wrap row. No faked placeholder div.
- [x] FIXED — **Token Row part**: committed pills + draft live in a dedicated `.token-input__tokens`-equivalent wrap row with its own size/density padding (`flex_wrap`, density gap, `pad_x`/`pad_y`).
- [x] FIXED — **remove button**: each committed token is a `Pill::removable(true)` (renders the `x` icon); an optional `on_remove(index, …)` handler wires removal. Omitted in disabled/read-only mode.
- [x] FIXED — **read-only state**: `can_edit()` gates the remove affordance and the draft is `read_only`; read-only renders distinctly (no remove buttons, non-editable draft) vs default.
- [x] FIXED — **forwarded spec props**: `name`, `max_length`, `aria_label`, `described_by`, `id`, `size`/`size_role`/`density`, `disabled`, `read_only` now reach the draft `TextInput`.
- [x] FIXED — vertical padding resolves from `space.control.y` + `token_input_pad_y_offset_rem(size)`; horizontal from `control.x` (density) + `token_input_pad_x_offset_rem(size)`. Gap from `token_input_gap_rem(density)`. No `0.3125`/`0.375` literals.
- [ ] **No hidden form inputs.** `name`-driven hidden `<input type="hidden">` payload slots (contract §2 + §4 Form Submission) — the name is forwarded to the draft, but GPUI has no DOM/form to emit hidden payload slots. Accepted (no-DOM channel).
- [ ] **No focus state paint.** Contract §5 `focus` (border/fill/shadow → focus treatment). Preview-loop: GPUI `:focus-within` is a runtime state owned by the consumer; the field renders its resting chrome. (Separator/dedupe/commit parsing is likewise consumer-owned draft logic.)
- accepted: no ARIA (gpui has no accessibility API).

## Jetstream gap (vs Svelte + contract)

**Entire target missing.** No component file, no builder, no specimen.

- [x] DONE: `js_token_input` field chrome (border `border.subtle`, `radius.control`, surface-mix fill, disabled opacity), wrapping token row, committed tokens via `js_pill` (neutral/subtle). Probe-tested.
- [x] FIXED — per-token remove `×`: each token is a flex-row chip (pill label + id-tagged `poodle-token-remove-{i}` button); omitted in disabled/read-only mode (removal semantics). Probe-tested.
- [x] FIXED — live text-input draft via `js_text_input` (placeholder-when-empty, `maxLength`, aria-label, id, disabled/read-only forwarded), grown into the wrap row.
- [x] FIXED — size + density inheritance: padding-block from `space.control.y` + `token_input_pad_y_offset_rem(size)`, padding-inline from `control.x` (density) + `token_input_pad_x_offset_rem(size)`, wrap gap from `token_input_gap_rem(density)`, token/draft font from `token_input_font_rem(size)`. Pills track field size.
- [x] FIXED — states: disabled (opacity), read-only (hides remove, non-editable draft), empty (placeholder only), populated (pills before draft). Probe-tested.
- [ ] Hidden form-input payload slots when `name` is set — no DOM/form channel in Jetstream. Accepted.
- [ ] Commit semantics: separator split, Enter/Tab commit, Backspace-removes-last, blur commit, trim, dedupe. Preview-loop: draft editing/commit + remove-click handling live in the preview event loop (the `poodle-token-remove-*` ids are emitted as hit targets).
- [x] DONE: Jetstream specimen (`packages/jetstream/preview/src/specimens/token_input.rs`) built covering all contract states (Default/Multiple-separators/Empty/Max-length/Long/Read-only/Disabled + Sizes/Densities) and registered (`pub mod`, `"token-input"` dispatch arm, PRIMITIVES registry entry). Build not yet re-verified — external `jetstream-renderer` wgpu break blocks the preview link (see status line).
- accepted: no ARIA channel.

## Specimen parity

- Svelte covers: Default (+ Field + live JSON), Multiple separators, Narrow and long values, Read only, Disabled, Sizes snippet, Densities snippet (`TokenInputSpecimen.svelte`).
- GPUI covers (rewritten): Default (populated + id/name), Multiple separators, Empty (placeholder only), Max length (8), Narrow + long values, Read only, Disabled, plus Sizes (xs–xl) and Densities tabs via `specimen_layout`. Every example is the real `TokenInput` (real `Pill` chips + `x` remove + real `TextInput` draft). Builds clean. Commit/remove wiring stays consumer-owned (preview event loop).
- Jetstream covers (created + registered): new `specimens/token_input.rs` — Default, Multiple separators, Empty, Max length, Narrow + long values, Read only, Disabled, Sizes (xs–xl), Densities. Every example is the real `js_token_input` (real `js_pill` chips + `×` remove + real `js_text_input` draft); registered via `pub mod token_input`, the `"token-input"` dispatch arm, and a PRIMITIVES registry entry. **Build NOT verified** — external `jetstream-renderer`/`jetstream-platform` (sibling repo) fails to compile against a newer `wgpu`; every error is under `/Dev/projects/jetstream/crates/`, none Poodle-side. Re-verify once the sibling finishes its wgpu migration.

## Notes

- `consv=gap` driver: Svelte's `resolveToken` + `onTokenReject` props/callbacks are undocumented in the contract. Both belong in the contract per "Svelte is parity authority".
- GPUI count (9) reflects that the component is a placeholder mockup, not a real implementation — it shows pills and placeholder text but has no editable input, remove buttons, hidden inputs, focus state, or commit logic. Per project policy a non-resolving stub specimen is "worse than no specimen"; flag the GPUI specimen as misleading.
- Jetstream count (11) is the from-scratch implementation plus its specimen. This is the top-priority missing implementation in this audit pair.
