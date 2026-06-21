<!-- parity consv=ok gpui=0 jetstream=1 specimen=ok pass=41 -->
# Parity: DetailShell

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/detail-shell.md`
- Svelte (authoritative): `packages/svelte/components/src/DetailShell.svelte`
- GPUI: `packages/gpui/components/src/composites/detail_shell.rs`
- Jetstream: `packages/jetstream/components/src/detail_shell.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/DetailShellSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/detail_shell.rs` · jetstream `packages/jetstream/preview/src/specimens/detail_shell.rs`

## Contract ↔ Svelte

Perfect parity. All props match 1:1: `title` (`null`), `scrollMode` (`"body"`), `state` (`"ready"`), `ariaLabel` (`null`), `stateTitle` (`null`), `stateMessage` (`null`), plus `header`/`stateContent`/`children` snippets. `state` enum (`ready`/`empty`/`loading`/`error`) resolves from `Exclude<BrowseState, "no-results">`. No divergence.

## GPUI gap (vs Svelte + contract)

State strings are hardcoded; several props unexposed; padding uses raw px.

- [x] FIXED Header/body padding now resolves from `space.panel.x`/`space.panel.y`; the root + region stacking gap is `space.stack.lg`. No raw px.
- [x] FIXED State-region spacing now token-resolved: doubled `space.panel.y` / 1.5× `space.panel.x` padding, `space.stack.lg` inner gap, `space.stack.md` spinner→copy gap.
- [x] FIXED `state_title`/`state_message` added to `DetailShellSpec` and piped through; the default heading falls back to "Detail state" (`effective_state_title()`). Hardcoded English strings removed.
- [x] FIXED `scrollMode` exposed via `scroll_owner` → `scroll_mode_value()` and applied: `Shell` scrolls the shell, `Content` scrolls the body region (`overflow_y_scroll`).
- accepted: no ARIA (gpui has no accessibility API). Now supports a real `with_state_content` custom slot in addition to the default state copy, and the state region renders its subtle surface + `radius-surface` corners.

## Jetstream gap (vs Svelte + contract)

Barebones pass-through: `div().bg().flex_col().grow()` wrapping optional header + content. Most of the contract is unimplemented.

- [x] FIXED Full `state` machine: ready renders the body slot; empty/loading/error render the state region instead.
- [x] FIXED `state_content` custom slot added (5th param) plus `state_title`/`state_message` (default "Detail state" fallback).
- [x] FIXED `title` now rendered as a heading-sized label in the header region.
- [x] FIXED `scrollMode` handled — `Shell` scrolls the shell, `Content` scrolls the body region (`overflow_scroll`).
- [ ] JsEl gap: no role/aria primitive, so `ariaLabel` is not emitted. (Noted — accepted runtime limit.)
- [x] FIXED Token-resolved internal structure: header/body padding from panel tokens, `space.stack.lg` region gap, state region with `color-mix` surface, doubled padding, and `radius-surface` corners. Loading prepends the shared grid `Spinner`.
- accepted: interaction/state transitions would live in preview `main.rs` event loop, but the states themselves now render in the component.

## Specimen parity

- Svelte covers: layout structure (4 region placeholders), multi-section with header (PageHeader + 3 DetailSections), loading (Spinner + message), error (custom stateTitle/stateMessage).
- GPUI covers: layout structure, multi-section with header, loading, error (now custom `stateTitle`/`stateMessage`). — parity OK.
- Jetstream covers: header+content, loading, error (custom title+message), empty shell. — parity OK.

## Specimen note

GPUI error specimen now passes custom `stateTitle`/`stateMessage` (matches Svelte). Jetstream specimen demonstrates header+content, loading, error (custom title+message), and empty shell.

## Notes

- `consv=ok`: contract and Svelte match.
- Pass 41: both targets now implement the full state machine. Added additive `DetailShellSpec` fields `state_title`/`state_message` plus helpers (`scroll_mode_value`, `effective_state_title`) and state-region token methods (stack gap, surface fill + mix, radius, padding, border, message/title colors). GPUI: token-resolved padding, state region with `color-mix` surface + `radius-surface`, custom `with_state_content` slot, scroll-owner overflow. Jetstream: full header/body/state rendering, grid spinner on loading, custom state slot, scroll-owner overflow; `js_detail_shell` gained a 5th `state_content` param (specimen updated). Probe tests cover ready header+body, non-ready hides body, custom error title+message, loading spinner, custom state-content override. Lone open todo is the Jetstream `ariaLabel` (no JsEl role primitive).
