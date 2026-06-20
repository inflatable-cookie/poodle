<!-- parity consv=ok gpui=6 jetstream=7 specimen=gap -->
# Parity: FormActions

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/form-actions.md`
- Svelte (authoritative): `packages/svelte/components/src/FormActions.svelte`
- GPUI: `packages/gpui/components/src/primitives/form_actions.rs`
- Jetstream: `packages/jetstream/components/src/form_actions.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/FormActionsSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/form_actions.rs` · jetstream `packages/jetstream/preview/src/specimens/form_actions.rs`

## Contract ↔ Svelte

Svelte matches the contract on every prop, anatomy part, alignment, density, and responsive rule. No divergence.

- Props align: `align` default `"end"`, `density` default `null` (inherits presentation), `showTopSeparation` default `true`, `showTopBorder` default `false`, `dangerItems` default `[]` — all match contract §3 (`FormActions.svelte:10-26`).
- Anatomy align: root + `__danger` inline snippet + `__danger-menu` overflow trigger all present (`FormActions.svelte:46-69`), matching contract §2.
- Alignment tokens align: `start`→`flex-start`, `end`→`flex-end`, `between`→`space-between` (`FormActions.svelte:107-117`) match contract §8 Root Alignment.
- Density align: `compact`/`default`/`comfortable` gap + top-padding + border-gap values (`FormActions.svelte:130-146`) match contract §8 Density + Divider Offset tables exactly.
- Responsive swap align: `@container (max-width: 31.25rem)` hides inline danger, shows menu only when both `danger` and `dangerItems` present (`FormActions.svelte:152-160`) — matches contract §8 Responsive Swap.
- ARIA align: collapsed danger trigger carries `ariaLabel="More actions"` (`FormActions.svelte:61,64`) — satisfies contract §6.
- Note (not a divergence): contract §8 lists `--poodle-form-actions-border-gap` default as `0.5rem` and the compact/comfortable density gap (`0.5rem`/`0.875rem`) + border-gap (`0.25rem`/`0.625rem`) as raw rem literals; Svelte uses the same raw rems (`FormActions.svelte:80,131-145`). Contract documents these as literal values, so this is contract-sanctioned, not a Svelte divergence.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded border-gap heuristic `separation.max(4.0) * 0.5` at `form_actions.rs:68` — contract §8 specifies density-keyed border-gap (`0.25`/`0.5`/`0.625rem`); resolve from a `border_gap_token()` per density, not a raw `4.0 * 0.5`.
- [ ] No `density` support — `FormActionsSpec` has no density field and GPUI never varies gap/padding/border-gap by density (`form_actions.rs:61-71`); contract §4 + §8 require compact/default/comfortable variants.
- [ ] No `showTopSeparation` builder — spec reads `shows_top_separation()` (`form_actions.rs:62`) but `FormActions` exposes no builder to set it (only `align`/`with_action` at `form_actions.rs:44-53`); preview cannot toggle footer-embedded mode.
- [ ] No `showTopBorder` builder — `shows_top_border()` is read (`form_actions.rs:67,88`) but no builder method sets it; bordered-separation state unreachable via the GPUI API.
- [ ] No `danger` snippet / `__danger` inline group — actions are a flat `Vec<AnyElement>` (`form_actions.rs:16,105-107`); contract §2 danger anatomy absent.
- [ ] No `dangerItems` overflow menu / responsive collapse — `__danger-menu` trigger and `@container` swap (contract §8) not implemented; no container-query equivalent.
- accepted: no ARIA (gpui has no accessibility API) — collapsed danger trigger `aria-label` (contract §6) cannot be emitted; moot until danger menu exists.

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded border-gap heuristic `separation.max(4.0) * 0.5` at `form_actions.rs:17` — same as GPUI; resolve density-keyed border-gap from a token per contract §8, not raw `4.0 * 0.5`.
- [ ] No `density` support — `js_form_actions` never varies gap/padding/border-gap by density (`form_actions.rs:10-20`); contract §4 + §8 require compact/default/comfortable.
- [ ] No `danger` snippet / `__danger` inline group — children are a flat `Vec<JsEl>` (`form_actions.rs:9,41-43`); contract §2 danger anatomy absent.
- [ ] No `dangerItems` overflow menu — `__danger-menu` trigger not rendered; contract §8 responsive swap absent.
- [ ] No responsive container-query swap — no `inline-size` container boundary or `max-width: 31.25rem` collapse (contract §8); narrow-container danger collapse unsupported.
- [ ] `FormActionAlign::Start` is a no-op arm (`form_actions.rs:36`) relying on default flex start — fine for `flex_row`, but confirm no `justify_start()` is needed for parity with explicit Svelte `flex-start`.
- [ ] No `showTopBorder` exercised in any caller path that varies density border-gap — `shows_top_border()` is read (`form_actions.rs:16,30`) but border-gap is the heuristic above, not the contract's density value.
- accepted: no ARIA channel for collapsed danger trigger `aria-label` (contract §6) — moot until danger menu exists.
- accepted: interaction (button click) lives in preview event loop, not the component.

## Specimen parity

- Svelte covers: End-aligned (default), Start-aligned, Space between, Responsive danger actions (with `dangerItems` + `danger` snippet, constrained to 20rem to show collapse), Bordered separation, Density ladder (compact/default/comfortable via `densities` snippet) (`FormActionsSpecimen.svelte`).
- GPUI covers: End-aligned, Start-aligned, Space between, Responsive danger actions (renders 3 buttons inline only — no actual overflow menu), Last-action click feedback. — missing: **Density ladder**, **Bordered separation** (`showTopBorder`), **Footer-embedded** (`showTopSeparation=false`), real **danger-collapse** overflow.
- Jetstream covers: End-aligned, Start-aligned, Between, Dialog footer custom actions (`showTopSeparation=false`). — missing: **Density ladder**, **Bordered separation** (`showTopBorder`), **Responsive danger actions** (no danger snippet / overflow menu).

## Notes

- consv=ok: Svelte is a faithful, complete implementation of the contract — every prop, anatomy part, density value, alignment, responsive rule, and ARIA requirement is present and matches.
- The dominant cross-target gap is **density** and **danger overflow**: `FormActionsSpec` (`packages/contracts/components/src/form_actions.rs`) has no `density` field and no danger-snippet / `dangerItems` model, so neither Rust target can implement contract §4 density states or the §8 responsive danger swap. Adding a `density` field + `border_gap_token()` (density-keyed) to the spec is the prerequisite for closing the first GPUI/Jetstream border-gap and density todos.
- Both Rust border-gap heuristics (`separation.max(4.0) * 0.5`) are the only hardcoded literals; there are zero hardcoded color literals or raw-px sizing literals in either component.
- The GPUI "Responsive danger actions" specimen and the Jetstream specimen both lack the real overflow-menu behavior; they render inline buttons only, so they demonstrate layout but not the danger-collapse contract state.
