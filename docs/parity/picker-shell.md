<!-- parity consv=gap gpui=0 jetstream=0 specimen=gap -->
<!-- gpui-specimen pass: GPUI specimen complete — added Popover variant and Modal variant groups
     (each: header title+description, search TextInput, body option rows, FormActions footer with
     Cancel/Add), plus Error and Empty state groups, joining existing inline-ready / loading /
     no-results / multiple-selection groups. Real PickerShell + TextInput + FormActions/Button only;
     no hand-rolled surfaces. Covers contract variants (inline/popover/modal) and all five states.
     gpui/preview builds clean. GPUI specimen done; Jetstream pending engine recovery — specimen=gap kept. -->
<!-- pass: both targets now branch popover vs modal (overlay/dialog elevation, 32rem popover width cap, modal elevated bg via color-mix); Jetstream title→rem_to_px(1.25) + border→rem_to_px(0.0625) + sr-only status clipped to 1×1; PickerShellSpec gained is_popover()/is_modal()/popover_max_width_rem(). consv=gap unchanged (Svelte-side effective_state_* shortfall, not a Rust target). -->
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

- FIXED — **Popover max-width mismatch.** Contract §7 + §8 said popover caps at `30rem`. Svelte is `width: 100%` + `max-width: min(32rem, calc(100vw - 2rem))` (`PickerShell.svelte:130-131`). Contract §8 popover table now `width: 100%` / `max-width: min(32rem, calc(100vw - 2rem))`; §7 prose updated to the 32rem + viewport-clamp wording.
- LEFT (Svelte-side gap) — **State fallback title default mismatch.** Contract §3/§4 require per-state fallbacks (`"Loading results"`, `"Something went wrong"`, `"Nothing here yet"`, `"No results"`). Svelte renders a single flat `"Picker state"` literal (`:101`) and no message fallback. Per "never weaken a contract's intended capability just because Svelte hasn't shipped it", the contract is **left unweakened**; §9 Svelte Notes now flags this as a known Svelte gap to fix in `PickerShell.svelte` (the shared Rust `PickerShellSpec::effective_state_title()`/`effective_state_message()` already implements it). This is the remaining `consv=gap` driver.
- LEFT (Svelte-side gap) — `stateMessage` fallback: same root issue as the title; Svelte has no per-state message fallback. Contract retained; folded into the §9 note above.
- Props otherwise match contract §3 exactly (name/type/default): `title`, `description`, `variant`, `state`, `ariaLabel`, `resultCount`, `selectionCount`, `stateTitle`, `stateMessage`, `statusText`, `statusId`, and the five snippets. Anatomy, data-attributes, and a11y (`role="status"`, `aria-live`, `aria-atomic`, sr-only, `aria-hidden` spinner) all present in Svelte.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] FIXED Hardcoded modal/popover shadow + sizing — now resolves `elevation_overlay_shadow()` (popover) / `elevation_dialog_shadow()` (modal) from the typed elevation tokens, and the popover width caps at `rem_to_px(spec.popover_max_width_rem())` = `32rem` (Svelte authority), not the old `480px`. No raw `hsla`/`px` shadow literals remain.
- [x] FIXED Variant distinction popover vs modal — now branches on `spec.is_popover()` / `spec.is_modal()`: popover applies overlay elevation + `w_full` + 32rem cap; modal applies dialog elevation + the elevated background (`color_mix(elevated, transparent, 0.96)`, contract §8 modal). Inline/popover keep the panel background (`color_mix(panel, transparent, 0.94)`).
- accepted: `title_size` is `px(rem_to_px(1.25))` — contract §8 title is `1.25rem` and **no typography token resolves to 1.25rem** (`typography.heading.size` = 1rem). Per the parity rule, `rem_to_px(<contract-exact rem>)` is not a violation. Token gap noted: a `typography.title.size` (or similar) @ 1.25rem would let this resolve from a token; until then the contract-exact rem is correct.
- accepted: Status sr-only is a `1px×1px overflow_hidden` box — functionally hidden; the `1px` literals are inherent to the hide pattern. Body `max_h(menu_max_h)` scroll cap is intentional (gives the scrollable body a bound the GPUI window can't otherwise supply); left as-is.
- accepted: no ARIA (gpui has no accessibility API) — status `role="status"`/`aria-live`/`aria-atomic` and section `aria-label` not emitted (`aria_label` stored on spec, unused).

## Jetstream gap (vs Svelte + contract)

- [x] FIXED Title font — now `text_size(rem_to_px(1.25))` (contract §8 title `1.25rem`), not a raw `20.0`. `text_weight(600)` retained at `:title` and `:state` — accepted: no Jetstream weight-resolution helper exists in `theme_ext`, and the contract title spec lists no weight token; the `600` mirrors `typography.heading.weight` but is not resolved (token gap noted below).
- [x] FIXED Border width — both shell + state borders now `border(rem_to_px(0.0625))` (contract `0.0625rem`), not raw `1.0`px.
- [x] FIXED Popover/modal variant treatment — `js_picker_shell` now branches `spec.is_popover()` / `spec.is_modal()`: popover applies `shadow_md()` + `w_full` + `max_w(rem_to_px(32))`; modal applies `shadow_lg()` + elevated background (`color_mix(elevated, ZERO, 0.96)`). Inline/popover keep the panel background (`color_mix(panel, ZERO, 0.94)`). DELTA: the JsEl runtime exposes only preset shadows (`shadow_sm/md/lg`), not token-resolved elevation — so `overlay→shadow_md`, `dialog→shadow_lg` approximate the contract elevation tokens; offset/blur/color are runtime presets, not resolved `elevation.*` values.
- [x] FIXED Status region — now wrapped in a `1×1 overflow_hidden` div (mirrors the contract sr-only clip + GPUI's 1px box), so it claims no layout space, instead of the old `.opacity(0.0)` label that still reserved vertical space. (Visual collapse only — Jetstream has no SR live-region channel.)
- accepted: no ARIA channel — `role="status"`/`aria-live`/`aria-atomic`, `aria-hidden` spinner, section `aria-label` not emitted. Snippet content (toolbar/selection/body/footer) is caller-supplied via fn params — consistent with Svelte snippets.
- token gaps: (1) no `typography.title.size` @ 1.25rem token (both targets use contract-exact `rem_to_px(1.25)`); (2) no Jetstream font-weight resolver, so the `600` heading weight is a literal mirroring `typography.heading.weight`. Both are noted, not blockers.

## Specimen parity

- Svelte covers: Inline ready (3 `Surface` candidates + resultCount 12), No-results (custom `stateTitle`/`stateMessage`), plus a **density ladder** (`showDensities`, with `selectionCount` 2). No toolbar/selection/footer snippets, no loading/error/empty, no popover/modal variants (`PickerShellSpecimen.svelte`).
- GPUI covers: Inline ready (with `TextInput` search + 5 result rows), Loading, No-results, Multiple selection (selected_count 2). **Richer than Svelte** on states. — missing: popover/modal variant demos, footer/selection-summary slots, density ladder. Result rows are hand-built `div`s (acceptable — they are caller content, not the shell).
- Jetstream covers: With content (search + `SelectionSummary` + body + footer with `Cancel`/`Confirm` buttons + `statusText`), Empty, Loading (custom message), Error (custom message). **Most complete slot coverage** (only target exercising toolbar+selection+footer). — missing: ready inline with `resultCount`/`description` header, no-results state, popover/modal variants.

## Notes

- The remaining `consv=gap` is driven by Svelte's `effective_state_*` shortfall: Svelte renders a flat `"Picker state"` fallback while the contract and the shared `PickerShellSpec` (Rust) both define per-state fallback copy. Both Rust targets call `effective_state_title()`/`effective_state_message()` and are correct; Svelte is the laggard. The fix is on the **Svelte** side (port the per-state fallbacks into `PickerShell.svelte`) — the contract is correct and deliberately not weakened, so the gap cannot close by editing the contract. The popover max-width mismatch (the other former driver) is fixed.
- Variant handling is now implemented on both Rust targets: GPUI branches popover (overlay elevation + 32rem cap) vs modal (dialog elevation + elevated bg); Jetstream does the same with preset-shadow approximations. Svelte already implemented all three. Popover/modal are still absent from all three **specimens**, so the variants are now covered by probe tests (Jetstream `popover_variant_caps_width_at_32rem`, `modal_variant_uses_elevated_background`) but not yet exercised in a preview — preview-loop verification deferred (specimens unchanged this pass).
- Width target reconciled: contract §7/§8 + Svelte + both Rust targets now agree on `32rem` (Svelte authority). GPUI's old `480px` literal is replaced by `rem_to_px(spec.popover_max_width_rem())` = 512px; Jetstream matches.
