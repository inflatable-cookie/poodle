<!-- parity consv=gap gpui=4 jetstream=4 specimen=gap -->
# Parity: PickerShell

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/picker-shell.md`
- Svelte (authoritative): `packages/svelte/components/src/PickerShell.svelte`
- GPUI: `packages/gpui/components/src/composites/picker_shell.rs`
- Jetstream: `packages/jetstream/components/src/picker_shell.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/PickerShellSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/picker_shell_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/picker_shell.rs`

## Contract ↔ Svelte

Mostly aligned; two concrete divergences, both Svelte-vs-contract value mismatches.

- **Popover max-width mismatch.** Contract §7 and §8 say popover caps at `30rem` ("popover variant caps width at `30rem`", token table `max-width: 30rem`). Svelte sets `max-width: min(32rem, calc(100vw - 2rem))` (`PickerShell.svelte:131`). Svelte is authority → **fix: update contract to `32rem` (and document the `min(…, 100vw - 2rem)` viewport clamp + `width: 100%`).**
- **State fallback title default mismatch.** Contract §3 says when `stateTitle` is null it "falls back by state" (`"Loading results"`, `"Something went wrong"`, `"Nothing here yet"`, `"No results"`). Svelte's fallback is a single literal `"Picker state"` (`:101`) — it does **not** branch by state. The per-state fallbacks live only in the Rust `PickerShellSpec::effective_state_title()` (`packages/contracts/components/src/picker_shell.rs:139-151`). Svelte is authority for surface but here Svelte is **less capable than contract+spec**, and the spec already encodes the richer behavior. **Fix: implement per-state fallback titles/messages in Svelte to match `effective_state_title`/`effective_state_message`; the bare `"Picker state"` is a Svelte bug.** This is the `consv=gap` driver.
- `stateMessage` fallback: contract §3 lists per-state fallback messages; Svelte renders `stateMessage` only when explicitly passed (`:102-104`), no fallback. Same root issue as title — Svelte lacks the contract's per-state fallbacks. **Fix with the title fix above.**
- Props otherwise match contract §3 exactly (name/type/default): `title`, `description`, `variant`, `state`, `ariaLabel`, `resultCount`, `selectionCount`, `stateTitle`, `stateMessage`, `statusText`, `statusId`, and the five snippets. Anatomy, data-attributes, and a11y (`role="status"`, `aria-live`, `aria-atomic`, sr-only, `aria-hidden` spinner) all present in Svelte.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded modal/popover shadow + sizing — `hsla(0.0,0.0,0.0,0.12)`/`hsla(0.0,0.0,0.0,0.08)`, `blur_radius: px(24.0)`/`px(8.0)`, `offset px(8.0)`/`px(2.0)`, `max_w(px(480.0))` (`picker_shell.rs:165-177`). Contract §8 wants `box-shadow: var(--poodle-elevation-overlay)` (popover) / `var(--poodle-elevation-dialog)` (modal) and Svelte caps at `32rem` (≈512px, not 480). Resolve shadow from an elevation token and width from a size token.
- [ ] No variant distinction popover vs modal — GPUI keys off `spec.is_modal_like()` (`:162`) which is true for **both** Popover and Modal, applying one shared shadow + `max_w`. Contract §8 specifies different shadows (overlay vs dialog) and different backgrounds (modal uses `background.elevated`; popover keeps panel bg). GPUI never reads `background.elevated` and never branches the two. Branch on `PickerVariant`.
- [ ] `title_size` hardcoded `px(rem_to_px(1.25))` (`:144`) — contract §8 title is `1.25rem`; resolve via a typography token, not a raw rem literal. (`body_size`/`label_size`/spacing all correctly use `resolve_px`/`resolve_radius` — keep.)
- [ ] Status sr-only treatment is a `1px×1px overflow_hidden` box (`:239-247`) — functionally hidden but not the full sr-only clip pattern, and (accepted) carries no `role="status"`/`aria-live`. The `1px` literals are inherent to the hide pattern; acceptable. Body `max_h(menu_max_h)` adds a scroll cap not present in Svelte (Svelte relies on parent height + `overflow-y:auto`); confirm intended.
- accepted: no ARIA (gpui has no accessibility API) — status `role="status"`/`aria-live`/`aria-atomic` and section `aria-label` not emitted (`aria_label` stored on spec, unused).

## Jetstream gap (vs Svelte + contract)

- [ ] Title font hardcoded `text_size(20.0)` (`picker_shell.rs:52`) — contract §8 title `1.25rem`; resolve a typography token like GPUI's `title_size`, not a raw `20.0`. Also `text_weight(600)` literals at `:53` and `:142`.
- [ ] Border width hardcoded `.border(1.0)` (`:36`, `:123`) — contract border is `0.0625rem`; resolve from a token/`rem_to_px(0.0625)` rather than raw `1.0`px.
- [ ] No popover/modal variant treatment at all — `js_picker_shell` never reads `spec.variant` or `is_modal_like()`; no shadow, no `max-width` cap, no `background.elevated` for modal. Contract §8 popover/modal variants (`box-shadow`, `max-width: 32rem`, modal bg) are entirely unimplemented. GPUI at least applies a (hardcoded) shadow; Jetstream applies none.
- [ ] Status region not hidden correctly — Jetstream emits the status label with `.opacity(0.0)` (`:100-107`). It still occupies layout (no sr-only clip / zero-size), so it reserves vertical space in the grid while being invisible. Svelte uses absolute `1px` sr-only removal from flow. Collapse it from layout (or omit, since there is no a11y channel).
- accepted: no ARIA channel — `role="status"`/`aria-live`/`aria-atomic`, `aria-hidden` spinner, section `aria-label` not emitted. Snippet content (toolbar/selection/body/footer) is caller-supplied via fn params — consistent with Svelte snippets.

## Specimen parity

- Svelte covers: Inline ready (3 `Surface` candidates + resultCount 12), No-results (custom `stateTitle`/`stateMessage`), plus a **density ladder** (`showDensities`, with `selectionCount` 2). No toolbar/selection/footer snippets, no loading/error/empty, no popover/modal variants (`PickerShellSpecimen.svelte`).
- GPUI covers: Inline ready (with `TextInput` search + 5 result rows), Loading, No-results, Multiple selection (selected_count 2). **Richer than Svelte** on states. — missing: popover/modal variant demos, footer/selection-summary slots, density ladder. Result rows are hand-built `div`s (acceptable — they are caller content, not the shell).
- Jetstream covers: With content (search + `SelectionSummary` + body + footer with `Cancel`/`Confirm` buttons + `statusText`), Empty, Loading (custom message), Error (custom message). **Most complete slot coverage** (only target exercising toolbar+selection+footer). — missing: ready inline with `resultCount`/`description` header, no-results state, popover/modal variants.

## Notes

- The `consv=gap` is driven by Svelte's `effective_state_*` shortfall: Svelte renders a flat `"Picker state"` fallback while the contract and the shared `PickerShellSpec` (Rust) both define per-state fallback copy. Both Rust targets call `effective_state_title()`/`effective_state_message()` and are correct; Svelte is the laggard. Either port the per-state fallbacks into `PickerShell.svelte` or pass explicit `stateTitle`/`stateMessage` everywhere — the former matches the contract.
- Variant handling is the biggest cross-target gap: Svelte fully implements inline/popover/modal CSS; GPUI collapses popover+modal into one hardcoded shadow; Jetstream implements neither. Popover/modal are also absent from all three specimens, so the divergence is untested.
- Width target disagreement: contract `30rem`, Svelte `32rem`, GPUI `480px` (~30rem). Pick one (Svelte authority = 32rem) and align contract + both Rust targets.
