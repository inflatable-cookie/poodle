<!-- parity consv=ok gpui=4 jetstream=6 specimen=gap -->
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

- [ ] Hardcoded header padding `px(16.0)` / `px(12.0)` at `detail_shell.rs:104-105` and body padding `px(16.0)` / `px(12.0)` at `:131` — resolve from panel-padding tokens.
- [ ] Hardcoded state-region spacing `px(8.0)` (`detail_shell.rs:144`), `px(32.0)` (`:145`, `:165`, `:178`) — resolve from tokens.
- [ ] `stateTitle` / `stateMessage` not exposed — state messages are hardcoded English strings ("Detail state", "An error occurred…", "No content available."); pipe through the spec props.
- [ ] `scrollMode` not exposed — contract emits `data-scroll-mode`; GPUI has no equivalent. Add to spec + apply.
- accepted: no ARIA (gpui has no accessibility API). `header`/`children` slots present; `stateContent` is partial (hardcoded states instead of custom slot).

## Jetstream gap (vs Svelte + contract)

Barebones pass-through: `div().bg().flex_col().grow()` wrapping optional header + content. Most of the contract is unimplemented.

- [ ] No `state` handling — loading/empty/error states not rendered at all.
- [ ] No `stateContent` slot, no `stateTitle`/`stateMessage`.
- [ ] No `title` rendering (header text from `title` prop).
- [ ] No `scrollMode` handling.
- [ ] No `ariaLabel` (Jetstream emits no role either — note once).
- [ ] No internal structure — no separators, no token-resolved padding between header/body.
- accepted: interaction/state transitions would live in preview `main.rs` event loop, but the states themselves must render in the component first.

## Specimen parity

- Svelte covers: layout structure (4 region placeholders), multi-section with header (PageHeader + 3 DetailSections), loading (Spinner + message), error (custom stateTitle/stateMessage).
- GPUI covers: layout structure, multi-section with header, loading, error (hardcoded message), empty (hardcoded message). — missing: **custom stateTitle/stateMessage** (hardcoded instead).
- Jetstream covers: nothing beyond header/content pass-through — no state variants, no title, no slots demonstrated. — missing: **all state variants**, **title**, **header/content/stateContent demonstration**.

## Specimen note

GPUI actually exceeds Svelte on the empty-state demo but regresses by hardcoding the error message where Svelte passes custom text. Jetstream specimen is effectively a stub.

## Notes

- `consv=ok`: contract and Svelte match.
- Jetstream is the larger gap — it implements ~2 of the component's behaviors (header + body slots) and none of the state machine.
